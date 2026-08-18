//! Last-syllable window (Unikey-style): do not apply marks to vowels already left behind.
//!
//! Returning `buf.len()` means "no syllable is open": the buffer cannot be a
//! Vietnamese syllable (foreign onset, coda or vowel cluster), so keys must be
//! kept as typed.

use crate::vowel::{is_d, is_dd, is_vowel, parse_vowel};

pub(crate) fn last_syllable_start(buf: &[char]) -> usize {
    let Some(last_v) = buf.iter().rposition(|&c| is_vowel(c)) else {
        return 0;
    };

    let mut cons: Vec<char> = Vec::new();
    let mut cons_idx: Vec<usize> = Vec::new();
    for i in last_v + 1..buf.len() {
        let Some(k) = consonant_key(buf[i]) else {
            return i;
        };
        cons.push(k);
        cons_idx.push(i);
    }

    if !cons.is_empty() {
        match longest_coda_prefix(&cons) {
            Some(coda_n) if coda_n < cons.len() => return cons_idx[coda_n],
            Some(_) => {}
            None => return buf.len(),
        }
    }

    let cluster = vowel_cluster_start(buf, last_v);
    let start = onset_start(buf, cluster);
    if !valid_onset(&buf[start..cluster]) || !valid_nucleus(buf, cluster, last_v) {
        return buf.len();
    }
    start
}

fn consonant_key(c: char) -> Option<char> {
    if is_vowel(c) {
        return None;
    }
    if is_d(c) || is_dd(c) {
        return Some('d');
    }
    let l = c.to_ascii_lowercase();
    if l.is_ascii_alphabetic() {
        Some(l)
    } else {
        None
    }
}

fn longest_coda_prefix(cons: &[char]) -> Option<usize> {
    const CODA: &[&[char]] = &[
        &['c', 'h'],
        &['n', 'h'],
        &['n', 'g'],
        &['c'],
        &['p'],
        &['t'],
        &['m'],
        &['n'],
    ];
    for c in CODA {
        if cons.starts_with(c) {
            return Some(c.len());
        }
    }
    None
}

/// Consonant clusters that can open a Vietnamese syllable. `gi`/`qu` are not
/// listed because their `i`/`u` is a vowel character and stays in the cluster.
fn valid_onset(onset: &[char]) -> bool {
    const ONSET: &[&[char]] = &[
        &['b'],
        &['c'],
        &['c', 'h'],
        &['d'],
        &['g'],
        &['g', 'h'],
        &['h'],
        &['k'],
        &['k', 'h'],
        &['l'],
        &['m'],
        &['n'],
        &['n', 'g'],
        &['n', 'g', 'h'],
        &['n', 'h'],
        &['p'],
        &['p', 'h'],
        &['q'],
        &['r'],
        &['s'],
        &['t'],
        &['t', 'h'],
        &['t', 'r'],
        &['v'],
        &['x'],
    ];
    if onset.is_empty() {
        return true;
    }
    if onset.len() > 3 {
        return false;
    }
    let mut key = ['\0'; 3];
    for (i, &c) in onset.iter().enumerate() {
        match consonant_key(c) {
            Some(k) => key[i] = k,
            None => return false,
        }
    }
    ONSET.iter().any(|o| *o == &key[..onset.len()])
}

/// Vowel clusters by base letter (shape and tone ignored, so `iê` counts as
/// `ie`). Marks arrive after their vowel, so half-typed clusters must pass too.
fn valid_nucleus(buf: &[char], cluster: usize, last_v: usize) -> bool {
    const NUCLEUS: &[&[char]] = &[
        &['a'],
        &['e'],
        &['i'],
        &['o'],
        &['u'],
        &['y'],
        &['a', 'i'],
        &['a', 'o'],
        &['a', 'u'],
        &['a', 'y'],
        &['e', 'o'],
        &['e', 'u'],
        &['i', 'a'],
        &['i', 'e'],
        &['i', 'u'],
        &['o', 'a'],
        &['o', 'e'],
        &['o', 'i'],
        &['o', 'o'],
        &['u', 'a'],
        &['u', 'e'],
        &['u', 'i'],
        &['u', 'o'],
        &['u', 'u'],
        &['u', 'y'],
        &['y', 'a'],
        &['y', 'e'],
        &['i', 'e', 'u'],
        &['o', 'a', 'i'],
        &['o', 'a', 'o'],
        &['o', 'a', 'y'],
        &['o', 'e', 'o'],
        &['u', 'a', 'i'],
        &['u', 'a', 'y'],
        &['u', 'o', 'i'],
        &['u', 'o', 'u'],
        &['u', 'y', 'a'],
        &['u', 'y', 'e'],
        &['u', 'y', 'u'],
        &['y', 'e', 'u'],
    ];
    let start = nucleus_start(buf, cluster);
    if start > last_v {
        return true;
    }
    let len = last_v + 1 - start;
    if len > 3 {
        return false;
    }
    let mut key = ['\0'; 3];
    for (i, &c) in buf[start..=last_v].iter().enumerate() {
        match parse_vowel(c) {
            Some(v) => key[i] = v.base,
            None => return false,
        }
    }
    NUCLEUS.iter().any(|n| *n == &key[..len])
}

/// `gi`/`qu`: the `i`/`u` belongs to the onset when another vowel follows.
fn nucleus_start(buf: &[char], cluster: usize) -> usize {
    if cluster != 1 || buf.len() < 3 {
        return cluster;
    }
    let head = buf[0].to_ascii_lowercase();
    let Some(v) = parse_vowel(buf[1]) else {
        return cluster;
    };
    let glide = (head == 'g' && v.base == 'i') || (head == 'q' && v.base == 'u');
    if glide && buf.iter().skip(2).any(|c| is_vowel(*c)) {
        return 2;
    }
    cluster
}

fn vowel_cluster_start(buf: &[char], last_v: usize) -> usize {
    let mut i = last_v;
    while i > 0 && is_vowel(buf[i - 1]) {
        i -= 1;
    }
    i
}

fn onset_start(buf: &[char], cluster_start: usize) -> usize {
    let mut i = cluster_start;
    let mut n = 0;
    while i > 0 && n < 3 {
        if is_vowel(buf[i - 1]) {
            break;
        }
        if consonant_key(buf[i - 1]).is_none() {
            break;
        }
        i -= 1;
        n += 1;
    }
    i
}
