// @expect verified
// @flag --bit-precise

pub fn main() {
    test_int1();
    test_tuple1();
    test_range1();
    test_range2();
    test_vec1();

    // As a really crude hack to avoid the number of paths
    // exploding under symbolic execution, we explore
    // the following in parallel (well, kinda... )
    if klee_annotations::verifier_abstract_value(false) {
        test_btreemap1();
    } else {
        test_btreeset1();
    }
}

use symbolic2::Strategy;

pub fn test_int1() {
    let a = Strategy::value(&(0..10u32));
    verifier::assert_eq!(a + 1, 1 + a);
    verifier::assert!(a < 10);
    verifier::assert!(0 <= a);
}

pub fn test_tuple1() {
    let s = (0u32.., 0u32..);
    let (a, b) = Strategy::value(&s);
    verifier::assert_eq!((a <= b), (b >= a));
}

pub fn test_range1() {
    let s = std::i32::MIN/2 .. std::i32::MAX/2;
    let (a, b) = Strategy::value(&(s.clone(), s.clone()));
    verifier::assert_eq!(a + b, b + a);
}

pub fn test_range2() {
    let s = std::i32::MIN/2 .. std::i32::MAX/2;
    let (a, b) = Strategy::value(&(s.clone(), s.clone()));
    verifier::assert!(a <= b || b <= a);
}

pub fn test_vec1() {
    let e = 0..10u32;
    let s = symbolic2::VecStrategy::new(5, e);
    let v = Strategy::value(&s);
    verifier::assert!(v.len() == 5);
    for x in &v {
        verifier::assert!(*x < 10);
    }
}

pub fn test_btreemap1() {
    let ks = -5..5i32;
    let vs = 10..20u32;
    let s = symbolic2::BTreeMapStrategy::new(5, ks, vs);
    let v = Strategy::value(&s);
    // Note that key collisions may reduce the number of entries
    // so the following assertion will fail.
    // verifier::assert!(v.len() == 5);

    verifier::assert!(v.len() <= 5);

    for (key, value) in v.iter() {
        verifier::assert!((-5..5i32).contains(key));
        verifier::assert!((*value) > 5);
    }
}

pub fn test_btreeset1() {
    let ks = -100..100i32;
    let s = symbolic2::BTreeSetStrategy::new(5, ks);
    let v = Strategy::value(&s);
    // Note that key collisions may reduce the number of entries
    // so the following assertion will fail.
    // verifier::assert!(v.len() == 5);

    verifier::assert!(v.len() <= 5);

    for x in v.iter() {
        verifier::assert!((-100..100i32).contains(x));
    }
}
