// @expect verified

use symbolic2::*;

verify!{
    (v in btree_map(-5..5i32, 10..20u32, 5)) {

        // Note that key collisions may reduce the number of entries
        // so the following assertion will fail.
        // verifier::assert!(v.len() == 5);
        verifier::assert!(v.len() <= 5);

        for (key, value) in v.iter() {
            verifier::assert!((-5..5i32).contains(key));
            verifier::assert!((*value) > 5);
        }
    }
}
