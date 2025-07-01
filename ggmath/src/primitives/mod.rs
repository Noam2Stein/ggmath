use super::*;

macro_rules! primitive_mod {
    ($ident:ident) => {
        pub mod $ident;

        #[allow(unused_imports)]
        pub use $ident::*;
    };
}

primitive_mod! { u8 }
primitive_mod! { u16 }
primitive_mod! { u32 }
primitive_mod! { u64 }
primitive_mod! { u128 }
primitive_mod! { usize }
primitive_mod! { i8 }
primitive_mod! { i16 }
primitive_mod! { i32 }
primitive_mod! { i64 }
primitive_mod! { i128 }
primitive_mod! { isize }
primitive_mod! { f32 }
primitive_mod! { f64 }
primitive_mod! { bool }

primitive_mod! { ordering }

#[macro_export(local_inner_macros)]
macro_rules! primitive_aliases {
    { $vis:vis $prefix:ident => $type:ty } => {
        #[cfg(feature = "primitive_aliases")]
        vector_aliases! { $vis $prefix => $type }

        #[cfg(feature = "primitive_aliases")]
        #[cfg(feature = "matrix")]
        matrix_aliases! { $vis $prefix => $type }

        #[cfg(feature = "primitive_aliases")]
        #[cfg(feature = "rectangle")]
        rectangle_aliases! { $vis $prefix => $type }
    };
}
