// @expect overflow
use symbolic2::Strategy;

pub fn main() {
    let a = (0 .. std::i32::MAX).value();
    let b = (0 .. std::i32::MAX).value();
    verifier::assert_eq!(a + b, b + a);
}
