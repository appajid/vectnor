//! Recent-file artwork, read and rendered on one desktop worker without opening a document.

use std::collections::BTreeMap;
use std::sync::mpsc::{Receiver, SyncSender, channel, sync_channel};

use egui::{ColorImage, Context, TextureHandle, TextureOptions};
use vectnor_engine::file_access::ScopeHandle;

use crate::{Services, SharedReadFn};

const MAX_BYTES: u64 = 32 * 1024 * 1024;
const WIDTH: u32 = 328;
const HEIGHT: u32 = 216;

struct Task {
    path: String,
    id: u64,
    read: SharedReadFn,
    scope: ScopeHandle,
    ctx: Context,
}

struct Done {
    path: String,
    id: u64,
    image: Option<ColorImage>,
}

#[derive(Default)]
struct Entry {
    id: u64,
    image: Option<ColorImage>,
    texture: Option<TextureHandle>,
}

#[derive(Default)]
pub(crate) struct Thumbnails {
    entries: BTreeMap<String, Entry>,
    worker: Option<(SyncSender<Task>, Receiver<Done>)>,
    next_id: u64,
    context: u64,
}

impl Thumbnails {
    /// Browser hosts cannot reread filesystem paths; remember the document just opened/saved.
    pub fn remember(&mut self, path: &str, doc: &vectnor_doc::Document) {
        // Optional previews must not turn a successful Open or Save into a failure.
        let image = vectnor_engine::guard::catch_panic(|| render(doc)).ok().flatten();
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        self.entries.insert(path.into(), Entry { id, image, texture: None });
    }

    pub fn invalidate(&mut self, path: &str) {
        self.entries.remove(path);
    }

    pub fn retain(&mut self, paths: &[String]) {
        self.entries.retain(|path, _| paths.contains(path));
    }

    fn worker(&mut self) -> Option<&SyncSender<Task>> {
        if cfg!(target_arch = "wasm32") {
            return None;
        }
        if self.worker.is_none() {
            let (tx, tasks) = sync_channel::<Task>(6);
            let (done_tx, done) = channel();
            std::thread::Builder::new()
                .name("vectnor-thumbnails".into())
                .spawn(move || {
                    while let Ok(task) = tasks.recv() {
                        // An unavailable/corrupt preview is decorative: retain a clickable card.
                        let image = task.scope.run(|| {
                            vectnor_engine::guard::catch_panic(|| {
                                vectnor_engine::file_access::check_read(&task.path)?;
                                let bytes = (task.read)(&task.path, MAX_BYTES)?;
                                Ok::<_, String>(preview(&task.path, &bytes))
                            })
                            .ok()
                            .and_then(Result::ok)
                            .flatten()
                        });
                        if done_tx.send(Done { path: task.path, id: task.id, image }).is_err() {
                            break;
                        }
                        task.ctx.request_repaint();
                    }
                })
                .ok()?;
            self.worker = Some((tx, done));
        }
        self.worker.as_ref().map(|(tx, _)| tx)
    }

    pub fn texture(&mut self, ctx: &Context, services: &Services, path: &str) -> Option<TextureHandle> {
        let tag = crate::graphics::context_tag(ctx);
        if self.context != tag {
            self.context = tag;
            for entry in self.entries.values_mut() {
                entry.texture = None;
            }
        }
        if let Some((_, done)) = &self.worker {
            for result in done.try_iter() {
                if let Some(entry) = self.entries.get_mut(&result.path)
                    && entry.id == result.id
                {
                    entry.image = result.image;
                }
            }
        }
        if !self.entries.contains_key(path) {
            let id = self.next_id;
            self.next_id = self.next_id.wrapping_add(1);
            self.entries.insert(path.into(), Entry { id, ..Default::default() });
            let queued = services.read_shared.clone().is_some_and(|read| {
                self.worker()
                    .is_some_and(|tx| tx.try_send(Task { path: path.into(), id, read, scope: ScopeHandle::capture(), ctx: ctx.clone() }).is_ok())
            });
            if !queued && services.read_shared.is_none() {
                // Hosts without worker-capable reads (web and in-memory tests) use their reader.
                let image = vectnor_engine::file_access::check_read(path)
                    .ok()
                    .and_then(|()| services.read.as_ref()?.as_ref()(path).ok())
                    .and_then(|bytes| preview(path, &bytes));
                if let Some(entry) = self.entries.get_mut(path) {
                    entry.image = image;
                }
            }
        }
        let entry = self.entries.get_mut(path)?;
        if entry.texture.is_none()
            && let Some(image) = &entry.image
        {
            entry.texture = Some(ctx.load_texture(format!("recent-{path}"), image.clone(), TextureOptions::LINEAR));
        }
        entry.texture.clone()
    }
}

fn decode(bytes: &[u8]) -> Result<ColorImage, String> {
    let mut reader = image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format().map_err(|e| e.to_string())?;
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(4096);
    limits.max_image_height = Some(4096);
    limits.max_alloc = Some(MAX_BYTES);
    reader.limits(limits);
    let image = reader.decode().map_err(|e| e.to_string())?;
    let image = if image.width() > WIDTH || image.height() > HEIGHT { image.thumbnail(WIDTH, HEIGHT) } else { image }.to_rgba8();
    Ok(ColorImage::from_rgba_unmultiplied([image.width() as usize, image.height() as usize], image.as_raw()))
}

/// A decorative preview may be absent; document Open remains the source of user-facing errors.
fn preview(path: &str, bytes: &[u8]) -> Option<ColorImage> {
    if bytes.len() as u64 > MAX_BYTES {
        return None;
    }
    if let Some(png) = vectnor_format::preview(bytes)
        && let Ok(image) = decode(&png)
    {
        return Some(image);
    }
    let doc = vectnor_engine::cmd::fileio::load(path, bytes).ok()?.doc;
    render(&doc)
}

fn render(doc: &vectnor_doc::Document) -> Option<ColorImage> {
    let region = doc.artboards.first().map(|board| board.rect).or_else(|| doc.art_bounds())?;
    if !region.width().is_finite() || !region.height().is_finite() || region.width() <= 0.0 || region.height() <= 0.0 {
        return None;
    }
    let scale = (f64::from(WIDTH) / region.width()).min(f64::from(HEIGHT) / region.height());
    let image = vectnor_render::Renderer::new().render_region(doc, region, scale, true);
    Some(ColorImage::from_rgba_premultiplied([image.width as usize, image.height as usize], &image.pixels))
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLUE: &[u8] =
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><rect width="100" height="100" fill="#0080ff"/></svg>"##;

    #[test]
    fn artwork_is_rendered_without_opening_a_document() {
        let image = preview("art.svg", BLUE).unwrap();
        assert_eq!(image.size, [216, 216]);
        assert_eq!(image.pixels[108 * 216 + 108], egui::Color32::from_rgb(0, 128, 255));
        assert!(preview("broken.vcnr", b"not a document").is_none());
    }

    #[test]
    fn native_preview_is_used_when_present() {
        let doc = vectnor_engine::cmd::fileio::load("art.svg", BLUE).unwrap().doc;
        let png = vectnor_render::Rendered { width: 2, height: 1, pixels: vec![255, 0, 0, 255, 255, 0, 0, 255] }.to_png().unwrap();
        let bytes = vectnor_format::save_with(&doc, &vectnor_format::SaveOptions { preview: Some(png), ..Default::default() }).unwrap();
        let image = preview("art.vcnr", &bytes).unwrap();
        assert_eq!(image.size, [2, 1]);
        assert!(image.pixels.iter().all(|pixel| *pixel == egui::Color32::RED));
    }

    #[test]
    fn remembered_previews_survive_a_new_graphics_context_and_are_invalidated_after_save() {
        let doc = vectnor_engine::cmd::fileio::load("art.svg", BLUE).unwrap().doc;
        let mut cache = Thumbnails::default();
        let services = Services::default();
        cache.remember("art.svg", &doc);
        let first = Context::default();
        assert!(cache.texture(&first, &services, "art.svg").is_some());
        let second = Context::default();
        assert!(cache.texture(&second, &services, "art.svg").is_some());
        cache.invalidate("art.svg");
        assert!(cache.texture(&second, &services, "art.svg").is_none(), "a saved path must not retain stale artwork");
        cache.retain(&[]);
        assert!(cache.entries.is_empty());
    }

    #[test]
    fn a_blocked_preview_worker_does_not_block_the_welcome_frame() {
        let (release, wait) = channel();
        let wait = std::sync::Mutex::new(wait);
        let services = Services {
            read_shared: Some(std::sync::Arc::new(move |_, limit| {
                assert_eq!(limit, MAX_BYTES);
                wait.lock().unwrap().recv().unwrap();
                Ok(BLUE.to_vec())
            })),
            ..Default::default()
        };
        let ctx = Context::default();
        let mut cache = Thumbnails::default();
        assert!(cache.texture(&ctx, &services, "art.svg").is_none(), "the first frame shows a placeholder, without waiting for disk");
        release.send(()).unwrap();
        let (_, done) = cache.worker.as_ref().unwrap();
        let result = done.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        assert!(result.image.is_some());
    }
}
