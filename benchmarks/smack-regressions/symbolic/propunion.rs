// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(a in (0..10i32).prop_union(20..30i32)) {
        verifier::assert!(a != 15);
        verifier::assert!((0..10).contains(&a) || (20..30).contains(&a));
    }
}

