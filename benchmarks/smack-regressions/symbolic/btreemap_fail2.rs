// @expect error

symbolic2::verify!{
    (v in symbolic2::BTreeMapStrategy::new(5, -5..5i32, 10..20u32)) {
        for (_, value) in v.iter() {
            verifier::assert!((*value) > 15);
        }
    }
}
