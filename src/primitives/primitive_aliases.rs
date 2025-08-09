use super::*;

repetitive! {
    @for [prim, prefix] in [
        ['u8, 'U8],
        ['u16, 'U16],
        ['u32, 'U],
        ['u64, 'U64],
        ['u128, 'U128],
        ['usize, 'Usize],
        ['i8, 'I8],
        ['i16, 'I16],
        ['i32, 'I],
        ['i64, 'I64],
        ['i128, 'I128],
        ['isize, 'Isize],
        ['f32, 'F],
        ['f64, 'D],
        ['bool, 'B],
    ] {
        #[doc = @str["Module with `" prim "` type aliases"]]
        #[cfg(feature = "primitive_aliases")]
        pub mod @[prim '_aliases] {
            use super::*;

            #[cfg(feature = "vector")]
            vector_aliases! { pub @prefix => @prim }

            #[cfg(feature = "matrix")]
            matrix_aliases! { pub @prefix => @prim }

            #[cfg(feature = "aabb")]
            aabb_aliases! { pub @prefix => @prim }
        }

        #[cfg(feature = "primitive_aliases")]
        pub use @[prim '_aliases]::*;
    }
}
