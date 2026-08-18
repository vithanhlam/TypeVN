//! VNI: 1-5 tones, 6 â/ê/ô, 7 ă/ơ/ươ, 8 ư, 9 đ, 0 strip tone.

use crate::english::is_tech_prefix;
use crate::syllable::last_syllable_start;
use crate::telex::{
    apply_circumflex, apply_dd, apply_horn_or_breve, apply_tone, merge_uhorn_o, strip_tones,
    undo_repeat_mark,
};
use crate::vowel::{parse_vowel, tone_from_vni};
use crate::MAX_COMPOSE_CHARS;

pub fn apply_vni(buf: &mut Vec<char>, c: char, origin: &[char]) {
    if buf.len() >= MAX_COMPOSE_CHARS {
        buf.push(c);
        if buf.len() > MAX_COMPOSE_CHARS {
            buf.truncate(MAX_COMPOSE_CHARS);
        }
        return;
    }

    if is_tech_prefix(buf, Some(c)) {
        buf.push(c);
        return;
    }

    if last_syllable_start(buf) > 0 {
        buf.push(c);
        return;
    }

    if undo_repeat_mark(buf, c, origin, apply_vni_raw) {
        return;
    }

    apply_vni_raw(buf, c);
}

fn apply_vni_raw(buf: &mut Vec<char>, c: char) {
    if c == '0' {
        if strip_tones(buf) {
            return;
        }
        buf.push(c);
        return;
    }

    if let Some(tone) = tone_from_vni(c) {
        if apply_tone(buf, tone) {
            return;
        }
        buf.push(c);
        return;
    }

    match c {
        '6' => {
            if apply_vni_circumflex(buf) {
                return;
            }
        }
        '7' => {
            if apply_horn_or_breve(buf) {
                return;
            }
        }
        '8' => {
            if apply_vni_uhorn(buf) {
                return;
            }
        }
        '9' => {
            if apply_dd(buf, 'd') {
                return;
            }
        }
        'o' | 'O' => {
            if merge_uhorn_o(buf, c) {
                return;
            }
            buf.push(c);
            return;
        }
        _ => {
            buf.push(c);
            return;
        }
    }

    buf.push(c);
}

fn apply_vni_circumflex(buf: &mut [char]) -> bool {
    let start = last_syllable_start(buf);
    for i in (start..buf.len()).rev() {
        if let Some(v) = parse_vowel(buf[i]) {
            if matches!(v.base, 'a' | 'e' | 'o') {
                return apply_circumflex(buf, v.base, false);
            }
        }
    }
    false
}

fn apply_vni_uhorn(buf: &mut [char]) -> bool {
    let start = last_syllable_start(buf);
    for i in (start..buf.len()).rev() {
        if parse_vowel(buf[i]).is_some_and(|v| v.base == 'u') {
            return apply_uhorn_at(buf, i);
        }
    }
    false
}

fn apply_uhorn_at(buf: &mut [char], i: usize) -> bool {
    let Some(mut v) = parse_vowel(buf[i]) else {
        return false;
    };
    if v.base != 'u' {
        return false;
    }
    if v.shape == crate::vowel::Shape::Horn {
        v.shape = crate::vowel::Shape::Plain;
        if let Some(ch) = crate::vowel::compose_vowel(v) {
            buf[i] = ch;
        }
        return false;
    }
    if v.shape != crate::vowel::Shape::Plain {
        return false;
    }
    v.shape = crate::vowel::Shape::Horn;
    match crate::vowel::compose_vowel(v) {
        Some(ch) => {
            buf[i] = ch;
            true
        }
        None => false,
    }
}
