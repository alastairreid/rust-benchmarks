// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(a in (0..).prop_filter(|x| x % 4 == 0)) {
        assert!(a % 2 == 0);
    }
}
