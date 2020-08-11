#[cfg(feature = "verifier-klee")]
pub use klee_annotations as annotations;

#[macro_export]
macro_rules! assume {
    ($condition:expr) => {
        if cfg!(feature = "verifier-klee") {
            $crate::annotations::assume($condition)
        } else {
            assert!($condition);
        }
    };
}

#[macro_export]
macro_rules! assert {
    ($condition:expr) => {
        if cfg!(feature = "verifier-klee") {
            $crate::annotations::verify($condition)
        } else {
            assert!($condition);
        }
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => (
        if cfg!(feature = "verifier-klee") {
            $crate::annotations::verify($left == $right)
        } else {
            assert_eq!($left, $right);
        }
    );
}

#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => (
        if cfg!(feature = "verifier-klee") {
            $crate::annotations::verify($left != $right)
        } else {
            assert!($left != $right);
        }
    );
}

#[macro_export]
macro_rules! unreachable {
    () => (
        if cfg!(feature = "verifier-klee") {
            $crate::annotations::report_error("unreachable");
        } else {
            unreachable!();
        }
    );
}

#[macro_export]
macro_rules! nondet {
    ($value:expr) => {
        if cfg!(feature = "verifier-klee") {
            $crate::annotations::abstract_value($value)
        } else {
            $value
        }
    };
}
