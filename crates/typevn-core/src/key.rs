//! IBus/GDK-compatible keyvals and modifier bits.

#[allow(non_snake_case, non_upper_case_globals)]
pub mod KEY {
    pub const space: u32 = 0x0020;
    pub const A: u32 = 0x0041;
    pub const Z: u32 = 0x005a;
    pub const a: u32 = 0x0061;
    pub const z: u32 = 0x007a;
    pub const BackSpace: u32 = 0xff08;
    pub const Tab: u32 = 0xff09;
    pub const Return: u32 = 0xff0d;
    pub const Escape: u32 = 0xff1b;
    pub const Home: u32 = 0xff50;
    pub const Left: u32 = 0xff51;
    pub const Up: u32 = 0xff52;
    pub const Right: u32 = 0xff53;
    pub const Down: u32 = 0xff54;
    pub const Page_Up: u32 = 0xff55;
    pub const Page_Down: u32 = 0xff56;
    pub const End: u32 = 0xff57;
    pub const Insert: u32 = 0xff63;
    pub const Delete: u32 = 0xffff;
    pub const Shift_L: u32 = 0xffe1;
    pub const Shift_R: u32 = 0xffe2;
    pub const Control_L: u32 = 0xffe3;
    pub const Control_R: u32 = 0xffe4;
    pub const Caps_Lock: u32 = 0xffe5;
    pub const Alt_L: u32 = 0xffe9;
    pub const Alt_R: u32 = 0xffea;
    pub const Pause: u32 = 0xff13;
    pub const Scroll_Lock: u32 = 0xff14;
    pub const Super_R: u32 = 0xffec;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub lock: bool,
    pub control: bool,
    pub alt: bool,
    pub super_key: bool,
    pub release: bool,
}

impl Modifiers {
    pub const fn from_ibus(state: u32) -> Self {
        // IBus modifier masks (ibus/ibusshare.h)
        const SHIFT: u32 = 1 << 0;
        const LOCK: u32 = 1 << 1;
        const CONTROL: u32 = 1 << 2;
        const MOD1: u32 = 1 << 3;
        const MOD4: u32 = 1 << 6;
        const SUPER: u32 = 1 << 26;
        const META: u32 = 1 << 28;
        const RELEASE: u32 = 1 << 30;
        Self {
            shift: state & SHIFT != 0,
            lock: state & LOCK != 0,
            control: state & CONTROL != 0,
            alt: state & (MOD1 | META) != 0,
            super_key: state & (SUPER | MOD4) != 0,
            release: state & RELEASE != 0,
        }
    }

    #[inline]
    pub const fn has_shortcut_mod(self) -> bool {
        self.control || self.alt || self.super_key
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyEvent {
    pub keyval: u32,
    pub keycode: u32,
    pub modifiers: Modifiers,
}

impl KeyEvent {
    pub const fn new(keyval: u32, keycode: u32, modifiers: Modifiers) -> Self {
        Self {
            keyval,
            keycode,
            modifiers,
        }
    }

    pub fn from_ibus(keyval: u32, keycode: u32, state: u32) -> Self {
        Self {
            keyval,
            keycode,
            modifiers: Modifiers::from_ibus(state),
        }
    }

    pub fn from_char(c: char) -> Self {
        let keyval = if (c as u32) < 0x80 {
            c as u32
        } else {
            c as u32
        };
        Self {
            keyval,
            keycode: 0,
            modifiers: Modifiers {
                shift: c.is_uppercase(),
                ..Modifiers::default()
            },
        }
    }

    pub fn printable(self) -> Option<char> {
        let v = self.keyval;
        if (0x20..0x7f).contains(&v) {
            char::from_u32(v)
        } else if (0x00a0..0x0100).contains(&v) {
            char::from_u32(v)
        } else {
            None
        }
    }

    pub fn is_backspace(self) -> bool {
        self.keyval == KEY::BackSpace
    }

    pub fn is_shift(self) -> bool {
        self.keyval == KEY::Shift_L || self.keyval == KEY::Shift_R
    }

    pub fn is_control(self) -> bool {
        self.keyval == KEY::Control_L || self.keyval == KEY::Control_R
    }

    pub fn is_navigation(self) -> bool {
        matches!(
            self.keyval,
            KEY::Left
                | KEY::Right
                | KEY::Up
                | KEY::Down
                | KEY::Home
                | KEY::End
                | KEY::Page_Up
                | KEY::Page_Down
                | KEY::Insert
                | KEY::Delete
        )
    }

    pub fn is_commit_punct(self) -> bool {
        matches!(
            self.printable(),
            Some(
                '.' | ',' | ';' | ':' | '!' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '"'
                    | '\'' | '`' | '/' | '\\' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '+'
                    | '=' | '<' | '>' | '|' | '~' | '_'
            )
        )
    }
}
