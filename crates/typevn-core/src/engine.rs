use crate::charset::Charset;
use crate::english::is_tech_prefix;
use crate::key::{KeyEvent, KEY};
use crate::repair::{repair_basic_errors, MacroStore};
use crate::syllable::last_syllable_start;
use crate::telex::{apply_telex, pop_char};
use crate::vni::apply_vni;
use crate::MAX_COMPOSE_CHARS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMode {
    Vietnamese,
    English,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypingMethod {
    Telex,
    Vni,
}

impl TypingMethod {
    pub fn name(self) -> &'static str {
        match self {
            Self::Telex => "Telex",
            Self::Vni => "VNI",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineAction {
    Commit(String),
    Preedit(String),
    PassThrough,
    Delete(usize),
    Reset,
    /// Commit composition, then let the current key reach the application.
    CommitThenPass(String),
    /// Hotkey consumed; show a short IBus status (Telex / VNI / Anh).
    Notify(String),
    /// Commit leftover composition, then show a status (mode / method change).
    CommitThenNotify(String, String),
}

pub struct VietnameseEngine {
    buffer: Vec<char>,
    /// Literal keys for the current token. This lets us restore an identifier
    /// after an early Telex key (notably `w`) was tentatively converted.
    raw_buffer: Vec<char>,
    literal_token: bool,
    cursor: usize,
    undo: Vec<Vec<char>>,
    input_mode: InputMode,
    typing_method: TypingMethod,
    charset: Charset,
    auto_repair: bool,
    hotkeys_enabled: bool,
    preedit_delay_ms: u32,
    macros: MacroStore,
}

impl Default for VietnameseEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl VietnameseEngine {
    pub fn new() -> Self {
        let cfg = crate::config::load();
        Self {
            buffer: Vec::with_capacity(MAX_COMPOSE_CHARS),
            raw_buffer: Vec::with_capacity(MAX_COMPOSE_CHARS),
            literal_token: false,
            cursor: 0,
            undo: Vec::with_capacity(MAX_COMPOSE_CHARS),
            input_mode: if cfg.english {
                InputMode::English
            } else {
                InputMode::Vietnamese
            },
            typing_method: if cfg.method_vni {
                TypingMethod::Vni
            } else {
                TypingMethod::Telex
            },
            charset: cfg.charset,
            auto_repair: cfg.auto_repair,
            hotkeys_enabled: cfg.hotkeys_enabled,
            preedit_delay_ms: cfg.preedit_delay_ms,
            macros: MacroStore::load(),
        }
    }

    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
        self.clear_buffer();
    }

    pub fn toggle_mode(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Vietnamese => InputMode::English,
            InputMode::English => InputMode::Vietnamese,
        };
        self.clear_buffer();
    }

    pub fn typing_method(&self) -> TypingMethod {
        self.typing_method
    }

    pub fn set_typing_method(&mut self, method: TypingMethod) {
        self.typing_method = method;
        self.clear_buffer();
    }

    pub fn toggle_typing_method(&mut self) {
        self.typing_method = match self.typing_method {
            TypingMethod::Telex => TypingMethod::Vni,
            TypingMethod::Vni => TypingMethod::Telex,
        };
        self.clear_buffer();
    }

    pub fn charset(&self) -> Charset {
        self.charset
    }

    pub fn set_charset(&mut self, charset: Charset) {
        self.charset = charset;
    }

    pub fn auto_repair(&self) -> bool {
        self.auto_repair
    }

    pub fn set_auto_repair(&mut self, on: bool) {
        self.auto_repair = on;
    }

    pub fn preedit_delay_ms(&self) -> u32 {
        self.preedit_delay_ms
    }

    pub fn toggle_auto_repair(&mut self) {
        self.auto_repair = !self.auto_repair;
    }

    pub fn set_hotkeys_enabled(&mut self, on: bool) {
        self.hotkeys_enabled = on;
    }

    pub fn cycle_charset(&mut self) {
        self.charset = self.charset.next();
    }

    pub fn buffer_str(&self) -> String {
        let raw: String = self.buffer.iter().collect();
        self.charset.encode(&raw)
    }

    pub fn reset(&mut self) {
        self.clear_buffer();
    }

    pub fn reload_config(&mut self) {
        let cfg = crate::config::load();
        self.typing_method = if cfg.method_vni {
            TypingMethod::Vni
        } else {
            TypingMethod::Telex
        };
        self.input_mode = if cfg.english {
            InputMode::English
        } else {
            InputMode::Vietnamese
        };
        self.charset = cfg.charset;
        self.auto_repair = cfg.auto_repair;
        self.hotkeys_enabled = cfg.hotkeys_enabled;
        self.preedit_delay_ms = cfg.preedit_delay_ms;
        self.clear_buffer();
    }

    fn clear_buffer(&mut self) {
        self.buffer.clear();
        self.raw_buffer.clear();
        self.literal_token = false;
        self.undo.clear();
        self.cursor = 0;
    }

    fn push_undo(&mut self) {
        if self.undo.len() >= MAX_COMPOSE_CHARS {
            self.undo.remove(0);
        }
        self.undo.push(self.buffer.clone());
    }

    fn take_buffer(&mut self) -> String {
        let raw: String = self.buffer.iter().collect();
        let raw = if self.auto_repair {
            self.macros.replace(&self.raw_buffer, raw)
        } else {
            raw
        };
        self.clear_buffer();
        self.charset.encode(&raw)
    }

    /// Process one key. Synchronous, ordered, no I/O, no threads.
    pub fn process_key(&mut self, key: KeyEvent) -> EngineAction {
        if key.modifiers.release {
            return EngineAction::PassThrough;
        }

        if self.hotkeys_enabled && self.is_mode_toggle(key) {
            let leftover = self.take_buffer();
            self.toggle_mode();
            let msg = match self.input_mode {
                InputMode::Vietnamese => "TypeVN · Việt",
                InputMode::English => "TypeVN · Anh",
            };
            if leftover.is_empty() {
                return EngineAction::Notify(msg.into());
            }
            return EngineAction::CommitThenNotify(leftover, msg.into());
        }

        if self.hotkeys_enabled && self.is_method_hotkey(key) {
            let leftover = self.take_buffer();
            if is_telex_hotkey(key) {
                self.typing_method = TypingMethod::Telex;
            } else if is_vni_hotkey(key) {
                self.typing_method = TypingMethod::Vni;
            } else {
                self.toggle_typing_method();
            }
            let msg = match self.typing_method {
                TypingMethod::Telex => "TypeVN · Telex",
                TypingMethod::Vni => "TypeVN · VNI",
            };
            if leftover.is_empty() {
                return EngineAction::Notify(msg.into());
            }
            return EngineAction::CommitThenNotify(leftover, msg.into());
        }

        if self.hotkeys_enabled && self.is_option_hotkey(key) {
            let leftover = self.take_buffer();
            match key.keyval {
                0x63 | 0x43 => self.cycle_charset(),
                0x72 | 0x52 => self.toggle_auto_repair(),
                _ => {}
            }
            let msg = match key.keyval {
                0x63 | 0x43 => format!("TypeVN · {}", self.charset.name()),
                0x72 | 0x52 => format!(
                    "TypeVN · tự sửa {}",
                    if self.auto_repair { "bật" } else { "tắt" }
                ),
                _ => "TypeVN".into(),
            };
            if leftover.is_empty() {
                return EngineAction::Notify(msg);
            }
            return EngineAction::CommitThenNotify(leftover, msg);
        }

        if self.should_passthrough_shortcut(key) {
            if self.buffer.is_empty() {
                return EngineAction::PassThrough;
            }
            let s = self.take_buffer();
            return EngineAction::CommitThenPass(s);
        }

        if self.input_mode == InputMode::English {
            return EngineAction::PassThrough;
        }

        if key.is_backspace() {
            return self.on_backspace();
        }

        if key.keyval == KEY::Escape {
            self.clear_buffer();
            return EngineAction::Reset;
        }

        if key.keyval == KEY::Return || key.keyval == KEY::Tab {
            if self.buffer.is_empty() {
                return EngineAction::PassThrough;
            }
            let s = self.take_buffer();
            return EngineAction::CommitThenPass(s);
        }

        if key.is_navigation() {
            if self.buffer.is_empty() {
                return EngineAction::PassThrough;
            }
            let s = self.take_buffer();
            return EngineAction::CommitThenPass(s);
        }

        if key.keyval == KEY::space {
            let mut s = self.take_buffer();
            s.push(' ');
            return EngineAction::Commit(s);
        }

        if key.is_commit_punct() {
            if let Some(ch) = key.printable() {
                let mut s = self.take_buffer();
                s.push(ch);
                return EngineAction::Commit(s);
            }
        }

        if let Some(ch) = key.printable() {
            if ch.is_ascii_alphabetic()
                || (self.typing_method == TypingMethod::Vni && ch.is_ascii_digit())
            {
                // Never silently truncate a long identifier. Commit a bounded
                // chunk and consume this key; the next key starts a new chunk.
                if self.buffer.len() >= MAX_COMPOSE_CHARS {
                    let mut s = self.take_buffer();
                    s.push(ch);
                    // A token longer than the composition window is an
                    // identifier. Keep its following chunks literal too.
                    self.literal_token = true;
                    return EngineAction::Commit(s);
                }
                self.push_undo();
                let origin = self
                    .undo
                    .get(self.undo.len().saturating_sub(2))
                    .cloned()
                    .unwrap_or_default();
                self.raw_buffer.push(ch);
                // Tech prefixes and already-literal tokens stay ASCII.
                // After Telex/VNI, rewind tentative marks once the token is no
                // longer a single open Vietnamese syllable (foreign shape or
                // glued multi-syllable). Only rewind when marks are present so
                // Unikey-style undo of `aa`/`ww` is preserved.
                if self.literal_token || is_tech_prefix(&self.raw_buffer, None) {
                    self.literal_token = true;
                    self.buffer.clone_from(&self.raw_buffer);
                } else {
                    match self.typing_method {
                        TypingMethod::Telex => apply_telex(&mut self.buffer, ch, &origin),
                        TypingMethod::Vni => apply_vni(&mut self.buffer, ch, &origin),
                    }
                    if last_syllable_start(&self.buffer) > 0
                        && self.buffer.iter().any(|c| !c.is_ascii())
                    {
                        self.literal_token = true;
                        self.buffer.clone_from(&self.raw_buffer);
                    }
                }
                if self.auto_repair && !self.literal_token {
                    repair_basic_errors(&mut self.buffer);
                }
                self.cursor = self.buffer.len();
                return EngineAction::Preedit(self.buffer_str());
            }
            if ch.is_ascii_digit() {
                let mut s = self.take_buffer();
                s.push(ch);
                return EngineAction::Commit(s);
            }
        }

        if !self.buffer.is_empty() {
            let s = self.take_buffer();
            return EngineAction::CommitThenPass(s);
        }

        EngineAction::PassThrough
    }

    fn on_backspace(&mut self) -> EngineAction {
        if self.buffer.is_empty() {
            return EngineAction::PassThrough;
        }
        if let Some(prev) = self.undo.pop() {
            self.buffer = prev;
        } else {
            pop_char(&mut self.buffer);
        }
        self.raw_buffer.pop();
        // Keep sticky-ASCII only while the remaining token still looks
        // foreign/tech; otherwise the next key can take marks again.
        self.literal_token = !self.raw_buffer.is_empty()
            && self.buffer == self.raw_buffer
            && (last_syllable_start(&self.buffer) > 0 || is_tech_prefix(&self.raw_buffer, None));
        self.cursor = self.buffer.len();
        if self.buffer.is_empty() {
            return EngineAction::Reset;
        }
        EngineAction::Preedit(self.buffer_str())
    }

    fn is_option_hotkey(&self, key: KeyEvent) -> bool {
        if !key.modifiers.control || !key.modifiers.alt || key.modifiers.super_key {
            return false;
        }
        matches!(key.keyval, 0x43 | 0x63 | 0x52 | 0x72)
    }

    fn is_method_hotkey(&self, key: KeyEvent) -> bool {
        if key.modifiers.super_key {
            return false;
        }
        if key.modifiers.control && key.modifiers.alt {
            return matches!(key.keyval, 0x54 | 0x74 | 0x56 | 0x76 | 0x4d | 0x6d);
        }
        // Ctrl+Shift+1/2: GDK sends '!' / '@' because Shift is held.
        if key.modifiers.control && key.modifiers.shift && !key.modifiers.alt {
            return is_telex_hotkey(key) || is_vni_hotkey(key);
        }
        false
    }

    fn is_mode_toggle(&self, key: KeyEvent) -> bool {
        if key.modifiers.super_key {
            return false;
        }
        if key.keyval == KEY::space && key.modifiers.shift && !key.modifiers.alt {
            return true;
        }
        if matches!(key.keyval, KEY::Pause | KEY::Scroll_Lock) && !key.modifiers.alt {
            return true;
        }
        if key.modifiers.alt {
            return false;
        }
        false
    }

    fn should_passthrough_shortcut(&self, key: KeyEvent) -> bool {
        if key.modifiers.super_key {
            return true;
        }
        if key.modifiers.control && key.modifiers.alt {
            return true;
        }
        if key.modifiers.alt {
            return true;
        }
        if key.modifiers.control {
            if key.is_shift() || key.is_control() {
                return false;
            }
            return true;
        }
        false
    }
}

fn is_telex_hotkey(key: KeyEvent) -> bool {
    matches!(key.keyval, 0x54 | 0x74 | 0x31 | 0x21)
}

fn is_vni_hotkey(key: KeyEvent) -> bool {
    matches!(key.keyval, 0x56 | 0x76 | 0x32 | 0x40)
}
