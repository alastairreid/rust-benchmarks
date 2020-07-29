// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(v in prop::collection::binary_heap(0..100u32, 5)) {
        assert!(v.len() == 5);
        for x in v.iter() {
            assert!(*x < 100);
        }

        // check first element larger than rest
        let mut v1 = v;
        let x0 = v1.pop().unwrap();
        for x in v1.iter() {
            assert!(*x <= x0);
        }
    }
}
