// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(v in vec(0..10u32, 5)) {
        verifier::assert!(v.len() == 5);
        for x in &v {
            verifier::assert!(*x < 10);
        }
    }
}
