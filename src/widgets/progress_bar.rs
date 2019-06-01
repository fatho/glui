use crate::core::{Rect, GluiFrame};

pub struct ProgressBar<S> {
    label: S,
    progress: f32,
    min_value: f32,
    max_value: f32,
    region: Rect,
}

impl ProgressBar<&'static str> {
    pub fn new() -> Self {
        ProgressBar::with_label("")
    }
}

impl<S> ProgressBar<S> where
    S: AsRef<str>
{
    pub fn with_label(label: S) -> ProgressBar<S> {
        ProgressBar {
            label: label,
            progress: 0.0,
            min_value: 0.0,
            max_value: 100.0,
            region: Rect::zero(),
        }
    }

    pub fn label<S2: AsRef<str>>(self, label: S2) -> ProgressBar<S2> {
        ProgressBar {
            label: label,
            progress: self.progress,
            min_value: self.min_value,
            max_value: self.max_value,
            region: self.region,
        }
    }

    pub fn range(self, min: f32, max: f32) -> Self {
        ProgressBar {
            min_value: min.min(max),
            max_value: max.max(min),
            .. self
        }
    }

    pub fn progress(self, progress: f32) -> Self {
        ProgressBar {
            progress: progress,
            .. self
        }
    }
    
    pub fn at(mut self, x: f64, y: f64) -> Self {
        self.region = Rect { x: x, y: y, .. self.region };
        self
    }
    
    pub fn size(mut self, w: f64, h: f64) -> Self {
        self.region = Rect { w: w, h: h, .. self.region };
        self
    }

    pub fn region(mut self, region: Rect) -> Self {
        self.region = region;
        self
    }

    pub fn reify(self, frame: &mut GluiFrame) {
        let text_bounds = frame.text_measure(self.label.as_ref());
        let text_rect = self.region.center(text_bounds);

        let mut progress = (self.progress - self.min_value) / (self.max_value - self.min_value);

        if ! progress.is_finite() {
            progress = 0.0;
        }

        let progress_offset = self.region.w as f32 * progress.min(1.).max(0.);

        let progress_clip = nanovg::Clip::Scissor(nanovg::Scissor {
            x: self.region.x as f32,
            y: self.region.y as f32,
            width: progress_offset,
            height: self.region.h as f32,
            transform: None,
        });

        let border = &frame.style().widget_border;

        frame.nanovg().path(|path| {
            path.rounded_rect(
                (self.region.x as f32, self.region.y as f32),
                (self.region.w as f32, self.region.h as f32),
                border.radius,
            );            

            path.fill(nanovg::Color::from_rgb(128, 255, 128), nanovg::FillOptions::default());
        }, nanovg::PathOptions {
            clip: progress_clip,
            .. nanovg::PathOptions::default()
        });

        frame.nanovg().path(|path| {
            path.rounded_rect(
                (self.region.x as f32, self.region.y as f32),
                (self.region.w as f32, self.region.h as f32),
                border.radius,
            );            

            path.stroke(border.color, nanovg::StrokeOptions::default());
        }, nanovg::PathOptions::default());

        frame.text_render(self.label.as_ref(), text_rect.top_left(), frame.style().foreground_color);
    }
}
