// @expect error

symbolic2::verify!{
    (v in symbolic2::VecDequeStrategy::new(5, 0..10u32)) {
        for x in &v {
            verifier::assert!(*x < 5);
        }
    }
}
