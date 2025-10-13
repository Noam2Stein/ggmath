pub mod f32;
pub mod f64;

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! specialize {
    (<$T:ty as $Scalar:ident<N, S>>::$method:ident($($arg:expr),* $(,)?)) => {
        (const {
            match (N, S::IS_SIMD) {
                (_, false) => unsafe {
                    core::mem::transmute::<
                        $crate::specialize!(@fnptr_type $($arg),*),
                        $crate::specialize!(@fnptr_type $($arg),*),
                    >(<$T as $Scalar<N, $crate::NonSimd>>::$method)
                }
                (2, true) => unsafe {
                    core::mem::transmute::<
                        $crate::specialize!(@fnptr_type $($arg),*),
                        $crate::specialize!(@fnptr_type $($arg),*),
                    >(<$T as $Scalar<2, $crate::Simd>>::$method)
                }
                (3, true) => unsafe {
                    core::mem::transmute::<
                        $crate::specialize!(@fnptr_type $($arg),*),
                        $crate::specialize!(@fnptr_type $($arg),*),
                    >(<$T as $Scalar<3, $crate::Simd>>::$method)
                }
                (4, true) => unsafe {
                    core::mem::transmute::<
                        $crate::specialize!(@fnptr_type $($arg),*),
                        $crate::specialize!(@fnptr_type $($arg),*),
                    >(<$T as $Scalar<4, $crate::Simd>>::$method)
                }
                (_, true) => core::panic!("unexpected vector type"),
            }
        })
        ($($arg),*)
    };

    (@fnptr_type) => {
        fn() -> _
    };
    (@fnptr_type $_:expr) => {
        fn(_) -> _
    };
    (@fnptr_type $_:expr, $__:expr) => {
        fn(_, _) -> _
    };
    (@fnptr_type $_:expr, $__:expr, $___:expr) => {
        fn(_, _, _) -> _
    };
}
