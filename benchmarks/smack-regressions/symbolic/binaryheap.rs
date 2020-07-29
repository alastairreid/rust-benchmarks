// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(v in binary_heap(0..100u32, 5)) {
        verifier::assert!(v.len() == 5);
        for x in v.iter() {
            verifier::assert!(*x < 100);
        }

        // check first element larger than rest
        let mut v1 = v;
        let x0 = v1.pop().unwrap();
        for x in v1.iter() {
            verifier::assert!(*x <= x0);
        }
    }
}
