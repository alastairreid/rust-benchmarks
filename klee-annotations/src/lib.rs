#![cfg_attr(feature = "verifier_panic_handler", feature(panic_info_message))]

use std::os::raw;
use std::ffi::CString;
use std::default::Default;

pub fn verifier_assume(cond: bool) {
    extern "C" { fn klee_assume(cond: usize); }
    unsafe { klee_assume(if cond {1} else {0}) }
}

pub fn verifier_verify(cond: bool) {
    if !cond {
        verifier_report_error("verification failed")
    }
}

pub fn verifier_abstract_value<T: Default>(_t: T) -> T {
    extern "C" { fn klee_make_symbolic(data: *mut raw::c_void, length: usize, name: *const raw::c_char); }

    let mut r = T::default();
    unsafe {
        let data   = std::mem::transmute(&mut r);
        let length = std::mem::size_of::<T>();
        let null = 0 as *const i8;
        klee_make_symbolic(data, length, null)
    }
    return r;
}

// Reject the current execution with a verification failure.
//
// In almost all circumstances, verifier_report_error should
// be used instead because it generates an error message.
pub fn verifier_abort() -> ! {
    extern "C" { fn klee_abort() -> !; }

    unsafe { klee_abort() }
}

// Reject the current execution path with a verification success.
// This is equivalent to verifier_assume(false)
// and the opposite of verifier_report_error.
//
// Typical usage is in generating symbolic values when the value
// does not meet some criteria.
pub fn verifier_reject() -> ! {
    extern "C" { fn klee_silent_exit(_ignored: u32) -> !; }
    unsafe { klee_silent_exit(0) }
}

// Reject the current execution with a verification failure
// and an error message.
pub fn verifier_report_error(message: &str) -> ! {
    extern "C" {
        fn klee_report_error(file: *const raw::c_char, line: usize, message: *const raw::c_char, suffix: *const raw::c_char) -> !;
    }

    let null = 0 as *const i8;
    let file = null; // ignored by KLEE
    let line = 0;    // ignored by KLEE
    let suffix = ""; // ignored by KLEE

    let message = CString::new(message).unwrap();
    let suffix  = CString::new(suffix).unwrap();
    unsafe {
        klee_report_error(file, line, message.as_ptr(), suffix.as_ptr())
    }
}

#[cfg(feature = "verifier_panic_handler")]
use core::panic::PanicInfo;

#[cfg(feature = "verifier_panic_handler")]
use std::fmt::Write;

#[cfg(feature = "verifier_panic_handler")]
fn verifier_panic_hook(info: &PanicInfo) {
    let mut message = String::new();
    match info.message() {
        None => message.write_str("panic"),
        Some(m) => message.write_fmt(*m)
    }.unwrap();
    verifier_report_error(&message)
}

// Calling this before starting verification ensures that
// panic messages are displayed by KLEE.
#[cfg(feature = "verifier_panic_handler")]
pub fn verifier_set_panic_hook() {
    std::panic::set_hook(Box::new(verifier_panic_hook))
}
