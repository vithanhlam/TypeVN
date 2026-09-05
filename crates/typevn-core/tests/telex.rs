use typevn_core::{Charset, EngineAction, InputMode, KeyEvent, Modifiers, VietnameseEngine, KEY};

fn feed(eng: &mut VietnameseEngine, s: &str) -> String {
    let mut committed = String::new();
    for c in s.chars() {
        match eng.process_key(KeyEvent::from_char(c)) {
            EngineAction::Commit(t)
            | EngineAction::CommitThenPass(t)
            | EngineAction::CommitThenNotify(t, _) => committed.push_str(&t),
            EngineAction::Preedit(_)
            | EngineAction::PassThrough
            | EngineAction::Reset
            | EngineAction::Notify(_) => {}
            EngineAction::Delete(_) => {}
        }
    }
    committed.push_str(&eng.buffer_str());
    committed
}

/// Engine that ignores the config of the machine running the tests.
fn engine() -> VietnameseEngine {
    let mut eng = VietnameseEngine::new();
    eng.set_input_mode(InputMode::Vietnamese);
    eng.set_typing_method(typevn_core::TypingMethod::Telex);
    eng.set_charset(Charset::Unicode);
    eng.set_auto_repair(true);
    eng.set_hotkeys_enabled(true);
    eng
}

fn telex(s: &str) -> String {
    let mut eng = engine();
    feed(&mut eng, s)
}

#[test]
fn readme_examples() {
    assert_eq!(telex("tieengs"), "tiếng");
    assert_eq!(telex("Vieetj"), "Việt");
    assert_eq!(telex("duwowngf"), "dường");
    assert_eq!(telex("dduwowngf"), "đường");
}

#[test]
fn basic_marks() {
    assert_eq!(telex("aa"), "â");
    assert_eq!(telex("aw"), "ă");
    assert_eq!(telex("ee"), "ê");
    assert_eq!(telex("oo"), "ô");
    assert_eq!(telex("ow"), "ơ");
    assert_eq!(telex("uw"), "ư");
    assert_eq!(telex("dd"), "đ");
    assert_eq!(telex("as"), "á");
    assert_eq!(telex("af"), "à");
    assert_eq!(telex("ar"), "ả");
    assert_eq!(telex("ax"), "ã");
    assert_eq!(telex("aj"), "ạ");
}

#[test]
fn undo_duplicate_mark() {
    assert_eq!(telex("aaa"), "aa");
    assert_eq!(telex("ass"), "as");
    assert_eq!(telex("ddd"), "dd");
}

#[test]
fn undo_w_like_unikey() {
    assert_eq!(telex("uw"), "ư");
    assert_eq!(telex("uww"), "uw");
    assert_eq!(telex("uwww"), "uww");
    assert_eq!(telex("uwweb"), "uweb");
    assert_eq!(telex("ww"), "w");
    assert_eq!(telex("oww"), "ow");
    assert_eq!(telex("aww"), "aw");
}

#[test]
fn z_strips_tone() {
    assert_eq!(telex("as"), "á");
    assert_eq!(telex("asz"), "a");
}

#[test]
fn words() {
    assert_eq!(telex("nguowif"), "người");
    assert_eq!(telex("hoaf"), "hoà");
    assert_eq!(telex("thuowng"), "thương");
    assert_eq!(telex("xin chaof"), "xin chào");
    assert_eq!(telex("dduwowcj"), "được");
    assert_eq!(telex("dduowcj"), "được");
    assert_eq!(telex("ddwocj"), "được");
    assert_eq!(telex("dudowcj"), "được");
    assert_eq!(telex("giups"), "giúp");
    assert_eq!(telex("Giups"), "Giúp");
    assert_eq!(telex("gif"), "gì");
    assert_eq!(telex("gias"), "giá");
    assert_eq!(telex("chinhr"), "chỉnh");
    assert_eq!(telex("suawr"), "sửa");
    assert_eq!(telex("suwar"), "sửa");
    assert_eq!(telex("quas"), "quá");
    assert_eq!(telex("cuar"), "của");
    assert_eq!(telex("Cuar"), "Của");
    assert_eq!(telex("muas"), "múa");
    assert_eq!(telex("tias"), "tía");
    assert_eq!(telex("khoer"), "khoẻ");
    assert_eq!(telex("thuyr"), "thuỷ");
    assert_eq!(telex("phair"), "phải");
    assert_eq!(telex("cungx"), "cũng");
    assert_eq!(telex("luaajt"), "luật");
    assert_eq!(telex("ngoaif"), "ngoài");
    assert_eq!(telex("tuooir"), "tuổi");
    assert_eq!(telex("vieejc"), "việc");
    assert_eq!(telex("yeeuf"), "yều");
    assert_eq!(telex("ddieefu"), "điều");
    assert_eq!(telex("khoong"), "không");
    assert_eq!(telex("nhuwngx"), "những");
}

#[test]
fn reported_words() {
    assert_eq!(telex("Truwa"), "Trưa");
    assert_eq!(telex("Truowng"), "Trương");
}

#[test]
fn community_macros_apply_at_word_boundary() {
    assert_eq!(telex("Loix "), "Lỗi ");
    assert_eq!(telex("loix "), "lỗi ");
}

#[test]
fn disabling_auto_repair_disables_macros() {
    let mut eng = engine();
    eng.set_auto_repair(false);
    assert_eq!(feed(&mut eng, "Loix "), "Lõi ");
}

#[test]
fn short_technical_prefixes_do_not_block_vietnamese_telex() {
    // Min length 4: `tru` must stay open for `trưa` / `trước` / `trường`.
    assert_eq!(telex("true"), "true");
    assert_eq!(telex("console"), "console");
    assert_eq!(telex("Truwa"), "Trưa");
    assert_eq!(telex("Truowng"), "Trương");
    assert_eq!(telex("Truowsc"), "Trước");
}

#[test]
fn space_commits() {
    let mut eng = engine();
    let a = eng.process_key(KeyEvent::from_char('a'));
    assert!(matches!(a, EngineAction::Preedit(_)));
    let b = eng.process_key(KeyEvent::from_char(' '));
    assert_eq!(b, EngineAction::Commit("a ".into()));
    assert!(eng.buffer_str().is_empty());
}

#[test]
fn backspace_restores() {
    let mut eng = engine();
    feed(&mut eng, "tieengs");
    assert_eq!(eng.buffer_str(), "tiếng");
    let a = eng.process_key(KeyEvent::new(KEY::BackSpace, 0, Modifiers::default()));
    assert_eq!(a, EngineAction::Preedit("tiêng".into()));
    let a = eng.process_key(KeyEvent::new(KEY::BackSpace, 0, Modifiers::default()));
    assert_eq!(a, EngineAction::Preedit("tiên".into()));
}

#[test]
fn english_mode_passthrough() {
    let mut eng = engine();
    eng.set_input_mode(InputMode::English);
    let a = eng.process_key(KeyEvent::from_char('a'));
    assert_eq!(a, EngineAction::PassThrough);
    assert_eq!(telex("aa"), "â");
}

#[test]
fn ctrl_shortcuts_passthrough() {
    let mut eng = engine();
    let key = KeyEvent::new(
        0x63,
        0,
        Modifiers {
            control: true,
            ..Modifiers::default()
        },
    );
    assert_eq!(eng.process_key(key), EngineAction::PassThrough);
}

#[test]
fn tech_passthrough_console() {
    assert_eq!(telex("console"), "console");
    assert_eq!(telex("class"), "class");
    assert_eq!(telex("const"), "const");
    assert_eq!(telex("status"), "status");
    // VN-shaped English that would otherwise take Telex marks.
    assert_eq!(telex("data"), "data");
    assert_eq!(telex("test"), "test");
    assert_eq!(telex("text"), "text");
    assert_eq!(telex("list"), "list");
    assert_eq!(telex("user"), "user");
    assert_eq!(telex("info"), "info");
    assert_eq!(telex("query"), "query");
    assert_eq!(telex("error"), "error");
    assert_eq!(telex("write"), "write");
    assert_eq!(telex("warn"), "warn");
}

#[test]
fn tech_word_removed_from_list_does_not_block_matching_vietnamese_syllable() {
    // "root", "then", "main" were briefly added to the tech list; each is
    // also a valid open Vietnamese syllable, so they must not lock the
    // buffer to literal ASCII before a tone key arrives.
    assert_eq!(telex("roots"), "rốt");
    assert_eq!(telex("thenj"), "thẹn");
    assert_eq!(telex("then"), "then");
    assert_eq!(telex("main"), "main");
}

#[test]
fn punctuation_commits() {
    let mut eng = engine();
    feed(&mut eng, "xin");
    let a = eng.process_key(KeyEvent::from_char('!'));
    assert_eq!(a, EngineAction::Commit("xin!".into()));
}

#[test]
fn caps_and_shift() {
    assert_eq!(telex("Aa"), "Â");
    assert_eq!(telex("DD"), "Đ");
}

#[test]
fn unicode_passthrough_via_commit_then_pass_empty() {
    let mut eng = engine();
    let key = KeyEvent::from_char('😀');
    let a = eng.process_key(key);
    assert_eq!(a, EngineAction::PassThrough);
}

#[test]
fn w_alone_is_uhorn() {
    assert_eq!(telex("w"), "ư");
}

#[test]
fn fast_sentence() {
    let s = telex("hoom nay toi dang thu nghiem toc ddooj gox tieengs Vieetj");
    assert!(s.contains("hôm"));
    assert!(s.contains("tiếng"));
    assert!(s.contains("Việt") || s.contains("việt"));
}

#[test]
fn charset_viqr_roundtrip_shape() {
    let mut eng = engine();
    eng.set_charset(Charset::Viqr);
    for c in "as".chars() {
        let _ = eng.process_key(KeyEvent::from_char(c));
    }
    assert_eq!(eng.buffer_str(), "a'");
}

#[test]
fn auto_repair_toggle() {
    let mut eng = engine();
    assert!(eng.auto_repair());
    eng.set_auto_repair(false);
    assert!(!eng.auto_repair());
}

#[test]
fn marks_stay_on_last_syllable_only() {
    // Extra consonants after a finished coda start a new syllable (Unikey).
    assert_eq!(telex("vithanhlma"), "vithanhlma");
    assert_eq!(telex("vithanhlms"), "vithanhlms");
}

#[test]
fn no_marks_on_meaningless_multi_syllable() {
    // Username / no-space Latin: do not turn `s` into sắc.
    assert_eq!(telex("vithanhlams"), "vithanhlams");
    assert_eq!(telex("vithanhlamseo"), "vithanhlamseo");
    assert_eq!(telex("vithanha"), "vithanha");
    assert_eq!(telex("xinchaof"), "xinchaof");
    // Real Vietnamese is typed one syllable (or with space).
    assert_eq!(telex("lams"), "lám");
    assert_eq!(telex("chaof"), "chào");
    assert_eq!(telex("xin chaof"), "xin chào");
}

#[test]
fn long_identifiers_stay_literal_and_are_not_truncated() {
    assert_eq!(telex("wordpress"), "wordpress");
    assert_eq!(telex("vithanhlamseo"), "vithanhlamseo");
    assert_eq!(
        telex("wordpressvithanhlamseowordpressasass"),
        "wordpressvithanhlamseowordpressasass"
    );
}

#[test]
fn adjacent_ascii_tokens_do_not_keep_tentative_telex_marks() {
    assert_eq!(telex("wordpressplugin"), "wordpressplugin");
    assert_eq!(telex("xinchaof"), "xinchaof");
    assert_eq!(telex("vithanhlams"), "vithanhlams");
}

#[test]
fn foreign_shape_rewinds_tentative_marks() {
    // Marks applied early must unwind once the token is clearly not Vietnamese.
    assert_eq!(telex("write"), "write");
    assert_eq!(telex("world"), "world");
    assert_eq!(telex("google"), "google");
    assert_eq!(telex("info"), "info");
    assert_eq!(telex("error"), "error");
}

#[test]
fn foreign_coda_blocks_marks() {
    // `ld`, `rd`, `g`, `s` … cannot end a Vietnamese syllable.
    assert_eq!(telex("shieldpress"), "shieldpress");
    assert_eq!(telex("shield"), "shield");
    assert_eq!(telex("google"), "google");
    assert_eq!(telex("hello"), "hello");
    assert_eq!(telex("world"), "world");
    assert_eq!(telex("email"), "email");
}

#[test]
fn foreign_onset_blocks_marks() {
    // `pr`, `sh`, `cl`, `f` … cannot open a Vietnamese syllable.
    assert_eq!(telex("press"), "press");
    assert_eq!(telex("facebook"), "facebook");
    assert_eq!(telex("frontend"), "frontend");
    assert_eq!(telex("script"), "script");
}

#[test]
fn foreign_vowel_cluster_blocks_marks() {
    // `ea`, `ou`, `ee` … are not Vietnamese vowel clusters.
    assert_eq!(telex("leaf"), "leaf");
    assert_eq!(telex("east"), "east");
    assert_eq!(telex("hour"), "hour");
    assert_eq!(telex("cargo"), "cargo");
    assert_eq!(telex("server"), "server");
    assert_eq!(telex("youtube"), "youtube");
}

#[test]
fn latin_words_shaped_like_vietnamese_still_take_marks() {
    // Not a regression: these are valid Vietnamese syllables, Unikey marks them too.
    assert_eq!(telex("rust"), "rút");
    assert_eq!(telex("lams"), "lám");
}

#[test]
fn vietnamese_clusters_keep_working() {
    assert_eq!(telex("nguyeenx"), "nguyễn");
    assert_eq!(telex("khuyur"), "khuỷu");
    assert_eq!(telex("ruwowuj"), "rượu");
    assert_eq!(telex("nguwowif"), "người");
    assert_eq!(telex("giowf"), "giờ");
    assert_eq!(telex("xuaan"), "xuân");
    assert_eq!(telex("quoocs"), "quốc");
    assert_eq!(telex("nghieeng"), "nghiêng");
    assert_eq!(telex("chuyeenj"), "chuyện");
    assert_eq!(telex("xoay"), "xoay");
    assert_eq!(telex("khuya"), "khuya");
    assert_eq!(telex("yeeu"), "yêu");
    assert_eq!(telex("keeu"), "kêu");
    assert_eq!(telex("cuwus"), "cứu");
    assert_eq!(telex("dduwas"), "đứa");
    assert_eq!(telex("toans"), "toán");
    assert_eq!(telex("beenhj"), "bệnh");
    assert_eq!(telex("hoocj"), "hộc");
    assert_eq!(telex("nghix"), "nghĩ");
}
