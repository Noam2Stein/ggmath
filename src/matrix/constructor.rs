use crate::{Alignment, Matrix, Scalar, Vector};

/// Creates a 2x2 column-major matrix from the provided arguments.
///
/// The macro accepts 2 column vectors as arguments.
///
/// This macro has been replaced by [`Matrix::from_columns`] and will be removed
/// in a future version.
///
/// # Examples
///
/// ```
/// # use ggmath::{Mat2, mat2, vec2};
/// #
/// let double: Mat2<f32> = mat2!(vec2!(2.0, 0.0), vec2!(0.0, 2.0));
/// ```
#[deprecated(since = "0.16.3", note = "replaced by `Matrix::from_columns`")]
#[macro_export]
macro_rules! mat2 {
    ($($arg:expr),*$(,)?) => {
        $crate::Matrix::<2, _, _>::from(($($arg,)*))
    };
}

/// Creates a 3x3 column-major matrix from the provided arguments.
///
/// The macro accepts 3 column vectors as arguments.
///
/// This macro has been replaced by [`Matrix::from_columns`] and will be removed
/// in a future version.
///
/// # Examples
///
/// ```
/// # use ggmath::{Mat3, mat3, vec3};
/// #
/// let double: Mat3<f32> = mat3!(
///     vec3!(2.0, 0.0, 0.0),
///     vec3!(0.0, 2.0, 0.0),
///     vec3!(0.0, 0.0, 2.0)
/// );
/// ```
#[deprecated(since = "0.16.3", note = "replaced by `Matrix::from_columns`")]
#[macro_export]
macro_rules! mat3 {
    ($($arg:expr),*$(,)?) => {
        $crate::Matrix::<3, _, _>::from(($($arg,)*))
    };
}

/// Creates a 4x4 column-major matrix from the provided arguments.
///
/// The macro accepts 4 column vectors as arguments.
///
/// This macro has been replaced by [`Matrix::from_columns`] and will be removed
/// in a future version.
///
/// # Examples
///
/// ```
/// # use ggmath::{Mat4, mat4, vec4};
/// #
/// let double: Mat4<f32> = mat4!(
///     vec4!(2.0, 0.0, 0.0, 0.0),
///     vec4!(0.0, 2.0, 0.0, 0.0),
///     vec4!(0.0, 0.0, 2.0, 0.0),
///     vec4!(0.0, 0.0, 0.0, 2.0)
/// );
/// ```
#[deprecated(since = "0.16.3", note = "replaced by `Matrix::from_columns`")]
#[macro_export]
macro_rules! mat4 {
    ($($arg:expr),*$(,)?) => {
        $crate::Matrix::<4, _, _>::from(($($arg,)*))
    };
}

////////////////////////////////////////////////////////////////////////////////
// Matrix2 Implementations
////////////////////////////////////////////////////////////////////////////////

impl<T, A: Alignment> From<(Vector<2, T, A>, Vector<2, T, A>)> for Matrix<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<2, T, A>, Vector<2, T, A>)) -> Self {
        Self::from_columns(&[value.0, value.1])
    }
}

impl<T, A: Alignment> From<(Matrix<2, T, A>,)> for Matrix<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Matrix<2, T, A>,)) -> Self {
        value.0
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix3 Implementations
////////////////////////////////////////////////////////////////////////////////

impl<T, A: Alignment> From<(Vector<3, T, A>, Vector<3, T, A>, Vector<3, T, A>)> for Matrix<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<3, T, A>, Vector<3, T, A>, Vector<3, T, A>)) -> Self {
        Self::from_columns(&[value.0, value.1, value.2])
    }
}

impl<T, A: Alignment> From<(Matrix<3, T, A>,)> for Matrix<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Matrix<3, T, A>,)) -> Self {
        value.0
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix4 Implementations
////////////////////////////////////////////////////////////////////////////////

impl<T, A: Alignment>
    From<(
        Vector<4, T, A>,
        Vector<4, T, A>,
        Vector<4, T, A>,
        Vector<4, T, A>,
    )> for Matrix<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(
        value: (
            Vector<4, T, A>,
            Vector<4, T, A>,
            Vector<4, T, A>,
            Vector<4, T, A>,
        ),
    ) -> Self {
        Self::from_columns(&[value.0, value.1, value.2, value.3])
    }
}

impl<T, A: Alignment> From<(Matrix<4, T, A>,)> for Matrix<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Matrix<4, T, A>,)) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix, Vector, utils::for_parameters};

    #[test]
    #[expect(deprecated)]
    fn test_constructors() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                mat2!(Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(z, w)),
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ])
            );
            assert_eq!(
                mat2!(mat2!(
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                )),
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ])
            );

            assert_eq!(
                mat3!(
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ),
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
            );
            assert_eq!(
                mat3!(mat3!(
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                )),
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
            );

            assert_eq!(
                mat4!(
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ),
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
            );
            assert_eq!(
                mat4!(mat4!(
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                )),
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
            );
        });
    }
}
