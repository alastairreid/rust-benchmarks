// @expect verified

use symbolic2::*;

verify!{
    ((a, b) in (0u32.., 0u32..)) {
        verifier::assert_eq!((a <= b), (b >= a));
    }
}
