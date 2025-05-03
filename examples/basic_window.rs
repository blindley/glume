// the gl crate is exported publicly
use glume::gl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initial configuration for the window
    let window_config = glume::window::WindowConfiguration {
        title: "Hello, world!".to_string(),
        size: (800, 600),
        gl_version: (4, 5),
    };

    let window = window_config.build_window();

    // after the window is created, we can call OpenGL functions, not before
    unsafe {
        use glume::gl_utils::standard_debug_callback;
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(standard_debug_callback), std::ptr::null());
    }

    window.run(|wc, event| {
        use glume::window::Event;
        match event {
            Event::Resized(width, height) => {
                unsafe {
                    gl::Viewport(0, 0, width as i32, height as i32);
                }
            }

            Event::RedrawRequested => {
                unsafe {
                    gl::ClearColor(0.2, 0.2, 0.2, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
            }

            Event::KeyPressed(key) => {
                use glume::window::VirtualKeyCode as Vk;
                match key {
                    Vk::Escape => wc.close(),
                    _ => (),
                }
            }

            _ => (),
        }

        Ok(())
    });
}
