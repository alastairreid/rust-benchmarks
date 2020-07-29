// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(v in prop::collection::btree_set(-100..100i32, 5)) {

        // Note that key collisions may reduce the number of entries
        // so the following assertion will fail.
        // assert!(v.len() == 5);
        assert!(v.len() <= 5);

        for x in v.iter() {
            assert!((-100..100i32).contains(x));
        }
    }
}
