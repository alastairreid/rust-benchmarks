// @expect error

symbolic2::verify!{
    (v in symbolic2::btree_map(-5..5i32, 10..20u32, 5)) {
        for (_, value) in v.iter() {
            verifier::assert!((*value) > 15);
        }
    }
}
