//! C ABI for the IBus adapter. Never logs typed content.

use crate::engine::{EngineAction, VietnameseEngine};
use crate::key::KeyEvent;
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::slice;

pub const TYPEVN_ACT_COMMIT: c_int = 0;
pub const TYPEVN_ACT_PREEDIT: c_int = 1;
pub const TYPEVN_ACT_PASSTHROUGH: c_int = 2;
pub const TYPEVN_ACT_DELETE: c_int = 3;
pub const TYPEVN_ACT_RESET: c_int = 4;
pub const TYPEVN_ACT_COMMIT_THEN_PASS: c_int = 5;
pub const TYPEVN_ACT_NOTIFY: c_int = 6;

#[no_mangle]
pub extern "C" fn typevn_engine_new() -> *mut VietnameseEngine {
    Box::into_raw(Box::new(VietnameseEngine::new()))
}

/// # Safety
/// `eng` must be a pointer from `typevn_engine_new` or null.
#[no_mangle]
pub unsafe extern "C" fn typevn_engine_free(eng: *mut VietnameseEngine) {
    if !eng.is_null() {
        drop(Box::from_raw(eng));
    }
}

/// # Safety
/// `eng` must be valid.
#[no_mangle]
pub unsafe extern "C" fn typevn_engine_reset(eng: *mut VietnameseEngine) {
    if let Some(e) = eng.as_mut() {
        e.reset();
    }
}

/// # Safety
/// `eng` valid; `out_text` has `out_cap` bytes; `delete_count` may be null.
#[no_mangle]
pub unsafe extern "C" fn typevn_process_key(
    eng: *mut VietnameseEngine,
    keyval: u32,
    keycode: u32,
    modifiers: u32,
    out_text: *mut c_char,
    out_cap: usize,
    delete_count: *mut u32,
) -> c_int {
    let Some(engine) = eng.as_mut() else {
        return TYPEVN_ACT_PASSTHROUGH;
    };

    let key = KeyEvent::from_ibus(keyval, keycode, modifiers);
    let action = engine.process_key(key);

    let (kind, text, del) = match action {
        EngineAction::Commit(s) => (TYPEVN_ACT_COMMIT, s, 0u32),
        EngineAction::Preedit(s) => (TYPEVN_ACT_PREEDIT, s, 0),
        EngineAction::PassThrough => (TYPEVN_ACT_PASSTHROUGH, String::new(), 0),
        EngineAction::Delete(n) => (TYPEVN_ACT_DELETE, String::new(), n as u32),
        EngineAction::Reset => (TYPEVN_ACT_RESET, String::new(), 0),
        EngineAction::CommitThenPass(s) => (TYPEVN_ACT_COMMIT_THEN_PASS, s, 0),
        EngineAction::Notify(s) => (TYPEVN_ACT_NOTIFY, s, 0),
    };

    if !delete_count.is_null() {
        *delete_count = del;
    }

    if !out_text.is_null() && out_cap > 0 {
        let bytes = text.as_bytes();
        let copy = bytes.len().min(out_cap.saturating_sub(1));
        ptr::copy_nonoverlapping(bytes.as_ptr(), out_text as *mut u8, copy);
        let rest = slice::from_raw_parts_mut(out_text.add(copy), out_cap - copy);
        rest.fill(0);
    }

    kind
}

#[no_mangle]
pub unsafe extern "C" fn typevn_engine_get_method(eng: *mut VietnameseEngine) -> c_int {
    match eng.as_ref() {
        Some(e) if e.typing_method() == crate::engine::TypingMethod::Vni => 1,
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn typevn_engine_get_english(eng: *mut VietnameseEngine) -> c_int {
    match eng.as_ref() {
        Some(e) if e.input_mode() == crate::engine::InputMode::English => 1,
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn typevn_engine_set_method(eng: *mut VietnameseEngine, vni: c_int) {
    if let Some(e) = eng.as_mut() {
        e.set_typing_method(if vni != 0 {
            crate::engine::TypingMethod::Vni
        } else {
            crate::engine::TypingMethod::Telex
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn typevn_engine_set_english(eng: *mut VietnameseEngine, en: c_int) {
    if let Some(e) = eng.as_mut() {
        e.set_input_mode(if en != 0 {
            crate::engine::InputMode::English
        } else {
            crate::engine::InputMode::Vietnamese
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn typevn_engine_reload(eng: *mut VietnameseEngine) {
    if let Some(e) = eng.as_mut() {
        e.reload_config();
    }
}

#[no_mangle]
pub extern "C" fn typevn_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}
