use klee_annotations::*;

pub trait Symbolic: 'static {
    /// Generate a symbolic value of `Self`.
    fn symbolic() -> Self;
}

impl Symbolic for () {
    fn symbolic() -> Self {
        ()
    }
}

// Generate implementation of Symbolic for every
// type where any N-byte bitpattern is a legal value.
// (e.g., integer and float types, not enumerations, etc.)
macro_rules! impl_symbolic_for_scalars {
    ( $( $ty:ty; )* ) => {
        $(
            impl Symbolic for $ty {
                fn symbolic() -> Self {
                    verifier_abstract_value(<$ty>::default())
                }
            }
        )*
    }
}

impl_symbolic_for_scalars! {
    u8;
    u16;
    u32;
    u64;
    u128;
    usize;
    i8;
    i16;
    i32;
    i64;
    i128;
    isize;
    f32;
    f64;
}

impl Symbolic for bool {
    fn symbolic() -> Self {
        let c = verifier_abstract_value(0u8);
        verifier_assume(c == 0 || c == 1);
        c == 1
    }
}

impl Symbolic for char {
    fn symbolic() -> Self {
        let c = verifier_abstract_value(0u32);
        match std::char::from_u32(c) {
            Some(r) => r,
            None => verifier_reject()
        }
    }
}
