use ggmath::{rectangle::*, scalar::*, vector::*};
use ggmath_testing::*;

pub fn test_rectangle() {
    test_n_t_a_r::<2, i32, VecAligned, RectCornered>();
    test_n_t_a_r::<3, i64, VecPacked, RectCentered>();
    test_n_t_a_r::<4, i32, VecAligned, RectMinMaxed>();
}

fn test_n_t_a_r<const N: usize, T: ScalarNum + TestableScalar, A: VecAlignment, R: RectRepr>()
where
    ScalarCount<N>: VecLen,
{
    let center = Vector::<N, T, A>::from_fn(|i| T::normal_value(i));
    let extents = Vector::<N, T, A>::from_fn(|i| T::normal_value(i + N));

    let min = center - extents;
    let max = center + extents;
    let size = extents + extents;

    let rect = Rectangle::<N, T, A, R>::from_center_extents(center, extents);

    assert_eq!(rect.min(), min);
    assert_eq!(rect.max(), max);
    assert_eq!(rect.center(), center);
    assert_eq!(rect.size(), size);
    assert_eq!(rect.extents(), extents);

    assert_eq!(Rectangle::<N, T, A, R>::from_min_size(min, size), rect);
    assert_eq!(Rectangle::<N, T, A, R>::from_max_size(max, size), rect);
    assert_eq!(
        Rectangle::<N, T, A, R>::from_center_size(center, size),
        rect
    );
    assert_eq!(
        Rectangle::<N, T, A, R>::from_min_extents(min, extents),
        rect
    );
    assert_eq!(
        Rectangle::<N, T, A, R>::from_max_extents(max, extents),
        rect
    );
    assert_eq!(
        Rectangle::<N, T, A, R>::from_center_extents(center, extents),
        rect
    );
    assert_eq!(Rectangle::<N, T, A, R>::from_min_max(min, max), rect);
    assert_eq!(Rectangle::<N, T, A, R>::from_min_center(min, center), rect);
    assert_eq!(Rectangle::<N, T, A, R>::from_center_max(center, max), rect);
}
