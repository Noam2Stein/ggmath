use serde::{Deserialize, Serialize};

use crate::{
    Affine, Alignment, Length, Mask, Matrix, Quaternion, Scalar, SupportedLength, Vector,
    utils::{transmute_generic, transmute_ref},
};

impl<const N: usize, T, A: Alignment> Serialize for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_array().serialize(serializer)
    }
}

impl<'de, const N: usize, T, A: Alignment> Deserialize<'de> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_array(match N {
            // SAFETY: Because `N == 2`, `[T; N]` and `[T; 2]` are the same
            // type.
            2 => unsafe {
                transmute_generic::<[T; 2], [T; N]>(Deserialize::deserialize(deserializer)?)
            },
            // SAFETY: Because `N == 3`, `[T; N]` and `[T; 3]` are the same
            // type.
            3 => unsafe {
                transmute_generic::<[T; 3], [T; N]>(Deserialize::deserialize(deserializer)?)
            },
            // SAFETY: Because `N == 4`, `[T; N]` and `[T; 4]` are the same
            // type.
            4 => unsafe {
                transmute_generic::<[T; 4], [T; N]>(Deserialize::deserialize(deserializer)?)
            },
            _ => unreachable!(),
        }))
    }
}

impl<const N: usize, T, A: Alignment> Serialize for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_col_array().serialize(serializer)
    }
}

impl<'de, const N: usize, T, A: Alignment> Deserialize<'de> for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_col_array(&match N {
            // SAFETY: Because `N == 2`, `[Vector<N, T, A>; N]` and
            // `[Vector<N, T, A>; 2]` are the same type.
            2 => unsafe {
                transmute_generic::<[Vector<N, T, A>; 2], [Vector<N, T, A>; N]>(
                    Deserialize::deserialize(deserializer)?,
                )
            },
            // SAFETY: Because `N == 3`, `[Vector<N, T, A>; N]` and
            // `[Vector<N, T, A>; 3]` are the same type.
            3 => unsafe {
                transmute_generic::<[Vector<N, T, A>; 3], [Vector<N, T, A>; N]>(
                    Deserialize::deserialize(deserializer)?,
                )
            },
            // SAFETY: Because `N == 4`, `[Vector<N, T, A>; N]` and
            // `[Vector<N, T, A>; 4]` are the same type.
            4 => unsafe {
                transmute_generic::<[Vector<N, T, A>; 4], [Vector<N, T, A>; N]>(
                    Deserialize::deserialize(deserializer)?,
                )
            },
            _ => unreachable!(),
        }))
    }
}

impl<T, A: Alignment> Serialize for Quaternion<T, A>
where
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_array().serialize(serializer)
    }
}

impl<'de, T, A: Alignment> Deserialize<'de> for Quaternion<T, A>
where
    T: Scalar + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_array(Deserialize::deserialize(deserializer)?))
    }
}

impl<const N: usize, T, A: Alignment> Serialize for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match N {
            // SAFETY: Because `N == 2`, `Affine<N, T, A>` and `Affine<2, T, A>`
            // are the same type.
            2 => unsafe {
                transmute_ref::<Affine<N, T, A>, Affine<2, T, A>>(self)
                    .as_col_array_ref()
                    .serialize(serializer)
            },

            // SAFETY: Because `N == 3`, `Affine<N, T, A>` and `Affine<3, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_ref::<Affine<N, T, A>, Affine<3, T, A>>(self)
                    .as_col_array_ref()
                    .serialize(serializer)
            },

            // SAFETY: Because `N == 4`, `Affine<N, T, A>` and `Affine<4, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_ref::<Affine<N, T, A>, Affine<4, T, A>>(self)
                    .as_col_array_ref()
                    .serialize(serializer)
            },

            _ => unreachable!(),
        }
    }
}

impl<'de, const N: usize, T, A: Alignment> Deserialize<'de> for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match N {
            // SAFETY: Because `N == 2`, `Affine<2, T, A>` and `Affine<N, T, A>`
            // are the same type.
            2 => unsafe {
                transmute_generic::<Affine<2, T, A>, Affine<N, T, A>>(
                    Affine::<2, T, A>::from_col_array(&Deserialize::deserialize(deserializer)?),
                )
            },

            // SAFETY: Because `N == 3`, `Affine<3, T, A>` and `Affine<N, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_generic::<Affine<3, T, A>, Affine<N, T, A>>(
                    Affine::<3, T, A>::from_col_array(&Deserialize::deserialize(deserializer)?),
                )
            },

            // SAFETY: Because `N == 4`, `Affine<4, T, A>` and `Affine<N, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_generic::<Affine<4, T, A>, Affine<N, T, A>>(
                    Affine::<4, T, A>::from_col_array(&Deserialize::deserialize(deserializer)?),
                )
            },

            _ => unreachable!(),
        })
    }
}

impl<const N: usize, T, A: Alignment> Serialize for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_array().serialize(serializer)
    }
}

impl<'de, const N: usize, T, A: Alignment> Deserialize<'de> for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_array(match N {
            // SAFETY: Because `N == 2`, `[bool; N]` and `[bool; 2]` are the
            // same type.
            2 => unsafe {
                transmute_generic::<[bool; 2], [bool; N]>(Deserialize::deserialize(deserializer)?)
            },
            // SAFETY: Because `N == 3`, `[bool; N]` and `[bool; 3]` are the
            // same type.
            3 => unsafe {
                transmute_generic::<[bool; 3], [bool; N]>(Deserialize::deserialize(deserializer)?)
            },
            // SAFETY: Because `N == 4`, `[bool; N]` and `[bool; 4]` are the
            // same type.
            4 => unsafe {
                transmute_generic::<[bool; 4], [bool; N]>(Deserialize::deserialize(deserializer)?)
            },
            _ => unreachable!(),
        }))
    }
}
