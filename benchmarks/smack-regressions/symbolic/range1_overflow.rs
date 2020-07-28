// @expect overflow

use symbolic2::*;

verify!{
    (a in 0 .. std::i32::MAX,
     b in 0 .. std::i32::MAX) {
        verifier::assert_eq!(a + b, b + a);
    }
}
