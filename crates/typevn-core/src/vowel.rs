//! Compact vowel + tone tables. All lookups are O(1) table scans of ~90 entries.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    None,
    Acute, // sắc
    Grave, // huyền
    Hook,  // hỏi
    Tilde, // ngã
    Dot,   // nặng
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Plain,
    Circumflex, // â ê ô
    Breve,      // ă
    Horn,       // ơ ư
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vowel {
    pub base: char,
    pub shape: Shape,
    pub tone: Tone,
    pub upper: bool,
}

const VOWELS: &[(char, Vowel)] = &{
    use Shape::*;
    use Tone::*;
    [
        ('a', Vowel { base: 'a', shape: Plain, tone: None, upper: false }),
        ('A', Vowel { base: 'a', shape: Plain, tone: None, upper: true }),
        ('á', Vowel { base: 'a', shape: Plain, tone: Acute, upper: false }),
        ('Á', Vowel { base: 'a', shape: Plain, tone: Acute, upper: true }),
        ('à', Vowel { base: 'a', shape: Plain, tone: Grave, upper: false }),
        ('À', Vowel { base: 'a', shape: Plain, tone: Grave, upper: true }),
        ('ả', Vowel { base: 'a', shape: Plain, tone: Hook, upper: false }),
        ('Ả', Vowel { base: 'a', shape: Plain, tone: Hook, upper: true }),
        ('ã', Vowel { base: 'a', shape: Plain, tone: Tilde, upper: false }),
        ('Ã', Vowel { base: 'a', shape: Plain, tone: Tilde, upper: true }),
        ('ạ', Vowel { base: 'a', shape: Plain, tone: Dot, upper: false }),
        ('Ạ', Vowel { base: 'a', shape: Plain, tone: Dot, upper: true }),
        ('ă', Vowel { base: 'a', shape: Breve, tone: None, upper: false }),
        ('Ă', Vowel { base: 'a', shape: Breve, tone: None, upper: true }),
        ('ắ', Vowel { base: 'a', shape: Breve, tone: Acute, upper: false }),
        ('Ắ', Vowel { base: 'a', shape: Breve, tone: Acute, upper: true }),
        ('ằ', Vowel { base: 'a', shape: Breve, tone: Grave, upper: false }),
        ('Ằ', Vowel { base: 'a', shape: Breve, tone: Grave, upper: true }),
        ('ẳ', Vowel { base: 'a', shape: Breve, tone: Hook, upper: false }),
        ('Ẳ', Vowel { base: 'a', shape: Breve, tone: Hook, upper: true }),
        ('ẵ', Vowel { base: 'a', shape: Breve, tone: Tilde, upper: false }),
        ('Ẵ', Vowel { base: 'a', shape: Breve, tone: Tilde, upper: true }),
        ('ặ', Vowel { base: 'a', shape: Breve, tone: Dot, upper: false }),
        ('Ặ', Vowel { base: 'a', shape: Breve, tone: Dot, upper: true }),
        ('â', Vowel { base: 'a', shape: Circumflex, tone: None, upper: false }),
        ('Â', Vowel { base: 'a', shape: Circumflex, tone: None, upper: true }),
        ('ấ', Vowel { base: 'a', shape: Circumflex, tone: Acute, upper: false }),
        ('Ấ', Vowel { base: 'a', shape: Circumflex, tone: Acute, upper: true }),
        ('ầ', Vowel { base: 'a', shape: Circumflex, tone: Grave, upper: false }),
        ('Ầ', Vowel { base: 'a', shape: Circumflex, tone: Grave, upper: true }),
        ('ẩ', Vowel { base: 'a', shape: Circumflex, tone: Hook, upper: false }),
        ('Ẩ', Vowel { base: 'a', shape: Circumflex, tone: Hook, upper: true }),
        ('ẫ', Vowel { base: 'a', shape: Circumflex, tone: Tilde, upper: false }),
        ('Ẫ', Vowel { base: 'a', shape: Circumflex, tone: Tilde, upper: true }),
        ('ậ', Vowel { base: 'a', shape: Circumflex, tone: Dot, upper: false }),
        ('Ậ', Vowel { base: 'a', shape: Circumflex, tone: Dot, upper: true }),
        ('e', Vowel { base: 'e', shape: Plain, tone: None, upper: false }),
        ('E', Vowel { base: 'e', shape: Plain, tone: None, upper: true }),
        ('é', Vowel { base: 'e', shape: Plain, tone: Acute, upper: false }),
        ('É', Vowel { base: 'e', shape: Plain, tone: Acute, upper: true }),
        ('è', Vowel { base: 'e', shape: Plain, tone: Grave, upper: false }),
        ('È', Vowel { base: 'e', shape: Plain, tone: Grave, upper: true }),
        ('ẻ', Vowel { base: 'e', shape: Plain, tone: Hook, upper: false }),
        ('Ẻ', Vowel { base: 'e', shape: Plain, tone: Hook, upper: true }),
        ('ẽ', Vowel { base: 'e', shape: Plain, tone: Tilde, upper: false }),
        ('Ẽ', Vowel { base: 'e', shape: Plain, tone: Tilde, upper: true }),
        ('ẹ', Vowel { base: 'e', shape: Plain, tone: Dot, upper: false }),
        ('Ẹ', Vowel { base: 'e', shape: Plain, tone: Dot, upper: true }),
        ('ê', Vowel { base: 'e', shape: Circumflex, tone: None, upper: false }),
        ('Ê', Vowel { base: 'e', shape: Circumflex, tone: None, upper: true }),
        ('ế', Vowel { base: 'e', shape: Circumflex, tone: Acute, upper: false }),
        ('Ế', Vowel { base: 'e', shape: Circumflex, tone: Acute, upper: true }),
        ('ề', Vowel { base: 'e', shape: Circumflex, tone: Grave, upper: false }),
        ('Ề', Vowel { base: 'e', shape: Circumflex, tone: Grave, upper: true }),
        ('ể', Vowel { base: 'e', shape: Circumflex, tone: Hook, upper: false }),
        ('Ể', Vowel { base: 'e', shape: Circumflex, tone: Hook, upper: true }),
        ('ễ', Vowel { base: 'e', shape: Circumflex, tone: Tilde, upper: false }),
        ('Ễ', Vowel { base: 'e', shape: Circumflex, tone: Tilde, upper: true }),
        ('ệ', Vowel { base: 'e', shape: Circumflex, tone: Dot, upper: false }),
        ('Ệ', Vowel { base: 'e', shape: Circumflex, tone: Dot, upper: true }),
        ('i', Vowel { base: 'i', shape: Plain, tone: None, upper: false }),
        ('I', Vowel { base: 'i', shape: Plain, tone: None, upper: true }),
        ('í', Vowel { base: 'i', shape: Plain, tone: Acute, upper: false }),
        ('Í', Vowel { base: 'i', shape: Plain, tone: Acute, upper: true }),
        ('ì', Vowel { base: 'i', shape: Plain, tone: Grave, upper: false }),
        ('Ì', Vowel { base: 'i', shape: Plain, tone: Grave, upper: true }),
        ('ỉ', Vowel { base: 'i', shape: Plain, tone: Hook, upper: false }),
        ('Ỉ', Vowel { base: 'i', shape: Plain, tone: Hook, upper: true }),
        ('ĩ', Vowel { base: 'i', shape: Plain, tone: Tilde, upper: false }),
        ('Ĩ', Vowel { base: 'i', shape: Plain, tone: Tilde, upper: true }),
        ('ị', Vowel { base: 'i', shape: Plain, tone: Dot, upper: false }),
        ('Ị', Vowel { base: 'i', shape: Plain, tone: Dot, upper: true }),
        ('o', Vowel { base: 'o', shape: Plain, tone: None, upper: false }),
        ('O', Vowel { base: 'o', shape: Plain, tone: None, upper: true }),
        ('ó', Vowel { base: 'o', shape: Plain, tone: Acute, upper: false }),
        ('Ó', Vowel { base: 'o', shape: Plain, tone: Acute, upper: true }),
        ('ò', Vowel { base: 'o', shape: Plain, tone: Grave, upper: false }),
        ('Ò', Vowel { base: 'o', shape: Plain, tone: Grave, upper: true }),
        ('ỏ', Vowel { base: 'o', shape: Plain, tone: Hook, upper: false }),
        ('Ỏ', Vowel { base: 'o', shape: Plain, tone: Hook, upper: true }),
        ('õ', Vowel { base: 'o', shape: Plain, tone: Tilde, upper: false }),
        ('Õ', Vowel { base: 'o', shape: Plain, tone: Tilde, upper: true }),
        ('ọ', Vowel { base: 'o', shape: Plain, tone: Dot, upper: false }),
        ('Ọ', Vowel { base: 'o', shape: Plain, tone: Dot, upper: true }),
        ('ô', Vowel { base: 'o', shape: Circumflex, tone: None, upper: false }),
        ('Ô', Vowel { base: 'o', shape: Circumflex, tone: None, upper: true }),
        ('ố', Vowel { base: 'o', shape: Circumflex, tone: Acute, upper: false }),
        ('Ố', Vowel { base: 'o', shape: Circumflex, tone: Acute, upper: true }),
        ('ồ', Vowel { base: 'o', shape: Circumflex, tone: Grave, upper: false }),
        ('Ồ', Vowel { base: 'o', shape: Circumflex, tone: Grave, upper: true }),
        ('ổ', Vowel { base: 'o', shape: Circumflex, tone: Hook, upper: false }),
        ('Ổ', Vowel { base: 'o', shape: Circumflex, tone: Hook, upper: true }),
        ('ỗ', Vowel { base: 'o', shape: Circumflex, tone: Tilde, upper: false }),
        ('Ỗ', Vowel { base: 'o', shape: Circumflex, tone: Tilde, upper: true }),
        ('ộ', Vowel { base: 'o', shape: Circumflex, tone: Dot, upper: false }),
        ('Ộ', Vowel { base: 'o', shape: Circumflex, tone: Dot, upper: true }),
        ('ơ', Vowel { base: 'o', shape: Horn, tone: None, upper: false }),
        ('Ơ', Vowel { base: 'o', shape: Horn, tone: None, upper: true }),
        ('ớ', Vowel { base: 'o', shape: Horn, tone: Acute, upper: false }),
        ('Ớ', Vowel { base: 'o', shape: Horn, tone: Acute, upper: true }),
        ('ờ', Vowel { base: 'o', shape: Horn, tone: Grave, upper: false }),
        ('Ờ', Vowel { base: 'o', shape: Horn, tone: Grave, upper: true }),
        ('ở', Vowel { base: 'o', shape: Horn, tone: Hook, upper: false }),
        ('Ở', Vowel { base: 'o', shape: Horn, tone: Hook, upper: true }),
        ('ỡ', Vowel { base: 'o', shape: Horn, tone: Tilde, upper: false }),
        ('Ỡ', Vowel { base: 'o', shape: Horn, tone: Tilde, upper: true }),
        ('ợ', Vowel { base: 'o', shape: Horn, tone: Dot, upper: false }),
        ('Ợ', Vowel { base: 'o', shape: Horn, tone: Dot, upper: true }),
        ('u', Vowel { base: 'u', shape: Plain, tone: None, upper: false }),
        ('U', Vowel { base: 'u', shape: Plain, tone: None, upper: true }),
        ('ú', Vowel { base: 'u', shape: Plain, tone: Acute, upper: false }),
        ('Ú', Vowel { base: 'u', shape: Plain, tone: Acute, upper: true }),
        ('ù', Vowel { base: 'u', shape: Plain, tone: Grave, upper: false }),
        ('Ù', Vowel { base: 'u', shape: Plain, tone: Grave, upper: true }),
        ('ủ', Vowel { base: 'u', shape: Plain, tone: Hook, upper: false }),
        ('Ủ', Vowel { base: 'u', shape: Plain, tone: Hook, upper: true }),
        ('ũ', Vowel { base: 'u', shape: Plain, tone: Tilde, upper: false }),
        ('Ũ', Vowel { base: 'u', shape: Plain, tone: Tilde, upper: true }),
        ('ụ', Vowel { base: 'u', shape: Plain, tone: Dot, upper: false }),
        ('Ụ', Vowel { base: 'u', shape: Plain, tone: Dot, upper: true }),
        ('ư', Vowel { base: 'u', shape: Horn, tone: None, upper: false }),
        ('Ư', Vowel { base: 'u', shape: Horn, tone: None, upper: true }),
        ('ứ', Vowel { base: 'u', shape: Horn, tone: Acute, upper: false }),
        ('Ứ', Vowel { base: 'u', shape: Horn, tone: Acute, upper: true }),
        ('ừ', Vowel { base: 'u', shape: Horn, tone: Grave, upper: false }),
        ('Ừ', Vowel { base: 'u', shape: Horn, tone: Grave, upper: true }),
        ('ử', Vowel { base: 'u', shape: Horn, tone: Hook, upper: false }),
        ('Ử', Vowel { base: 'u', shape: Horn, tone: Hook, upper: true }),
        ('ữ', Vowel { base: 'u', shape: Horn, tone: Tilde, upper: false }),
        ('Ữ', Vowel { base: 'u', shape: Horn, tone: Tilde, upper: true }),
        ('ự', Vowel { base: 'u', shape: Horn, tone: Dot, upper: false }),
        ('Ự', Vowel { base: 'u', shape: Horn, tone: Dot, upper: true }),
        ('y', Vowel { base: 'y', shape: Plain, tone: None, upper: false }),
        ('Y', Vowel { base: 'y', shape: Plain, tone: None, upper: true }),
        ('ý', Vowel { base: 'y', shape: Plain, tone: Acute, upper: false }),
        ('Ý', Vowel { base: 'y', shape: Plain, tone: Acute, upper: true }),
        ('ỳ', Vowel { base: 'y', shape: Plain, tone: Grave, upper: false }),
        ('Ỳ', Vowel { base: 'y', shape: Plain, tone: Grave, upper: true }),
        ('ỷ', Vowel { base: 'y', shape: Plain, tone: Hook, upper: false }),
        ('Ỷ', Vowel { base: 'y', shape: Plain, tone: Hook, upper: true }),
        ('ỹ', Vowel { base: 'y', shape: Plain, tone: Tilde, upper: false }),
        ('Ỹ', Vowel { base: 'y', shape: Plain, tone: Tilde, upper: true }),
        ('ỵ', Vowel { base: 'y', shape: Plain, tone: Dot, upper: false }),
        ('Ỵ', Vowel { base: 'y', shape: Plain, tone: Dot, upper: true }),
    ]
};

#[inline]
pub fn parse_vowel(c: char) -> Option<Vowel> {
    VOWELS.iter().find(|(ch, _)| *ch == c).map(|(_, v)| *v)
}

pub fn compose_vowel(v: Vowel) -> Option<char> {
    VOWELS
        .iter()
        .find(|(_, x)| x.base == v.base && x.shape == v.shape && x.tone == v.tone && x.upper == v.upper)
        .map(|(ch, _)| *ch)
}

#[inline]
pub fn is_vowel(c: char) -> bool {
    parse_vowel(c).is_some()
}

pub fn has_special_shape(c: char) -> bool {
    parse_vowel(c).is_some_and(|v| v.shape != Shape::Plain)
}

pub fn is_d(c: char) -> bool {
    matches!(c, 'd' | 'D' | 'đ' | 'Đ')
}

pub fn is_dd(c: char) -> bool {
    matches!(c, 'đ' | 'Đ')
}

pub fn to_dd(c: char) -> char {
    if c.is_uppercase() { 'Đ' } else { 'đ' }
}

pub fn from_dd(c: char) -> char {
    if c == 'Đ' { 'D' } else { 'd' }
}

pub fn tone_from_telex(c: char) -> Option<Tone> {
    match c.to_ascii_lowercase() {
        's' => Some(Tone::Acute),
        'f' => Some(Tone::Grave),
        'r' => Some(Tone::Hook),
        'x' => Some(Tone::Tilde),
        'j' => Some(Tone::Dot),
        _ => None,
    }
}

pub fn tone_from_vni(c: char) -> Option<Tone> {
    match c {
        '1' => Some(Tone::Acute),
        '2' => Some(Tone::Grave),
        '3' => Some(Tone::Hook),
        '4' => Some(Tone::Tilde),
        '5' => Some(Tone::Dot),
        _ => None,
    }
}
