use super::*;

repetitive! {
    @for prim in ['f32, 'f64] {
        #[cfg(feature = "vector")]
        impl<const N: usize, A: VecAlignment> Vector<N, @prim, A>
        where
            Usize<N>: VecLen,
        {
            /// Vector of the rounded values of the input elements.
            #[inline(always)]
            pub fn round(self) -> Self {
                self.map(@prim::round)
            }

            /// Vector of the floor values of the input elements.
            #[inline(always)]
            pub fn floor(self) -> Self {
                self.map(@prim::floor)
            }

            /// Vector of the ceiling values of the input elements.
            #[inline(always)]
            pub fn ceil(self) -> Self {
                self.map(@prim::ceil)
            }

            /// Vector of the truncated values of the input elements.
            #[inline(always)]
            pub fn trunc(self) -> Self {
                self.map(@prim::trunc)
            }

            /// Vector of the "anti truncated" values of the input elements.
            /// "anti truncated" means rounding away from zero.
            #[inline(always)]
            pub fn atrunc(self) -> Self {
                self.map(|x| {
                    if x.is_sign_positive() {
                        x.ceil()
                    } else {
                        x.floor()
                    }
                })
            }

            /// Vector of the fractional part of the input elements.
            #[inline(always)]
            pub fn fract(self) -> Self {
                self.map(@prim::fract)
            }
        }
    }
}
