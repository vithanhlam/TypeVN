//! Small prefix table for technical/English passthrough.
//! Intentionally tiny — not a dictionary. Prefixes shorter than 4 are ignored
//! so common Telex (`as` → `á`) still works.
//!
//! Do not add words whose 4-letter prefix is a common Vietnamese syllable
//! (e.g. `theory` would lock `theo`). Prefer words that are valid Telex
//! syllables in English (`data`, `test`, `user`) or that start with `w`
//! before foreign phonotactics can recover them.

const TECH: &[&str] = &[
    "array",
    "assert",
    "async",
    "await",
    "boolean",
    "break",
    "buffer",
    "button",
    "callback",
    "catch",
    "class",
    "clone",
    "close",
    "commit",
    "component",
    "config",
    "console",
    "const",
    "context",
    "continue",
    "cookie",
    "count",
    "data",
    "debug",
    "default",
    "delete",
    "else",
    "elif",
    "email",
    "enum",
    "error",
    "event",
    "export",
    "extends",
    "false",
    "file",
    "float",
    "foreach",
    "from",
    "function",
    "github",
    "gitlab",
    "global",
    "header",
    "height",
    "host",
    "html",
    "http",
    "https",
    "icon",
    "image",
    "import",
    "include",
    "index",
    "info",
    "init",
    "input",
    "install",
    "item",
    "java",
    "javascript",
    "join",
    "json",
    "kotlin",
    "label",
    "lambda",
    "left",
    "length",
    "link",
    "list",
    "load",
    "local",
    "login",
    "logout",
    "main",
    "match",
    "meta",
    "method",
    "middleware",
    "model",
    "module",
    "name",
    "next",
    "node",
    "none",
    "null",
    "number",
    "object",
    "open",
    "option",
    "package",
    "page",
    "param",
    "params",
    "parse",
    "password",
    "path",
    "port",
    "post",
    "printf",
    "private",
    "prop",
    "props",
    "public",
    "push",
    "python",
    "query",
    "react",
    "read",
    "redux",
    "render",
    "request",
    "response",
    "result",
    "return",
    "right",
    "root",
    "route",
    "ruby",
    "save",
    "scala",
    "schema",
    "scope",
    "script",
    "select",
    "self",
    "send",
    "server",
    "session",
    "setup",
    "size",
    "slice",
    "sort",
    "split",
    "stack",
    "start",
    "state",
    "static",
    "status",
    "stop",
    "store",
    "string",
    "struct",
    "style",
    "super",
    "swift",
    "switch",
    "system",
    "table",
    "template",
    "test",
    "text",
    "then",
    "this",
    "throw",
    "token",
    "true",
    "type",
    "typeof",
    "typescript",
    "undefined",
    "unit",
    "user",
    "utf8",
    "utf16",
    "uuid",
    "valid",
    "value",
    "video",
    "void",
    "warn",
    "while",
    "width",
    "window",
    "with",
    "wordpress",
    "write",
    "yaml",
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
