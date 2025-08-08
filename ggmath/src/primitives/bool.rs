use std::mem::transmute_copy;

use super::*;

primitive_aliases! { pub B => bool }

#[cfg(feature = "vector")]
impl Scalar for bool {
    // A vector of bools can act like a uint when performing bitwise operations.
    // This means that the alignment of a vector of bools should be the alignment of the corresponding uint type.
    type Vec2Alignment = Align<{ align_of::<u16>() }>;
    type Vec3Alignment = Align<{ align_of::<u32>() }>;
    type Vec4Alignment = Align<{ align_of::<u32>() }>;

    #[inline(always)]
    fn vec3_bitand(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&lhs) };
        let rhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&rhs) };

        let output_int = lhs_int & rhs_int;

        Some(unsafe { transmute_copy::<u32, Vec3<Self>>(&output_int) })
    }

    #[inline(always)]
    fn vec3_bitor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&lhs) };
        let rhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&rhs) };

        let output_int = lhs_int | rhs_int;

        Some(unsafe { transmute_copy::<u32, Vec3<Self>>(&output_int) })
    }

    #[inline(always)]
    fn vec3_bitxor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&lhs) };
        let rhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&rhs) };

        let output_int = lhs_int ^ rhs_int;

        Some(unsafe { transmute_copy::<u32, Vec3<Self>>(&output_int) })
    }
}

repetitive! {
    #[cfg(feature = "vector")]
    impl<const N: usize, A: VecAlignment> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
    {
        /// Vector of all `false` values.
        pub const FALSE: Self = Self::splat(false);
        /// Vector of all `true` values.
        pub const TRUE: Self = Self::splat(true);

        @for prim in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize, 'i8, 'i16, 'i32, 'i64, 'i128, 'isize] {
            /// Convert the vector to a vector of the given primitive type.
            /// This uses the `as` keyword to perform the conversion.
            #[inline(always)]
            pub const fn @['as_ prim](self) -> Vector<N, @prim, A> {
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

const _: () = assert!(size_of::<bool>() == 1);
const _: () = assert!(align_of::<bool>() == 1);
