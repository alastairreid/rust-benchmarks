////////////////////////////////////////////////////////////////
// Proptest-based tests exploring how to use proptest with
// enumerations
////////////////////////////////////////////////////////////////

// @expect verified

use proptest::prelude::*;
use core::fmt::Debug;

////////////////////////////////////////////////////////////////
// An enumeration type
//
// Example taken from proptest
////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
enum MyEnum {
    Big(u64),
    Medium(u32),
    Little(i16),
}

fn my_enum_strategy(s: u8) -> impl Strategy<Value = MyEnum> {
    prop_oneof![
        (0..i16::from(s)).prop_map(MyEnum::Little),
        (0..u32::from(s)).prop_map(MyEnum::Medium),
        (0..u64::from(s)).prop_map(MyEnum::Big),
    ]
}

proptest!{
    fn enum_test1(x in my_enum_strategy(10)) {
        match x {
            MyEnum::Big(b) => assert!(b < 10),
            MyEnum::Medium(m) => assert!(m < 10),
            MyEnum::Little(l) => assert!(l < 10)
        }
    }
}

fn main() {
    enum_test1();
}

////////////////////////////////////////////////////////////////
// End
////////////////////////////////////////////////////////////////
