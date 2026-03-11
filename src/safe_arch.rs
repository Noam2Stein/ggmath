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
