use super::*;

primitive_aliases! { pub B => bool }

impl Scalar for bool {
    type Vec2Alignment = Align<1>;
    type Vec3Alignment = Align<1>;
    type Vec4Alignment = Align<1>;
}

impl<const N: usize, A: VecAlignment> Vector<N, bool, A>
where
    Usize<N>: VecLen,
{
    /// Vector of all `false` values.
    pub const FALSE: Self = Self::splat(false);
    /// Vector of all `true` values.
    pub const TRUE: Self = Self::splat(true);

    macro_loop! {
        @for prim in [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize] {
            /// Convert the vector to a vector of the given primitive type.
            /// This uses the `as` keyword to perform the conversion.
            pub const fn @[as_ @prim](self) -> Vector<N, @prim, A> {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    output.as_array_mut()[i] = self.to_array()[i] as @prim;
                    i += 1;
                }

                output
            }
        }
    }
}
