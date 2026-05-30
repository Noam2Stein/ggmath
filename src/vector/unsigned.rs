use crate::{
    Alignment, Length, PrimitiveUnsigned, SupportedLength, Vector, utils::transmute_generic,
};

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: PrimitiveUnsigned,
{
    /// Returns the bit patterns of `self` reinterpreted as signed integers of
    /// the same size.
    ///
    /// This produces the same result as [`as`] conversions, but ensures that
    /// the bit-width remains the same.
    ///
    /// [`as`]: https://rust-for-c-programmers.com/ch16/16_2_primitive_casting_with_as.html
    #[inline]
    #[must_use]
    #[expect(private_interfaces)]
    pub const fn cast_signed(self) -> Vector<N, T::I, A> {
        if const { size_of::<Vector<N, T, A>>() == size_of::<Vector<N, T::I, A>>() } {
            // SAFETY: Both types accept all bit-patterns.
            unsafe { transmute_generic::<Vector<N, T, A>, Vector<N, T::I, A>>(self) }
        } else {
            // SAFETY: Both types accept all bit-patterns.
            Vector::from_array(unsafe { transmute_generic::<[T; N], [T::I; N]>(self.to_array()) })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vec3A, utils::for_parameters};

    #[test]
    fn test_cast_signed() {
        for_parameters!(|T: PrimitiveUnsigned| {
            let vector = Vec3A::<T>::new(1, T::MAX / 2, T::MAX);
            assert_eq!(vector.cast_signed(), vector.map(T::cast_signed));
        });
    }
}
