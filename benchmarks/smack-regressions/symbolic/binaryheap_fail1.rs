// @expect error

use proptest::prelude::*;

proptest!{
    fn main(v in prop::collection::binary_heap(0..100u32, 5)) {
        for x in v.iter() {
            assert!(*x < 10);
        }
    }
}
