ggmath_proc_macros::vec_interface!(
    impl Default:

    ScalarDefault: Scalar + Default,

    fn default() -> Self {
        Vector::from_array([<T as Default>::default(); N])
    }
);
