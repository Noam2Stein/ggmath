/// Bypasses a language limitation to safely call intrinsics.
///
/// Calling functions annotated with `#[target_feature]` is unsafe even if the
/// features are enabled for the current target architecture.
///
/// `safe_arch` hides the `#[target_feature]` annotation and fails compilation
/// if the features are not enabled for the current target architecture.
///
/// # Examples
///
/// `x86_64` `sse` example:
///
/// ```ignore
/// safe_arch! {
///     #![target_feature(enable = "sse")]
///
///     fn useless_code() {
///         println!("{:?}", core::arch::x86_64::_mm_set_ps(1.0, 2.0, 3.0, 4.0));
///     }
/// }
///
/// useless_code();
/// ```
#[allow(unused_macros)]
macro_rules! safe_arch {
    (
        #![target_feature(enable = $feature:literal)]

        $(
            $(#[$meta:meta])* fn $f:ident($($param:ident: $Param:ty),* $(,)?) $(-> $Ret:ty)? $body:block
        )*
    ) => {
        $(
            $(#[$meta])*
            fn $f($($param: $Param),*) $(-> $Ret)? {
                #[inline]
                #[target_feature(enable = $feature)]
                fn $f($($param: $Param),*) $(-> $Ret)? $body

                #[cfg(not(target_feature = $feature))]
                compile_error!(concat!("target feature is not enabled"));

                // SAFETY: If `$feature` is disabled, there will be a compile
                // error.
                unsafe { $f($($param),*) }
            }
        )*
    };
}

#[allow(unused_imports)]
pub(crate) use safe_arch;
