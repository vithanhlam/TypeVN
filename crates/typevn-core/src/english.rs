//! Small prefix table for technical/English passthrough.
//! Intentionally tiny — not a dictionary. Prefixes shorter than 4 are ignored
//! so common Telex (`as` → `á`) still works.

const TECH: &[&str] = &[
    "async",
    "await",
    "break",
    "catch",
    "class",
    "clone",
    "commit",
    "console",
    "const",
    "continue",
    "debug",
    "default",
    "delete",
    "export",
    "extends",
    "false",
    "function",
    "github",
    "gitlab",
    "https",
    "import",
    "include",
    "install",
    "length",
    "module",
    "null",
    "number",
    "object",
    "package",
    "printf",
    "private",
    "public",
    "push",
    "return",
    "static",
    "status",
    "string",
    "struct",
    "switch",
    "throw",
    "true",
    "typeof",
    "undefined",
    "utf8",
    "utf16",
    "while",
    "window",
    "wordpress",
    "yield",
];

fn ascii_lower_buf(chars: &[char], extra: Option<char>, out: &mut [u8]) -> usize {
    let mut n = 0;
    for &c in chars {
        if n >= out.len() {
            break;
        }
        let l = c.to_ascii_lowercase();
        if l.is_ascii() {
            out[n] = l as u8;
            n += 1;
        } else {
            return 0;
        }
    }
    if let Some(c) = extra {
        if n < out.len() {
            let l = c.to_ascii_lowercase();
            if l.is_ascii() {
                out[n] = l as u8;
                n += 1;
            } else {
                return 0;
            }
        }
    }
    n
}

/// True when `chars` (+ optional extra letter) is a prefix of a technical token.
pub fn is_tech_prefix(chars: &[char], extra: Option<char>) -> bool {
    let mut tmp = [0u8; 32];
    let n = ascii_lower_buf(chars, extra, &mut tmp);
    // Three-letter prefixes are too ambiguous in Vietnamese. In particular,
    // `tru` is a prefix of English `true`, but is also the very common
    // Vietnamese onset+nucleus in `trưa`, `trước`, `trường`, ... .
    if n < 4 {
        return false;
    }
    let prefix = match std::str::from_utf8(&tmp[..n]) {
        Ok(s) => s,
        Err(_) => return false,
    };
    TECH.iter().any(|w| w.starts_with(prefix))
}
