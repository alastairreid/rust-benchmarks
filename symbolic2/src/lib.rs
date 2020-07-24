use klee_annotations::*;
use std::marker::PhantomData;
use std::sync::Arc;

pub trait Strategy {
    type Value;
    fn value(&self) -> Self::Value;
}

// The most trivial strategy
pub struct Const<T> {
    c: T,
}
impl<T> Const<T> {
    pub fn new(c: T) -> Self {
        Self {
            c,
        }
    }
}
impl<T: Copy> Strategy for Const<T> {
    type Value = T;
    fn value(&self) -> Self::Value {
        self.c
    }
}

// For primitive types, all possible values are legal
pub struct Any<T> {
    _marker: PhantomData<T>,
}
impl<T> Any<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl Strategy for Any<bool> {
    type Value = bool;
    fn value(&self) -> Self::Value {
        let c = verifier_abstract_value(0u8);
        verifier_assume(c == 0 || c == 1);
        c == 1
    }
}

impl Strategy for Any<char> {
    type Value = char;
    fn value(&self) -> Self::Value {
        let c = verifier_abstract_value(0u32);
        match std::char::from_u32(c) {
            Some(r) => r,
            None => verifier_reject()
        }
    }
}

pub struct Filter<S, F> {
    source: S,
    fun: Arc<F>,
}
impl<S, F> Filter<S, F> {
    pub fn new(source: S, fun: F) -> Self {
        Self {
            source,
            fun: Arc::new(fun),
        }
    }
}
impl<S: Strategy, F: Fn(&S::Value) -> bool> Strategy for Filter<S, F> {
    type Value = S::Value;
    fn value(&self) -> Self::Value {
        let val = self.source.value();
        verifier_assume((self.fun)(&val));
        val
    }
}

pub struct Map<S, F> {
    source: S,
    fun: Arc<F>,
}
impl<S, F> Map<S, F> {
    pub fn new(source: S, fun: F) -> Self {
        Self {
            source,
            fun: Arc::new(fun),
        }
    }
}
impl<S: Strategy, T, F: Fn(&S::Value) -> T> Strategy for Map<S, F> {
    type Value = T;
    fn value(&self) -> Self::Value {
        let val = self.source.value();
        (self.fun)(&val)
    }
}


macro_rules! numeric_api {
    ( $( $typ:ty; )* ) => {
        $(
            impl Strategy for ::core::ops::Range<$typ> {
                type Value = $typ;
                fn value(&self) -> Self::Value {
                    let r = verifier_abstract_value(<$typ>::default());
                    verifier_assume(self.start <= r);
                    verifier_assume(r < self.end);
                    r
                }
            }

            impl Strategy for ::core::ops::RangeInclusive<$typ> {
                type Value = $typ;
                fn value(&self) -> Self::Value {
                    let r = verifier_abstract_value(<$typ>::default());
                    verifier_assume(*self.start() <= r);
                    verifier_assume(r <= *self.end());
                    r
                }
            }

            impl Strategy for ::core::ops::RangeFrom<$typ> {
                type Value = $typ;
                fn value(&self) -> Self::Value {
                    let r = verifier_abstract_value(<$typ>::default());
                    verifier_assume(self.start <= r);
                    r
                }
            }

            impl Strategy for ::core::ops::RangeTo<$typ> {
                type Value = $typ;
                fn value(&self) -> Self::Value {
                    let r = verifier_abstract_value(<$typ>::default());
                    verifier_assume(r < self.end);
                    r
                }
            }

            impl Strategy for ::core::ops::RangeToInclusive<$typ> {
                type Value = $typ;
                fn value(&self) -> Self::Value {
                    let r = verifier_abstract_value(<$typ>::default());
                    verifier_assume(r <= self.end);
                    r
                }
            }

        )*
    }
}

numeric_api! {
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

macro_rules! strategic_tuple {
    {$($idx:tt => $s:ident;)*} => {

        impl<$($s: Strategy),*> Strategy for ($($s),*) {
            type Value = ($($s::Value,)*);
            fn value(&self) -> Self::Value {
                ($(self.$idx.value()),*)
            }
        }
    };
}
// todo: It should be possible to write a macro that generates this sequence
strategic_tuple!{}
// A Tuple1 instance would create a warning
strategic_tuple!{0=>A; 1=>B;}
strategic_tuple!{0=>A; 1=>B; 2=>C;}
strategic_tuple!{0=>A; 1=>B; 2=>C; 3=>D;}
strategic_tuple!{0=>A; 1=>B; 2=>C; 3=>D; 4=>E;}
strategic_tuple!{0=>A; 1=>B; 2=>C; 3=>D; 4=>E; 5=>F;}
strategic_tuple!{0=>A; 1=>B; 2=>C; 3=>D; 4=>E; 5=>F; 6=>G;}
strategic_tuple!{0=>A; 1=>B; 2=>C; 3=>D; 4=>E; 5=>F; 6=>G; 7=>H;}
strategic_tuple!{0=>A; 1=>B; 2=>C; 3=>D; 4=>E; 5=>F; 6=>G; 7=>H; 8=>I;}
strategic_tuple!{0=>A; 1=>B; 2=>C; 3=>D; 4=>E; 5=>F; 6=>G; 7=>H; 8=>I; 9=>J;}
strategic_tuple!{0=>A; 1=>B; 2=>C; 3=>D; 4=>E; 5=>F; 6=>G; 7=>H; 8=>I; 9=>J; 10=>K;}
strategic_tuple!{0=>A; 1=>B; 2=>C; 3=>D; 4=>E; 5=>F; 6=>G; 7=>H; 8=>I; 9=>J; 10=>K; 11=>L;}

// Array strategy where S is element strategy and T is [S::Value; n] for some n
pub struct ArrayStrategy<S, T> {
    s: S,
    _marker: PhantomData<T>,
}
impl<S, T> ArrayStrategy<S, T> {
    pub fn new(s: S) -> Self {
        Self {
            s,
            _marker: PhantomData,
        }
    }
}

macro_rules! small_array {
    ($n:tt $name:ident : $($elt:ident),*) => {
        pub fn $name<S: Strategy> (s: S) -> ArrayStrategy<S, [S::Value; $n]>
        {
            ArrayStrategy {
                s,
                _marker: PhantomData,
            }
        }

        impl<S: Strategy> Strategy for ArrayStrategy<S, [S::Value; $n]>
        {
            type Value = [S::Value; $n];
            fn value(&self) -> Self::Value {
                $(let $elt = self.s.value();)*
                [$($elt),*]
            }
        }
    }
}

// todo: it should be possible to write a macro that generates this sequence
small_array!(0  uniform0 : );
small_array!(1  uniform1 : a0);
small_array!(2  uniform2 : a0, a1);
small_array!(3  uniform3 : a0, a1, a2);
small_array!(4  uniform4 : a0, a1, a2, a3);
small_array!(5  uniform5 : a0, a1, a2, a3, a4);
small_array!(6  uniform6 : a0, a1, a2, a3, a4, a5);
small_array!(7  uniform7 : a0, a1, a2, a3, a4, a5, a6);
small_array!(8  uniform8 : a0, a1, a2, a3, a4, a5, a6, a7);
small_array!(9  uniform9 : a0, a1, a2, a3, a4, a5, a6, a7, a8);
small_array!(10 uniform10: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
small_array!(11 uniform11: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10);
small_array!(12 uniform12: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11);
small_array!(13 uniform13: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12);
small_array!(14 uniform14: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13);
small_array!(15 uniform15: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14);
small_array!(16 uniform16: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15);
small_array!(17 uniform17: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16);
small_array!(18 uniform18: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17);
small_array!(19 uniform19: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18);
small_array!(20 uniform20: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19);
small_array!(21 uniform21: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
                           a20);
small_array!(22 uniform22: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
                           a20, a21);
small_array!(23 uniform23: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
                           a20, a21, a22);
small_array!(24 uniform24: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
                           a20, a21, a22, a23);
small_array!(25 uniform25: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
                           a20, a21, a22, a23, a24);
small_array!(26 uniform26: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
                           a20, a21, a22, a23, a24, a25);
small_array!(27 uniform27: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
                           a20, a21, a22, a23, a24, a25, a26);
small_array!(28 uniform28: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
                           a20, a21, a22, a23, a24, a25, a26, a27);
small_array!(29 uniform29: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
                           a20, a21, a22, a23, a24, a25, a26, a27, a28);
small_array!(30 uniform30: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
                           a20, a21, a22, a23, a24, a25, a26, a27, a28, a29);
small_array!(31 uniform31: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
                           a20, a21, a22, a23, a24, a25, a26, a27, a28, a29,
                           a30);
small_array!(32 uniform32: a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,
                           a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
                           a20, a21, a22, a23, a24, a25, a26, a27, a28, a29,
                           a30, a31);



pub struct OptionStrategy<S> {
    s: S,
}
impl<S> OptionStrategy<S> {
    pub fn new(s: S) -> Self {
        Self {
            s,
        }
    }
}
impl<S: Strategy> Strategy for OptionStrategy<S>
where
    S: Strategy + Clone,
{
    type Value = Option<S::Value>;
    fn value(&self) -> Self::Value {
        if Any::<bool>::new().value() {
            Some(self.s.value())
        } else {
            None
        }
    }
}

pub struct ResultStrategy<A, B> {
    a: A,
    b: B,
}
impl<A, B> ResultStrategy<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
        }
    }
}
impl<A, B> Strategy for ResultStrategy<A, B>
where
    A: Strategy + Clone,
    B: Strategy + Clone,
{
    type Value = Result<A::Value, B::Value>;
    fn value(&self) -> Self::Value {
        if Any::<bool>::new().value() {
            Ok(self.a.value())
        } else {
            Err(self.b.value())
        }
    }
}

pub struct VecStrategy<S> {
    size: usize, // concrete size to be more friendly to concolic/DSE
    elements: S,
}
impl<S> VecStrategy<S> {
    pub fn new(size: usize, elements: S) -> Self {
        Self {
            size,
            elements,
        }
    }
}
impl<S: Strategy> Strategy for VecStrategy<S>
where
    S: Strategy + Clone,
{
    type Value = Vec<S::Value>;
    fn value(&self) -> Self::Value {
        // Note that choosing a small, symbolic size causes KLEE to complain so
        // the length must be concrete.
        // let len = Strategy::value(&(..=self.size));
        let len = self.size;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(self.elements.value());
        }
        vec
    }
}

