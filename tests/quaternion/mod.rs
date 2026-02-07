mod float;
mod trait_impl;

mod f32 {
    use ggmath::{Aligned, Unaligned};

    #[test]
    fn aligned() {
        super::float::test_f32::<Aligned>();
    }

    #[test]
    fn unaligned() {
        super::float::test_f32::<Unaligned>();
    }
}

mod f64 {
    use ggmath::{Aligned, Unaligned};

    #[test]
    fn aligned() {
        super::float::test_f64::<Aligned>();
    }

    #[test]
    fn unaligned() {
        super::float::test_f64::<Unaligned>();
    }
}
