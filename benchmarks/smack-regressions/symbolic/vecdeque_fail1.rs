// @expect error

use symbolic2::*;

proptest!{
    fn main(v in vec_deque(0..10u32, 5)) {
        for x in &v {
            verifier::assert!(*x < 5);
        }
    }
}
