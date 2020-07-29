// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(v in prop::collection::linked_list(0..10u32, 5)) {
        assert!(v.len() == 5);
        for x in &v {
            assert!(*x < 10);
        }
    }
}
