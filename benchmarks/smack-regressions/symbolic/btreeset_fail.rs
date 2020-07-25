// @expect error
use symbolic2::Strategy;

pub fn main() {
    let v = symbolic2::BTreeSetStrategy::new(5, -100..100i32).value();
    for x in v.iter() {
        verifier::assert!((0..100i32).contains(x));
    }
}
