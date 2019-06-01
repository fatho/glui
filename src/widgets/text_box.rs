use crate::core::{Id, Rect, GluiFrame, VirtualKeyCode};
use nanovg::{PathOptions, StrokeOptions};

pub struct TextBox<'a, S> {
    hint: S,
    region: Rect,
    state: &'a mut TextBoxState,
}

pub struct TextBoxState  {
    text: Vec<char>,
    // selection: Option<Selection>,
    cursor: usize,
}

impl TextBoxState {
    pub fn new() -> Self {
        TextBoxState {
            text: Vec::new(),
            cursor: 0,
        }
    }

    pub fn to_string(&self) -> String {
        self.text.iter().collect()
    }

    /// Remove the character before the cursor.
    pub fn backspace(&mut self) {
        if self.cursor > 0 && self.cursor <= self.text.len() {
            self.text.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }

    /// Remove the character after the cursor.
    pub fn delete(&mut self) {
        if self.cursor < self.text.len() {
            self.text.remove(self.cursor);
        }
    }

    /// Insert a character at the current cursor position and advance the cursor.
    pub fn insert(&mut self, ch: char) {
        self.text.insert(self.cursor, ch);
        self.cursor += 1;
    }

    pub fn left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.cursor < self.text.len() {
            self.cursor += 1;
        }
    }
}

impl<'a> TextBox<'a, &'static str> {
    pub fn new(state: &'a mut TextBoxState) -> Self {
        TextBox {
            hint: "",
            region: Rect::zero(),
            state: state,
        }
    }
}

impl<'a, S> TextBox<'a, S> where
    S: AsRef<str>
{
    pub fn hint<S2: AsRef<str>>(self, hint: S2) -> TextBox<'a, S2> {
        TextBox {
            hint: hint,
            region: self.region,
            state: self.state,
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
        let clicked = frame.clickable_widget(id, self.region);
        let focused = frame.focusable_widget(id, clicked, |key_event| {
            if key_event.state.is_pressed() {
                match key_event.key {
                    VirtualKeyCode::Left => self.state.left(),
                    VirtualKeyCode::Right => self.state.right(),
                    _ => {},
                }
            }
            true
        });

        let mut changed = false;

        if focused {
            for ch in frame.uistate_mut().text_input.drain(..) {
                println!("{:?}", ch);
                if ch == '\u{8}' { // Backspace
                    self.state.backspace();
                    changed = true;
                } else if Self::is_input_char_valid(ch) {
                    self.state.insert(ch);
                    changed = true;
                }
            }
        }

        frame.nanovg().path(|path| {
            let border = &frame.style().widget_border;

            path.rounded_rect(
                (self.region.x as f32, self.region.y as f32),
                (self.region.w as f32, self.region.h as f32),
                border.radius,
            );

            path.stroke(border.color, Default::default());
        }, PathOptions::default());

        if ! focused && self.state.text.is_empty() {
            let hint_bounds = frame.text_measure(self.hint.as_ref());
            let hint_rect = Rect {
                x: self.region.x + 5.,
                .. self.region.center(hint_bounds)
            };
            frame.text_render(self.hint.as_ref(), hint_rect.top_left(), frame.style().hint_color);
        }

        let text = self.state.to_string();

        let text_bounds = frame.text_measure(text.as_str());
        let text_rect = Rect {
            x: self.region.x + 5.,
            .. self.region.center(text_bounds)
        };
        frame.text_render(text.as_str(), text_rect.top_left(), frame.style().foreground_color);

        if focused {
            let cursor_x = frame.nanovg().text_glyph_positions(text_rect.top_left().as_f32(), text)
                .nth(self.state.cursor)
                .map(|glyph| glyph.x)
                .unwrap_or((text_rect.x + text_rect.w) as f32);
            
            // draw cursor when focused
            frame.nanovg().path(|path| {
                path.move_to((cursor_x, text_rect.y as f32));
                path.line_to((cursor_x, (text_rect.y + text_rect.h) as f32));
                path.stroke(frame.style().foreground_color, StrokeOptions::default());
            }, PathOptions::default());
        }

        changed
    }

    fn is_input_char_valid(ch: char) -> bool {
        ! (ch == '\r' || ch == '\n' || ch == '\t')
    }
}
