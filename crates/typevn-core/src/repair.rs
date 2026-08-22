//! Small, deterministic repairs and user-editable macros.
//!
//! This module is intentionally separate from the key-to-mark conversion.
//! New compatibility repairs can be added here without making `telex.rs`
//! harder to reason about.

use crate::syllable::last_syllable_start;
use crate::vowel::{compose_vowel, parse_vowel, Shape, Tone};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const BUILTIN_MACROS: &str = include_str!("../../../data/macros.json");

#[derive(Debug, Default)]
pub(crate) struct MacroStore {
    entries: HashMap<String, String>,
}

impl MacroStore {
    /// Load the built-in community list, then overlay the user's list.
    /// Invalid or unreadable files are ignored so typing never fails to start.
    pub(crate) fn load() -> Self {
        let mut entries = parse_macros(BUILTIN_MACROS);
        if let Ok(text) = fs::read_to_string(user_macro_path()) {
            entries.extend(parse_macros(&text));
        }
        Self { entries }
    }

    pub(crate) fn replace(&self, raw: &[char], converted: String) -> String {
        let raw: String = raw.iter().collect();
        self.entries.get(&raw).cloned().unwrap_or(converted)
    }
}

fn parse_macros(text: &str) -> HashMap<String, String> {
    serde_json::from_str(text).unwrap_or_default()
}

fn user_macro_path() -> PathBuf {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let mut home = PathBuf::from(std::env::var_os("HOME").unwrap_or_default());
            home.push(".config");
            home
        });
    base.join("typevn").join("macros.json")
}

/// Repair `uă` into `ưa` after a user typed the horn/breve key at the end.
///
/// This is the common transitional form produced by `suawr`/`dduwas` while
/// the word is still being composed. It also moves a pending tone to `ư`.
pub(crate) fn repair_basic_errors(buf: &mut [char]) {
    if buf.len() < 2 {
        return;
    }
    let start = last_syllable_start(buf);
    if start + 1 >= buf.len() {
        return;
    }
    for i in start..buf.len() - 1 {
        let Some(vu) = parse_vowel(buf[i]) else {
            continue;
        };
        let Some(va) = parse_vowel(buf[i + 1]) else {
            continue;
        };
        if vu.base != 'u' || va.base != 'a' || va.shape != Shape::Breve {
            continue;
        }
        let mut nu = vu;
        let mut na = va;
        nu.shape = Shape::Horn;
        na.shape = Shape::Plain;
        if na.tone != Tone::None && nu.tone == Tone::None {
            nu.tone = na.tone;
            na.tone = Tone::None;
        }
        if let (Some(cu), Some(ca)) = (compose_vowel(nu), compose_vowel(na)) {
            buf[i] = cu;
            buf[i + 1] = ca;
        }
    }
}
