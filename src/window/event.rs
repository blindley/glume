use super::{VirtualKeyCode, MouseButton};

#[derive(Debug, Clone)]
pub enum Event {
    EventLoopStarted,
    CloseRequested,
    Resized((u32, u32)),
    RedrawRequested,
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    Tick(TickEvent),
    CursorEntered,
    CursorLeft,
    CursorMoved((f32, f32)),
    ModifiersChanged(ModifierState),
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
