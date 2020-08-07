// @expect verified

////////////////////////////////////////////////////////////////
// Proptest-based tests exploring how to use proptest with
// trait objects
////////////////////////////////////////////////////////////////

use proptest::prelude::*;
use core::fmt::Debug;

////////////////////////////////////////////////////////////////
// A trait and a couple of implementations
////////////////////////////////////////////////////////////////

trait Foo: Debug {
    fn foo(&self) -> i32;
}

// A boxed trait object type
type FB = Box<dyn Foo>;

#[derive(Debug)]
struct A {
    a: i8,
}
impl Foo for A {
    fn foo(&self) -> i32 {
        self.a.into()
    }
}
fn a_to_foo(a: i8) -> FB { Box::new(A{a}) }

#[derive(Debug)]
struct B {
    b: i16,
}
impl Foo for B {
    fn foo(&self) -> i32 {
        self.b.into()
    }
}
fn b_to_foo(b: i16) -> FB { Box::new(B{b}) }


////////////////////////////////////////////////////////////////
// Proptest-based tests exploring how to use proptest with
// trait objects
////////////////////////////////////////////////////////////////

proptest!{
    fn dynamic(x in (0..10i8).prop_map(a_to_foo).boxed()) {
        // println!("x = {:?}", x);
        let y : i32 = x.foo();
        assert!(y != 15);
        assert!((0..10).contains(&y));
    }
}

proptest!{
    fn dynamic_union(r in (0..10i8).prop_map(|x| a_to_foo(x)).boxed().prop_union(
                          (1000i16..).prop_map(b_to_foo).boxed())) {
        // println!("r = {:?}", r);
        assert!(r.foo() < 10 || r.foo() > 100);
    }
}

// Slightly awkward dance so that the code can be run using 'cargo test'
// or using 'cargo run' depending on which you prefer.
#[test]
fn t1() { dynamic(); }

#[test]
fn t2() { dynamic_union(); }

fn main() {
    dynamic();
    dynamic_union();
}

////////////////////////////////////////////////////////////////
// End
////////////////////////////////////////////////////////////////
