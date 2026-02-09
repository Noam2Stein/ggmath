use crate::{Alignment, Matrix, Scalar, Vector};

/// Creates a 2x2 column major matrix from the provided arguments.
///
/// The macro accepts 2 column vectors as arguments.
///
/// # Examples
///
/// ```
/// use ggmath::{Mat2, mat2, vec2};
///
/// let double: Mat2<f32> = mat2!(vec2!(2.0, 0.0), vec2!(0.0, 2.0));
/// ```
#[macro_export]
macro_rules! mat2 {
    ($($arg:expr),*$(,)?) => {
        $crate::Matrix::<2, _, _>::from(($($arg,)*))
    };
}

/// Creates a 3x3 column major matrix from the provided arguments.
///
/// The macro accepts 3 column vectors as arguments.
///
/// # Examples
///
/// ```
/// use ggmath::{Mat3, mat3, vec3};
///
/// let double: Mat3<f32> = mat3!(
///     vec3!(2.0, 0.0, 0.0),
///     vec3!(0.0, 2.0, 0.0),
///     vec3!(0.0, 0.0, 2.0)
/// );
/// ```
#[macro_export]
macro_rules! mat3 {
    ($($arg:expr),*$(,)?) => {
        $crate::Matrix::<3, _, _>::from(($($arg,)*))
    };
}

/// Creates a 4x4 column major matrix from the provided arguments.
///
/// The macro accepts 4 column vectors as arguments.
///
/// # Examples
///
/// ```
/// use ggmath::{Mat4, mat4, vec4};
///
/// let double: Mat4<f32> = mat4!(
///     vec4!(2.0, 0.0, 0.0, 0.0),
///     vec4!(0.0, 2.0, 0.0, 0.0),
///     vec4!(0.0, 0.0, 2.0, 0.0),
///     vec4!(0.0, 0.0, 0.0, 2.0)
/// );
/// ```
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
        Self::new(value.0, value.1)
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
        Self::new(value.0, value.1, value.2)
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
        Self::new(value.0, value.1, value.2, value.3)
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
