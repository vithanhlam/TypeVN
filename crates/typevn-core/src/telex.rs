//! Telex transformations on an in-memory char buffer.

use crate::english::is_tech_prefix;
use crate::syllable::last_syllable_start;
use crate::vowel::{
    compose_vowel, from_dd, has_special_shape, is_d, is_dd, is_vowel, parse_vowel, to_dd,
    tone_from_telex, Shape, Tone,
};
use crate::MAX_COMPOSE_CHARS;

pub fn apply_telex(buf: &mut Vec<char>, c: char, origin: &[char]) {
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

    if undo_repeat_mark(buf, c, origin, apply_telex_raw) {
        return;
    }

    apply_telex_raw(buf, c);
}

pub(crate) fn undo_repeat_mark(
    buf: &mut Vec<char>,
    c: char,
    origin: &[char],
    raw: fn(&mut Vec<char>, char),
) -> bool {
    if buf.is_empty() {
        return false;
    }
    let mut probe = origin.to_vec();
    raw(&mut probe, c);
    if probe.as_slice() != buf.as_slice() {
        return false;
    }
    let in_place = origin.len() == buf.len() && origin != buf;
    let bare_w = origin.is_empty()
        && buf.len() == 1
        && c.to_ascii_lowercase() == 'w'
        && parse_vowel(buf[0]).is_some_and(|v| v.base == 'u' && v.shape == Shape::Horn);
    if !in_place && !bare_w {
        return false;
    }
    buf.clear();
    buf.extend_from_slice(origin);
    buf.push(c);
    true
}

fn apply_telex_raw(buf: &mut Vec<char>, c: char) {
    let lower = c.to_ascii_lowercase();

    if lower == 'z' {
        if strip_tones(buf) {
            return;
        }
        buf.push(c);
        return;
    }

    if let Some(tone) = tone_from_telex(c) {
        if apply_tone(buf, tone) {
            return;
        }
        buf.push(c);
        return;
    }

    match lower {
        'd' => {
            if apply_dd(buf, c) {
                return;
            }
        }
        'a' | 'e' | 'o' => {
            if lower == 'o' && merge_uhorn_o(buf, c) {
                return;
            }
            if apply_circumflex(buf, lower, c.is_uppercase()) {
                return;
            }
        }
        'w' => {
            if apply_horn_or_breve(buf) {
                return;
            }
            if buf.is_empty() {
                buf.push(if c.is_uppercase() { 'Ư' } else { 'ư' });
                return;
            }
            let last = *buf.last().unwrap();
            if last == 'w' || last == 'W' {
                buf.push(c);
                return;
            }
            if !is_vowel(last) {
                buf.push(if c.is_uppercase() { 'Ư' } else { 'ư' });
                return;
            }
        }
        _ => {}
    }

    buf.push(c);
}

pub(crate) fn apply_dd(buf: &mut [char], typed: char) -> bool {
    let start = last_syllable_start(buf);
    for i in (start..buf.len()).rev() {
        if is_d(buf[i]) {
            if is_dd(buf[i]) {
                buf[i] = from_dd(buf[i]);
                return false;
            }
            buf[i] = if typed.is_uppercase() || buf[i].is_uppercase() {
                'Đ'
            } else {
                to_dd(buf[i])
            };
            return true;
        }
        if is_vowel(buf[i]) {
            break;
        }
    }
    // `dud` → `đu` so `dudowcj` → được (đ at onset, second d is the Telex mark).
    if start < buf.len() {
        let first = &mut buf[start];
        if is_d(*first) && !is_dd(*first) {
            *first = if typed.is_uppercase() || first.is_uppercase() {
                'Đ'
            } else {
                to_dd(*first)
            };
            return true;
        }
    }
    false
}

/// `ư` + `o` → `ươ` (`ddwocj` → được).
pub(crate) fn merge_uhorn_o(buf: &mut Vec<char>, typed_o: char) -> bool {
    let Some(&last) = buf.last() else {
        return false;
    };
    let Some(v) = parse_vowel(last) else {
        return false;
    };
    if v.base != 'u' || v.shape != Shape::Horn {
        return false;
    }
    let o = crate::vowel::Vowel {
        base: 'o',
        shape: Shape::Horn,
        tone: Tone::None,
        upper: typed_o.is_uppercase(),
    };
    let Some(ch) = compose_vowel(o) else {
        return false;
    };
    buf.push(ch);
    true
}

pub(crate) fn apply_tone(buf: &mut [char], tone: Tone) -> bool {
    let Some(idx) = tone_target(buf) else {
        return false;
    };
    let Some(mut v) = parse_vowel(buf[idx]) else {
        return false;
    };
    if v.tone == tone {
        v.tone = Tone::None;
        if let Some(ch) = compose_vowel(v) {
            buf[idx] = ch;
        }
        return false;
    }
    v.tone = tone;
    match compose_vowel(v) {
        Some(ch) => {
            buf[idx] = ch;
            true
        }
        None => false,
    }
}

pub(crate) fn strip_tones(buf: &mut [char]) -> bool {
    let start = last_syllable_start(buf);
    let mut changed = false;
    for ch in buf[start..].iter_mut() {
        if let Some(mut v) = parse_vowel(*ch) {
            if v.tone != Tone::None {
                v.tone = Tone::None;
                if let Some(composed) = compose_vowel(v) {
                    *ch = composed;
                    changed = true;
                }
            }
        }
    }
    changed
}

pub(crate) fn apply_circumflex(buf: &mut [char], letter: char, upper: bool) -> bool {
    let Some(idx) = shape_target(buf, letter) else {
        return false;
    };
    let Some(mut v) = parse_vowel(buf[idx]) else {
        return false;
    };
    if v.base != letter {
        return false;
    }
    if v.shape == Shape::Circumflex {
        v.shape = Shape::Plain;
        if let Some(ch) = compose_vowel(v) {
            buf[idx] = ch;
        }
        return false;
    }
    if v.shape != Shape::Plain {
        return false;
    }
    v.shape = Shape::Circumflex;
    if upper {
        v.upper = true;
    }
    match compose_vowel(v) {
        Some(ch) => {
            buf[idx] = ch;
            true
        }
        None => false,
    }
}

pub(crate) fn apply_horn_or_breve(buf: &mut [char]) -> bool {
    // uo + w → ươ ; ươ + w → keep ươ (do not undo / append)
    if buf.len() >= 2 {
        let i = buf.len() - 1;
        let j = buf.len() - 2;
        if let (Some(vo), Some(vu)) = (parse_vowel(buf[i]), parse_vowel(buf[j])) {
            if vu.base == 'u' && vo.base == 'o' {
                if vu.shape == Shape::Horn && vo.shape == Shape::Horn {
                    return true;
                }
                if vo.shape == Shape::Plain
                    && matches!(vu.shape, Shape::Plain | Shape::Horn)
                {
                    let mut nu = vu;
                    let mut no = vo;
                    nu.shape = Shape::Horn;
                    no.shape = Shape::Horn;
                    if let (Some(cu), Some(co)) = (compose_vowel(nu), compose_vowel(no)) {
                        buf[j] = cu;
                        buf[i] = co;
                        return true;
                    }
                }
            }
            if vu.base == 'u' && vo.base == 'a' {
                if vu.shape == Shape::Horn && vo.shape == Shape::Plain {
                    return true;
                }
                if matches!(vu.shape, Shape::Plain | Shape::Horn)
                    && matches!(vo.shape, Shape::Plain | Shape::Breve)
                {
                    let mut nu = vu;
                    let mut na = vo;
                    nu.shape = Shape::Horn;
                    na.shape = Shape::Plain;
                    if na.tone != Tone::None && nu.tone == Tone::None {
                        nu.tone = na.tone;
                        na.tone = Tone::None;
                    }
                    if let (Some(cu), Some(ca)) = (compose_vowel(nu), compose_vowel(na)) {
                        buf[j] = cu;
                        buf[i] = ca;
                        return true;
                    }
                }
            }
        }
    }

    let start = last_syllable_start(buf);
    for i in (start..buf.len()).rev() {
        let Some(mut v) = parse_vowel(buf[i]) else {
            break;
        };
        match v.base {
            'a' => {
                if v.shape == Shape::Breve {
                    v.shape = Shape::Plain;
                    if let Some(ch) = compose_vowel(v) {
                        buf[i] = ch;
                    }
                    return false;
                }
                if v.shape != Shape::Plain {
                    break;
                }
                v.shape = Shape::Breve;
                if let Some(ch) = compose_vowel(v) {
                    buf[i] = ch;
                    return true;
                }
            }
            'o' | 'u' => {
                if v.shape == Shape::Horn {
                    v.shape = Shape::Plain;
                    if let Some(ch) = compose_vowel(v) {
                        buf[i] = ch;
                    }
                    return false;
                }
                if v.shape != Shape::Plain {
                    break;
                }
                v.shape = Shape::Horn;
                if let Some(ch) = compose_vowel(v) {
                    buf[i] = ch;
                    return true;
                }
            }
            _ => break,
        }
    }
    false
}

pub(crate) fn auto_repair_marks(buf: &mut [char]) {
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
        if vu.base == 'u' && va.base == 'a' && va.shape == Shape::Breve {
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
}

fn vowel_indices(buf: &[char]) -> Vec<usize> {
    let skip_i = gi_onset_skip_i(buf);
    let skip_u = qu_onset_skip_u(buf);
    buf.iter()
        .enumerate()
        .filter(|(i, c)| {
            if skip_i && *i == 1 {
                return false;
            }
            if skip_u && *i == 1 {
                return false;
            }
            is_vowel(**c)
        })
        .map(|(i, _)| i)
        .collect()
}

fn gi_onset_skip_i(buf: &[char]) -> bool {
    if buf.len() < 3 {
        return false;
    }
    let g = buf[0].to_ascii_lowercase();
    let i = buf[1];
    if g != 'g' {
        return false;
    }
    let Some(v) = parse_vowel(i) else {
        return false;
    };
    if v.base != 'i' {
        return false;
    }
    buf.iter().skip(2).any(|c| is_vowel(*c))
}

fn qu_onset_skip_u(buf: &[char]) -> bool {
    if buf.len() < 3 {
        return false;
    }
    if buf[0].to_ascii_lowercase() != 'q' {
        return false;
    }
    let Some(v) = parse_vowel(buf[1]) else {
        return false;
    };
    if v.base != 'u' {
        return false;
    }
    buf.iter().skip(2).any(|c| is_vowel(*c))
}

/// New-style tone placement. `gi`/`qu` are onsets, not vowels.
fn tone_target(buf: &[char]) -> Option<usize> {
    let start = last_syllable_start(buf);
    tone_target_in(&buf[start..]).map(|i| i + start)
}

fn tone_target_in(buf: &[char]) -> Option<usize> {
    let skip_i = gi_onset_skip_i(buf);
    let skip_u = qu_onset_skip_u(buf);
    let mut specials: Vec<usize> = Vec::with_capacity(4);
    for (i, &c) in buf.iter().enumerate() {
        if skip_i && i == 1 {
            continue;
        }
        if skip_u && i == 1 {
            continue;
        }
        if has_special_shape(c) {
            specials.push(i);
        }
    }
    if !specials.is_empty() {
        for &i in &specials {
            if let Some(v) = parse_vowel(buf[i]) {
                if v.base == 'o' && v.shape == Shape::Horn {
                    return Some(i);
                }
            }
        }
        return Some(specials[0]);
    }
    let idx = vowel_indices(buf);
    match idx.len() {
        0 => None,
        1 => Some(idx[0]),
        _ => {
            let bases: Vec<char> = idx
                .iter()
                .filter_map(|&i| parse_vowel(buf[i]).map(|v| v.base))
                .collect();
            if bases.len() >= 2 {
                let n = bases.len();
                let p = (bases[n - 2], bases[n - 1]);
                // oa/oe/uy: dấu trên nguyên âm sau (hoà, khoẻ, thủy)
                if matches!(p, ('o', 'a') | ('o', 'e') | ('u', 'y')) {
                    return Some(idx[n - 1]);
                }
                // ua/ia/ya: dấu trên u/i (của, tía) — không phải cuả
                if matches!(p, ('u', 'a') | ('i', 'a') | ('y', 'a')) {
                    return Some(idx[n - 2]);
                }
            }
            let last_base = *bases.last()?;
            if matches!(last_base, 'i' | 'o' | 'u' | 'y') && bases.len() >= 2 {
                return Some(idx[idx.len() - 2]);
            }
            Some(*idx.last()?)
        }
    }
}

fn shape_target(buf: &[char], letter: char) -> Option<usize> {
    let start = last_syllable_start(buf);
    for i in (start..buf.len()).rev() {
        if let Some(v) = parse_vowel(buf[i]) {
            if v.base == letter {
                return Some(i);
            }
        }
    }
    None
}

pub fn pop_char(buf: &mut Vec<char>) -> bool {
    buf.pop().is_some()
}
