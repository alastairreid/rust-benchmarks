// @expect verified

use proptest::prelude::*;

proptest!{
    fn main((a, b) in (1..65536).prop_ind_flat_map(|a| (Just(a), 0..a))) {
        assert!(a > b);
    }
}
