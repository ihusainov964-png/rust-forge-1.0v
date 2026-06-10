// ui/widgets.rs — Reusable UI components (egui 0.22 compatible)

use egui::{Color32, Response, RichText, Ui};
use crate::ui::theme::*;

pub fn section_header(ui: &mut Ui, title: &str, icon: &str) {
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new(icon).size(18.0));
        ui.label(RichText::new(title).size(16.0).color(C_TEXT_BRIGHT).strong());
    });
    ui.separator();
    ui.add_space(2.0);
}

pub fn checkbox_tip(ui: &mut Ui, checked: &mut bool, label: &str, tip: &str) -> Response {
    let cb = ui.checkbox(checked, label);
    if !tip.is_empty() { cb.on_hover_text(tip) } else { cb }
}

pub fn action_button(ui: &mut Ui, label: &str) -> Response {
    let btn = egui::Button::new(RichText::new(label).size(15.0).strong())
        .fill(C_RUST_DARK)
        .stroke(egui::Stroke::new(1.5, C_RUST));
    let resp = ui.add(btn);
    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}

pub fn secondary_button(ui: &mut Ui, label: &str) -> Response {
    let btn = egui::Button::new(label)
        .fill(C_METAL_MID)
        .stroke(egui::Stroke::new(1.0, C_METAL_LIGHT));
    ui.add(btn)
}

pub fn badge(ui: &mut Ui, text: &str, color: Color32) {
    let char_w = 7.2f32; // approx char width at 11px
    let size = egui::vec2(text.len() as f32 * char_w + 16.0, 20.0);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    ui.painter().rect_filled(rect, egui::Rounding::same(4.0), color.linear_multiply(0.20));
    ui.painter().rect_stroke(rect, egui::Rounding::same(4.0), egui::Stroke::new(1.0, color));
    ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, text, egui::FontId::proportional(11.0), color);
}

pub fn render_notifications(ui: &mut Ui, notifs: &mut Vec<crate::config::Notification>) {
    let now = std::time::Instant::now();
    notifs.retain(|n| now.duration_since(n.timestamp).as_secs_f32() < 4.5);
    for notif in notifs.iter() {
        let (icon, color) = match notif.kind {
            crate::config::NotificationKind::Success => ("✅", C_SUCCESS),
            crate::config::NotificationKind::Warning => ("⚠️", C_WARNING),
            crate::config::NotificationKind::Error   => ("❌", C_ERROR),
            crate::config::NotificationKind::Info    => ("ℹ️", C_INFO),
        };
        let age = now.duration_since(notif.timestamp).as_secs_f32();
        let alpha = if age > 3.5 { 1.0 - (age - 3.5) } else { 1.0 }.max(0.0);
        egui::Frame::none()
            .fill(C_METAL_MID.linear_multiply(alpha))
            .stroke(egui::Stroke::new(1.0, color.linear_multiply(alpha)))
            .inner_margin(egui::style::Margin::symmetric(10.0, 6.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(icon);
                    ui.label(RichText::new(&notif.message).color(color.linear_multiply(alpha)));
                });
            });
        ui.add_space(3.0);
    }
}

pub fn info_row(ui: &mut Ui, label: &str, value: &str, value_color: Option<Color32>) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("{}:", label)).color(C_TEXT_DIM).size(13.0));
        let rt = RichText::new(value).size(13.0);
        let rt = if let Some(c) = value_color { rt.color(c) } else { rt.color(C_TEXT) };
        ui.label(rt);
    });
}

pub fn labeled_slider_f32(
    ui: &mut Ui, label: &str, value: &mut f32,
    range: std::ops::RangeInclusive<f32>, suffix: &str, tip: &str,
) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).size(13.0).color(C_TEXT));
        let resp = ui.add(egui::Slider::new(value, range).suffix(suffix).clamp_to_range(true));
        if !tip.is_empty() { resp.on_hover_text(tip); }
    });
}

pub fn labeled_slider_u8(ui: &mut Ui, label: &str, value: &mut u8, min: u8, max: u8, tip: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).size(13.0).color(C_TEXT));
        let mut v = *value as f32;
        let resp = ui.add(egui::Slider::new(&mut v, min as f32..=max as f32).integer().clamp_to_range(true));
        *value = v as u8;
        if !tip.is_empty() { resp.on_hover_text(tip); }
    });
}
