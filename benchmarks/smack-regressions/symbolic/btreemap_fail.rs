// @expect error
use symbolic2::Strategy;

pub fn main() {
    let v = symbolic2::BTreeMapStrategy::new(5, -5..5i32, 10..20u32).value();
    for (key, _) in v.iter() {
        verifier::assert!((0..5i32).contains(key));
    }
}
