

use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

use crate::keys::VirtualKeyCode;
pub use gl;

pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
    pub gl_version: (u8, u8),
}

pub struct WindowController<'a> {
    exit: bool,
    windowed_context: &'a glutin::WindowedContext<glutin::PossiblyCurrent>,
}

impl WindowController<'_> {
    pub fn get_proc_address(&self, s: &str) -> *const std::ffi::c_void {
        self.windowed_context.get_proc_address(s) as *const _
    }

    pub fn set_title(&self, title: &str) {
        self.windowed_context.window().set_title(title);
    }

    pub fn close(&mut self) {
        self.exit = true;
    }

    pub fn request_redraw(&self) {
        self.windowed_context.window().request_redraw();
    }
}

pub enum Event {
    WindowInitialized,
    CloseRequested,
    Resized((u32, u32)),
    RedrawRequested,
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
}

pub struct Window {
    event_loop: EventLoop<()>,
    windowed_context: glutin::WindowedContext<glutin::PossiblyCurrent>,
}

impl Window {
    pub fn new(window_settings: WindowSettings) -> Self {
        let el = EventLoop::new();
        let wb = WindowBuilder::new();
        let wb = wb.with_title(window_settings.title);

        let inner_size = glutin::dpi::LogicalSize::new(window_settings.size.0, window_settings.size.1);
        let wb = wb.with_inner_size(inner_size);

        let windowed_context = ContextBuilder::new();
        let windowed_context = windowed_context.with_gl_profile(glutin::GlProfile::Core);
        let windowed_context = windowed_context.with_gl(glutin::GlRequest::Specific(
            glutin::Api::OpenGl,
            window_settings.gl_version,
        ));

        let windowed_context = windowed_context.build_windowed(wb, &el).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

        Self {
            event_loop: el,
            windowed_context,
        }
    }

    
    pub fn run<F>(self, event_handler: F) -> !
    where
        F: 'static + FnMut(&mut WindowController, Event) -> Result<(), Box<dyn std::error::Error>>
    {
        let mut event_handler = event_handler;

        let exit = {
            let mut wc = WindowController {
                exit: false,
                windowed_context: &self.windowed_context,
            };

            if let Err(e) = event_handler(&mut wc, Event::WindowInitialized) {
                eprintln!("Error: {}", e);
                wc.exit = true;
            }

            wc.exit
        };

        if exit {
            std::process::exit(0);
        }

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            let mut wc = WindowController {
                exit: false,
                windowed_context: &self.windowed_context,
            };

            use glutin::event::Event as Ev;
            use glutin::event::WindowEvent as WinEv;
            use glutin::event::ElementState;

            let result = match event {
                Ev::LoopDestroyed => Ok(()),
                Ev::WindowEvent { event, .. } => match event {
                    WinEv::Resized(physical_size) => {
                        self.windowed_context.resize(physical_size);
                        event_handler(&mut wc, Event::Resized(physical_size.into()))
                    }

                    WinEv::CloseRequested => {
                        wc.exit = true;
                        event_handler(&mut wc, Event::CloseRequested)
                    }

                    WinEv::KeyboardInput { input, .. } => {
                        if let Some(vk) = input.virtual_keycode {
                            match input.state {
                                ElementState::Pressed => event_handler(&mut wc, Event::KeyPressed(vk)),
                                ElementState::Released => event_handler(&mut wc, Event::KeyReleased(vk)),
                            }
                        } else {
                            Ok(())
                        }
                    },
                    _ => Ok(()),
                },

                Ev::RedrawRequested(_) => {
                    event_handler(&mut wc, Event::RedrawRequested)
                        .and_then(|_| {
                            self.windowed_context.swap_buffers()?;
                            Ok(())
                        })
                },

                _ => Ok(()),
            };

            if let Err(e) = result {
                eprintln!("Error: {}", e);
                wc.exit = true;
            }

            if wc.exit {
                *control_flow = ControlFlow::Exit;
            }
        });
    }
}
