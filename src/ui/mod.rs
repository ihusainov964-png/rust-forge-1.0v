// ui/mod.rs

pub mod theme;
pub mod tabs;
pub mod widgets;

pub use theme::apply_theme;
pub use widgets::{action_button, render_notifications, secondary_button};

pub mod prelude {
    pub use super::theme::*;
    pub use super::widgets::*;
}
