use super::{Glui, GluiState, Id, Style, Point, Size, Rect, KeyEvent, VirtualKeyCode, ModifiersState, ButtonState};

pub struct GluiFrame<'a, 'b> {
    glui: &'a mut Glui,
    frame: nanovg::Frame<'b>,
    /// Whether UI needs to be redrawn immediately after rendering the current frame.
    redraw: bool,
}

impl<'a, 'b> GluiFrame<'a, 'b> {
    pub fn begin(glui: &'a mut Glui, frame: nanovg::Frame<'b>) -> Self {
        glui.uistate.begin();
        GluiFrame {
            glui: glui,
            frame: frame,
            redraw: false,
        }
    }

    pub fn style(&self) -> &Style {
        &self.glui.style
    }

    pub fn nanovg(&self) -> &nanovg::Frame<'b> {
        &self.frame
    }

    pub fn uistate(&self) -> &GluiState {
        &self.glui.uistate
    }

    pub fn invalidate(&mut self) {
        self.redraw = true;
    }

    pub fn requested_redraw(&self) -> bool {
        self.redraw
    }

    pub fn requested_close(&mut self) -> bool {
        ! self.glui.uistate.window_close
    }

    pub fn font<S: AsRef<str>>(&self, name: S) -> Option<nanovg::Font<'b>> {
        nanovg::Font::find(self.frame.context(), name).ok()
    }

    /// Check whether the widget with the given ID got a click event for the given region.
    pub fn clickable_widget(&mut self, id: Id, region: Rect) -> bool {
        if region.contains(self.glui.uistate.mouse_pos) {
            self.glui.uistate.hover_widget = Some(id);

            if self.glui.uistate.active_widget.is_none() && self.glui.uistate.mouse_left.is_pressed() {
                self.glui.uistate.active_widget = Some(id);
            }
        }

        let pressed = self.glui.uistate.hover_widget == Some(id)
            && self.glui.uistate.active_widget == Some(id)
            && ! self.glui.uistate.mouse_left.is_pressed();
        
        pressed
    }

    /// Handle the interactions of a focusable widget with the current focus.
    /// The `assume_focus` parameter causes a widget to immediately assume focus
    /// which could be the case after e.g. being clicked.
    /// The callback is called for each key event and should return false for
    /// preventing the default behavior of such an event (e.g. a focus switch).
    pub fn focusable_widget<F>(&mut self, id: Id, assume_focus: bool, mut handle_key: F) -> bool where
        F: FnMut(&KeyEvent) -> bool
    {
        if self.glui.uistate.focus_widget.is_none() || assume_focus {
            self.glui.uistate.focus_widget = Some(id);
        }

        if self.glui.uistate.has_focus(id) {
            while let Some(event) = self.glui.uistate.key_input.pop_front() {
                if handle_key(&event) {
                    // Default behaviors for keys

                    let focus_key = VirtualKeyCode::Tab;
                    let next_mod = ModifiersState::default();
                    let prev_mod = ModifiersState { shift: true, .. Default::default() };

                    if event.key == focus_key && event.state == ButtonState::Pressed {
                        if event.modifiers == next_mod {
                            self.glui.uistate.focus_widget = None;
                        } else if event.modifiers == prev_mod {
                            self.glui.uistate.focus_widget = self.glui.uistate.last_focusable_widget;
                        }
                    }
                }
            }
        }

        self.glui.uistate.last_focusable_widget = Some(id);
        
        self.glui.uistate.focus_widget == Some(id)
    }

    fn text_options(&self) -> nanovg::TextOptions {
        nanovg::TextOptions {
            size: self.glui.style.font_size,
            color: nanovg::Color::from_rgb(0, 0, 0),
            align: nanovg::Alignment::new().top().left(),
            clip: nanovg::Clip::None,
            transform: None,
            ..Default::default()
        }
    }

    pub fn text_measure(&self, text: &str) -> Size {
        let default_font = &self.glui.style.font_name;
        self.font(default_font).map(|font| {
            let (_, bounds) = self.frame.text_bounds(
                font,
                (0.0, 0.0),
                text,
                self.text_options()
            );

            Size {
                w: bounds.max_x as f64 - bounds.min_x as f64,
                h: bounds.max_y as f64 - bounds.min_y as f64
            }
        }).unwrap_or(Size::zero())
    }

    pub fn text_render(&self, text: &str, position: Point) {
        let default_font = &self.glui.style.font_name;
        if let Some(font) = self.font(default_font) {
            self.frame.text(
                font,
                (position.x as f32, position.y as f32),
                text,
                self.text_options()
            );
        }
    }
}

impl<'a, 'b> Drop for GluiFrame<'a, 'b> {
    fn drop(&mut self) {
        self.glui.uistate.end();
    }
}
