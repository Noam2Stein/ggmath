use crate::{Aligned, Alignment, Length, Scalar, Unaligned, Vector, length::TwoOrThree};

/// TODO
#[expect(private_bounds)]
pub struct Rotor<const N: usize, T, A: Alignment>(
    pub(crate) <Length<N> as TwoOrThree>::Select<Vector<2, T, A>, Vector<4, T, A>>,
)
where
    Length<N>: TwoOrThree,
    T: Scalar;

/// TODO
pub type Rot2<T> = Rotor<2, T, Unaligned>;

/// TODO
pub type Rot3<T> = Rotor<3, T, Unaligned>;

/// TODO
pub type Rot2A<T> = Rotor<2, T, Aligned>;

/// TODO
pub type Rot3A<T> = Rotor<3, T, Aligned>;

#[cfg(test)]
mod tests {
    extern crate std;

    use crate::{Rot2, Rot2A, Rot3, Rot3A, Vec2A, Vec4A, test_utils::for_types};

    #[test]
    fn test_layout() {
        for_types!(|T: PrimitiveNumber| {
            assert_eq!(size_of::<Rot2<T>>(), size_of::<T>() * 2);
            assert_eq!(align_of::<Rot2<T>>(), align_of::<T>());

            assert_eq!(size_of::<Rot3<T>>(), size_of::<T>() * 4);
            assert_eq!(align_of::<Rot3<T>>(), align_of::<T>());

            assert_eq!(size_of::<Rot2A<T>>(), size_of::<Vec2A<T>>());
            assert_eq!(align_of::<Rot2A<T>>(), align_of::<Vec2A<T>>());

            assert_eq!(size_of::<Rot3A<T>>(), size_of::<Vec4A<T>>());
            assert_eq!(align_of::<Rot3A<T>>(), align_of::<Vec4A<T>>());
        });
    }
}
