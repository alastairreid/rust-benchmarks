// @expect verified
// @flag --bit-precise

use symbolic2::Strategy;

pub fn main() {
    test_int1();
    test_tuple1();
    test_range1();
    test_range2();
}


pub fn test_int1() {
    let a = Strategy::value(&(0..10u32));
    verifier::assert_eq!(a + 1, 1 + a);
    verifier::assert!(a < 10);
    verifier::assert!(0 <= a);
}

pub fn test_tuple1() {
    let s = (0u32.., 0u32..);
    let (a, b) = Strategy::value(&s);
    verifier::assert_eq!((a <= b), (b >= a));
}

pub fn test_range1() {
    let s = std::i32::MIN/2 .. std::i32::MAX/2;
    let (a, b) = Strategy::value(&(s.clone(), s.clone()));
    verifier::assert_eq!(a + b, b + a);
}

pub fn test_range2() {
    let s = std::i32::MIN/2 .. std::i32::MAX/2;
    let (a, b) = Strategy::value(&(s.clone(), s.clone()));
    verifier::assert!(a <= b || b <= a);
}
