// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(a in 0..10u32, b in 10..20u32) {
        assert_ne!(a, b)
    }
}
