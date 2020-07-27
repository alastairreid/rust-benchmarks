// @expect verified

symbolic2::verify!{
    (v in symbolic2::btree_set(-100..100i32, 5)) {

        // Note that key collisions may reduce the number of entries
        // so the following assertion will fail.
        // verifier::assert!(v.len() == 5);
        verifier::assert!(v.len() <= 5);

        for x in v.iter() {
            verifier::assert!((-100..100i32).contains(x));
        }
    }
}
