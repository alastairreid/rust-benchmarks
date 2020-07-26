// @expect error

symbolic2::verify!{
    (v in symbolic2::BinaryHeapStrategy::new(5, 0..100u32)) {
        for x in v.iter() {
            verifier::assert!(*x < 10);
        }
    }
}
