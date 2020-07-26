#![cfg_attr(feature = "verifier-panic-handler", feature(panic_info_message))]

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
    #[link(name = "kleeRuntest")]
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
//
// This originally used the function "klee_report_error"
// but this is not supported by the KLEE runtest library
pub fn verifier_report_error(message: &str) -> ! {
    if true {
        extern "C" {
            fn write(fd: isize, s: *const u8, count: usize);
        }
        let prefix  = "KLEE: ERROR:".as_bytes();
        let message = message.as_bytes();
        let newline = "\n".as_bytes();
        unsafe {
            write(2, prefix.as_ptr(),  prefix.len());
            write(2, message.as_ptr(), message.len());
            write(2, newline.as_ptr(), newline.len());
            verifier_abort();
        }
    } else {
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
}

#[cfg(feature = "verifier-panic-handler")]
use core::panic::PanicInfo;

#[cfg(feature = "verifier-panic-handler")]
use std::fmt::Write;

#[cfg(feature = "verifier-panic-handler")]
// Panic hook for ignoring errors generated
// while generating symbolic values.
//
// todo: this runs the risk of vacuous proofs
// if no symbolic values are generated
fn verifier_ignore_panic_hook(info: &PanicInfo) {
    verifier_reject()
}

#[cfg(feature = "verifier-panic-handler")]
// Panic hooks for reporting errors generated
// after generating all the symbolic values.
fn verifier_show_panic_hook(info: &PanicInfo) {
    let mut message = String::new();
    match info.message() {
        None => message.write_str("panic"),
        Some(m) => message.write_fmt(*m)
    }.unwrap();
    verifier_report_error(&message)
}

// Calling this before starting generating symbolic values
// so that errors produced while generating values are
// not confused with actual verification errors.
#[cfg(feature = "verifier-panic-handler")]
pub fn verifier_set_ignore_panic_hook() {
    std::panic::set_hook(Box::new(verifier_ignore_panic_hook))
}

// Calling this before starting verification ensures that
// panic messages are displayed by KLEE.
#[cfg(feature = "verifier-panic-handler")]
pub fn verifier_set_show_panic_hook() {
    std::panic::set_hook(Box::new(verifier_show_panic_hook))
}
