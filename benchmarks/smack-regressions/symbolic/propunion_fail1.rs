// @expect error

use symbolic2::*;

verify!{
    (a in (0..10i32).prop_union(20..30i32)) {
        verifier::assert!(a != 25);
    }
}

