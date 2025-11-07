////////////////////////////////////////////////////////////////////////////////
// Constructor Macros
////////////////////////////////////////////////////////////////////////////////

declare_constructor_macro! { mat2 => Matrix<2, _, Simd> }
declare_constructor_macro! { mat3 => Matrix<3, _, Simd> }
declare_constructor_macro! { mat4 => Matrix<4, _, Simd> }
declare_constructor_macro! { mat2s => Matrix<2, _, NonSimd> }
declare_constructor_macro! { mat3s => Matrix<3, _, NonSimd> }
declare_constructor_macro! { mat4s => Matrix<4, _, NonSimd> }
declare_constructor_macro! { mat2g => Matrix<2, _, _> }
declare_constructor_macro! { mat3g => Matrix<3, _, _> }
declare_constructor_macro! { mat4g => Matrix<4, _, _> }

macro_rules! declare_constructor_macro {
    { $(#[$meta:meta])* $name:ident => Matrix<$N:expr, _, $S:ident> } => {
        declare_constructor_macro! { @core $i $(#[$meta])* $name => Matrix<$N, _, $crate::$S> }
    };

    { $(#[$meta:meta])* $name:ident => Matrix<$N:expr, _, _> } => {
        declare_constructor_macro! { @core $i $(#[$meta])* $name => Matrix<$N, _, _> }
    };

    { @core $dollar:tt i $(#[$meta:meta])* $name:ident => Matrix<$N:expr, _, $S:ty> } => {
        $(#[$meta])*
        #[macro_export]
        macro_rules! $name {
            ($dollar($dollar arg:expr),* $dollar (,)?) => {
                $crate::Matrix::<$N, _, $S>::from_vectors([$dollar ($dollar arg,)*])
            }
        }

        pub use $name;
    };
}

use declare_constructor_macro;
