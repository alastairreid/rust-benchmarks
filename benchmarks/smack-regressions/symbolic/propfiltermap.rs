// @expect verified

use symbolic2::*;

proptest!{
    fn main(a in (0u32..).prop_filter_map(|x| if x % 2 == 0 { Some(x*2) } else { None })) {
        verifier::assert!(a % 4 == 0);
    }
}
