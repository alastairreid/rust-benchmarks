// @expect error
use symbolic2::Strategy;

pub fn main() {
    let v = symbolic2::BinaryHeapStrategy::new(5, 0..100u32).value();
    for x in v.iter() {
        verifier::assert!(*x < 10);
    }
}
