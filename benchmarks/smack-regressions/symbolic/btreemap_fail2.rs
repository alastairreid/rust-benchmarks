// @expect error

use proptest::prelude::*;

proptest!{
    fn main(v in prop::collection::btree_map(-5..5i32, 10..20u32, 5)) {
        for (_, value) in v.iter() {
            assert!((*value) > 15);
        }
    }
}
