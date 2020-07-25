// @expect verified
use symbolic2::Strategy;

pub fn main() {
    let v = symbolic2::BTreeSetStrategy::new(5, -100..100i32).value();
    // Note that key collisions may reduce the number of entries
    // so the following assertion will fail.
    // verifier::assert!(v.len() == 5);

    verifier::assert!(v.len() <= 5);

    for x in v.iter() {
        verifier::assert!((-100..100i32).contains(x));
    }
}
