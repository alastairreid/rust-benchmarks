// @expect verified

use symbolic2::*;

verify!{
    ((a, b) in (1..65536).prop_ind_flat_map(|a| (Just(a), 0..a))) {
        verifier::assert!(a > b);
    }
}
