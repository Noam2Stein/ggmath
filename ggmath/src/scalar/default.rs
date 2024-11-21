use super::*;

ggmath_proc_macros::vector_interface!(
    ScalarDefault: Scalar + Default;

    impl Default:

    fn default() -> Self {
        Vector::from_array([<T as Default>::default(); N])
    }
);
