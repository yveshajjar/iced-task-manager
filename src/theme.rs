use iced::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AppTheme {
    Light,
    Dark,
}

impl AppTheme {
    pub fn colors(&self) -> ThemeColors {
        match self {
            AppTheme::Light => LIGHT,
            AppTheme::Dark => DARK,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeColors {
    pub app_bg: Color,
    pub sidebar_bg: Color,
    pub sidebar_border: Color,

    pub task_bg: Color,
    pub task_border: Color,
    pub task_text: Color,

    pub task_completed_bg: Color,
    pub task_completed_border: Color,
    pub task_completed_text: Color,

    pub input_bg: Color,
    pub input_bg_hover: Color,
    pub input_border: Color,
    pub input_border_hover: Color,
    pub input_border_focused: Color,

    pub text_main: Color,
    pub text_muted: Color,
    pub text_placeholder: Color,

    pub blue_bg: Color,
    pub blue_bg_hover: Color,
    pub blue_text: Color,
    pub blue_border: Color,

    pub red_bg: Color,
    pub red_bg_hover: Color,
    pub red_text: Color,
    pub red_border: Color,

    pub amber_bg: Color,
    pub amber_bg_hover: Color,
    pub amber_text: Color,
    pub amber_border: Color,

    pub inactive_bg: Color,
    pub inactive_bg_hover: Color,
    pub inactive_text: Color,
    pub inactive_border: Color,
    pub inactive_border_hover: Color,
}

const LIGHT: ThemeColors = ThemeColors {
    app_bg: Color::from_rgb8(255, 255, 255),
    sidebar_bg: Color::from_rgb8(248, 250, 252),
    sidebar_border: Color::from_rgb8(226, 232, 240),

    task_bg: Color::from_rgb8(248, 250, 252),
    task_border: Color::from_rgb8(226, 232, 240),
    task_text: Color::from_rgb8(30, 41, 59),

    task_completed_bg: Color::from_rgb8(250, 250, 250),
    task_completed_border: Color::from_rgb8(229, 231, 235),
    task_completed_text: Color::from_rgb8(148, 163, 184),

    input_bg: Color::from_rgb8(248, 250, 252),
    input_bg_hover: Color::from_rgb8(241, 245, 249),
    input_border: Color::from_rgb8(226, 232, 240),
    input_border_hover: Color::from_rgb8(203, 213, 225),
    input_border_focused: Color::from_rgb8(147, 197, 253),

    text_main: Color::from_rgb8(30, 41, 59),
    text_muted: Color::from_rgb8(148, 163, 184),
    text_placeholder: Color::from_rgb8(148, 163, 184),

    blue_bg: Color::from_rgb8(239, 246, 255),
    blue_bg_hover: Color::from_rgb8(219, 234, 254),
    blue_text: Color::from_rgb8(37, 99, 235),
    blue_border: Color::from_rgb8(147, 197, 253),

    red_bg: Color::from_rgb8(254, 242, 242),
    red_bg_hover: Color::from_rgb8(254, 226, 226),
    red_text: Color::from_rgb8(220, 38, 38),
    red_border: Color::from_rgb8(254, 202, 202),

    amber_bg: Color::from_rgb8(255, 251, 235),
    amber_bg_hover: Color::from_rgb8(254, 243, 199),
    amber_text: Color::from_rgb8(146, 64, 14),
    amber_border: Color::from_rgb8(253, 230, 138),

    inactive_bg: Color::from_rgb8(255, 255, 255),
    inactive_bg_hover: Color::from_rgb8(248, 250, 252),
    inactive_text: Color::from_rgb8(100, 116, 139),
    inactive_border: Color::from_rgb8(226, 232, 240),
    inactive_border_hover: Color::from_rgb8(203, 213, 225),
};

const DARK: ThemeColors = ThemeColors {
    app_bg: Color::from_rgb8(15, 23, 42),
    sidebar_bg: Color::from_rgb8(17, 24, 39),
    sidebar_border: Color::from_rgb8(51, 65, 85),

    task_bg: Color::from_rgb8(30, 41, 59),
    task_border: Color::from_rgb8(71, 85, 105),
    task_text: Color::from_rgb8(226, 232, 240),

    task_completed_bg: Color::from_rgb8(24, 33, 49),
    task_completed_border: Color::from_rgb8(51, 65, 85),
    task_completed_text: Color::from_rgb8(100, 116, 139),

    input_bg: Color::from_rgb8(30, 41, 59),
    input_bg_hover: Color::from_rgb8(51, 65, 85),
    input_border: Color::from_rgb8(71, 85, 105),
    input_border_hover: Color::from_rgb8(100, 116, 139),
    input_border_focused: Color::from_rgb8(96, 165, 250),

    text_main: Color::from_rgb8(226, 232, 240),
    text_muted: Color::from_rgb8(148, 163, 184),
    text_placeholder: Color::from_rgb8(148, 163, 184),

    blue_bg: Color::from_rgb8(30, 64, 175),
    blue_bg_hover: Color::from_rgb8(37, 99, 235),
    blue_text: Color::from_rgb8(219, 234, 254),
    blue_border: Color::from_rgb8(96, 165, 250),

    red_bg: Color::from_rgb8(69, 26, 26),
    red_bg_hover: Color::from_rgb8(91, 33, 33),
    red_text: Color::from_rgb8(252, 165, 165),
    red_border: Color::from_rgb8(127, 29, 29),

    amber_bg: Color::from_rgb8(69, 46, 20),
    amber_bg_hover: Color::from_rgb8(92, 62, 25),
    amber_text: Color::from_rgb8(253, 230, 138),
    amber_border: Color::from_rgb8(120, 53, 15),

    inactive_bg: Color::from_rgb8(17, 24, 39),
    inactive_bg_hover: Color::from_rgb8(30, 41, 59),
    inactive_text: Color::from_rgb8(148, 163, 184),
    inactive_border: Color::from_rgb8(51, 65, 85),
    inactive_border_hover: Color::from_rgb8(71, 85, 105),
};
