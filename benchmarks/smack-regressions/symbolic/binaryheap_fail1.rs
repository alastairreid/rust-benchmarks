// @expect error

use proptest::prelude::*;

proptest!{
    fn main(v in binary_heap(0..100u32, 5)) {
        for x in v.iter() {
            verifier::assert!(*x < 10);
        }
    }
}
