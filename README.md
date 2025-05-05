A simple Windowing and OpenGL context creation framework.

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
        gl::Enable(gl::DEBUG_OUTPUT);
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
```

## Example with persistent state

```rust no_run
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
        gl::Enable(gl::DEBUG_OUTPUT);
    }

    // if you have persistent state, initialize it here, including OpenGL resources
    let mut app = ExampleApp::new();

    println!("Press space to change the state, or escape to close the window.");

    // don't forget to move the app into the closure
    window.run(move |wc, event| {
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

                app.render();
            }

            Event::KeyPressed(key) => {
                use glume::window::VirtualKeyCode as Vk;
                match key {
                    Vk::Escape => wc.close(),
                    Vk::Space => {
                        app.state_counter = (app.state_counter + 1) % 4;
                        wc.request_redraw();
                    }
                    _ => (),
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
    fn new() -> Self {
        let program = create_example_program();
        let vao = create_example_vertex_array();

        Self {
            program,
            vao,
            state_counter: 0,
        }
    }

    fn render(&self) {
        unsafe {
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, self.state_counter, 3);
        }
    }
}

fn create_example_program() -> u32 {
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

    let vshader = compile_shader(vcode, gl::VERTEX_SHADER);
    let fshader = compile_shader(fcode, gl::FRAGMENT_SHADER);

    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vshader);
        gl::AttachShader(program, fshader);
        gl::LinkProgram(program);
        gl::DetachShader(program, vshader);
        gl::DetachShader(program, fshader);
        gl::DeleteShader(vshader);
        gl::DeleteShader(fshader);

        program
    }
}

fn compile_shader(source: &str, shader_type: u32) -> u32 {
    let shader = unsafe { gl::CreateShader(shader_type) };
    let c_str = std::ffi::CString::new(source).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }

    shader
}

fn create_example_vertex_array() -> u32 {
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

    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        let size = (vertices.len() * std::mem::size_of::<f32>()) as isize;
        let ptr = vertices.as_ptr() as *const _;
        gl::NamedBufferData(
            vbo,
            size,
            ptr,
            gl::STATIC_DRAW,
        );
    }

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

    vao
}
```
