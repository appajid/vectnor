//! Scrollbars for the canvas camera, independent of the document and its undo history.

use egui::{Pos2, Rect, Sense, Ui, Vec2, vec2};
use vectnor_geom::Point;

use crate::{VectnorApp, state::View, theme::Tokens};

const BAR: f32 = 14.0;
const MARGIN: f64 = 30.0;

#[derive(Clone, Copy)]
struct PreviousViewport {
    uid: u64,
    full: Rect,
    canvas: Rect,
    view: View,
}

#[derive(Clone, Copy)]
struct Axis {
    min: f64,
    max: f64,
    center: f64,
    extent: f64,
}

impl Axis {
    fn new(min: f64, max: f64, center: f64, extent: f64, zoom: f64) -> Option<Self> {
        // Preserve unrestricted Hand/wheel panning: an off-artboard camera extends the range.
        let low = min - MARGIN / zoom + extent / 2.0;
        let high = max + MARGIN / zoom - extent / 2.0;
        let axis = Self { min: low.min(center), max: high.max(center), center, extent };
        (axis.min.is_finite() && axis.max.is_finite() && (axis.max - axis.min + extent).is_finite() && extent > 0.0).then_some(axis)
    }

    fn thumb(self, length: f32) -> (f32, f32) {
        let span = self.max - self.min;
        let size = ((self.extent / (span + self.extent)) as f32 * length).clamp(22.0_f32.min(length), length);
        let fraction = if span > 0.0 { ((self.center - self.min) / span).clamp(0.0, 1.0) } else { 0.5 };
        ((length - size) * fraction as f32, size)
    }
}

pub(crate) struct Scrollbars {
    pub canvas: Rect,
    horizontal: Option<Axis>,
    vertical: Option<Axis>,
}

impl Scrollbars {
    pub fn layout(app: &mut VectnorApp, ui: &Ui, full: Rect) -> Self {
        let empty = Self { canvas: full, horizontal: None, vertical: None };
        if app.ui.screen_mode == 3 || full.width() <= BAR || full.height() <= BAR {
            return empty;
        }
        let Some(view) = app.view().copied() else { return empty };
        let Some(bounds) = bounds(app, ui) else { return empty };
        let Some(mut layout) = Self::from_bounds(full, bounds, &view) else { return empty };
        let id = egui::Id::new("canvas-scroll-viewport");
        if let Some(previous) = ui.data(|data| data.get_temp::<PreviousViewport>(id))
            && app.session.active().is_some_and(|st| st.uid == previous.uid)
            && previous.full == full
            && previous.canvas != layout.canvas
            && previous.view.zoom == view.zoom
            && previous.view.center == view.center
            && previous.view.rotation == view.rotation
        {
            // Keep the pointer's document point fixed across an input-driven bar transition.
            // Explicit camera commands (Fit, Set Zoom) keep their requested center instead.
            let delta = crate::canvas::Xf::new(previous.canvas, &view).delta_to_doc(layout.canvas.center() - previous.canvas.center());
            let center = view.center + delta;
            if center.x.is_finite() && center.y.is_finite() {
                let _ = app.run("view.setZoom", serde_json::json!({"zoom": view.zoom * 100.0, "center": [center.x, center.y]}));
                if let Some(view) = app.view()
                    && let Some(updated) = Self::from_bounds(full, bounds, view)
                {
                    layout = updated;
                }
            }
        }
        layout
    }

    pub fn remember(app: &VectnorApp, ui: &Ui, full: Rect, canvas: Rect) {
        if let (Some(st), Some(view)) = (app.session.active(), app.view()) {
            ui.data_mut(|data| {
                data.insert_temp(egui::Id::new("canvas-scroll-viewport"), PreviousViewport { uid: st.uid, full, canvas, view: *view })
            });
        }
    }

    fn from_bounds(full: Rect, bounds: vectnor_geom::Rect, view: &View) -> Option<Self> {
        if !view.zoom.is_finite() || view.zoom <= 0.0 || !view.rotation.is_finite() || !view.center.x.is_finite() || !view.center.y.is_finite() {
            return None;
        }
        let (sn, cs) = view.rotation.to_radians().sin_cos();
        let project = |p: Point| Point::new(p.x * cs - p.y * sn, p.x * sn + p.y * cs);
        let corners =
            [Point::new(bounds.x0, bounds.y0), Point::new(bounds.x1, bounds.y0), Point::new(bounds.x1, bounds.y1), Point::new(bounds.x0, bounds.y1)];
        let center = project(view.center);
        let mut low = Point::new(f64::INFINITY, f64::INFINITY);
        let mut high = Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
        for p in corners.map(project) {
            if !p.x.is_finite() || !p.y.is_finite() {
                return None;
            }
            low.x = low.x.min(p.x);
            low.y = low.y.min(p.y);
            high.x = high.x.max(p.x);
            high.y = high.y.max(p.y);
        }
        if !(high.x - low.x).is_finite() || !(high.y - low.y).is_finite() || !center.x.is_finite() || !center.y.is_finite() {
            return None;
        }
        let overflow = |min: f64, max: f64, center: f64, pixels: f32| {
            let extent = f64::from(pixels) / view.zoom;
            max - min > extent || center - extent / 2.0 > min + MARGIN / view.zoom || center + extent / 2.0 < max - MARGIN / view.zoom
        };
        let mut horizontal = false;
        let mut vertical = false;
        // Reserving one bar may make the other necessary.
        for _ in 0..3 {
            horizontal |= overflow(low.x, high.x, center.x, full.width() - if vertical { BAR } else { 0.0 });
            vertical |= overflow(low.y, high.y, center.y, full.height() - if horizontal { BAR } else { 0.0 });
        }
        let canvas = Rect::from_min_max(full.min, full.max - vec2(if vertical { BAR } else { 0.0 }, if horizontal { BAR } else { 0.0 }));
        Some(Self {
            canvas,
            horizontal: if horizontal { Some(Axis::new(low.x, high.x, center.x, f64::from(canvas.width()) / view.zoom, view.zoom)?) } else { None },
            vertical: if vertical { Some(Axis::new(low.y, high.y, center.y, f64::from(canvas.height()) / view.zoom, view.zoom)?) } else { None },
        })
    }

    pub fn show(&self, app: &mut VectnorApp, ui: &mut Ui) {
        let Some(view) = app.view().copied() else { return };
        let (sn, cs) = view.rotation.to_radians().sin_cos();
        let mut center = Point::new(view.center.x * cs - view.center.y * sn, view.center.x * sn + view.center.y * cs);
        let mut changed = false;
        if let Some(axis) = self.horizontal {
            let track = Rect::from_min_size(self.canvas.left_bottom(), vec2(self.canvas.width(), BAR));
            if let Some(value) = scrollbar(ui, track, axis, true) {
                center.x = value;
                changed = true;
            }
        }
        if let Some(axis) = self.vertical {
            let track = Rect::from_min_size(self.canvas.right_top(), vec2(BAR, self.canvas.height()));
            if let Some(value) = scrollbar(ui, track, axis, false) {
                center.y = value;
                changed = true;
            }
        }
        if self.horizontal.is_some() && self.vertical.is_some() {
            ui.painter().rect_filled(Rect::from_min_size(self.canvas.right_bottom(), Vec2::splat(BAR)), 0.0, Tokens::get(ui.ctx()).panel_darker);
        }
        if changed {
            let center = Point::new(center.x * cs + center.y * sn, -center.x * sn + center.y * cs);
            if center.x.is_finite() && center.y.is_finite() {
                // The same command used by navigation/control hosts; no document edits or undo.
                let _ = app.run("view.setZoom", serde_json::json!({"zoom": view.zoom * 100.0, "center": [center.x, center.y]}));
                ui.ctx().request_repaint();
            }
        }
    }
}

fn bounds(app: &VectnorApp, ui: &Ui) -> Option<vectnor_geom::Rect> {
    let st = app.session.active()?;
    let key = (st.uid, st.revision);
    let id = egui::Id::new("canvas-scroll-bounds");
    type Cache = ((u64, u64), Option<vectnor_geom::Rect>);
    if let Some((cached, bounds)) = ui.data(|data| data.get_temp::<Cache>(id))
        && cached == key
    {
        return bounds;
    }
    let bounds =
        st.doc.artboards.iter().fold(st.doc.art_bounds(), |bounds, board| Some(bounds.map_or(board.rect, |bounds| bounds.union(board.rect))));
    ui.data_mut(|data| data.insert_temp(id, (key, bounds)));
    bounds
}

fn scrollbar(ui: &mut Ui, track: Rect, axis: Axis, horizontal: bool) -> Option<f64> {
    let length = if horizontal { track.width() } else { track.height() };
    let along = |p: Pos2| if horizontal { p.x - track.left() } else { p.y - track.top() };
    let thumb_rect = |offset, size| {
        if horizontal {
            Rect::from_min_size(track.min + vec2(offset, 3.0), vec2(size, BAR - 6.0))
        } else {
            Rect::from_min_size(track.min + vec2(3.0, offset), vec2(BAR - 6.0, size))
        }
    };
    let (offset, size) = axis.thumb(length);
    let thumb = thumb_rect(offset, size);
    let id = ui.make_persistent_id(("canvas-scrollbar", horizontal));
    let response = ui.interact(track, id, Sense::click_and_drag());
    let label = if horizontal { tl!("Horizontal") } else { tl!("Vertical") };
    response.widget_info(|| egui::WidgetInfo::slider(ui.is_enabled(), axis.center, label));
    let mut value = axis.center;
    if response.is_pointer_button_down_on()
        && let Some(pos) = response.interact_pointer_pos()
    {
        let grab_id = id.with("grab");
        if ui.input(|input| input.pointer.primary_pressed()) {
            let grab = if thumb.contains(pos) { along(pos) - offset } else { size / 2.0 };
            ui.data_mut(|data| data.insert_temp(grab_id, grab));
            response.request_focus();
        }
        let grab = ui.data(|data| data.get_temp::<f32>(grab_id)).unwrap_or(size / 2.0);
        let fraction = (along(pos) - grab) / (length - size).max(1.0);
        value = axis.min + f64::from(fraction.clamp(0.0, 1.0)) * (axis.max - axis.min);
    }
    if response.has_focus() {
        let delta = ui.input(|input| {
            let mut delta = 0.0;
            for (key, sign) in [(egui::Key::ArrowLeft, -1.0), (egui::Key::ArrowUp, -1.0), (egui::Key::ArrowRight, 1.0), (egui::Key::ArrowDown, 1.0)] {
                if input.key_pressed(key) {
                    delta += sign * axis.extent * 0.05;
                }
            }
            for (key, sign) in [(egui::Key::PageUp, -1.0), (egui::Key::PageDown, 1.0)] {
                if input.key_pressed(key) {
                    delta += sign * axis.extent * 0.9;
                }
            }
            delta
        });
        value = (value + delta).clamp(axis.min, axis.max);
        if ui.input(|input| input.key_pressed(egui::Key::Home)) {
            value = axis.min;
        }
        if ui.input(|input| input.key_pressed(egui::Key::End)) {
            value = axis.max;
        }
    }
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_filled(track, 0.0, t.panel_darker);
    let (offset, size) = Axis { center: value, ..axis }.thumb(length);
    ui.painter().rect_filled(
        thumb_rect(offset, size),
        4.0,
        if response.is_pointer_button_down_on() {
            t.accent
        } else if response.hovered() {
            t.text
        } else {
            t.text_dim
        },
    );
    (value != axis.center).then_some(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::pos2;
    use serde_json::json;

    fn frame(app: &mut VectnorApp, ctx: &egui::Context, events: Vec<egui::Event>) {
        let input = egui::RawInput { screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(800.0, 600.0))), events, ..Default::default() };
        let mut output = ctx.run_ui(input, |ui| crate::canvas::show(app, ui));
        output.textures_delta.clear();
    }

    fn app() -> VectnorApp {
        let mut app = VectnorApp::new(vectnor_engine::Session::new(), Default::default());
        app.run("file.new", json!({"width":1000,"height":800})).unwrap();
        app
    }

    #[test]
    fn bars_follow_zoom_rotation_and_free_panning() {
        let full = Rect::from_min_size(Pos2::ZERO, vec2(800.0, 600.0));
        let bounds = vectnor_geom::Rect::new(0.0, 0.0, 1000.0, 800.0);
        let mut view = View { zoom: 0.5, center: bounds.center(), fitted: true, ..Default::default() };
        let fit = Scrollbars::from_bounds(full, bounds, &view).unwrap();
        assert!(fit.horizontal.is_none() && fit.vertical.is_none());
        assert_eq!(fit.canvas, full);
        view.zoom = 2.0;
        let zoomed = Scrollbars::from_bounds(full, bounds, &view).unwrap();
        assert!(zoomed.horizontal.is_some() && zoomed.vertical.is_some());
        assert_eq!(zoomed.canvas.size(), vec2(786.0, 586.0));
        for rotation in [0.0, 30.0, 90.0, -135.0] {
            view.rotation = rotation;
            view.center = Point::new(-10_000.0, 20_000.0);
            let panned = Scrollbars::from_bounds(full, bounds, &view).unwrap();
            for axis in [panned.horizontal.unwrap(), panned.vertical.unwrap()] {
                assert!(axis.min <= axis.center && axis.center <= axis.max);
                let (offset, size) = axis.thumb(600.0);
                assert!(offset >= 0.0 && size >= 22.0 && offset + size <= 600.001);
            }
        }
        // Only the overflowing direction gets a bar.
        view = View { zoom: 1.0, center: Point::new(500.0, 100.0), fitted: true, ..Default::default() };
        let wide = Scrollbars::from_bounds(full, vectnor_geom::Rect::new(0.0, 0.0, 1000.0, 200.0), &view).unwrap();
        assert!(wide.horizontal.is_some() && wide.vertical.is_none());
    }

    #[test]
    fn invalid_camera_or_bounds_do_not_produce_scrollbars() {
        let full = Rect::from_min_size(Pos2::ZERO, vec2(800.0, 600.0));
        let bounds = vectnor_geom::Rect::new(0.0, 0.0, 1000.0, 800.0);
        for zoom in [0.0, f64::NAN, f64::INFINITY] {
            assert!(Scrollbars::from_bounds(full, bounds, &View { zoom, ..Default::default() }).is_none());
        }
        let invalid = vectnor_geom::Rect::new(0.0, 0.0, f64::INFINITY, 800.0);
        assert!(Scrollbars::from_bounds(full, invalid, &View::default()).is_none());
        let extreme = View { center: Point::new(f64::MAX, f64::MAX), rotation: 45.0, ..Default::default() };
        assert!(Scrollbars::from_bounds(full, bounds, &extreme).is_none());
    }

    #[test]
    fn dragging_each_thumb_pans_without_drawing_or_changing_zoom() {
        for horizontal in [true, false] {
            for rotation in [0.0_f64, 90.0] {
                let mut app = app();
                let ctx = egui::Context::default();
                crate::theme::install_fonts(&ctx);
                app.run("view.setZoom", json!({"zoom":200,"center":[500,400]})).unwrap();
                app.view_mut().unwrap().rotation = rotation;
                app.select_tool("rectangle");
                frame(&mut app, &ctx, vec![]);
                let canvas = app.canvas_rect.unwrap();
                let at = if horizontal {
                    pos2(canvas.center().x, canvas.bottom() + BAR / 2.0)
                } else {
                    pos2(canvas.right() + BAR / 2.0, canvas.center().y)
                };
                let press =
                    |pos, pressed| egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed, modifiers: Default::default() };
                let before = *app.view().unwrap();
                frame(&mut app, &ctx, vec![egui::Event::PointerMoved(at), press(at, true)]);
                let moved = at + if horizontal { vec2(80.0, 0.0) } else { vec2(0.0, 80.0) };
                frame(&mut app, &ctx, vec![egui::Event::PointerMoved(moved)]);
                frame(&mut app, &ctx, vec![press(moved, false)]);
                let after = *app.view().unwrap();
                assert_eq!(after.zoom, before.zoom);
                assert_eq!(after.rotation, before.rotation);
                let delta = after.center - before.center;
                let (sn, cs) = rotation.to_radians().sin_cos();
                let screen = vectnor_geom::Vec2::new(delta.x * cs - delta.y * sn, delta.x * sn + delta.y * cs);
                assert!(
                    if horizontal { screen.x > 0.0 && screen.y.abs() < 0.01 } else { screen.y > 0.0 && screen.x.abs() < 0.01 },
                    "{horizontal}: {screen:?}"
                );
                assert!(app.session.active().unwrap().doc.art_bounds().is_none(), "a scrollbar drag never reaches the Rectangle tool");
                assert!(!app.session.active().unwrap().is_dirty());
            }
        }
    }

    #[test]
    fn fit_and_welcome_hide_bars_and_presentation_stays_unobstructed() {
        let mut app = app();
        let ctx = egui::Context::default();
        crate::theme::install_fonts(&ctx);
        frame(&mut app, &ctx, vec![]);
        let fitted = app.canvas_rect.unwrap();
        app.run("view.setZoom", json!({"zoom":200})).unwrap();
        frame(&mut app, &ctx, vec![]);
        assert_eq!(app.canvas_rect.unwrap().size(), fitted.size() - Vec2::splat(BAR));
        app.run("view.fitArtboard", json!({})).unwrap();
        frame(&mut app, &ctx, vec![]);
        assert_eq!(app.canvas_rect.unwrap(), fitted);
        assert_eq!(app.view().unwrap().center, Point::new(500.0, 400.0), "Fit keeps its exact requested center");
        app.run("app.home", json!({})).unwrap();
        app.canvas_rect = None;
        frame(&mut app, &ctx, vec![]);
        assert!(app.canvas_rect.is_none());
        app.ui.home = None;
        app.run("view.setZoom", json!({"zoom":200})).unwrap();
        app.run("view.screenMode", json!({"mode":3})).unwrap();
        frame(&mut app, &ctx, vec![]);
        assert_eq!(app.canvas_rect.unwrap().size(), vec2(800.0, 600.0));
    }
}
