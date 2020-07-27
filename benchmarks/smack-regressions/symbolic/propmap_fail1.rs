// @expect error

use symbolic2::*;

verify!{
    (a in (0..10i32).prop_map(|x| x+50)) {
        verifier::assert!(a < 10);
    }
}
