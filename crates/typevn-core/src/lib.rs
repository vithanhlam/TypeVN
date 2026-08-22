//! TypeVN core: synchronous Vietnamese Telex engine.
//!
//! No IBus, GUI, I/O, threads, or network in the hot path.

mod capi;
mod charset;
mod config;
mod engine;
mod english;
mod key;
mod repair;
mod syllable;
mod telex;
mod vni;
mod vowel;

pub use charset::Charset;
pub use config::{load as load_config, save as save_config, TypeVnConfig};
pub use engine::{EngineAction, InputMode, TypingMethod, VietnameseEngine};
pub use key::{KeyEvent, Modifiers, KEY};

pub const MAX_COMPOSE_CHARS: usize = 32;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Keep C ABI symbols in the final IBus binary (linker GC).
pub fn link_capi() {
    let _ = capi::typevn_engine_new as *const ();
    let _ = capi::typevn_engine_free as *const ();
    let _ = capi::typevn_engine_reset as *const ();
    let _ = capi::typevn_process_key as *const ();
    let _ = capi::typevn_engine_get_method as *const ();
    let _ = capi::typevn_engine_get_english as *const ();
    let _ = capi::typevn_engine_set_method as *const ();
    let _ = capi::typevn_engine_set_english as *const ();
    let _ = capi::typevn_engine_reload as *const ();
}
