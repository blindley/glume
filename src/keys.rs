
pub use glutin::event::VirtualKeyCode as VirtualKeyCode;

fn num_pad(vk: VirtualKeyCode, num_lock: bool) -> Option<char> {
    use VirtualKeyCode::*;

    let ch = match vk {
        Numpad0 => '0',
        Numpad1 => '1',
        Numpad2 => '2',
        Numpad3 => '3',
        Numpad4 => '4',
        Numpad5 => '5',
        Numpad6 => '6',
        Numpad7 => '7',
        Numpad8 => '8',
        Numpad9 => '9',
        NumpadAdd => '+',
        NumpadDivide => '/',
        NumpadDecimal => '.',
        NumpadMultiply => '*',
        NumpadSubtract => '-',
        _ => return None,
    };

    if num_lock || !ch.is_numeric() {
        Some(ch)
    } else {
        None
    }
}

pub fn key_as_char(vk: VirtualKeyCode, shift: bool, caps_lock: bool, num_lock: bool) -> Option<char> {
    use VirtualKeyCode::*;

    let ch = match vk {
        Key1 => '1',
        Key2 => '2',
        Key3 => '3',
        Key4 => '4',
        Key5 => '5',
        Key6 => '6',
        Key7 => '7',
        Key8 => '8',
        Key9 => '9',
        Key0 => '0',
        A => 'a',
        B => 'b',
        C => 'c',
        D => 'd',
        E => 'e',
        F => 'f',
        G => 'g',
        H => 'h',
        I => 'i',
        J => 'j',
        K => 'k',
        L => 'l',
        M => 'm',
        N => 'n',
        O => 'o',
        P => 'p',
        Q => 'q',
        R => 'r',
        S => 's',
        T => 't',
        U => 'u',
        V => 'v',
        W => 'w',
        X => 'x',
        Y => 'y',
        Z => 'z',
        Grave => '`',
        Minus => '-',
        Equals => '=',
        Tab => '\t',
        LBracket => '[',
        RBracket => ']',
        Backslash => '\\',
        Semicolon => ';',
        Apostrophe => '\'',
        Return => '\n',
        Comma => ',',
        Period => '.',
        Slash => '/',
        Space => ' ',
        _ => return num_pad(vk, num_lock),
    };

    if ch.is_alphabetic() {
        if shift ^ caps_lock {
            Some(ch.to_ascii_uppercase())
        } else {
            Some(ch)
        }
    } else {
        if shift {
            match ch {
                '1' => Some('!'),
                '2' => Some('@'),
                '3' => Some('#'),
                '4' => Some('$'),
                '5' => Some('%'),
                '6' => Some('^'),
                '7' => Some('&'),
                '8' => Some('*'),
                '9' => Some('('),
                '0' => Some(')'),
                '-' => Some('_'),
                '=' => Some('+'),
                '`' => Some('~'),
                '[' => Some('{'),
                ']' => Some('}'),
                '\\' => Some('|'),
                ';' => Some(':'),
                '\'' => Some('"'),
                ',' => Some('<'),
                '.' => Some('>'),
                '/' => Some('?'),
                _ => None,
            }
        } else {
            Some(ch)
        }
    }
}
