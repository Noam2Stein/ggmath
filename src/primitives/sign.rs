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
    ] {
        @let prim_is_sint = prim_is_int && prim_is_signed;
        @let prim_is_uint = prim_is_int && !prim_is_signed;

        #[cfg(feature = "vector")]
        impl<const N: usize, A: VecAlignment> Vector<N, @prim, A>
        where
            Usize<N>: VecLen,
        {
            /// Returns a vector of bools, where each element is `true` is the input element is positive.
            ///
            /// Basically `[self.x > 0, self.y > 0, ...]`.
            #[inline(always)]
            pub const fn positive_mask(self) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) > 0 as @prim;
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element is zero.
            ///
            /// Basically `[self.x == 0, self.y == 0, ...]`.
            #[inline(always)]
            pub const fn zero_mask(self) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) == 0 as @prim;
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element is negative.
            ///
            /// Basically `[self.x < 0, self.y < 0, ...]`.
            #[inline(always)]
            #[cfg(@prim_is_signed)]
            pub const fn negative_mask(self) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) < 0 as @prim;
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element is non-negative.
            ///
            /// Basically `[self.x >= 0, self.y >= 0, ...]`.
            #[inline(always)]
            #[cfg(@prim_is_sint)]
            pub const fn bin_positive_mask(self) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) >= 0 as @prim;
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element's binary sign is positive.
            ///
            /// This returns true if the element is `+0.0`, and false if the element is `-0.0`.
            ///
            /// Basically `[self.x.is_sign_positive(), self.y.is_sign_positive(), ...]`.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub const fn bin_positive_mask(self) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i).is_sign_positive();
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element's binary sign is negative.
            ///
            /// This returns true if the element is `-0.0`, and false if the element is `+0.0`.
            ///
            /// Basically `[self.x.is_sign_negative(), self.y.is_sign_negative(), ...]`.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub const fn bin_negative_mask(self) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i).is_sign_negative();
                    i += 1;
                }

                output
            }

            /// Returns a vector of the absolute values of the elements.
            #[inline(always)]
            #[cfg(@prim_is_signed)]
            pub const fn abs(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i).abs();
                    i += 1;
                }

                output
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `0` if the number is `0`,
            /// - `1` if the number is positive.
            #[inline(always)]
            #[cfg(@prim_is_uint)]
            pub const fn signum(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = if self.index(i) == 0 { 0 } else { 1 };
                    i += 1;
                }

                output
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `0` if the number is `0`,
            /// - `1` if the number is positive,
            /// - `-1` if the number is negative.
            #[inline(always)]
            #[cfg(@prim_is_sint)]
            pub const fn signum(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i).signum();
                    i += 1;
                }

                output
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `+0.0` if the number is `+0.0`,
            /// - `-0.0` if the number is `-0.0`,
            /// - `1.0` if the number is positive,
            /// - `-1.0` if the number is negative.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub const fn tri_signum(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = if self.index(i).is_sign_positive() {
                        if self.index(i) == 0.0 {
                            0.0
                        } else {
                            1.0
                        }
                    } else {
                        if self.index(i) == 0.0 {
                            -0.0
                        } else {
                            -1.0
                        }
                    };
                    i += 1;
                }

                output
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `1` if the number is positive or zero,
            /// - `-1` if the number is negative.
            #[inline(always)]
            #[cfg(@prim_is_sint)]
            pub const fn bin_signum(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = if self.index(i) >= 0 { 1 } else { -1 };
                    i += 1;
                }

                output
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `1.0` if the number is positive or `+0.0`,
            /// - `-1.0` if the number is negative or `-0.0`.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub const fn bin_signum(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i).signum();
                    i += 1;
                }

                output
            }
        }
    }
}
