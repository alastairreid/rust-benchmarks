// @expect overflow

use proptest::prelude::*;

proptest!{
    fn main(
        a in 0 .. std::i32::MAX,
        b in 0 .. std::i32::MAX,
    ) {
        assert_eq!(a + b, b + a);
    }
}
