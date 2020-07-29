// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(a in 0..10u32, b in 10..10u32) {
        verifier::assert_ne!(a, b)
    }
}
