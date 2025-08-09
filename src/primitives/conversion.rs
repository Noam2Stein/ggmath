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
        @let prim_is_sint = prim_is_int && prim_is_signed;
        @let prim_is_uint = prim_is_int && !prim_is_signed;

        #[cfg(feature = "vector")]
        impl<const N: usize, A: VecAlignment> Vector<N, @prim, A>
        where
            Usize<N>: VecLen,
        {
            @for [other_prim, other_prim_is_int, other_prim_is_float, other_prim_is_signed] in [
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
                @let other_prim_is_sint = other_prim_is_int && other_prim_is_signed;
                @let other_prim_is_uint = other_prim_is_int && !other_prim_is_signed;

                @if other_prim != prim
                    && other_prim != 'bool
                    && !(prim == 'bool && other_prim_is_float)
                {
                    /// Convert the vector to a vector of the given primitive type.
                    /// This uses the `as` keyword to perform the conversion.
                    #[inline(always)]
                    pub const fn @['as_ other_prim](self) -> Vector<N, @other_prim, A> {
                        let mut output = Vector::splat(0 as @other_prim);

                        let mut i = 0;
                        while i < N {
                            output.as_array_mut()[i] = self.to_array()[i] as @other_prim;
                            i += 1;
                        }

                        output
                    }
                }
            }
        }
    }
}
