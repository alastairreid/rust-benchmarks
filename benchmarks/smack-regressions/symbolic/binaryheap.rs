// @expect verified

symbolic2::verify!{
    (v in symbolic2::BinaryHeapStrategy::new(5, 0..100u32)) {
        verifier::assert!(v.len() == 5);
        for x in v.iter() {
            verifier::assert!(*x < 100);
        }

        // check first element larger than rest
        let mut v1 = v;
        let x0 = v1.pop().unwrap();
        for x in v1.iter() {
            verifier::assert!(*x <= x0);
        }
    }
}
