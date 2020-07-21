// @expect verified
// @flag --bit-precise

pub fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
}

pub fn test1() {
    let a: u32 = symbolic::Symbolic::symbolic();
    verifier::assume!(a < std::u32::MAX);
    verifier::assert_eq!(a + 1, 1 + a);
}

pub fn test2() {
    let a: u32 = symbolic::Symbolic::symbolic();
    let b: u32 = symbolic::Symbolic::symbolic();
    verifier::assume!(a <= std::u32::MAX / 2);
    verifier::assume!(b <= std::u32::MAX / 2);
    verifier::assert_eq!(a + b, b + a);
}

pub fn test3() {
    let a: i32 = symbolic::Symbolic::symbolic();
    let b: i32 = symbolic::Symbolic::symbolic();
    verifier::assume!(std::i32::MIN / 2 <= a);
    verifier::assume!(a <= std::i32::MAX / 2);
    verifier::assume!(std::i32::MIN / 2 <= b);
    verifier::assume!(b <= std::i32::MAX / 2);
    verifier::assert_eq!(a + b, b + a);
}

pub fn test4() {
    // symbolic tuples
    let (a, b): (u32, u32) = symbolic::Symbolic::symbolic();
    verifier::assert!(a <= b || b <= a);
}

pub fn test5() {
    // symbolic vectors - this check blows up at the moment
    klee_annotations::verifier_reject(); // abort this test - otherwise KLEE times out
    let v: Vec<u32> = symbolic::Symbolic::symbolic();
    verifier::assume!(v.len() == 0);
    for x in &v {
        verifier::assert!(*x == std::u32::MAX || *x + 1 != *x);
    }
}
