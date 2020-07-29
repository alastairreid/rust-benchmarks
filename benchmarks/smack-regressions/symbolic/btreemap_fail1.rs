// @expect error

use proptest::prelude::*;

proptest!{
    fn main(v in prop::collection::btree_map(-5..5i32, 10..20u32, 5)) {
        for (key, _) in v.iter() {
            assert!((0..5i32).contains(key));
        }
    }
}
