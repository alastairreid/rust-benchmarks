use klee_annotations::*;
// use core::cell::{Cell, RefCell, UnsafeCell};
// use core::iter;
use core::mem;
use core::ops::{Range, RangeBounds, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};
// use core::str;
// use core::time::Duration;
// use std::borrow::{Cow, ToOwned};
// use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
// use std::ffi::{CString, OsString};
// use std::path::PathBuf;
// use std::rc::Rc;
// use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicUsize, Ordering};
// use std::sync::{Arc, Mutex};

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

impl<A: Symbolic> Symbolic for Option<A> {
    fn symbolic() -> Self {
        if <bool as Symbolic>::symbolic() {
            Some(Symbolic::symbolic())
        } else {
            None
        }
    }
}

impl<A: Symbolic, B: Symbolic> Symbolic for std::result::Result<A, B> {
    fn symbolic() -> Self {
        if <bool as Symbolic>::symbolic() {
            Ok(<A as Symbolic>::symbolic())
        } else {
            Err(<B as Symbolic>::symbolic())
        }
    }
}

macro_rules! symbolic_tuple {
    () => {};
    ($last: ident $($xs: ident)*) => {
        symbolic_tuple!($($xs)*);

        impl<$($xs: Symbolic,)* $last: Symbolic> Symbolic for ($($xs,)* $last,) {
            fn symbolic() -> Self {
                ($($xs::symbolic(),)* Symbolic::symbolic(),)
            }
        }
    };
}
symbolic_tuple!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);

macro_rules! symbolic_array {
    {$n:expr, ($t:ident, $a:ident) $(($ts:ident, $as:ident))*} => {
        symbolic_array!{($n - 1), $(($ts, $as))*}

        impl<T: Symbolic> Symbolic for [T; $n] {
            fn symbolic() -> [T; $n] {
                [
                    Symbolic::symbolic(),
                    $(<$ts as Symbolic>::symbolic()),*
                ]
            }
        }
    };
    ($n: expr,) => {};
}

impl<T: Symbolic> Symbolic for [T; 0] {
    fn symbolic() -> [T; 0] {
        []
    }
}

symbolic_array! { 32, (T, a) (T, b) (T, c) (T, d) (T, e) (T, f) (T, g) (T, h)
(T, i) (T, j) (T, k) (T, l) (T, m) (T, n) (T, o) (T, p)
(T, q) (T, r) (T, s) (T, u) (T, v) (T, w) (T, x) (T, y)
(T, z) (T, aa) (T, ab) (T, ac) (T, ad) (T, ae) (T, af)
(T, ag) }

macro_rules! impl_range {
    (
        $range:ty,
        $value_ty:ty,
        $fun:ident($fun_closure:expr)
    ) => {
        impl<S> Symbolic for $range
        where
            S: Symbolic + Clone + PartialOrd,
        {
            fn symbolic() -> Self {
                let value: $value_ty = Symbolic::symbolic();
                $fun(value, $fun_closure)
            }
        }
    };
}

fn bounded_range<CB, I, R>(bounds: (I, I), cb: CB) -> R
where
    CB: Fn((I, I)) -> R,
    I: PartialOrd,
    R: RangeBounds<I>,
{
    let (mut start, mut end) = bounds;
    if start > end {
        mem::swap(&mut start, &mut end);
    }
    cb((start, end))
}

fn unbounded_range<CB, I, R>(bound: I, cb: CB) -> R
where
    CB: Fn(I) -> R,
    R: RangeBounds<I>,
{
    cb(bound)
}

impl_range!(
    Range<S>,
    (S, S),
    bounded_range(|(a, b)| a..b)
);
impl_range!(
    RangeFrom<S>,
    S,
    unbounded_range(|a| a..)
);
impl_range!(
    RangeInclusive<S>,
    (S, S),
    bounded_range(|(a, b)| a..=b)
);
impl_range!(
    RangeTo<S>,
    S,
    unbounded_range(|b| ..b)
);
impl_range!(
    RangeToInclusive<S>,
    S,
    unbounded_range(|b| ..=b)
);
