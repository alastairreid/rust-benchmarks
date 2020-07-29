// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(v in prop::collection::btree_map(-5..5i32, 10..20u32, 5)) {

        // Note that key collisions may reduce the number of entries
        // so the following assertion will fail.
        // assert!(v.len() == 5);
        assert!(v.len() <= 5);

        for (key, value) in v.iter() {
            assert!((-5..5i32).contains(key));
            assert!((*value) > 5);
        }
    }
}
