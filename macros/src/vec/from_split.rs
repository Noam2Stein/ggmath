#[macro_export]
macro_rules! impl_from_split_transmute {
    ($ty:ty) => {
        impl ggmath::FromVec2Splits for [$t; 2] {}
        impl ggmath::FromVec3Splits for [$t; 4] {}
        impl ggmath::FromVec4Splits for [$t; 4] {}
    };
}
