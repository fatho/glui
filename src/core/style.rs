pub use nanovg::Color;

pub struct Style {
    pub font_size: f32,
    pub font_name: String,

    pub clickable_hover: Color,
    pub clickable_active: Color,
    pub clickable_normal: Color,
    pub widget_border: BorderStyle,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            font_size: 16.,
            font_name: "default".to_owned(),
            clickable_active: Color::from_rgb(230, 230, 230),
            clickable_hover: Color::from_rgb(240, 240, 240),
            clickable_normal: Color::from_rgb(255, 255, 255),
            widget_border: BorderStyle::default(),
        }
    }
}

pub struct BorderStyle {
    pub color: Color,
    pub radius: f32,
}

impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle {
            color: Color::from_rgb(0, 0, 0),
            radius: 5.0
        }
    }
}