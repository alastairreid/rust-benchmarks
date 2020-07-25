// @expect verified
use symbolic2::Strategy;

pub fn main() {
    let a = (0..10u32).value();
    let b = (10..10u32).value();
    verifier::assert_ne!(a, b)
}
