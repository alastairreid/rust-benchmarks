// @expect verified
// @flag --bit-precise

pub fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
}

use symbolic2::Strategy;

pub fn test1() {
    let a = Strategy::value(&(0..10u32));
    verifier::assert_eq!(a + 1, 1 + a);
    verifier::assert!(a < 10);
    verifier::assert!(0 <= a);
}

pub fn test2() {
    let s = (0u32.., 0u32..);
    let (a, b) = Strategy::value(&s);
    verifier::assert_eq!((a <= b), (b >= a));
}

pub fn test3() {
    let s = std::i32::MIN/2 .. std::i32::MAX/2;
    let (a, b) = Strategy::value(&(s.clone(), s.clone()));
    verifier::assert_eq!(a + b, b + a);
}

pub fn test4() {
    // symbolic tuples
    let s = std::i32::MIN/2 .. std::i32::MAX/2;
    let (a, b) = Strategy::value(&(s.clone(), s.clone()));
    verifier::assert!(a <= b || b <= a);
}

pub fn test5() {
    let e = 0..10u32;
    let s = symbolic2::VecStrategy::new(5, e);
    let v: Vec<u32> = Strategy::value(&s);
    verifier::assert!(v.len() == 5);
    for x in &v {
        verifier::assert!(*x < 10);
    }
}
