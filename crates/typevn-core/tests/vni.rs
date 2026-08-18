use typevn_core::{EngineAction, InputMode, KeyEvent, Modifiers, TypingMethod, VietnameseEngine, KEY};

fn feed(eng: &mut VietnameseEngine, s: &str) -> String {
    let mut committed = String::new();
    for c in s.chars() {
        match eng.process_key(KeyEvent::from_char(c)) {
            EngineAction::Commit(t) | EngineAction::CommitThenPass(t) => committed.push_str(&t),
            _ => {}
        }
    }
    committed.push_str(&eng.buffer_str());
    committed
}

/// Engine that ignores the config of the machine running the tests.
fn engine() -> VietnameseEngine {
    let mut eng = VietnameseEngine::new();
    eng.set_input_mode(InputMode::Vietnamese);
    eng.set_typing_method(TypingMethod::Telex);
    eng.set_charset(typevn_core::Charset::Unicode);
    eng.set_auto_repair(true);
    eng
}

fn vni(s: &str) -> String {
    let mut eng = engine();
    eng.set_typing_method(TypingMethod::Vni);
    feed(&mut eng, s)
}

#[test]
fn vni_basic() {
    assert_eq!(vni("a1"), "á");
    assert_eq!(vni("a2"), "à");
    assert_eq!(vni("a3"), "ả");
    assert_eq!(vni("a4"), "ã");
    assert_eq!(vni("a5"), "ạ");
    assert_eq!(vni("a6"), "â");
    assert_eq!(vni("a7"), "ă");
    assert_eq!(vni("u8"), "ư");
    assert_eq!(vni("u88"), "u8");
    assert_eq!(vni("o7"), "ơ");
    assert_eq!(vni("d9"), "đ");
}

#[test]
fn vni_words() {
    assert_eq!(vni("tie6ng1"), "tiếng");
    assert_eq!(vni("Vie6t5"), "Việt");
    assert_eq!(vni("d9uo7c5"), "được");
    assert_eq!(vni("d9uo7ng2"), "đường");
    assert_eq!(vni("nguo7i2"), "người");
}

#[test]
fn vni_zero_strips_tone() {
    assert_eq!(vni("a10"), "a");
}

#[test]
fn switch_method_hotkey() {
    let mut eng = engine();
    assert_eq!(eng.typing_method(), TypingMethod::Telex);
    let key = KeyEvent::new(
        0x76,
        0,
        Modifiers {
            control: true,
            alt: true,
            ..Modifiers::default()
        },
    );
    let _ = eng.process_key(key);
    assert_eq!(eng.typing_method(), TypingMethod::Vni);
}

#[test]
fn shift_space_toggles_viet_anh() {
    let mut eng = engine();
    assert_eq!(eng.input_mode(), InputMode::Vietnamese);
    let key = KeyEvent::new(
        KEY::space,
        0,
        Modifiers {
            shift: true,
            ..Modifiers::default()
        },
    );
    let _ = eng.process_key(key);
    assert_eq!(eng.input_mode(), InputMode::English);
    let _ = eng.process_key(key);
    assert_eq!(eng.input_mode(), InputMode::Vietnamese);
}

#[test]
fn ctrl_shift_1_selects_telex() {
    let mut eng = engine();
    eng.set_typing_method(TypingMethod::Vni);
    let key = KeyEvent::new(
        0x21,
        0,
        Modifiers {
            control: true,
            shift: true,
            ..Modifiers::default()
        },
    );
    let _ = eng.process_key(key);
    assert_eq!(eng.typing_method(), TypingMethod::Telex);
}

#[test]
fn pause_toggles_viet_anh() {
    let mut eng = engine();
    let key = KeyEvent::new(KEY::Pause, 0, Modifiers::default());
    let _ = eng.process_key(key);
    assert_eq!(eng.input_mode(), InputMode::English);
}
