// @expect error

symbolic2::verify!{
    (v in symbolic2::BTreeMapStrategy::new(5, -5..5i32, 10..20u32)) {
        for (key, _) in v.iter() {
            verifier::assert!((0..5i32).contains(key));
        }
    }
}
