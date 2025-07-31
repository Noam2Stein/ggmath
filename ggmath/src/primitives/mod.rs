use super::*;

macro_loop! {
    @for prim_mod in [
        u8,
        u16,
        u32,
        u64,
        u128,
        usize,
        i8,
        i16,
        i32,
        i64,
        i128,
        isize,
        f32,
        f64,
        bool,
        ordering,
        maybe_uninit,
    ] {
        #[doc = @("Module with " + @prim_mod + " type aliases")]
        pub mod @prim_mod;

        #[allow(unused_imports)]
        pub use @prim_mod::*;
    }
}

/// Internal macro to define primitive aliases for vector and matrix types.
#[macro_export(local_inner_macros)]
macro_rules! primitive_aliases {
    { $vis:vis $prefix:ident => $type:ty } => {
        #[cfg(feature = "primitive_aliases")]
        vector_aliases! { $vis $prefix => $type }

        #[cfg(feature = "primitive_aliases")]
        #[cfg(feature = "matrix")]
        matrix_aliases! { $vis $prefix => $type }

        #[cfg(feature = "primitive_aliases")]
        #[cfg(feature = "aabb")]
        aabb_aliases! { $vis $prefix => $type }
    };
}
