use glutin::{WindowEvent, ElementState};
use glui::core;

pub struct GlutinHost {
    gl_context: glutin::WindowedContext<glutin::PossiblyCurrent>,
    events_loop: glutin::EventsLoop,
    nanovg_context: nanovg::Context,
}

impl GlutinHost {
    pub fn new(title: &str) -> Result<Self, ()> {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title);
        
        let gl_context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4)
            .with_srgb(true)
            .build_windowed(window, &events_loop)
            .map_err(|_| ())?;

        let gl_context = unsafe { gl_context.make_current().map_err(|_| ())? };

        unsafe {
            gl::load_with(|symbol| gl_context.get_proc_address(symbol) as *const _);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        }

        let nanovg_context = nanovg::ContextBuilder::new()
            .stencil_strokes()
            .build()?;

        let host = GlutinHost {
            events_loop: events_loop,
            gl_context: gl_context,
            nanovg_context: nanovg_context,
        };

        Ok(host)
    }

    pub fn add_font<S: AsRef<str>>(&mut self, name: S, data: &[u8]) -> nanovg::CreateFontResult {
        nanovg::Font::from_memory(&self.nanovg_context, name, data)
    }

    pub fn wait_events(&mut self, glui: &mut core::Glui) {
        let uistate = &mut glui.uistate;
        let mut handle_event = |event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => 
                        uistate.window_close = true,
                    WindowEvent::CursorMoved { position: pos, .. } => 
                        uistate.mouse_pos = core::Point { x: pos.x, y: pos.y },
                    WindowEvent::MouseInput { button: glutin::MouseButton::Left, state, .. } => 
                        uistate.mouse_left = if state == ElementState::Pressed {
                            core::ButtonState::Pressed
                        } else {
                            core::ButtonState::Released
                        },
                    _ => {},
                }
            }
            _ => {}
        };
        
        // Wait for first event, then handle all remaining events in queue
        self.events_loop.run_forever(|event| { handle_event(event); glutin::ControlFlow::Break });
        self.events_loop.poll_events(&mut handle_event);
    }

    pub fn render<R, F>(&mut self, glui: &mut core::Glui, mut render: F) -> R where
        F: FnMut(&mut core::GluiFrame) -> R
    {
        // Only then start rendering the UI
        let window = self.gl_context.window();
        let dpi_factor = window.get_hidpi_factor();
        let logical_size = window.get_inner_size().unwrap();
        let physical_size = logical_size.to_physical(dpi_factor);

        unsafe {
            gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
            gl::Clear(
                gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT,
            );
        }

        let mut result = None;
        let events_proxy = self.events_loop.create_proxy();

        self.nanovg_context.frame(
            (logical_size.width as f32, logical_size.height as f32),
            dpi_factor as f32,
            |frame| {
                let mut glui_frame = core::GluiFrame::begin(glui, frame);
                result = Some(render(&mut glui_frame));
                if glui_frame.requested_redraw() {
                    // Queue event in order to prevent the next wait_events from blocking
                    events_proxy.wakeup().expect("The event loop should be alive here");
                }
            }
        );

        result.unwrap()
    }

    pub fn submit_frame(&mut self) {
        self.gl_context.swap_buffers().unwrap();
    }
}
