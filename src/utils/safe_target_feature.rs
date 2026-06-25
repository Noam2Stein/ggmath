/// Bypasses a language limitation to safely call functions marked with
/// `#[target_feature(...)]` when that feature is enabled at compilation time.
///
/// The macro has a set of supported target features, and can be modified to
/// support more.
///
/// The input syntax is a sequence of function declarations.
macro_rules! safe_target_feature {
    ($(
        $(#[$meta:meta])*
        fn $f:ident($($param:ident: $Param:ty),* $(,)?) $(-> $Ret:ty)? $body:block
    )*) => {$(
        $(#[$meta])*
        fn $f($($param: $Param),*) $(-> $Ret)? {
            // Use this pattern to add more features. Make sure to keep the
            // cfg correct, else there will be unsoundness.
            #[cfg_attr(target_feature = "sse2", target_feature(enable = "sse2"))]
            #[cfg_attr(target_feature = "ssse3", target_feature(enable = "ssse3"))]
            #[inline]
            fn $f($($param: $Param),*) $(-> $Ret)? $body

            // SAFETY: The function only requires target features that are
            // enabled via cfg.
            unsafe { $f($($param),*) }
        }
    )*};
}
pub(crate) use safe_target_feature;
