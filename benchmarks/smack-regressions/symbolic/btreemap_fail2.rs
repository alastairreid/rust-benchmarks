// @expect error
use symbolic2::Strategy;

pub fn main() {
    let v = symbolic2::BTreeMapStrategy::new(5, -5..5i32, 10..20u32).value();
    for (_, value) in v.iter() {
        verifier::assert!((*value) > 15);
    }
}
