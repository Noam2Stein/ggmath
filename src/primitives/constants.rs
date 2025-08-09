use super::*;

repetitive! {
    @for [prim, prim_is_int, prim_is_float, prim_is_signed] in [
        ['u8, true, false, false],
        ['u16, true, false, false],
        ['u32, true, false, false],
        ['u64, true, false, false],
        ['u128, true, false, false],
        ['usize, true, false, false],
        ['i8, true, false, true],
        ['i16, true, false, true],
        ['i32, true, false, true],
        ['i64, true, false, true],
        ['i128, true, false, true],
        ['isize, true, false, true],
        ['f32, false, true, true],
        ['f64, false, true, true],
        ['bool, false, false, false],
    ] {
        @let prim_is_num = prim_is_int || prim_is_float;
        @let prim_is_sint = prim_is_int && prim_is_signed;
        @let prim_is_uint = prim_is_int && !prim_is_signed;

        #[cfg(feature = "vector")]
        impl<const N: usize, A: VecAlignment> Vector<N, @prim, A>
        where
            Usize<N>: VecLen,
        {
            @if prim == 'bool {
                /// Vector of all `false` values.
                pub const FALSE: Self = Self::splat(false);
                /// Vector of all `true` values.
                pub const TRUE: Self = Self::splat(true);
            }

            @if prim_is_num {
                #[doc = @str["vector of all `" prim "::MIN` values"]]
                pub const MIN: Self = Self::splat(@prim::MIN);
                #[doc = @str["vector of all `" prim "::MAX` values"]]
                pub const MAX: Self = Self::splat(@prim::MAX);

                #[doc = @str["vector of all `0` values"]]
                pub const ZERO: Self = Self::splat(0 as @prim);
                #[doc = @str["vector of all `1` values"]]
                pub const ONE: Self = Self::splat(1 as @prim);
            }

            @if prim_is_signed {
                #[doc = @str["vector of all `-1` values"]]
                pub const NEG_ONE: Self = Self::splat(-1 as @prim);
            }

            @if prim_is_float {
                #[doc = @str["vector of all `" prim "::NAN` values"]]
                pub const NAN: Self = Self::splat(@prim::NAN);
                #[doc = @str["vector of all `" prim "::INFINITY` values"]]
                pub const INFINITY: Self = Self::splat(@prim::INFINITY);
                #[doc = @str["vector of all `" prim "::NEG_INFINITY` values"]]
                pub const NEG_INFINITY: Self = Self::splat(@prim::NEG_INFINITY);
            }
        }
    }
}
