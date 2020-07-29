// @expect error

use proptest::prelude::*;

proptest!{
    fn test(v in prop::collection::vec(0..10u32, 5)) {
        for x in &v {
            assert!(*x < 5);
        }
    }
}

fn main() {
    test();
}
