//! Help → About Vectnor.

use crate::VectnorApp;
use crate::theme;

/// The About window, while `UiState::about` is set.
pub(super) fn show(app: &mut VectnorApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let mut open = true;
    let mut close = false;
    egui::Window::new(tl!("About Vectnor"))
        .collapsible(false)
        .resizable(false)
        .open(&mut open)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(ctx.content_rect().center())
        .frame(egui::Frame::window(&ctx.global_style()).inner_margin(egui::Margin::same(18)))
        .show(ctx, |ui| {
            // Tabs: About · Contributors · Models (the credits are compiled in, see `credits`).
            let tab_id = egui::Id::new("about_tab");
            let mut tab = ui.data_mut(|d| d.get_temp::<u8>(tab_id)).unwrap_or(0);
            ui.horizontal(|ui| {
                for (i, label) in [tl!("About"), tl!("Contributors"), tl!("Models")].into_iter().enumerate() {
                    let i = i as u8;
                    if ui.selectable_label(tab == i, label).clicked() {
                        tab = i;
                    }
                }
            });
            ui.data_mut(|d| d.insert_temp(tab_id, tab));
            ui.separator();
            match tab {
                1 => {
                    ui.set_width(660.0);
                    crate::credits::contributors_ui(app, ui);
                }
                2 => {
                    ui.set_width(660.0);
                    crate::credits::models_ui(ui);
                }
                _ => close = about_tab(app, ui),
            }
        });
    app.ui.about = open && !close;
}

/// The About tab: branded artwork with developer credit, live version, licences and GPU.
fn about_tab(app: &mut VectnorApp, ui: &mut egui::Ui) -> bool {
    let width = (ui.ctx().content_rect().width() - 80.0).clamp(240.0, 570.0);
    ui.set_width(width);
    let response =
        crate::brand::artwork(ui, "vectnor-about", include_bytes!("../../../../assets/brand/about.png"), egui::vec2(width, width * 600.0 / 760.0));
    let scale = width / 760.0;
    let origin = response.rect.min;
    ui.painter().text(
        origin + egui::vec2(263.0, 211.0) * scale,
        egui::Align2::LEFT_CENTER,
        format!("Version {}", env!("CARGO_PKG_VERSION")),
        theme::semibold(15.0 * scale),
        egui::Color32::from_rgb(80, 97, 129),
    );
    let close_rect = egui::Rect::from_min_size(origin + egui::vec2(629.0, 536.0) * scale, egui::vec2(85.0, 38.0) * scale);
    let close = ui.interact(close_rect, ui.id().with("about-close"), egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand).clicked();
    ui.add_space(12.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(tl!("MIT OR Apache-2.0. Fonts: Source Sans 3, Inter, JetBrains Mono (OFL). Icons: Lucide (ISC) + Vectnor."))
                .size(11.0),
        )
        .wrap(),
    );
    // The GPU the window renders with, for bug reports (Preferences › Performance picks it).
    if let Some(adapter) = &app.graphics_adapter {
        ui.add_space(6.0);
        ui.label(egui::RichText::new(crate::i18n::fmt(tl!("Graphics: {adapter}"), &[("adapter", adapter.as_str())])).size(11.0));
    }
    close
}
