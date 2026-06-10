// ui/theme.rs — RustForge dark industrial theme (egui 0.22 compatible)

use egui::{Color32, FontId, Rounding, Stroke, Style};

pub const C_RUST:        Color32 = Color32::from_rgb(204, 85,  34);
pub const C_RUST_DARK:   Color32 = Color32::from_rgb(140, 55,  20);
pub const C_RUST_BRIGHT: Color32 = Color32::from_rgb(240, 120, 50);
pub const C_METAL:       Color32 = Color32::from_rgb(42,  42,  46);
pub const C_METAL_DARK:  Color32 = Color32::from_rgb(22,  22,  25);
pub const C_METAL_MID:   Color32 = Color32::from_rgb(55,  55,  62);
pub const C_METAL_LIGHT: Color32 = Color32::from_rgb(72,  72,  80);
pub const C_TEXT:        Color32 = Color32::from_rgb(218, 210, 195);
pub const C_TEXT_DIM:    Color32 = Color32::from_rgb(148, 138, 120);
pub const C_TEXT_BRIGHT: Color32 = Color32::from_rgb(255, 245, 230);
pub const C_SUCCESS:     Color32 = Color32::from_rgb(100, 200, 100);
pub const C_WARNING:     Color32 = Color32::from_rgb(230, 180,  40);
pub const C_ERROR:       Color32 = Color32::from_rgb(220,  70,  60);
pub const C_INFO:        Color32 = Color32::from_rgb(80,  160, 220);

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = Style::default();

    style.spacing.item_spacing   = egui::vec2(8.0, 6.0);
    style.spacing.window_margin  = egui::style::Margin::same(14.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.indent         = 18.0;
    style.spacing.slider_width   = 160.0;

    let r4 = Rounding::same(4.0);
    let r6 = Rounding::same(6.0);
    style.visuals.window_rounding = r6;
    style.visuals.menu_rounding   = r6;
    style.visuals.widgets.noninteractive.rounding = r4;
    style.visuals.widgets.inactive.rounding       = r4;
    style.visuals.widgets.hovered.rounding        = r4;
    style.visuals.widgets.active.rounding         = r4;
    style.visuals.widgets.open.rounding           = r4;

    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(C_TEXT);
    style.visuals.window_fill  = C_METAL_DARK;
    style.visuals.window_stroke = Stroke::new(1.0, C_METAL_MID);
    style.visuals.panel_fill   = C_METAL_DARK;

    style.visuals.widgets.noninteractive.bg_fill  = C_METAL;
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, C_METAL_MID);
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, C_TEXT_DIM);

    style.visuals.widgets.inactive.bg_fill  = C_METAL_MID;
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, C_METAL_LIGHT);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, C_TEXT);

    style.visuals.widgets.hovered.bg_fill  = C_METAL_LIGHT;
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, C_RUST);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, C_RUST_BRIGHT);

    style.visuals.widgets.active.bg_fill  = C_RUST_DARK;
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.5, C_RUST_BRIGHT);
    style.visuals.widgets.active.fg_stroke = Stroke::new(2.0, C_RUST_BRIGHT);

    style.visuals.selection.bg_fill = C_RUST_DARK.linear_multiply(1.4);
    style.visuals.selection.stroke  = Stroke::new(1.0, C_RUST);
    style.visuals.hyperlink_color   = C_RUST_BRIGHT;
    style.visuals.code_bg_color     = C_METAL;
    style.visuals.faint_bg_color    = Color32::from_rgba_unmultiplied(255, 255, 255, 4);

    ctx.set_style(style);

    ctx.set_style({
        let mut s = (*ctx.style()).clone();
        s.text_styles.insert(
            egui::TextStyle::Heading,
            FontId::new(20.0, egui::FontFamily::Proportional),
        );
        s.text_styles.insert(
            egui::TextStyle::Body,
            FontId::new(14.0, egui::FontFamily::Proportional),
        );
        s.text_styles.insert(
            egui::TextStyle::Small,
            FontId::new(11.0, egui::FontFamily::Proportional),
        );
        s.text_styles.insert(
            egui::TextStyle::Button,
            FontId::new(14.0, egui::FontFamily::Proportional),
        );
        s.text_styles.insert(
            egui::TextStyle::Monospace,
            FontId::new(13.0, egui::FontFamily::Monospace),
        );
        std::sync::Arc::new(s)
    });
}
