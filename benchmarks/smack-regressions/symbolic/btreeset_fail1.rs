// @expect error

use proptest::prelude::*;

proptest!{
    fn main(v in prop::collection::btree_set(-100..100i32, 5)) {
        for x in v.iter() {
            assert!((0..100i32).contains(x));
        }
    }
}
