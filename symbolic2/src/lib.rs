use std::marker::PhantomData;

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
    fn value(&self) -> bool {
        let c = klee_annotations::verifier_abstract_value(0u8);
        klee_annotations::verifier_assume(c == 0 || c == 1);
        c == 1
    }
}

// For simple types, we can add assumptions after the fact
// the assumptions are concrete, not symbolic!
//
// Note that this should be avoided for data structures
// whose size can vary significantly.
use std::sync::Arc;

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
        klee_annotations::verifier_assume((self.fun)(&val));
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
    ($typ:ident) => {
        impl Strategy for ::core::ops::Range<$typ> {
            type Value = $typ;
            fn value(&self) -> Self::Value {
                let r = klee_annotations::verifier_abstract_value(<$typ>::default());
                klee_annotations::verifier_assume(self.start <= r);
                klee_annotations::verifier_assume(r < self.end);
                r
            }
        }

        impl Strategy for ::core::ops::RangeInclusive<$typ> {
            type Value = $typ;
            fn value(&self) -> Self::Value {
                let r = klee_annotations::verifier_abstract_value(<$typ>::default());
                klee_annotations::verifier_assume(*self.start() <= r);
                klee_annotations::verifier_assume(r <= *self.end());
                r
            }
        }

        impl Strategy for ::core::ops::RangeFrom<$typ> {
            type Value = $typ;
            fn value(&self) -> Self::Value {
                let r = klee_annotations::verifier_abstract_value(<$typ>::default());
                klee_annotations::verifier_assume(self.start <= r);
                r
            }
        }

        impl Strategy for ::core::ops::RangeTo<$typ> {
            type Value = $typ;
            fn value(&self) -> Self::Value {
                let r = klee_annotations::verifier_abstract_value(<$typ>::default());
                klee_annotations::verifier_assume(r < self.end);
                r
            }
        }

        impl Strategy for ::core::ops::RangeToInclusive<$typ> {
            type Value = $typ;
            fn value(&self) -> Self::Value {
                let r = klee_annotations::verifier_abstract_value(<$typ>::default());
                klee_annotations::verifier_assume(r <= self.end);
                r
            }
        }

    };
}

numeric_api!(u8);
numeric_api!(u16);
numeric_api!(u32);
numeric_api!(u64);
numeric_api!(u128);
numeric_api!(usize);
numeric_api!(i8);
numeric_api!(i16);
numeric_api!(i32);
numeric_api!(i64);
numeric_api!(i128);
numeric_api!(isize);

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

