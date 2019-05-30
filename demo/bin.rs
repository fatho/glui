use glui::mk_id;
use glui::widgets::Button;
use glui::core::{Glui};

mod host;

use host::GlutinHost;

struct MyState {
    counter: i32,
}

fn main() {
    let mut host = GlutinHost::new("Counter").unwrap();
    host.add_font("default", include_bytes!("../resources/Roboto-Regular.ttf")).unwrap();

    let mut glui = Glui::new();

    let mut state = MyState {
        counter: 0,
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

            let update_counter = Button::with_label(&label)
                .at(10., 10.)
                .size(150., 24.)
                .reify(mk_id!(), frame);

            if update_counter {
                state.counter += 1;
                frame.request_redraw();
            }

            true
        });
        host.submit_frame();
    }
}
