pub mod basic_renderers;
pub mod image_renderer;
pub mod system_text;

use crate::gl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntRect {
    pub pos: [i32; 2],
    pub size: [i32; 2],
}

impl IntRect {
    pub fn gl_viewport(&self) {
        unsafe {
            gl::Viewport(self.pos[0], self.pos[1], self.size[0], self.size[1]);
            gl::Scissor(self.pos[0], self.pos[1], self.size[0], self.size[1]);
            gl::Enable(gl::SCISSOR_TEST);
        }
    }
}

impl Default for IntRect {
    fn default() -> Self {
        Self {
            pos: [0, 0],
            size: [0, 0],
        }
    }
}

pub trait Renderer {
    fn set_viewport(&mut self, viewport_rect: IntRect);
    fn render(&self);
}
