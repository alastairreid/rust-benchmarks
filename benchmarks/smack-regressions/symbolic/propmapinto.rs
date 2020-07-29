// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(a in (0u8..).prop_map_into::<u32>()) {
        assert!(a < 256);
    }
}
