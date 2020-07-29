// @expect error

use proptest::prelude::*;

proptest!{
    fn main(a in (0u16..).prop_map_into::<u32>()) {
        assert!(a < 256);
    }
}

