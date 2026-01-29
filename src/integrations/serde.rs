use serde::{Deserialize, Serialize};

use crate::{Alignment, Length, Scalar, SupportedLength, Vector, utils::transmute_generic};

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
        Ok(Self::from_array(unsafe {
            match N {
                2 => transmute_generic::<[T; 2], [T; N]>(Deserialize::deserialize(deserializer)?),
                3 => transmute_generic::<[T; 3], [T; N]>(Deserialize::deserialize(deserializer)?),
                4 => transmute_generic::<[T; 4], [T; N]>(Deserialize::deserialize(deserializer)?),
                _ => unreachable!(),
            }
        }))
    }
}
