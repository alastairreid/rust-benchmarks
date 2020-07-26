// @expect error

symbolic2::verify!{
    (v in symbolic2::BTreeSetStrategy::new(5, -100..100i32)) {
        for x in v.iter() {
            verifier::assert!((0..100i32).contains(x));
        }
    }
}
