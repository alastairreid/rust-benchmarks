// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(a in (0u32..1000).prop_filter_map("%2", |x| if x % 2 == 0 { Some(x*2) } else { None })) {
        assert!(a % 4 == 0);
    }
}
