A simple to use all-in-one OpenGL application framework.

# Examples

## Barebones Example

```rust no_run
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
            Event::Resized((width, height)) => {
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

            Event::KeyEvent(key_event) => {
                use glume::window::{VirtualKeyCode as Vk, KeyState};
                if key_event.state == KeyState::Pressed {
                    match key_event.key {
                        Vk::Escape => wc.close(),
                        _ => (),
                    }
                }
            }

            _ => (),
        }

        Ok(())
    });
}
```

## Example with persistent state

```rust no_run
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

    // if you have persistent state, initialize it here, including OpenGL resources
    let mut app = ExampleApp::new()?;

    // don't forget to move the app into the closure
    window.run(move |wc, event| {
        use glume::window::Event;
        match event {
            Event::Resized((width, height)) => {
                unsafe {
                    gl::Viewport(0, 0, width as i32, height as i32);
                }
            }

            Event::RedrawRequested => {
                unsafe {
                    gl::ClearColor(0.2, 0.2, 0.2, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                app.render();
            }

            Event::KeyEvent(key_event) => {
                use glume::window::{VirtualKeyCode as Vk, KeyState};
                if key_event.state == KeyState::Pressed {
                    match key_event.key {
                        Vk::Escape => wc.close(),
                        Vk::Space => {
                            app.state_counter = (app.state_counter + 1) % 4;
                            wc.request_redraw();
                        }
                        _ => (),
                    }
                }
            }

            _ => (),
        }

        Ok(())
    });
}

struct ExampleApp {
    program: u32,
    vao: u32,
    state_counter: i32,
}

impl ExampleApp {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let program = create_example_program()?;
        let vao = create_example_vertex_array()?;

        Ok(Self {
            program,
            vao,
            state_counter: 0,
        })
    }

    fn render(&self) {
        unsafe {
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, self.state_counter, 3);
        }
    }
}

fn create_example_program() -> Result<u32, Box<dyn std::error::Error>> {
    let vcode = r#"
        #version 450 core
        layout (location=0) in vec2 position;
        layout (location=1) in vec3 color;
        out vec3 v_color;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            v_color = color;
        }
    "#;

    let fcode = r#"
        #version 450 core
        in vec3 v_color;
        out vec4 f_color;
        void main() {
            f_color = vec4(v_color, 1.0);
        }
    "#;

    let vshader = glume::gl_utils::compile_shader(vcode, gl::VERTEX_SHADER)?;
    let fshader = glume::gl_utils::compile_shader(fcode, gl::FRAGMENT_SHADER)?;
    let shaders = &[vshader, fshader];

    let program = glume::gl_utils::link_shader_program(shaders)?;

    unsafe {
        gl::DeleteShader(vshader);
        gl::DeleteShader(fshader);
    }

    Ok(program)
}

fn create_example_vertex_array() -> Result<u32, Box<dyn std::error::Error>> {
    let vertices: &[f32] = &[
        // positions
        -0.5, 0.0,
        0.0, 0.5,
        0.5, 0.0,
        0.0, -0.5,
        -0.5, 0.0,
        0.0, 0.5,

        // colors
        1.0, 1.0, 0.0,
        0.0, 1.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 0.0,
        0.0, 1.0, 1.0,
    ];

    let vbo = glume::gl_utils::create_buffer_f32(vertices, gl::STATIC_DRAW)?;

    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let stride = 0;

        let offset = 0 as *const _;
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, offset);
        gl::EnableVertexAttribArray(0);

        let offset = (12 * std::mem::size_of::<f32>()) as *const _;
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, offset);
        gl::EnableVertexAttribArray(1);
    }

    Ok(vao)
}
```
