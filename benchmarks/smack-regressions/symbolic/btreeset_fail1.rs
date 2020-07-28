// @expect error

use symbolic2::*;

proptest!{
    fn main(v in btree_set(-100..100i32, 5)) {
        for x in v.iter() {
            verifier::assert!((0..100i32).contains(x));
        }
    }
}
