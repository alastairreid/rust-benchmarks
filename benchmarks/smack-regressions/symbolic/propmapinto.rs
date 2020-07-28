// @expect verified

use symbolic2::*;

proptest!{
    // todo: the need to put a comma after the type is not ideal but
    // is the best I could come up with at the time
    (a: u32, in (0u8..).prop_map_into()) {
        verifier::assert!(a < 256u32);
    }
}
