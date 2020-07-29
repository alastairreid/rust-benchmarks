// @expect verified

use proptest::prelude::*;

proptest!{
    fn main((a, b) in (0u32.., 0u32..)) {
        verifier::assert_eq!((a <= b), (b >= a));
    }
}
