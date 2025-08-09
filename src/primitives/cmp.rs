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
            /// Returns a vector of bools, where each element is `true` is the input element is equal to the corresponding element of the other vector.
            ///
            /// Basically `[self.x == other.x, self.y == other.y, ...]`.
            #[inline(always)]
            pub const fn eq_mask(self, other: Vector<N, @prim, impl VecAlignment>) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) == other.index(i);
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element is not equal to the corresponding element of the other vector.
            ///
            /// Basically `[self.x != other.x, self.y != other.y, ...]`.
            #[inline(always)]
            pub const fn ne_mask(self, other: Vector<N, @prim, impl VecAlignment>) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) != other.index(i);
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element is less than the corresponding element of the other vector.
            ///
            /// Basically `[self.x < other.x, self.y < other.y, ...]`.
            #[inline(always)]
            pub const fn lt_mask(self, other: Vector<N, @prim, impl VecAlignment>) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) < other.index(i);
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element is greater than the corresponding element of the other vector.
            ///
            /// Basically `[self.x > other.x, self.y > other.y, ...]`.
            #[inline(always)]
            pub const fn gt_mask(self, other: Vector<N, @prim, impl VecAlignment>) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) > other.index(i);
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element is less than or equal to the corresponding element of the other vector.
            ///
            /// Basically `[self.x <= other.x, self.y <= other.y, ...]`.
            #[inline(always)]
            pub const fn le_mask(self, other: Vector<N, @prim, impl VecAlignment>) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) <= other.index(i);
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element is greater than or equal to the corresponding element of the other vector.
            ///
            /// Basically `[self.x >= other.x, self.y >= other.y, ...]`.
            #[inline(always)]
            pub const fn ge_mask(self, other: Vector<N, @prim, impl VecAlignment>) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) >= other.index(i);
                    i += 1;
                }

                output
            }

            /// Returns a vector of the minimum values between the two vector's elements.
            ///
            /// Basically `[self.x.min(other.x), self.y.min(other.y), ...]`.
            #[inline(always)]
            pub const fn min(self, other: Vector<N, @prim, impl VecAlignment>) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = if self.index(i) < other.index(i) {
                        self.index(i)
                    } else {
                        other.index(i)
                    };
                    i += 1;
                }

                output
            }

            /// Returns a vector of the maximum values between the two vector's elements.
            ///
            /// Basically `[self.x.max(other.x), self.y.max(other.y), ...]`.
            #[inline(always)]
            pub const fn max(self, other: Vector<N, @prim, impl VecAlignment>) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = if self.index(i) > other.index(i) {
                        self.index(i)
                    } else {
                        other.index(i)
                    };
                    i += 1;
                }

                output
            }

            /// Returns a vector of the input elements clamped between the minimum and maximum vectors.
            ///
            /// Basically `[self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y), ...]`.
            #[inline(always)]
            pub const fn clamp(
                self,
                min: Vector<N, @prim, impl VecAlignment>,
                max: Vector<N, @prim, impl VecAlignment>,
            ) -> Self {
                self.min(max).max(min)
            }

            /// Returns a vector of the absolute difference between the elements and the corresponding elements of the other vector.
            /// This is equivalent to `abs(self - rhs)`, not `abs(self) - abs(rhs)`.
            #[inline(always)]
            #[cfg(@prim_is_uint)]
            pub const fn abs_diff(self, rhs: Vector<N, @prim, impl VecAlignment>) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = if self.index(i) > rhs.index(i) {
                        self.index(i) - rhs.index(i)
                    } else {
                        rhs.index(i) - self.index(i)
                    };
                    i += 1;
                }

                output
            }

            /// Returns a vector of the absolute difference between the elements and the corresponding elements of the other vector.
            /// This is equivalent to `abs(self - rhs)`, not `abs(self) - abs(rhs)`.
            #[inline(always)]
            #[cfg(@prim_is_signed)]
            pub const fn abs_diff(self, rhs: Vector<N, @prim, impl VecAlignment>) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = (self.index(i) - rhs.index(i)).abs();
                    i += 1;
                }

                output
            }
        }
    }
}
