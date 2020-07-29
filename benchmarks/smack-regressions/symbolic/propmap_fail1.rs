// @expect error

use proptest::prelude::*;

proptest!{
    fn main(a in (0..10i32).prop_map(|x| x+50)) {
        verifier::assert!(a < 10);
    }
}
