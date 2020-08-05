use std::default::Default;
use std::os::raw;

#[link(name = "kleeRuntest")]
extern "C" {
    fn klee_make_symbolic(data: *mut raw::c_void, length: usize, name: *const raw::c_char);
    fn klee_assume(cond: usize);
    fn klee_abort() -> !;
    fn klee_silent_exit(_ignored: u32) -> !;
    fn klee_is_replay() -> i32;
}

// Create an abstract value of type <T>
//
// This should only be used on types that occupy contiguous memory
// and where all possible bit-patterns are legal.
// e.g., u8/i8, ... u128/i128, f32/f64
pub fn verifier_abstract_value<T: Default>(_t: T) -> T {
    // The value '_t' is currently ignored.
    // It could be used to initialize 'r' instead but that
    // could be confusing while debugging the library so
    // using T::default() seemed to be safer?
    let mut r = T::default();
    unsafe {
        let data   = std::mem::transmute(&mut r);
        let length = std::mem::size_of::<T>();
        let null = 0 as *const i8;
        klee_make_symbolic(data, length, null)
    }
    return r;
}

// Add an assumption
pub fn verifier_assume(cond: bool) {
    unsafe { klee_assume(if cond {1} else {0}) }
}

// Reject the current execution with a verification failure.
//
// In almost all circumstances, verifier_report_error should
// be used instead because it generates an error message.
pub fn verifier_abort() -> ! {
    unsafe { klee_abort() }
}

// Reject the current execution path with a verification success.
// This is equivalent to verifier_assume(false)
// and the opposite of verifier_report_error.
//
// Typical usage is in generating symbolic values when the value
// does not meet some criteria.
pub fn verifier_reject() -> ! {
    unsafe { klee_silent_exit(0) }
}

// Detect whether the program is being run symbolically in KLEE
// or being replayed using the kleeRuntest runtime.
//
// This is used to decide whether to display the values of
// variables that may be either symbolic or concrete.
pub fn verifier_is_replay() -> bool {
    unsafe { klee_is_replay() != 0 }
}

// Reject the current execution with a verification failure
// and an error message.
pub fn verifier_report_error(message: &str) -> ! {
    // Mimic the format of klee_report_error
    // (We don't use klee_report_error because it is not
    // supported by the kleeRuntest library.)
    eprintln!("KLEE: ERROR:{}", message);
    verifier_abort();
}

// Check an assertion
pub fn verifier_verify(cond: bool) {
    if !cond {
        verifier_report_error("verification failed");
    }
}

