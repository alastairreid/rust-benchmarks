// @expect error

use symbolic2::*;

proptest!{
    fn main((a, b) in (1..65536).prop_flat_map(|a| (Just(a), 0..a))) {
        verifier::assert!(a <= b);
    }
}
