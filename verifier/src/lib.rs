#[macro_export]
macro_rules! assume {
    ($condition:expr) => {
        if cfg!(feature = "verifier-klee") {
            klee_annotations::verifier_assume($condition)
        } else {
            assert!($condition);
        }
    };
}

#[macro_export]
macro_rules! assert {
    ($condition:expr) => {
        if cfg!(feature = "verifier-klee") {
            klee_annotations::verifier_verify($condition)
        } else {
            assert!($condition);
        }
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => (
        if cfg!(feature = "verifier-klee") {
            klee_annotations::verifier_verify($left == $right)
        } else {
            assert_eq!($left, $right);
        }
    );
}

#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => (
        if cfg!(feature = "verifier-klee") {
            klee_annotations::verifier_verify($left != $right)
        } else {
            assert!($left != $right);
        }
    );
}

#[macro_export]
macro_rules! unreachable {
    () => (
        if cfg!(feature = "verifier-klee") {
            klee_annotations::verifier_report_error("unreachable");
        } else {
            unreachable!();
        }
    );
}

#[macro_export]
macro_rules! nondet {
    ($value:expr) => {
        if cfg!(feature = "verifier-klee") {
            klee_annotations::verifier_abstract_value($value)
        } else {
            $value
        }
    };
}
