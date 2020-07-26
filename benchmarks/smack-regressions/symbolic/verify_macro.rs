// @expect verified

symbolic2::verify!{
    (x in ..10u32, y in ..10u32) {
        assert_eq!(2*x, x+x);
        // assert_eq!(x+y, y+x);
    }
}
