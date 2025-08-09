use super::*;

repetitive! {
    @for [prim, prim_is_float] in [
        ['u8, false],
        ['u16, false],
        ['u32, false],
        ['u64, false],
        ['u128, false],
        ['usize, false],
        ['i8, false],
        ['i16, false],
        ['i32, false],
        ['i64, false],
        ['i128, false],
        ['isize, false],
        ['f32, true],
        ['f64, true],
    ] {
        #[cfg(feature = "vector")]
        impl<const N: usize, A: VecAlignment> Vector<N, @prim, A>
        where
            Usize<N>: VecLen,
        {
            /// Returns the magnitude of the vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub fn mag(self) -> @prim {
                self.mag_sq().sqrt()
            }

            /// Returns the square of the vector's magnitude.
            ///
            /// This is faster than getting the magnitude,
            /// because it doesn't require performing a square root.
            #[inline(always)]
            pub const fn mag_sq(self) -> @prim {
                let mut output = 0 as @prim;

                let mut i = 0;
                while i < N {
                    output += self.index(i) * self.index(i);
                    i += 1;
                }

                output
            }

            /// Returns a vector with the same direction as the original, but with a magnitude of `1.0`.
            ///
            /// Calling this on a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub fn normalized(self) -> Self {
                self / self.mag()
            }

            /// Returns a vector with the same direction as the original, but with the given magnitude.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub fn with_mag(self, mag: @prim) -> Self {
                self / self.mag() * mag
            }

            /// Returns a vector with the same direction as the original, but with the magnitude at least the given value.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub fn with_min_mag(self, min_mag: @prim) -> Self {
                if self.mag_sq() < min_mag * min_mag {
                    self.with_mag(min_mag)
                } else {
                    self
                }
            }

            /// Returns a vector with the same direction as the original, but with the magnitude at most the given value.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub fn with_max_mag(self, max_mag: @prim) -> Self {
                if self.mag_sq() > max_mag * max_mag {
                    self.with_mag(max_mag)
                } else {
                    self
                }
            }

            /// Returns a vector with the same direction as the original, but with the magnitude clamped between the given values.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub fn clamp_mag(self, min_mag: @prim, max_mag: @prim) -> Self {
                if self.mag_sq() < min_mag * min_mag {
                    self.with_mag(min_mag)
                } else if self.mag_sq() > max_mag * max_mag {
                    self.with_mag(max_mag)
                } else {
                    self
                }
            }

            /// Returns a vector with the same direction as the original, but with the given square magnitude.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub fn with_mag_sq(self, mag_sq: @prim) -> Self {
                self.with_mag(mag_sq.sqrt())
            }

            /// Returns a vector with the same direction as the original, but with the square magnitude at least the given value.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub fn with_min_mag_sq(self, min_mag_sq: @prim) -> Self {
                if self.mag_sq() < min_mag_sq {
                    self.with_mag_sq(min_mag_sq)
                } else {
                    self
                }
            }

            /// Returns a vector with the same direction as the original, but with the square magnitude at most the given value.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub fn with_max_mag_sq(self, max_mag_sq: @prim) -> Self {
                if self.mag_sq() > max_mag_sq {
                    self.with_mag_sq(max_mag_sq)
                } else {
                    self
                }
            }

            /// Returns a vector with the same direction as the original, but with the square magnitude clamped between the given values.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub fn clamp_mag_sq(self, min_mag_sq: @prim, max_mag_sq: @prim) -> Self {
                self.with_min_mag_sq(min_mag_sq).with_max_mag_sq(max_mag_sq)
            }

            // Deprecated

            /// Returns the square of the vector's magnitude.
            ///
            /// This is faster than getting the magnitude,
            /// because it doesn't require performing a square root.
            #[inline(always)]
            #[deprecated(note = "Renamed to `mag_sq`")]
            pub fn square_mag(self) -> @prim {
                self.mag_sq()
            }

            /// Returns a vector with the same direction as the original, but with a magnitude of `1.0`.
            ///
            /// Calling this on a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            #[deprecated(note = "Renamed to `normalized`")]
            pub fn normalize(self) -> Self {
                self.normalize()
            }

            /// Returns a vector with the same direction as the original, but with the given square magnitude.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            #[deprecated(note = "Renamed to `with_mag_sq`")]
            pub fn with_square_mag(self, square_mag: @prim) -> Self {
                self.with_mag_sq(square_mag)
            }

            /// Returns a vector with the same direction as the original, but with the square magnitude at least the given value.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            #[deprecated(note = "Renamed to `with_min_mag_sq`")]
            pub fn with_min_square_mag(self, min_square_mag: @prim) -> Self {
                self.with_min_mag_sq(min_square_mag)
            }

            /// Returns a vector with the same direction as the original, but with the square magnitude at most the given value.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            #[deprecated(note = "Renamed to `with_max_mag_sq`")]
            pub fn with_max_square_mag(self, max_square_mag: @prim) -> Self {
                self.with_max_mag_sq(max_square_mag)
            }

            /// Returns a vector with the same direction as the original, but with the square magnitude clamped between the given values.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            #[deprecated(note = "Renamed to `clamp_mag_sq`")]
            pub fn clamp_square_mag(self, min_square_mag: @prim, max_square_mag: @prim) -> Self {
                self.clamp_mag_sq(min_square_mag, max_square_mag)
            }
        }
    }
}
