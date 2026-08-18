//! Load/save ~/.config/typevn/config once (not on the keypress hot path).

use crate::charset::Charset;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeVnConfig {
    pub method_vni: bool,
    pub charset: Charset,
    pub auto_repair: bool,
    pub auto_start: bool,
    pub english: bool,
    pub hotkeys_enabled: bool,
}

impl Default for TypeVnConfig {
    fn default() -> Self {
        Self {
            method_vni: false,
            charset: Charset::Unicode,
            auto_repair: true,
            auto_start: true,
            english: false,
            hotkeys_enabled: true,
        }
    }
}

pub fn config_path() -> PathBuf {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let mut h = PathBuf::from(std::env::var_os("HOME").unwrap_or_default());
            h.push(".config");
            h
        });
    base.join("typevn").join("config")
}

pub fn load() -> TypeVnConfig {
    let mut cfg = TypeVnConfig::default();
    let Ok(text) = fs::read_to_string(config_path()) else {
        return cfg;
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        match k.trim() {
            "method" => {
                cfg.method_vni = matches!(v.trim().to_ascii_lowercase().as_str(), "vni");
            }
            "charset" => {
                cfg.charset = match v.trim().to_ascii_lowercase().as_str() {
                    "vni" | "vni-windows" => Charset::VniWindows,
                    "viqr" => Charset::Viqr,
                    "tcvn3" | "tcvn" => Charset::Tcvn3,
                    _ => Charset::Unicode,
                };
            }
            "auto_repair" => {
                cfg.auto_repair = !matches!(v.trim(), "0" | "false" | "off");
            }
            "auto_start" => {
                cfg.auto_start = !matches!(v.trim(), "0" | "false" | "off");
            }
            "english" => {
                cfg.english = matches!(v.trim(), "1" | "true" | "on");
            }
            "hotkeys_enabled" => {
                cfg.hotkeys_enabled = !matches!(v.trim(), "0" | "false" | "off");
            }
            _ => {}
        }
    }
    if let Ok(v) = std::env::var("TYPEVN_CHARSET") {
        cfg.charset = match v.to_ascii_lowercase().as_str() {
            "vni" | "vni-windows" => Charset::VniWindows,
            "viqr" => Charset::Viqr,
            "tcvn3" | "tcvn" => Charset::Tcvn3,
            "unicode" => Charset::Unicode,
            _ => cfg.charset,
        };
    }
    if let Ok(v) = std::env::var("TYPEVN_AUTO_REPAIR") {
        cfg.auto_repair = !matches!(v.as_str(), "0" | "false" | "off");
    }
    cfg
}

pub fn save(cfg: &TypeVnConfig) -> std::io::Result<()> {
    let path = config_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut extras = String::new();
    if let Ok(old) = fs::read_to_string(&path) {
        for line in old.lines() {
            let Some((k, _)) = line.split_once('=') else {
                continue;
            };
            let k = k.trim();
            if matches!(
                k,
                "method" | "charset" | "auto_repair" | "auto_start" | "english"
                    | "hotkeys_enabled"
            ) {
                continue;
            }
            extras.push_str(line);
            extras.push('\n');
        }
    }
    let body = format!(
        "method={}\ncharset={}\nauto_repair={}\nauto_start={}\nenglish={}\nhotkeys_enabled={}\n{extras}",
        if cfg.method_vni { "vni" } else { "telex" },
        match cfg.charset {
            Charset::Unicode => "unicode",
            Charset::VniWindows => "vni",
            Charset::Viqr => "viqr",
            Charset::Tcvn3 => "tcvn3",
        },
        if cfg.auto_repair { "true" } else { "false" },
        if cfg.auto_start { "true" } else { "false" },
        if cfg.english { "true" } else { "false" },
        if cfg.hotkeys_enabled { "true" } else { "false" },
    );
    fs::write(path, body)
}
