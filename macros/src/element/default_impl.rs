#[macro_export]
macro_rules! impl_element_default {
    ($ty:ty) => {
        impl ggmath::default_impl::ElementDefaultImpl for $ty {}
    };
}
