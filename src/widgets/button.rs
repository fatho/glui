use crate::core::{Id, Rect, GluiFrame, VirtualKeyCode};

pub struct Button<S> {
    label: S,
    region: Rect,
}

impl<S> Button<S> where
    S: AsRef<str>
{
    pub fn with_label(label: S) -> Button<S> {
        Button {
            label: label,
            region: Rect::zero(),
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

    pub fn reify(self, id: Id, frame: &mut GluiFrame) -> bool {
        let mut clicked = frame.clickable_widget(id, self.region);
        let focused = frame.focusable_widget(id, clicked, |key_event| {
            if key_event.state.is_pressed() && key_event.key == VirtualKeyCode::Return {
                clicked = true;
            }
            true
        });

        let text_bounds = frame.text_measure(self.label.as_ref());
        let text_rect = self.region.center(text_bounds);

        frame.nanovg().path(|path| {
            let border = &frame.style().widget_border;

            path.rounded_rect(
                (self.region.x as f32, self.region.y as f32),
                (self.region.w as f32, self.region.h as f32),
                border.radius,
            );
            
            let is_active = frame.uistate().is_active(id);
            let is_hover = frame.uistate().is_hover(id);

            let fill_color = if is_active && is_hover {
                frame.style().clickable_active
            } else if is_active || is_hover {
                frame.style().clickable_hover
            } else {
                frame.style().clickable_normal
            };

            let width = if focused { 2.0 } else { 1.0 };

            path.fill(fill_color, nanovg::FillOptions::default());
            path.stroke(border.color, nanovg::StrokeOptions {
                width: width,
                .. Default::default()
            });
        }, nanovg::PathOptions::default());

        frame.text_render(self.label.as_ref(), text_rect.top_left(), frame.style().foreground_color);

        clicked
    }
}
