use klee_annotations::*;
use std::marker::PhantomData;
use std::sync::Arc;

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque, LinkedList};
// use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

// Trait representing a set of values from which one can be chosen
//
// The primary method is `value` chooses a value from the set.
//
// The other methods are copied from the proptest Strategy trait - see the documentation
// for proptest.
//
// Implementations of this trait are datatypes such as Any, Const, VecStrategy, etc.
// and, in some cases, these datatypes mirror the type structure for which they
// generate values.
//
// Strategies for composite types (tuples, vectors, etc.) typically contain
// strategies for generating components of that type (e.g., the struct fields,
// array/vector elements, etc.)
pub trait Strategy {
    type Value;
    fn value(&self) -> Self::Value;

    fn prop_map<O, F: Fn(Self::Value) -> O>(
        self,
        fun: F,
    ) -> Map<Self, F>
    where
        Self: Sized,
    {
        Map {
            source: self,
            fun: Arc::new(fun),
        }
    }

    fn prop_map_into<O>(self) -> MapInto<Self, O>
    where
        Self: Sized,
        Self::Value: Into<O>,
    {
        MapInto {
            source: self,
            output: PhantomData,
        }
    }

    fn prop_flat_map<S: Strategy, F: Fn(Self::Value) -> S>(
        self,
        fun: F,
    ) -> Flatten<Map<Self, F>>
    where
        Self: Sized,
    {
        Flatten {
            source: Map {
                source: self,
                fun: Arc::new(fun),
            },
        }
    }

    // Todo: In proptest, the only difference between prop_flat_map
    // and prop_ind_flat_map is in how they shrink.
    // So it is not clear that there is any point in having
    // this method. Or, maybe this method should exist for compatibility
    // but it should just call prop_flat_map
    fn prop_ind_flat_map<S: Strategy, F: Fn(Self::Value) -> S>(
        self,
        fun: F,
    ) -> IndFlatten<Map<Self, F>>
    where
        Self: Sized,
    {
        IndFlatten {
            source: Map {
                source: self,
                fun: Arc::new(fun),
            },
        }
    }

    // Todo: In proptest, the only difference between prop_flat_map
    // and prop_ind_flat_map2 is in how they shrink and that
    // prop_ind_flat_map2 returns a tuple of type `(Self::Value, S)`.
    // Maybe, it is not needed or, for compatibility with proptest,
    // it should be implemented with a call to prop_flat_map.
    fn prop_ind_flat_map2<S: Strategy, F: Fn(Self::Value) -> S>(
        self,
        fun: F,
    ) -> IndFlattenMap<Self, F>
    where
        Self: Sized,
    {
        IndFlattenMap {
            source: self,
            fun: Arc::new(fun),
        }
    }

    fn prop_filter<F: Fn(&Self::Value) -> bool>(
        self,
        _whence: &str,
        fun: F,
    ) -> Filter<Self, F>
    where
        Self: Sized,
    {
        Filter {
            source: self,
            fun: Arc::new(fun),
        }
    }

    fn prop_filter_map<F: Fn(Self::Value) -> Option<O>, O>(
        self,
        _whence: &str,
        fun: F,
    ) -> FilterMap<Self, F>
    where
        Self: Sized,
    {
        FilterMap {
            source: self,
            fun: Arc::new(fun),
        }
    }

    fn prop_union(self, other: Self) -> Union<Self>
    where
        Self: Sized,
    {
        Union {
            x: self,
            y: other,
        }
    }

    fn boxed(self) -> BoxedStrategy<Self::Value>
    where
        Self: Sized + 'static,
    {
        BoxedStrategy {
            b: Box::new(self),
        }
    }

}

#[macro_export]
macro_rules! proptest {
  (
      $(#[$meta:meta])*
      fn $test_name:ident($($parm:tt in $strategy:expr),+ $(,)?) $body:block
  ) => {
      $(#[$meta])*
      fn $test_name() {
          klee_annotations::verifier_set_ignore_panic_hook();
          $(let $parm = $crate::prelude::Strategy::value(&$strategy);)*
          if klee_annotations::verifier_is_ktest() {
              $(println!("  Value {} = {:?}", std::stringify!($parm), $parm);)*
          }
          klee_annotations::verifier_set_show_panic_hook();
          $body
      }
  };
}

// The remainder of this file consists of implementations of the Strategy trait.
// In most cases, this consists of defining a new struct type to represent
// the strategy, defining functions to construct that struct type and
// then implementing the Strategy trait for that type.


// The most trivial strategy
pub struct Just<T: Clone>(
    pub T,
);
impl<T: Clone> Strategy for Just<T> {
    type Value = T;
    fn value(&self) -> Self::Value {
        self.0.clone()
    }
}


impl<T> Strategy for fn() -> T {
    type Value = T;

    fn value(&self) -> Self::Value {
        self()
    }
}


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


pub struct Map<S: Strategy, F> {
    source: S,
    fun: Arc<F>,
}
impl<S: Strategy, T, F: Fn(S::Value) -> T> Strategy for Map<S, F> {
    type Value = T;
    fn value(&self) -> Self::Value {
        let val = self.source.value();
        (self.fun)(val)
    }
}


pub struct MapInto<S: Strategy, T> {
    source: S,
    output: PhantomData<T>,
}
impl<S: Strategy, T> Strategy for MapInto<S, T>
where
    S::Value: Into<T>,
{
    type Value = T;
    fn value(&self) -> Self::Value {
        let val = self.source.value();
        val.into()
    }
}


pub struct IndFlatten<S> {
    source: S,
}
impl<S: Strategy> Strategy for IndFlatten<S>
where
    S::Value: Strategy,
{
    type Value = <S::Value as Strategy>::Value;
    fn value(&self) -> Self::Value {
        self.source.value().value()
    }
}


pub struct IndFlattenMap<S, F> {
    source: S,
    fun: Arc<F>,
}
impl<S: Strategy, T: Strategy, F: Fn(S::Value) -> T> Strategy for IndFlattenMap<S, F>
where
    S::Value: Copy,
{
    type Value = (S::Value, T::Value);
    fn value(&self) -> Self::Value {
        let s = self.source.value();
        let r = (self.fun)(s).value();
        (s, r)
    }
}


pub struct Flatten<S> {
    source: S,
}
impl<S: Strategy> Strategy for Flatten<S>
where
    S::Value: Strategy,
{
    type Value = <S::Value as Strategy>::Value;
    fn value(&self) -> Self::Value {
        let val = self.source.value();
        val.value()
    }
}


pub struct Filter<S: Strategy, F> {
    source: S,
    fun: Arc<F>,
}
impl<S: Strategy, F: Fn(&S::Value) -> bool> Strategy for Filter<S, F> {
    type Value = S::Value;
    fn value(&self) -> Self::Value {
        let val = self.source.value();
        verifier_assume((self.fun)(&val));
        val
    }
}


pub struct FilterMap<S: Strategy, F> {
    source: S,
    fun: Arc<F>,
}
impl<S: Strategy, F: Fn(S::Value) -> Option<T>, T> Strategy for FilterMap<S, F> {
    type Value = T;
    fn value(&self) -> Self::Value {
        let val = self.source.value();
        match (self.fun)(val) {
            Some(r) => r,
            None => verifier_reject()
        }
    }
}


pub struct Union<S: Strategy> {
    x: S,
    y: S,
}
impl<S: Strategy> Strategy for Union<S> {
    type Value = S::Value;
    fn value(&self) -> Self::Value {
        if verifier_abstract_value(0u8) == 0 {
            self.x.value()
        } else {
            self.y.value()
        }
    }
}

pub struct BoxedStrategy<T> {
    b: Box<dyn Strategy<Value = T>>
}
impl<T: Strategy> Strategy for BoxedStrategy<T>
{
    type Value = T;
    fn value(&self) -> Self::Value {
        self.b.value()
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
pub struct ArrayStrategy<S: Strategy, T> {
    s: S,
    _marker: PhantomData<T>,
}
impl<S: Strategy, T> ArrayStrategy<S, T> {
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

pub struct OptionStrategy<S: Strategy> {
    s: S,
}
impl<S: Strategy> OptionStrategy<S> {
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

pub struct ResultStrategy<A: Strategy, B: Strategy> {
    a: A,
    b: B,
}
impl<A: Strategy, B: Strategy> ResultStrategy<A, B> {
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

pub struct VecStrategy<S: Strategy> {
    element: S,
    size: usize, // concrete size to be more friendly to concolic/DSE
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
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(self.element.value());
        }
        v
    }
}

pub fn vec<S: Strategy>(
    element: S,
    size: usize,
) -> VecStrategy<S> {
    VecStrategy {
        element,
        size,
    }
}


pub struct VecDequeStrategy<S: Strategy> {
    element: S,
    size: usize, // concrete size to be more friendly to concolic/DSE
}
impl<S: Strategy> Strategy for VecDequeStrategy<S>
where
    S: Strategy + Clone,
{
    type Value = VecDeque<S::Value>;
    fn value(&self) -> Self::Value {
        // Note that choosing a small, symbolic size causes KLEE to complain so
        // the length must be concrete.
        // let len = Strategy::value(&(..=self.size));
        let len = self.size;
        let mut v = VecDeque::with_capacity(len);
        for _ in 0..len {
            v.push_front(self.element.value());
        }
        v
    }
}

pub fn vec_deque<S: Strategy>(
    element: S,
    size: usize,
) -> VecDequeStrategy<S> {
    VecDequeStrategy {
        element,
        size,
    }
}


pub struct LinkedListStrategy<S: Strategy> {
    element: S,
    size: usize, // concrete size to be more friendly to concolic/DSE
}
impl<S: Strategy> Strategy for LinkedListStrategy<S>
where
    S: Strategy + Clone,
{
    type Value = LinkedList<S::Value>;
    fn value(&self) -> Self::Value {
        let len = self.size;
        let mut v = LinkedList::new();
        for _ in 0..len {
            v.push_front(self.element.value());
        }
        v
    }
}

pub fn linked_list<S: Strategy>(
    element: S,
    size: usize,
) -> LinkedListStrategy<S> {
    LinkedListStrategy {
        element,
        size,
    }
}

pub struct BTreeMapStrategy<K: Strategy, V: Strategy> {
    keys: K,
    value: V,
    size: usize, // concrete size to be more friendly to concolic/DSE
}
impl<K: Strategy, V: Strategy> Strategy for BTreeMapStrategy<K, V>
where
    K::Value : Ord + Copy
{
    type Value = BTreeMap<K::Value, V::Value>;
    fn value(&self) -> Self::Value {
        // Having a range of sizes up to some limit is acceptable
        // but I think it adds some overhead with little gain.
        // let len = Strategy::value(&(..=self.size));
        let len = self.size;
        let mut r = BTreeMap::new();

        // Keys are generated in increasing order to
        // reduce the number of effectively equivalent
        // paths through the generation code.
        let mut k = self.keys.value();
        for _ in 0..len {
            r.insert(k, self.value.value());
            let next = self.keys.value();
            verifier_assume(k <= next); // generate entries in fixed order
            k = next;
        }
        r
    }
}

pub fn btree_map<K: Strategy, V: Strategy>(
    keys: K,
    value: V,
    size: usize,
) -> BTreeMapStrategy<K, V>
where
    K::Value: Ord,
{
    BTreeMapStrategy {
        size,
        keys,
        value,
    }
}

pub struct BTreeSetStrategy<S: Strategy> {
    element: S,
    size: usize, // concrete size to be more friendly to concolic/DSE
}
impl<S: Strategy> Strategy for BTreeSetStrategy<S>
where
    S::Value : Ord + Copy
{
    type Value = BTreeSet<S::Value>;
    fn value(&self) -> Self::Value {
        // Having a range of sizes up to some limit is acceptable
        // but I think it adds some overhead with little gain.
        // let len = Strategy::value(&(..=self.size));
        let len = self.size;
        let mut r = BTreeSet::new();

        // Keys are generated in increasing order to
        // reduce the number of effectively equivalent
        // paths through the generation code.
        let mut k = self.element.value();
        for _ in 0..len {
            r.insert(k);
            let next = self.element.value();
            verifier_assume(k <= next); // generate entries in fixed order
            k = next;
        }
        r
    }
}

pub fn btree_set<S: Strategy>(
    element: S,
    size: usize,
) -> BTreeSetStrategy<S>
where
    S::Value: Ord,
{
    BTreeSetStrategy {
        element,
        size,
    }
}


pub struct BinaryHeapStrategy<S: Strategy> {
    element: S,
    size: usize, // concrete size to be more friendly to concolic/DSE
}
impl<S: Strategy> Strategy for BinaryHeapStrategy<S>
where
    S::Value : Ord + Copy
{
    type Value = BinaryHeap<S::Value>;
    fn value(&self) -> Self::Value {
        // Having a range of sizes up to some limit is acceptable
        // but I think it adds some overhead with little gain.
        // let len = Strategy::value(&(..=self.size));
        let len = self.size;
        let mut r = BinaryHeap::with_capacity(len);

        // Keys are generated in increasing order to
        // reduce the number of effectively equivalent
        // paths through the generation code.
        // (This would not be a good idea if we were checking BinaryHeap
        // but our goal is to checking code that uses BinaryHeap.)
        let mut k = self.element.value();
        for _ in 0..len {
            r.push(k);
            let next = self.element.value();
            verifier_assume(k <= next); // generate entries in fixed order
            k = next;
        }
        r
    }
}

pub fn binary_heap<S: Strategy>(
    element: S,
    size: usize,
    )
-> BinaryHeapStrategy<S>
where
    S::Value: Ord,
{
    BinaryHeapStrategy {
        element,
        size,
    }
}
