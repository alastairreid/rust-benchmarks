// @expect verified
// @flag --bit-precise

extern crate klee_annotations;
extern crate symbolic;

pub fn main() {
    let a: u32 = symbolic::Symbolic::symbolic();
    let b: u32 = symbolic::Symbolic::symbolic();
    verifier::assert_eq!(a + b, b + a);
    verifier::assert_eq!(a + 1, 1 + a);
}
