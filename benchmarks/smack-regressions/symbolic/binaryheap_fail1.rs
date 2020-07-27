// @expect error

symbolic2::verify!{
    (v in symbolic2::binary_heap(0..100u32, 5)) {
        for x in v.iter() {
            verifier::assert!(*x < 10);
        }
    }
}
