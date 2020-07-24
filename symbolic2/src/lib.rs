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

impl<A: Strategy, B: Strategy> Strategy for (A, B) {
    type Value = (A::Value, B::Value);
    fn value(&self) -> Self::Value {
        let (a, b) = self;
        let sa = a.value();
        let sb = b.value();
        (sa, sb)
    }
}

// todo: figure out how to write a macro that will generate the above for any tuple size


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

