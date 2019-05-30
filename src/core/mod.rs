mod util;
mod id;
mod state;
mod frame;
mod style;

pub use id::*;
pub use util::*;
pub use frame::*;
pub use state::*;
pub use style::*;

pub struct Glui {
    pub uistate: GluiState,
    pub style: Style,
}

impl Glui {

    pub fn new() -> Self {
        Glui {
            uistate: GluiState::new(),
            style: Style::default(),
        }
    }
    
}