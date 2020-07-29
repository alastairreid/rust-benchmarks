// @expect verified

use proptest::prelude::*;

proptest!{
    fn test(v in prop::collection::vec(0..10u32, 5)) {
        assert!(v.len() == 5);
        for x in &v {
            assert!(*x < 10);
        }
    }
}

fn main() {
    test();
}
