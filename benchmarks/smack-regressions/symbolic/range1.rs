// @expect verified

use proptest::prelude::*;

proptest!{
    fn main(
        a in (std::i32::MIN/2 .. std::i32::MAX/2),
        b in (std::i32::MIN/2 .. std::i32::MAX/2),
    ) {
        verifier::assert_eq!(a + b, b + a);
    }
}
