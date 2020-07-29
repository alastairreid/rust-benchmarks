// @expect error

use proptest::prelude::*;

proptest!{
    fn main(a in (0..10i32).prop_union(20..30i32)) {
        assert!(a != 25);
    }
}

