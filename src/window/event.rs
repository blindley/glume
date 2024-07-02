use super::{VirtualKeyCode, MouseButton};

#[derive(Debug, Clone)]
pub enum Event {
    EventLoopStarted,
    CloseRequested,
    Suspended,
    Resumed,
    Tick(TickEvent),
    Moved((i32, i32)),
    Resized((u32, u32)),
    Focused(bool),
    RedrawRequested,
    ModifiersChanged(ModifierState),
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    CursorEntered,
    CursorLeft,
    CursorMoved((f32, f32)),
    MouseWheel(MouseScrollDelta),
    DroppedFile(std::path::PathBuf),
    HoveredFile(std::path::PathBuf),
    HoveredFileCancelled,
    ReceivedCharacter(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModifierState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickEvent {
    pub ticks_passed: u32,
    pub time: std::time::Instant,
}

#[derive(Debug, Clone, Copy)]
pub enum MouseScrollDelta {
    LineDelta(f32, f32),
    PixelDelta(f32, f32),
}

impl From<glutin::event::MouseScrollDelta> for MouseScrollDelta {
    fn from(delta: glutin::event::MouseScrollDelta) -> Self {
        use glutin::event::MouseScrollDelta as GlutinDelta;
        use MouseScrollDelta::{LineDelta, PixelDelta};
        match delta {
            GlutinDelta::LineDelta(x, y) => LineDelta(x as f32, y as f32),
            GlutinDelta::PixelDelta(delta) => PixelDelta(delta.x as f32, delta.y as f32),
        }
    }
}
