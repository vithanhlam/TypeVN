//! Output charset. Composition stays Unicode in RAM; convert only when emitting.

use crate::vowel::{parse_vowel, Shape, Tone};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Charset {
    Unicode,
    VniWindows,
    Viqr,
    Tcvn3,
}

impl Charset {
    pub fn name(self) -> &'static str {
        match self {
            Self::Unicode => "Unicode",
            Self::VniWindows => "VNI",
            Self::Viqr => "VIQR",
            Self::Tcvn3 => "TCVN3",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Unicode => Self::VniWindows,
            Self::VniWindows => Self::Viqr,
            Self::Tcvn3 => Self::Unicode,
            Self::Viqr => Self::Tcvn3,
        }
    }

    pub fn encode(self, s: &str) -> String {
        match self {
            Self::Unicode => s.to_string(),
            Self::VniWindows => s.chars().map(to_vni_win).collect(),
            Self::Viqr => s.chars().map(to_viqr).collect(),
            Self::Tcvn3 => s.chars().map(to_tcvn3).collect(),
        }
    }
}

fn to_viqr(c: char) -> String {
    if matches!(c, 'đ') {
        return "dd".into();
    }
    if matches!(c, 'Đ') {
        return "DD".into();
    }
    let Some(v) = parse_vowel(c) else {
        return c.to_string();
    };
    let mut o = String::new();
    let base = if v.upper {
        v.base.to_ascii_uppercase()
    } else {
        v.base
    };
    o.push(base);
    match v.shape {
        Shape::Circumflex => o.push('^'),
        Shape::Breve => o.push('('),
        Shape::Horn => o.push('+'),
        Shape::Plain => {}
    }
    match v.tone {
        Tone::None => {}
        Tone::Acute => o.push('\''),
        Tone::Grave => o.push('`'),
        Tone::Hook => o.push('?'),
        Tone::Tilde => o.push('~'),
        Tone::Dot => o.push('.'),
    }
    o
}

fn to_vni_win(c: char) -> String {
    if c == 'đ' {
        return "ñ".into();
    }
    if c == 'Đ' {
        return "Ñ".into();
    }
    let Some(v) = parse_vowel(c) else {
        return c.to_string();
    };
    let base = if v.upper {
        v.base.to_ascii_uppercase()
    } else {
        v.base
    };
    let pair = match (v.shape, v.tone) {
        (Shape::Plain, Tone::None) => return base.to_string(),
        (Shape::Plain, Tone::Acute) => "ù",
        (Shape::Plain, Tone::Grave) => "ø",
        (Shape::Plain, Tone::Hook) => "û",
        (Shape::Plain, Tone::Tilde) => "õ",
        (Shape::Plain, Tone::Dot) => "ï",
        (Shape::Circumflex, Tone::None) => "ê",
        (Shape::Circumflex, Tone::Acute) => "á",
        (Shape::Circumflex, Tone::Grave) => "à",
        (Shape::Circumflex, Tone::Hook) => "å",
        (Shape::Circumflex, Tone::Tilde) => "ã",
        (Shape::Circumflex, Tone::Dot) => "ä",
        (Shape::Breve, Tone::None) => "e",
        (Shape::Breve, Tone::Acute) => "é",
        (Shape::Breve, Tone::Grave) => "è",
        (Shape::Breve, Tone::Hook) => "ú",
        (Shape::Breve, Tone::Tilde) => "ü",
        (Shape::Breve, Tone::Dot) => "ë",
        (Shape::Horn, Tone::None) => {
            return if v.base == 'u' {
                if v.upper { "Ö".into() } else { "ö".into() }
            } else if v.upper {
                "Ô".into()
            } else {
                "ô".into()
            };
        }
        (Shape::Horn, Tone::Acute) => "í",
        (Shape::Horn, Tone::Grave) => "ì",
        (Shape::Horn, Tone::Hook) => "û",
        (Shape::Horn, Tone::Tilde) => "ó",
        (Shape::Horn, Tone::Dot) => "ò",
    };
    let mut o = String::new();
    o.push(base);
    o.push_str(pair);
    o
}

/// TCVN3 (VN3) lowercase-oriented mapping; other chars pass through as Unicode.
fn to_tcvn3(c: char) -> String {
    let mapped = match c {
        'à' => '\u{00b5}',
        'á' => '\u{00b8}',
        'ả' => '\u{00b6}',
        'ã' => '\u{00b7}',
        'ạ' => '\u{00b9}',
        'ă' => '\u{00a8}',
        'ằ' => '\u{00bb}',
        'ắ' => '\u{00be}',
        'ẳ' => '\u{00bc}',
        'ẵ' => '\u{00bd}',
        'ặ' => '\u{00c6}',
        'â' => '\u{00a9}',
        'ầ' => '\u{00c7}',
        'ấ' => '\u{00ca}',
        'ẩ' => '\u{00c8}',
        'ẫ' => '\u{00c9}',
        'ậ' => '\u{00cb}',
        'è' => '\u{00cc}',
        'é' => '\u{00d0}',
        'ẻ' => '\u{00ce}',
        'ẽ' => '\u{00cf}',
        'ẹ' => '\u{00d1}',
        'ê' => '\u{00aa}',
        'ề' => '\u{00d2}',
        'ế' => '\u{00d5}',
        'ể' => '\u{00d3}',
        'ễ' => '\u{00d4}',
        'ệ' => '\u{00d6}',
        'ì' => '\u{00d7}',
        'í' => '\u{00dd}',
        'ỉ' => '\u{00d8}',
        'ĩ' => '\u{00dc}',
        'ị' => '\u{00de}',
        'ò' => '\u{00df}',
        'ó' => '\u{00e3}',
        'ỏ' => '\u{00e1}',
        'õ' => '\u{00e2}',
        'ọ' => '\u{00e4}',
        'ô' => '\u{00ab}',
        'ồ' => '\u{00e5}',
        'ố' => '\u{00e8}',
        'ổ' => '\u{00e6}',
        'ỗ' => '\u{00e7}',
        'ộ' => '\u{00e9}',
        'ơ' => '\u{00ac}',
        'ờ' => '\u{00ea}',
        'ớ' => '\u{00ed}',
        'ở' => '\u{00eb}',
        'ỡ' => '\u{00ec}',
        'ợ' => '\u{00ee}',
        'ù' => '\u{00ef}',
        'ú' => '\u{00f3}',
        'ủ' => '\u{00f1}',
        'ũ' => '\u{00f2}',
        'ụ' => '\u{00f4}',
        'ư' => '\u{00ad}',
        'ừ' => '\u{00f5}',
        'ứ' => '\u{00f8}',
        'ử' => '\u{00f6}',
        'ữ' => '\u{00f7}',
        'ự' => '\u{00f9}',
        'ỳ' => '\u{00fa}',
        'ý' => '\u{00fd}',
        'ỷ' => '\u{00fb}',
        'ỹ' => '\u{00fc}',
        'ỵ' => '\u{00fe}',
        'đ' => '\u{00ae}',
        'Ă' => '\u{00a1}',
        'Â' => '\u{00a2}',
        'Ê' => '\u{00a3}',
        'Ô' => '\u{00a4}',
        'Ơ' => '\u{00a5}',
        'Ư' => '\u{00a6}',
        'Đ' => '\u{00a7}',
        _ => c,
    };
    mapped.to_string()
}
