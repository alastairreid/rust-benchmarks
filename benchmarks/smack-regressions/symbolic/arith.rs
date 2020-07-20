// @expect verified
// @flag --bit-precise

pub fn main() {
    test1();
    test2();
    test3();
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
