use crate::{Scalar, VecAlignment, Vector};

/// Constructs an aligned vec2 from the given values.
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 2.
/// This means that the options are:
/// - (scalar, scalar)
/// - (vec2)
#[macro_export]
macro_rules! vec2 {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<2, _, $crate::vector::VecAligned>::from(($($expr),*,))
    };
}

/// Constructs an aligned vec3 from the given values.
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 3.
/// This means that the options are:
/// - (scalar, scalar, scalar)
/// - (vec2, scalar)
/// - (scalar, vec2)
/// - (vec3)
#[macro_export]
macro_rules! vec3 {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<3, _, $crate::vector::VecAligned>::from(($($expr),*,))
    };
}

/// Constructs an aligned vec4 from the given values.
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 4.
/// This means that the options are:
/// - (scalar, scalar, scalar, scalar)
/// - (vec2, scalar, scalar)
/// - (scalar, vec2, scalar)
/// - (scalar, scalar, vec2)
/// - (vec2, vec2)
/// - (vec3, scalar)
/// - (scalar, vec3)
/// - (vec4)
#[macro_export]
macro_rules! vec4 {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<4, _, $crate::vector::VecAligned>::from(($($expr),*,))
    };
}

/// Constructs a packed vec2 from the given values.
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 2.
/// This means that the options are:
/// - (scalar, scalar)
/// - (vec2)
#[macro_export]
macro_rules! vec2p {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<2, _, $crate::vector::VecPacked>::from(($($expr),*,))
    };
}

/// Constructs a packed vec3 from the given values.
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 3.
/// This means that the options are:
/// - (scalar, scalar, scalar)
/// - (vec2, scalar)
/// - (scalar, vec2)
/// - (vec3)
#[macro_export]
macro_rules! vec3p {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<3, _, $crate::vector::VecPacked>::from(($($expr),*,))
    };
}

/// Constructs a packed vec4 from the given values.
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 4.
/// This means that the options are:
/// - (scalar, scalar, scalar, scalar)
/// - (vec2, scalar, scalar)
/// - (scalar, vec2, scalar)
/// - (scalar, scalar, vec2)
/// - (vec2, vec2)
/// - (vec3, scalar)
/// - (scalar, vec3)
/// - (vec4)
#[macro_export]
macro_rules! vec4p {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<4, _, $crate::vector::VecPacked>::from(($($expr),*,))
    };
}

/// Constructs a vec2 from the given values that needs type inference to decide if its [`VecAligned`] or [`VecPacked`].
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 2.
/// This means that the options are:
/// - (scalar, scalar)
/// - (vec2)
#[macro_export]
macro_rules! vec2g {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<2, _, _>::from(($($expr),*,))
    };
}

/// Constructs a vec3 from the given values that needs type inference to decide if its [`VecAligned`] or [`VecPacked`].
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 3.
/// This means that the options are:
/// - (scalar, scalar, scalar)
/// - (vec2, scalar)
/// - (scalar, vec2)
/// - (vec3)
#[macro_export]
macro_rules! vec3g {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<3, _, _>::from(($($expr),*,))
    };
}

/// Constructs a vec4 from the given values that needs type inference to decide if its [`VecAligned`] or [`VecPacked`].
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 4.
/// This means that the options are:
/// - (scalar, scalar, scalar, scalar)
/// - (vec2, scalar, scalar)
/// - (scalar, vec2, scalar)
/// - (scalar, scalar, vec2)
/// - (vec2, vec2)
/// - (vec3, scalar)
/// - (scalar, vec3)
/// - (vec4)
#[macro_export]
macro_rules! vec4g {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<4, _, _>::from(($($expr),*,))
    };
}

impl<T: Scalar, A: VecAlignment> From<(T, T)> for Vector<2, T, A> {
    #[inline(always)]
    fn from(value: (T, T)) -> Self {
        Self::from_array([value.0, value.1])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<2, T, A0>,)> for Vector<2, T, A> {
    #[inline(always)]
    fn from(value: (Vector<2, T, A0>,)) -> Self {
        Self::from_array([value.0[0], value.0[1]])
    }
}
impl<T: Scalar, A: VecAlignment> From<(T, T, T)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (T, T, T)) -> Self {
        Self::from_array([value.0, value.1, value.2])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(T, Vector<2, T, A0>)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (T, Vector<2, T, A0>)) -> Self {
        Self::from_array([value.0, value.1[0], value.1[1]])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<2, T, A0>, T)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (Vector<2, T, A0>, T)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.1])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<3, T, A0>,)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (Vector<3, T, A0>,)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.0[2]])
    }
}
impl<T: Scalar, A: VecAlignment> From<(T, T, T, T)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (T, T, T, T)) -> Self {
        Self::from_array([value.0, value.1, value.2, value.3])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(T, T, Vector<2, T, A0>)>
    for Vector<4, T, A>
{
    #[inline(always)]
    fn from(value: (T, T, Vector<2, T, A0>)) -> Self {
        Self::from_array([value.0, value.1, value.2[0], value.2[1]])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(T, Vector<2, T, A0>, T)>
    for Vector<4, T, A>
{
    #[inline(always)]
    fn from(value: (T, Vector<2, T, A0>, T)) -> Self {
        Self::from_array([value.0, value.1[0], value.1[1], value.2])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(T, Vector<3, T, A0>)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (T, Vector<3, T, A0>)) -> Self {
        Self::from_array([value.0, value.1[0], value.1[1], value.1[2]])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<2, T, A0>, T, T)>
    for Vector<4, T, A>
{
    #[inline(always)]
    fn from(value: (Vector<2, T, A0>, T, T)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.1, value.2])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<2, T, A0>, Vector<2, T, A0>)>
    for Vector<4, T, A>
{
    #[inline(always)]
    fn from(value: (Vector<2, T, A0>, Vector<2, T, A0>)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.1[0], value.1[1]])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<3, T, A0>, T)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (Vector<3, T, A0>, T)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.0[2], value.1])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<4, T, A0>,)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (Vector<4, T, A0>,)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.0[2], value.0[3]])
    }
}
