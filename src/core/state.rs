use super::{Point, Id};

/// Internal state of the UI.
pub struct GluiState {
    // MOUSE STATE
    pub mouse_pos: Point,
    pub mouse_left: ButtonState,

    // WINDOW STATE
    pub window_close: bool,

    // WIDGET STATE

    /// The ID of the widget below the cursor
    pub hover_widget: Option<Id>,
    /// The ID of the widget where the mouse button was pressed, if any.
    pub active_widget: Option<Id>,
}

impl GluiState {
    pub fn new() -> Self {
        GluiState {
            mouse_pos: Point { x: 0.0, y: 0.0 },
            mouse_left: ButtonState::Released,
            window_close: false,
            hover_widget: None,
            active_widget: None,
        }
    }

    /// Should be called before starting to build the current frame in order
    /// to correctly set up the internal state.
    pub fn begin(&mut self) {
        self.hover_widget = None;
    }

    /// Should be called after building a frame in order to reset the state
    /// to make the next iteration work properly.
    pub fn end(&mut self) {
        if self.mouse_left.is_pressed() {
            // If no item became active after pressing the left mouse button,
            // we set the active item to an invalid ID in order to prevent any
            // other item from becoming active just by moving the mouse above it.
            if self.active_widget.is_none() {
                self.active_widget = Some(Id::invalid());
            }
        } else {
            // If the left mouse button isn't held down, nothing is active.
            self.active_widget = None;
        }
    }

    pub fn is_hover(&self, widget: Id) -> bool {
        self.hover_widget == Some(widget)
    }

    pub fn is_active(&self, widget: Id) -> bool {
        self.active_widget == Some(widget)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ButtonState {
    Pressed,
    Released,
}

impl ButtonState {
    pub fn is_pressed(&self) -> bool {
        *self == ButtonState::Pressed
    }
}