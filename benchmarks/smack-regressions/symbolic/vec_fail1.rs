// @expect error

symbolic2::verify!{
    (v in symbolic2::vec(0..10u32, 5)) {
        for x in &v {
            verifier::assert!(*x < 5);
        }
    }
}
