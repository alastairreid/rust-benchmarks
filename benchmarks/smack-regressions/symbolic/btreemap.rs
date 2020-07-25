// @expect verified
use symbolic2::Strategy;

pub fn main() {
    let v = symbolic2::BTreeMapStrategy::new(5, -5..5i32, 10..20u32).value();
    // Note that key collisions may reduce the number of entries
    // so the following assertion will fail.
    // verifier::assert!(v.len() == 5);

    verifier::assert!(v.len() <= 5);

    for (key, value) in v.iter() {
        verifier::assert!((-5..5i32).contains(key));
        verifier::assert!((*value) > 5);
    }
}
