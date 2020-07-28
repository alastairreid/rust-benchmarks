// @expect error

use symbolic2::*;

verify!{
    (v in binary_heap(0..100u32, 5)) {
        // check first element larger than rest
        let mut v1 = v;
        let x0 = v1.pop().unwrap();
        for x in v1.iter() {
            verifier::assert!(*x < x0);
        }
    }
}
