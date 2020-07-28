// @expect verified

use symbolic2::*;

verify!{
    (v in vec(0..10u32, 5)) {
        verifier::assert!(v.len() == 5);
        for x in &v {
            verifier::assert!(*x < 10);
        }
    }
}
