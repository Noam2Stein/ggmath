use crate::{Alignment, Scalar, Vector};

/// Creates a 2-dimensional vector from the provided arguments.
///
/// The macro accepts scalars and vectors as arguments, as long as they can be
/// combined to form a 2-component vector. If only a single scalar argument is
/// provided, it is duplicated across all components.
///
/// # Examples
///
/// ```
/// use ggmath::{Vec2, vec2};
///
/// let one_two: Vec2<f32> = vec2!(1.0, 2.0);
/// let one_two: Vec2<f32> = vec2!(vec2!(1.0, 2.0));
/// let one_one: Vec2<f32> = vec2!(1.0);
/// ```
#[macro_export]
macro_rules! vec2 {
    ($($arg:expr),*$(,)?) => {
        $crate::Vector::<2, _, _>::from(($($arg,)*))
    };
}

/// Creates a 3-dimensional vector from the provided arguments.
///
/// The macro accepts scalars and vectors as arguments, as long as they can be
/// combined to form a 3-component vector. If only a single scalar argument is
/// provided, it is duplicated across all components.
///
/// # Examples
///
/// ```
/// use ggmath::{Vec3, vec2, vec3};
///
/// let one_two_three: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
/// let one_two_three: Vec3<f32> = vec3!(vec2!(1.0, 2.0), 3.0);
/// let one_two_three: Vec3<f32> = vec3!(vec3!(1.0, 2.0, 3.0));
/// let one_one_one: Vec3<f32> = vec3!(1.0);
/// ```
#[macro_export]
macro_rules! vec3 {
    ($($arg:expr),*$(,)?) => {
        $crate::Vector::<3, _, _>::from(($($arg,)*))
    };
}

/// Creates a 4-dimensional vector from the provided arguments.
///
/// The macro accepts scalars and vectors as arguments, as long as they can be
/// combined to form a 4-component vector. If only a single scalar argument is
/// provided, it is duplicated across all components.
///
/// # Examples
///
/// ```
/// use ggmath::{Vec4, vec2, vec3, vec4};
///
/// let one_two_three_four: Vec4<f32> = vec4!(1.0, 2.0, 3.0, 4.0);
/// let one_two_three_four: Vec4<f32> = vec4!(vec2!(1.0, 2.0), vec2!(3.0, 4.0));
/// let one_two_three_four: Vec4<f32> = vec4!(vec3!(1.0, 2.0, 3.0), 4.0);
/// let one_two_three_four: Vec4<f32> = vec4!(vec4!(1.0, 2.0, 3.0, 4.0));
/// let one_one_one_one: Vec4<f32> = vec4!(1.0);
/// ```
#[macro_export]
macro_rules! vec4 {
    ($($arg:expr),*$(,)?) => {
        $crate::Vector::<4, _, _>::from(($($arg,)*))
    };
}

////////////////////////////////////////////////////////////////////////////////
// Vector2 Implementations
////////////////////////////////////////////////////////////////////////////////

impl<T, A: Alignment> From<(T, T)> for Vector<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T, A: Alignment> From<(Vector<2, T, A>,)> for Vector<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<2, T, A>,)) -> Self {
        value.0
    }
}

impl<T, A: Alignment> From<(T,)> for Vector<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T,)) -> Self {
        Self::new(value.0, value.0)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector3 Implementations
////////////////////////////////////////////////////////////////////////////////

impl<T, A: Alignment> From<(T, T, T)> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl<T, A: Alignment> From<(T, Vector<2, T, A>)> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, Vector<2, T, A>)) -> Self {
        Self::new(value.0, value.1[0], value.1[1])
    }
}

impl<T, A: Alignment> From<(Vector<2, T, A>, T)> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<2, T, A>, T)) -> Self {
        Self::new(value.0[0], value.0[1], value.1)
    }
}

impl<T, A: Alignment> From<(Vector<3, T, A>,)> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<3, T, A>,)) -> Self {
        value.0
    }
}

impl<T, A: Alignment> From<(T,)> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T,)) -> Self {
        Self::new(value.0, value.0, value.0)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector4 Implementations
////////////////////////////////////////////////////////////////////////////////

impl<T, A: Alignment> From<(T, T, T, T)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl<T, A: Alignment> From<(T, T, Vector<2, T, A>)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, T, Vector<2, T, A>)) -> Self {
        Self::new(value.0, value.1, value.2[0], value.2[1])
    }
}

impl<T, A: Alignment> From<(T, Vector<2, T, A>, T)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, Vector<2, T, A>, T)) -> Self {
        Self::new(value.0, value.1[0], value.1[1], value.2)
    }
}

impl<T, A: Alignment> From<(T, Vector<3, T, A>)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, Vector<3, T, A>)) -> Self {
        Self::new(value.0, value.1[0], value.1[1], value.1[2])
    }
}

impl<T, A: Alignment> From<(Vector<2, T, A>, T, T)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<2, T, A>, T, T)) -> Self {
        Self::new(value.0[0], value.0[1], value.1, value.2)
    }
}

impl<T, A: Alignment> From<(Vector<2, T, A>, Vector<2, T, A>)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<2, T, A>, Vector<2, T, A>)) -> Self {
        Self::new(value.0[0], value.0[1], value.1[0], value.1[1])
    }
}

impl<T, A: Alignment> From<(Vector<3, T, A>, T)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<3, T, A>, T)) -> Self {
        Self::new(value.0[0], value.0[1], value.0[2], value.1)
    }
}

impl<T, A: Alignment> From<(Vector<4, T, A>,)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<4, T, A>,)) -> Self {
        value.0
    }
}

impl<T, A: Alignment> From<(T,)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T,)) -> Self {
        Self::new(value.0, value.0, value.0, value.0)
    }
}
