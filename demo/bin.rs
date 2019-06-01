use glui::mk_id;
use glui::widgets::{Button, ProgressBar, TextBox, TextBoxState};
use glui::core::{Glui};

mod host;

use host::GlutinHost;

struct MyState {
    counter: i32,
    email: TextBoxState,
}

fn main() {
    let mut host = GlutinHost::new("Counter").unwrap();
    host.add_font("default", include_bytes!("../resources/Roboto-Regular.ttf")).unwrap();

    let mut glui = Glui::new();

    let mut state = MyState {
        counter: 0,
        email: TextBoxState::new(),
    };
    let mut running = true;

    while running {
        host.wait_events(&mut glui);
        running = host.render(&mut glui, |frame| {
            if ! frame.requested_close() {
                // Exit immediately when closing was requested.
                return false;
            }

            let label = format!("Already clicked {} times", state.counter);

            let increment = Button::with_label(&label)
                .at(10., 10.)
                .size(150., 24.)
                .reify(mk_id!(), frame);

            if increment {
                state.counter += 1;
                frame.invalidate();
            }

            let decrement = Button::with_label("Decrement")
                .at(10., 38.)
                .size(150., 24.)
                .reify(mk_id!(), frame);

            if decrement {
                state.counter -= 1;
                frame.invalidate();
            }

            let reset = Button::with_label("Reset")
                .at(10., 66.)
                .size(150., 24.)
                .reify(mk_id!(), frame);

            if reset {
                state.counter = 0;
                frame.invalidate();
            }

            let progress = state.counter as f32;

            ProgressBar::new()
                .progress(progress)
                .label(format!("{:.0}%", progress))
                .at(10., 94.)
                .size(150., 24.)
                .reify(frame);

            TextBox::new(&mut state.email)
                .hint("E-Mail address")
                .at(10., 122.)
                .size(150., 24.)
                .reify(mk_id!(), frame);

            true
        });
        host.submit_frame();
    }
}
