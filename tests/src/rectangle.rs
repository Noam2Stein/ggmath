use ggmath::{
    rectangle::{RectCentered, RectCornered, RectMinMaxed, RectRepr, Rectangle, ScalarRect},
    testing::{rect_test_assert, TestResult, TestableScalar},
    vector::{ScalarCount, VecAligned, VecAlignment, VecLen, VecPacked, Vector},
};

pub fn test_rectangle() -> TestResult {
    test_n_t_a_r::<2, i32, VecAligned, RectCornered>()?;
    test_n_t_a_r::<2, i32, VecAligned, RectCentered>()?;
    test_n_t_a_r::<2, i32, VecAligned, RectMinMaxed>()?;
    test_n_t_a_r::<2, i32, VecPacked, RectCornered>()?;
    test_n_t_a_r::<2, i32, VecPacked, RectCentered>()?;
    test_n_t_a_r::<2, i32, VecPacked, RectMinMaxed>()?;

    test_n_t_a_r::<3, i32, VecAligned, RectCornered>()?;
    test_n_t_a_r::<3, i32, VecAligned, RectCentered>()?;
    test_n_t_a_r::<3, i32, VecAligned, RectMinMaxed>()?;
    test_n_t_a_r::<3, i32, VecPacked, RectCornered>()?;
    test_n_t_a_r::<3, i32, VecPacked, RectCentered>()?;
    test_n_t_a_r::<3, i32, VecPacked, RectMinMaxed>()?;

    test_n_t_a_r::<4, i32, VecAligned, RectCornered>()?;
    test_n_t_a_r::<4, i32, VecAligned, RectCentered>()?;
    test_n_t_a_r::<4, i32, VecAligned, RectMinMaxed>()?;
    test_n_t_a_r::<4, i32, VecPacked, RectCornered>()?;
    test_n_t_a_r::<4, i32, VecPacked, RectCentered>()?;
    test_n_t_a_r::<4, i32, VecPacked, RectMinMaxed>()?;

    Ok(())
}

fn test_n_t_a_r<const N: usize, T: ScalarRect + TestableScalar, A: VecAlignment, R: RectRepr>(
) -> TestResult
where
    ScalarCount<N>: VecLen,
{
    for values in T::get_4_c_r::<N, 2>() {
        let center = Vector::from_array(values[0]);
        let extents = Vector::from_array(values[1]);
        let min = center - extents;
        let max = center + extents;
        let size = extents + extents;

        let rect = Rectangle::<N, T, A, R>::from_center_extents(center, extents);

        rect_test_assert!(min: rect.min(), min; rect);
        rect_test_assert!(max: rect.max(), max; rect);
        rect_test_assert!(center: rect.center(), center; rect);
        rect_test_assert!(size: rect.size(), size; rect);
        rect_test_assert!(extents: rect.extents(), extents; rect);

        rect_test_assert!(from_min_size: Rectangle::<N, T, A, R>::from_min_size(min, size), rect; rect);
        rect_test_assert!(from_max_size: Rectangle::<N, T, A, R>::from_max_size(max, size), rect; rect);
        rect_test_assert!(from_center_size: Rectangle::<N, T, A, R>::from_center_size(center, size), rect; rect);
        rect_test_assert!(from_min_extents: Rectangle::<N, T, A, R>::from_min_extents(min, extents), rect; rect);
        rect_test_assert!(from_max_extents: Rectangle::<N, T, A, R>::from_max_extents(max, extents), rect; rect);
        rect_test_assert!(from_center_extents: Rectangle::<N, T, A, R>::from_center_extents(center, extents), rect; rect);
        rect_test_assert!(from_min_max: Rectangle::<N, T, A, R>::from_min_max(min, max), rect; rect);
        rect_test_assert!(from_min_center: Rectangle::<N, T, A, R>::from_min_center(min, center), rect; rect);
        rect_test_assert!(from_center_max: Rectangle::<N, T, A, R>::from_center_max(center, max), rect; rect);
    }

    Ok(())
}
