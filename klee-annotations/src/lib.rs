#![cfg_attr(feature = "verifier-panic-handler", feature(panic_info_message))]

use std::os::raw;
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
}

// (In part because pthread support is broken at the moment)
// we only want to display values when running with the ktest runtime
// so we need a way to tell which mode we are running in.
pub fn verifier_is_replay() -> bool {
    #[link(name = "kleeRuntest")]
    extern "C" { fn klee_is_replay() -> i32; }
    unsafe { klee_is_replay() != 0 }
}
