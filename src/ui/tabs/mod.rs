// ui/tabs/mod.rs

pub mod launch;
pub mod graphics;
pub mod system;
pub mod profiles;
pub mod about;

pub use launch::draw_launch_tab;
pub use graphics::draw_graphics_tab;
pub use system::draw_system_tab;
pub use profiles::draw_profiles_tab;
pub use about::draw_about_tab;
