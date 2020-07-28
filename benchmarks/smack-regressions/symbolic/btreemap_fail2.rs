// @expect error

use symbolic2::*;

proptest!{
    fn main(v in btree_map(-5..5i32, 10..20u32, 5)) {
        for (_, value) in v.iter() {
            verifier::assert!((*value) > 15);
        }
    }
}
