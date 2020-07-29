// @expect error

use proptest::prelude::*;

proptest!{
    fn main(v in prop::collection::vec_deque(0..10u32, 5)) {
        for x in &v {
            assert!(*x < 5);
        }
    }
}
