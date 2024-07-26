use bevy::color::Color;

pub mod action_button;
pub mod end_screen;
pub mod resource_panel;
pub mod resource_slider;
pub mod debt;

pub const BORDER_COLOR: &str = "#8080b3";
pub const BACKGROUND_COLOR: &str = "#2a2a3a";
pub const FONT_PATH: &str = "fonts/karma/Karma Suture.otf";

pub fn hex2color(hex: &str) -> Color {
    let hex = hex.strip_prefix('#').unwrap();
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    Color::srgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}