// @expect verified
use symbolic2::Strategy;

pub fn main() {
    let (a, b) = (0u32.., 0u32..).value();
    verifier::assert_eq!((a <= b), (b >= a));
}
