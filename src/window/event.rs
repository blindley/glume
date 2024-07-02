use super::VirtualKeyCode;

#[derive(Debug, Clone)]
pub enum Event {
    EventLoopStarted,
    CloseRequested,
    Resized((u32, u32)),
    RedrawRequested,
    KeyEvent(KeyEvent),
    Tick(TickEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModifierState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: VirtualKeyCode,
    pub state: KeyState,
    pub modifiers: ModifierState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickEvent {
    pub ticks_passed: u32,
    pub time: std::time::Instant,
}
