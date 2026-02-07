use serde::{Deserialize, Serialize};

use crate::{Alignment, Length, Mask, Scalar, SupportedLength, Vector, utils::transmute_generic};

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
