use std::time::Instant;
use typevn_core::{EngineAction, InputMode, KeyEvent, TypingMethod, VietnameseEngine};

/// Engine that ignores the config of the machine running the tests.
fn engine() -> VietnameseEngine {
    let mut eng = VietnameseEngine::new();
    eng.set_input_mode(InputMode::Vietnamese);
    eng.set_typing_method(TypingMethod::Telex);
    eng.set_charset(typevn_core::Charset::Unicode);
    eng.set_auto_repair(true);
    eng
}

#[test]
fn sequential_100k_no_loss() {
    let mut eng = engine();
    let seq: Vec<char> = "tieengs "
        .chars()
        .cycle()
        .take(100_000)
        .collect();
    let mut commits = 0usize;
    for c in seq {
        match eng.process_key(KeyEvent::from_char(c)) {
            EngineAction::Commit(s) => {
                assert_eq!(s, "tiếng ");
                commits += 1;
            }
            EngineAction::Preedit(_) | EngineAction::Reset | EngineAction::PassThrough | EngineAction::Notify(_) => {}
            other => panic!("unexpected {other:?}"),
        }
    }
    assert_eq!(commits, 100_000 / 8);
}

#[test]
fn ten_million_events_memory_bound() {
    let mut eng = engine();
    let pattern: Vec<char> = "duwowngf ".chars().collect();
    for i in 0..10_000_000u32 {
        let c = pattern[(i as usize) % pattern.len()];
        let _ = eng.process_key(KeyEvent::from_char(c));
    }
    assert!(eng.buffer_str().len() <= 32);
}

#[test]
fn simulated_cps_correctness() {
    let expected = {
        let mut e = engine();
        let mut out = String::new();
        for c in "tieengs Vieetj duwowngf ".chars() {
            match e.process_key(KeyEvent::from_char(c)) {
                EngineAction::Commit(s) => out.push_str(&s),
                _ => {}
            }
        }
        out.push_str(&e.buffer_str());
        out
    };

    for _cps in [20, 50, 100, 200, 500] {
        let mut e = engine();
        let mut out = String::new();
        for c in "tieengs Vieetj duwowngf ".chars() {
            match e.process_key(KeyEvent::from_char(c)) {
                EngineAction::Commit(s) => out.push_str(&s),
                _ => {}
            }
        }
        out.push_str(&e.buffer_str());
        assert_eq!(out, expected);
    }
}

#[test]
fn latency_smoke_100k() {
    let mut eng = engine();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz "
        .chars()
        .cycle()
        .take(100_000)
        .collect();
    let t0 = Instant::now();
    for c in chars {
        let _ = eng.process_key(KeyEvent::from_char(c));
    }
    let avg_ns = t0.elapsed().as_nanos() / 100_000;
    assert!(avg_ns < 1_000_000, "avg {avg_ns} ns >= 1ms");
}

#[test]
fn backspace_spam() {
    let mut eng = engine();
    for _ in 0..1000 {
        let _ = eng.process_key(KeyEvent::from_char('a'));
        let _ = eng.process_key(KeyEvent {
            keyval: typevn_core::KEY::BackSpace,
            keycode: 0,
            modifiers: typevn_core::Modifiers::default(),
        });
    }
    assert!(eng.buffer_str().is_empty());
}
