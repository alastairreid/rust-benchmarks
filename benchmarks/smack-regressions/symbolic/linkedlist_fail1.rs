// @expect error

symbolic2::verify!{
    (v in symbolic2::linked_list(0..10u32, 5)) {
        for x in &v {
            verifier::assert!(*x < 5);
        }
    }
}
