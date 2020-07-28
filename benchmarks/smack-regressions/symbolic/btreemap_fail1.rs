// @expect error

use symbolic2::*;

verify!{
    (v in btree_map(-5..5i32, 10..20u32, 5)) {
        for (key, _) in v.iter() {
            verifier::assert!((0..5i32).contains(key));
        }
    }
}
