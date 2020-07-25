// @expect verified
use symbolic2::Strategy;

pub fn main() {
    let a = (std::i32::MIN/2 .. std::i32::MAX/2).value();
    let b = (std::i32::MIN/2 .. std::i32::MAX/2).value();
    verifier::assert_eq!(a + b, b + a);
}
