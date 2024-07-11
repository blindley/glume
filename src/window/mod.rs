mod event;
pub use event::*;

use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

type Error = Box<dyn std::error::Error>;
type WindowedContext = glutin::WindowedContext<glutin::PossiblyCurrent>;

pub use glutin::event::VirtualKeyCode;
pub use glutin::event::MouseButton;

#[derive(Debug, Clone)]
pub struct WindowConfiguration {
    pub title: String,
    pub size: (u32, u32),
    pub gl_version: (u8, u8),
}

impl WindowConfiguration {
    pub fn build_window(&self) -> Window {
        Window::new(self.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProcessEventStatus {
    pub exit: bool,
    pub wait_until: Option<std::time::Instant>,
}

pub struct WindowController<'a> {
    status: ProcessEventStatus,
    windata: &'a mut WinData,
}

impl<'a> WindowController<'a> {
    fn new(windata: &'a mut WinData) -> Self {
        Self {
            status: ProcessEventStatus { exit: false, wait_until: None },
            windata,
        }
    }

    pub fn set_title(&self, title: &str) {
        self.windata.windowed_context.window().set_title(title);
    }

    pub fn close(&mut self) {
        self.status.exit = true;
    }

    pub fn request_redraw(&self) {
        self.windata.windowed_context.window().request_redraw();
    }

    pub fn set_tick_duration(&mut self, duration: std::time::Duration) {
        self.windata.tick_duration = duration;
        self.windata.next_tick = std::time::Instant::now() + duration;
    }

    pub fn get_modifiers(&self) -> ModifierState {
        self.windata.modifiers
    }
}

struct WinData {
    windowed_context: WindowedContext,
    tick_duration: std::time::Duration,
    next_tick: std::time::Instant,
    modifiers: ModifierState,
}

pub struct Window {
    event_loop: EventLoop<()>,
    windata: WinData,
}

impl Window {
    fn new(window_settings: WindowConfiguration) -> Self {
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

        let tick_duration = std::time::Duration::from_secs(1);

        let modifiers = ModifierState {
            shift: false,
            ctrl: false,
            alt: false,
            super_: false,
        };

        let windata = WinData {
            windowed_context,
            tick_duration,
            next_tick: std::time::Instant::now() + tick_duration,
            modifiers,
        };

        Self {
            event_loop: el,
            windata,
        }
    }

    
    pub fn run<F>(mut self, event_handler: F) -> !
    where
        F: 'static + FnMut(&mut WindowController, Event) -> Result<(), Error>
    {
        let mut event_handler = event_handler;

        self.event_loop.run(move |event, _, control_flow| {
            match process_event(&mut self.windata, event, &mut event_handler) {
                Ok(status) => {
                    if status.exit {
                        *control_flow = ControlFlow::Exit;
                    } else if let Some(wait_until) = status.wait_until {
                        *control_flow = ControlFlow::WaitUntil(wait_until);
                    }
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    *control_flow = ControlFlow::Exit;
                }
            }
        });
    }
}

fn process_event<F>(windata: &mut WinData, event: glutin::event::Event<()>, event_handler: &mut F)
    -> Result<ProcessEventStatus, Error>
where
    F: FnMut(&mut WindowController, Event) -> Result<(), Error>
{
    let mut wc = WindowController::new(windata);

    use glutin::event::Event as Ev;
    use glutin::event::WindowEvent as WinEv;
    use glutin::event::ElementState;

    wc.status.wait_until = Some(wc.windata.next_tick);

    match event {
        Ev::LoopDestroyed => (),

        Ev::NewEvents(cause) => {
            use glutin::event::StartCause;
            match cause {
                StartCause::Init => {
                    let now = std::time::Instant::now();
                    wc.windata.next_tick = now + wc.windata.tick_duration;
                    wc.status.wait_until = Some(wc.windata.next_tick);
                    event_handler(&mut wc, Event::EventLoopStarted)?;
                },
                StartCause::ResumeTimeReached { .. } => {
                    let now = std::time::Instant::now();
                    let mut ticks_passed = 0;
                    while now >= wc.windata.next_tick {
                        wc.windata.next_tick += wc.windata.tick_duration;
                        ticks_passed += 1;
                    }

                    if ticks_passed > 0 {
                        let last_tick = wc.windata.next_tick - wc.windata.tick_duration;
                        let tick_event = TickEvent {
                            ticks_passed,
                            time: last_tick,
                        };
                        event_handler(&mut wc, Event::Tick(tick_event))?;
                    }

                    wc.status.wait_until = Some(wc.windata.next_tick);
                },
                _ => (),
            }
        }

        Ev::WindowEvent { event, .. } => match event {
            WinEv::Resized(physical_size) => {
                wc.windata.windowed_context.resize(physical_size);
                let (w, h) = physical_size.into();
                event_handler(&mut wc, Event::Resized(w, h))?;
            }

            WinEv::CloseRequested => {
                wc.status.exit = true;
                event_handler(&mut wc, Event::CloseRequested)?;
            }

            WinEv::KeyboardInput { input, .. } => {
                if let Some(vk) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => event_handler(&mut wc, Event::KeyPressed(vk))?,
                        ElementState::Released => event_handler(&mut wc, Event::KeyReleased(vk))?,
                    }
                }
            },

            WinEv::MouseInput { state, button, .. } => {
                match state {
                    ElementState::Pressed =>
                        event_handler(&mut wc, Event::MouseButtonPressed(button))?,
                    ElementState::Released =>
                        event_handler(&mut wc, Event::MouseButtonReleased(button))?,
                };
            },

            WinEv::CursorEntered { .. } => {
                event_handler(&mut wc, Event::CursorEntered)?;
            },

            WinEv::CursorLeft { .. } => {
                event_handler(&mut wc, Event::CursorLeft)?;
            },

            WinEv::CursorMoved { position, .. } => {
                let (x, y) = (position.x as f32, position.y as f32);
                event_handler(&mut wc, Event::CursorMoved(x, y))?;
            },

            WinEv::ModifiersChanged(modifiers) => {
                #[allow(deprecated)]
                let modifiers = ModifierState {
                    shift: modifiers.shift(),
                    ctrl: modifiers.ctrl(),
                    alt: modifiers.alt(),
                    super_: modifiers.logo(),
                };

                wc.windata.modifiers = modifiers;
                event_handler(&mut wc, Event::ModifiersChanged(modifiers))?;
            },

            WinEv::MouseWheel { delta, .. } => {
                let delta = delta.into();
                event_handler(&mut wc, Event::MouseWheel(delta))?;
            },

            WinEv::Focused(focused) => {
                event_handler(&mut wc, Event::Focused(focused))?;
            },

            WinEv::Moved(position) => {
                let (x, y) = (position.x, position.y);
                event_handler(&mut wc, Event::Moved(x, y))?;
            },

            WinEv::DroppedFile(path) => {
                event_handler(&mut wc, Event::DroppedFile(path))?;
            },

            WinEv::HoveredFile(path) => {
                event_handler(&mut wc, Event::HoveredFile(path))?;
            },

            WinEv::HoveredFileCancelled => {
                event_handler(&mut wc, Event::HoveredFileCancelled)?;
            },

            WinEv::ReceivedCharacter(c) => {
                event_handler(&mut wc, Event::ReceivedCharacter(c))?;
            },

            _ => ()
        },

        Ev::RedrawRequested(_) => {
            event_handler(&mut wc, Event::RedrawRequested)?;
            wc.windata.windowed_context.swap_buffers()?;
        },

        Ev::Suspended => {
            event_handler(&mut wc, Event::Suspended)?;
        },

        Ev::Resumed => {
            event_handler(&mut wc, Event::Resumed)?;
        },

        _ => ()
    }

    Ok(wc.status)
}
