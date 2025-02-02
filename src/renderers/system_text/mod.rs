use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/system_text_font.rs"));

type Error = Box<dyn std::error::Error>;

pub struct SystemTextRenderer {
    program: u32,
    character_vertices: HashMap<char, Vec<f32>>,
}

#[derive(Debug, Clone)]
pub struct TextLine {
    pub text: String,
    pub position: (f32, f32),
    pub char_size: (f32, f32),
}

pub struct SystemText {
    vao: u32,
    buffer: u32,
    num_indices: usize,
}

impl SystemText {
    pub fn new(renderer: &SystemTextRenderer, text: &[TextLine]) -> Result<Self, Error> {
        let char_scale = (0.8, 0.7);
        unsafe {
            let mut vertices = Vec::new();
            for t in text {
                let char_scale = (char_scale.0 * t.char_size.0, char_scale.1 * t.char_size.1);
                let mut char_start = t.position;
                for c in t.text.chars() {
                    if let Some(v) = renderer.character_vertices.get(&c) {
                        for i in 0..v.len() / 2 {
                            let index = i * 2;
                            let vx = char_start.0 + v[index] * char_scale.0;
                            let vy = char_start.1 - v[index + 1] * char_scale.1;
                            vertices.push(vx);
                            vertices.push(vy);
                        }
                    }
                    char_start.0 += t.char_size.0;

                    if c == '\n' {
                        char_start.0 = t.position.0;
                        char_start.1 -= t.char_size.1;
                    }
                }
            }

            let num_indices = vertices.len();

            let (vao, buffer) = create_vertex_array()?;

            gl::NamedBufferData(
                buffer,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            Ok(Self {
                vao,
                buffer,
                num_indices,
            })
        }
    }
}

impl Drop for SystemText {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.buffer);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl SystemTextRenderer {
    pub fn new() -> Result<Self, Error> {
        let program = create_program()?;
        let character_vertices = create_character_vertices();

        Ok(Self {
            program,
            character_vertices,
        })
    }

    pub fn render(&self, text: &SystemText) {
        unsafe {
            gl::UseProgram(self.program);
            gl::BindVertexArray(text.vao);
            gl::DrawArrays(gl::LINES, 0, text.num_indices as i32);
        }
    }
}

impl Drop for SystemTextRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}

fn create_character_vertices() -> HashMap<char, Vec<f32>> {
    let mut vertices = HashMap::new();

    for (c, v) in CHARACTER_VERTICES {
        vertices.insert(c, v.to_vec());
    }

    // Use uppercase vertices for lowercase characters
    for i in 0..26 {
        let lower = (i + 97) as u8 as char;
        let upper = (i + 65) as u8 as char;

        if !vertices.contains_key(&lower) {
            let v = vertices.get(&upper).unwrap().to_vec();
            vertices.insert(lower, v);
        }
    }

    vertices
}

fn create_vertex_array() -> Result<(u32, u32), Error> {
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut buffer = 0;
        gl::GenBuffers(1, &mut buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);

        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (2 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );

        gl::EnableVertexAttribArray(0);

        Ok((vao, buffer))
    }
}

fn create_program() -> Result<u32, Error> {
    use crate::gl_utils::{compile_shader, link_shader_program};

    let vshader_code = include_str!("shaders/vshader.glsl");
    let fshader_code = include_str!("shaders/fshader.glsl");

    let vshader = compile_shader(vshader_code, gl::VERTEX_SHADER)?;
    let fshader = compile_shader(fshader_code, gl::FRAGMENT_SHADER)?;

    let shaders = &[vshader, fshader];

    let program = link_shader_program(shaders)?;

    unsafe {
        gl::DeleteShader(vshader);
        gl::DeleteShader(fshader);
    }

    Ok(program)
}
