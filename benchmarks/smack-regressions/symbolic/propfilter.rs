// @expect verified

use symbolic2::*;

proptest!{
    fn main(a in (0..).prop_filter(|x| x % 4 == 0)) {
        verifier::assert!(a % 2 == 0);
    }
}
