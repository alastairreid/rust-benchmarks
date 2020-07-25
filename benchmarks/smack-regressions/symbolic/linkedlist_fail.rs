// @expect error
use symbolic2::Strategy;

pub fn main() {
    let v = symbolic2::ListStrategy::new(5, 0..10u32).value();
    for x in &v {
        verifier::assert!(*x < 5);
    }
}
