// @expect verified
use symbolic2::Strategy;

pub fn main() {
    let v = symbolic2::VecDequeStrategy::new(5, 0..10u32).value();
    verifier::assert!(v.len() == 5);
    for x in &v {
        verifier::assert!(*x < 10);
    }
}
