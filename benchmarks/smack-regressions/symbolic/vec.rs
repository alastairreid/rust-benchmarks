// @expect verified

symbolic2::verify!{
    (v in symbolic2::VecStrategy::new(5, 0..10u32)) {
		verifier::assert!(v.len() == 5);
		for x in &v {
			verifier::assert!(*x < 10);
		}
	}
}
