use std::env;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn typevn_ibus_main(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

fn main() {
    // Production: startup log only, never key contents.
    typevn_core::link_capi();
    eprintln!("typevn: starting ibus-typevn {} (vithanhlam)", typevn_core::VERSION);

    let args: Vec<CString> = env::args()
        .map(|a| CString::new(a).unwrap_or_else(|_| CString::new("ibus-typevn").unwrap()))
        .collect();
    let mut ptrs: Vec<*mut c_char> = args.iter().map(|s| s.as_ptr() as *mut c_char).collect();
    ptrs.push(std::ptr::null_mut());

    let code = unsafe { typevn_ibus_main(args.len() as c_int, ptrs.as_mut_ptr()) };
    if code != 0 {
        std::process::exit(code);
    }
}
