
use crate::gl;
use crate::renderers::{Renderer, IntRect};

/// A renderer that does nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NullRenderer;

impl Renderer for NullRenderer {
    fn set_viewport(&mut self, _viewport_rect: IntRect) {}
    fn render(&self) {}
}

/// Fills the viewport with a solid color.
pub struct ClearColorRenderer {
    viewport_rect: IntRect,
    color: [f32; 4],
}

impl ClearColorRenderer {
    pub fn new(viewport_rect: IntRect, color: [f32; 4]) -> Self {
        Self {
            viewport_rect,
            color,
        }
    }
}

impl Renderer for ClearColorRenderer {
    fn set_viewport(&mut self, viewport_rect: IntRect) {
        self.viewport_rect = viewport_rect;
    }

    fn render(&self) {
        self.viewport_rect.gl_viewport();
        unsafe {
            gl::ClearColor(self.color[0], self.color[1], self.color[2], self.color[3]);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

/// Defines the split point of a split renderer.
/// Use a negative value to specify a split point relative to the far edge of the viewport.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SplitPoint {
    /// Absolute split point in pixels.
    Absolute(i32),

    /// Ratio of the viewport size (0.0 to 1.0).
    Ratio(f32),
}

impl SplitPoint {
    pub fn to_absolute(&self, viewport_size: i32) -> i32 {
        let mut sp = match self {
            SplitPoint::Absolute(x) => *x,
            SplitPoint::Ratio(r) => (viewport_size as f32 * r) as i32,
        };

        if sp < 0 {
            sp = viewport_size + sp
        }

        sp.clamp(0, viewport_size)
    }
}

struct SplitRenderer {
    viewport_rect: IntRect,
    horizontal: bool,
    split_point: SplitPoint,
    r1: Box<dyn Renderer>,
    r2: Box<dyn Renderer>,
}

impl SplitRenderer {
    pub fn new(viewport_rect: IntRect, horizontal: bool, split_point: SplitPoint, r1: Box<dyn Renderer>, r2: Box<dyn Renderer>) -> Self {
        let mut self_ = Self {
            viewport_rect,
            horizontal,
            split_point,
            r1,
            r2,
        };

        self_.reset_subrenderer_viewports();
        self_
    }

    pub fn get_r1(&self) -> &dyn Renderer {
        self.r1.as_ref()
    }

    pub fn get_r2(&self) -> &dyn Renderer {
        self.r2.as_ref()
    }

    pub fn get_r1_mut(&mut self) -> &mut dyn Renderer {
        self.r1.as_mut()
    }

    pub fn get_r2_mut(&mut self) -> &mut dyn Renderer {
        self.r2.as_mut()
    }

    pub fn set_split_point(&mut self, split_point: SplitPoint) {
        self.split_point = split_point;
        self.reset_subrenderer_viewports();
    }

    fn reset_subrenderer_viewports(&mut self) {
        let (r1v, r2v) = if self.horizontal {
            let sp = self.split_point.to_absolute(self.viewport_rect.size[0]);
            let r1v = IntRect {
                pos: self.viewport_rect.pos,
                size: [sp, self.viewport_rect.size[1]],
            };
            let r2v = IntRect {
                pos: [self.viewport_rect.pos[0] + sp, self.viewport_rect.pos[1]],
                size: [self.viewport_rect.size[0] - sp, self.viewport_rect.size[1]],
            };

            (r1v, r2v)
        } else {
            let sp = self.split_point.to_absolute(self.viewport_rect.size[1]);
            let r1v = IntRect {
                pos: self.viewport_rect.pos,
                size: [self.viewport_rect.size[0], sp],
            };
            let r2v = IntRect {
                pos: [self.viewport_rect.pos[0], self.viewport_rect.pos[1] + sp],
                size: [self.viewport_rect.size[0], self.viewport_rect.size[1] - sp],
            };

            (r1v, r2v)
        };

        self.r1.set_viewport(r1v);
        self.r2.set_viewport(r2v);
    }
}

impl Renderer for SplitRenderer {
    fn set_viewport(&mut self, viewport_rect: IntRect) {
        self.viewport_rect = viewport_rect;
        self.reset_subrenderer_viewports();
    }

    fn render(&self) {
        self.r1.render();
        self.r2.render();
    }
}

/// Splits the viewport between a left and right renderer.
pub struct HSplitRenderer {
    split_renderer: SplitRenderer,
}

impl HSplitRenderer {
    pub fn new(viewport_rect: IntRect, split_point: SplitPoint, left: Box<dyn Renderer>, right: Box<dyn Renderer>) -> Self {
        let split_renderer = SplitRenderer::new(viewport_rect, true, split_point, left, right);
        Self { split_renderer }
    }

    pub fn get_left(&self) -> &dyn Renderer {
        self.split_renderer.get_r1()
    }

    pub fn get_right(&self) -> &dyn Renderer {
        self.split_renderer.get_r2()
    }

    pub fn get_left_mut(&mut self) -> &mut dyn Renderer {
        self.split_renderer.get_r1_mut()
    }

    pub fn get_right_mut(&mut self) -> &mut dyn Renderer {
        self.split_renderer.get_r2_mut()
    }

    pub fn set_split_point(&mut self, split_point: SplitPoint) {
        self.split_renderer.set_split_point(split_point);
    }
}

impl Renderer for HSplitRenderer {
    fn set_viewport(&mut self, viewport_rect: IntRect) {
        self.split_renderer.set_viewport(viewport_rect);
    }

    fn render(&self) {
        self.split_renderer.render();
    }
}

/// Splits the viewport between a top and bottom renderer.
pub struct VSplitRenderer {
    split_renderer: SplitRenderer,
}

impl VSplitRenderer {
    pub fn new(viewport_rect: IntRect, split_point: SplitPoint, top: Box<dyn Renderer>, bottom: Box<dyn Renderer>) -> Self {
        let split_renderer = SplitRenderer::new(viewport_rect, false, split_point, top, bottom);
        Self { split_renderer }
    }

    pub fn get_top(&self) -> &dyn Renderer {
        self.split_renderer.get_r1()
    }

    pub fn get_bottom(&self) -> &dyn Renderer {
        self.split_renderer.get_r2()
    }

    pub fn get_top_mut(&mut self) -> &mut dyn Renderer {
        self.split_renderer.get_r1_mut()
    }

    pub fn get_bottom_mut(&mut self) -> &mut dyn Renderer {
        self.split_renderer.get_r2_mut()
    }

    pub fn set_split_point(&mut self, split_point: SplitPoint) {
        self.split_renderer.set_split_point(split_point);
    }
}

impl Renderer for VSplitRenderer {
    fn set_viewport(&mut self, viewport_rect: IntRect) {
        self.split_renderer.set_viewport(viewport_rect);
    }

    fn render(&self) {
        self.split_renderer.render();
    }
}

/// Renders one renderer inside another, with a specified inset.
/// The inset is the distance from the edge of the viewport to the edge of the inner renderer.
pub struct InsetRenderer {
    viewport_rect: IntRect,
    inset: i32,
    outside_renderer: Box<dyn Renderer>,
    inside_renderer: Box<dyn Renderer>,
}

impl InsetRenderer {
    pub fn new(viewport_rect: IntRect, inset: i32, outside_renderer: Box<dyn Renderer>, inside_renderer: Box<dyn Renderer>) -> Self {
        let mut self_ = Self {
            viewport_rect,
            inset,
            outside_renderer,
            inside_renderer,
        };

        self_.reset_subrenderer_viewports();
        self_
    }

    pub fn get_outside(&self) -> &dyn Renderer {
        self.outside_renderer.as_ref()
    }

    pub fn get_inside(&self) -> &dyn Renderer {
        self.inside_renderer.as_ref()
    }

    pub fn get_outside_mut(&mut self) -> &mut dyn Renderer {
        self.outside_renderer.as_mut()
    }

    pub fn get_inside_mut(&mut self) -> &mut dyn Renderer {
        self.inside_renderer.as_mut()
    }

    pub fn set_inset(&mut self, inset: i32) {
        self.inset = inset;
        self.reset_subrenderer_viewports();
    }

    fn reset_subrenderer_viewports(&mut self) {
        let ix = self.viewport_rect.pos[0] + self.inset;
        let iy = self.viewport_rect.pos[1] + self.inset;
        let iw = self.viewport_rect.size[0] - 2 * self.inset;
        let ih = self.viewport_rect.size[1] - 2 * self.inset;
        let isize_ = if iw < 0 || ih < 0 {
            [0, 0]
        } else {
            [iw, ih]
        };

        let irect = IntRect {
            pos: [ix, iy],
            size: isize_,
        };

        self.outside_renderer.set_viewport(self.viewport_rect);
        self.inside_renderer.set_viewport(irect);
    }
}

impl Renderer for InsetRenderer {
    fn set_viewport(&mut self, viewport_rect: IntRect) {
        self.viewport_rect = viewport_rect;
        self.reset_subrenderer_viewports();
    }

    fn render(&self) {
        self.outside_renderer.render();
        self.inside_renderer.render();
    }
}

pub struct FixedAspectRatioRenderer {
    viewport_rect: IntRect,
    aspect_ratio: f32,
    renderer: Box<dyn Renderer>,
}

impl FixedAspectRatioRenderer {
    pub fn new(viewport_rect: IntRect, aspect_ratio: f32, renderer: Box<dyn Renderer>) -> Self {

        let mut self_ = Self {
            viewport_rect,
            aspect_ratio,
            renderer,
        };

        self_.reset_subrenderer_viewports();

        self_
    }

    fn reset_subrenderer_viewports(&mut self) {
        let viewport_size = self.viewport_rect.size;
        let new_width = (viewport_size[1] as f32 * self.aspect_ratio) as i32;
        let new_height = (viewport_size[0] as f32 / self.aspect_ratio) as i32;

        let new_size = if new_width < viewport_size[0] {
            [new_width, viewport_size[1]]
        } else {
            [viewport_size[0], new_height]
        };

        let new_pos = [
            self.viewport_rect.pos[0] + (viewport_size[0] - new_size[0]) / 2,
            self.viewport_rect.pos[1] + (viewport_size[1] - new_size[1]) / 2,
        ];

        let new_viewport_rect = IntRect {
            pos: new_pos,
            size: new_size,
        };

        self.renderer.set_viewport(new_viewport_rect);
    }

    pub fn get_subrenderer(&self) -> &dyn Renderer {
        self.renderer.as_ref()
    }

    pub fn get_subrenderer_mut(&mut self) -> &mut dyn Renderer {
        self.renderer.as_mut()
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.reset_subrenderer_viewports();
    }
}

impl Renderer for FixedAspectRatioRenderer {
    fn set_viewport(&mut self, viewport_rect: IntRect) {
        self.viewport_rect = viewport_rect;
        self.reset_subrenderer_viewports();
    }

    fn render(&self) {
        self.renderer.render();
    }
}
