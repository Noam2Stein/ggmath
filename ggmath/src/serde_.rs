use std::mem::transmute_copy;

use serde::{Deserialize, Serialize};

use super::*;

// Vector

#[cfg(feature = "vector")]
impl<const N: usize, T: Scalar + Serialize, A: VecAlignment> Serialize for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_array().serialize(serializer)
    }
}

#[cfg(feature = "vector")]
impl<'de, const N: usize, T: Scalar + Deserialize<'de>, A: VecAlignment> Deserialize<'de>
    for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match N {
            2 => {
                let arr = <[T; 2]>::deserialize(deserializer)?;
                Ok(Self::from_array(unsafe {
                    transmute_copy::<[T; 2], [T; N]>(&arr)
                }))
            }
            3 => {
                let arr = <[T; 3]>::deserialize(deserializer)?;
                Ok(Self::from_array(unsafe {
                    transmute_copy::<[T; 3], [T; N]>(&arr)
                }))
            }
            4 => {
                let arr = <[T; 4]>::deserialize(deserializer)?;
                Ok(Self::from_array(unsafe {
                    transmute_copy::<[T; 4], [T; N]>(&arr)
                }))
            }

            _ => unreachable!("vector length is expected to be 2, 3, or 4"),
        }
    }
}

// Matrix

#[cfg(feature = "matrix")]
impl<const C: usize, const R: usize, T: Scalar + Serialize, A: VecAlignment, M: MatMajorAxis>
    Serialize for Matrix<R, C, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.resolve() {
            ResolvedMatrix::ColumnMajor(m) => m.columns().serialize(serializer),
            ResolvedMatrix::RowMajor(m) => m.rows().serialize(serializer),
        }
    }
}

#[cfg(feature = "matrix")]
impl<
    'de,
    const C: usize,
    const R: usize,
    T: Scalar + Deserialize<'de>,
    A: VecAlignment,
    M: MatMajorAxis,
> Deserialize<'de> for Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    #[inline(always)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match M::ENUM {
            MatMajorAxisEnum::ColumnMajor => match C {
                2 => {
                    let arr = <[Vector<R, T, A>; 2]>::deserialize(deserializer)?;

                    Matrix::from_columns(unsafe {
                        transmute_copy::<[Vector<R, T, A>; 2], [Vector<R, T, A>; C]>(&arr)
                    })
                }
                3 => {
                    let arr = <[Vector<R, T, A>; 3]>::deserialize(deserializer)?;

                    Matrix::from_columns(unsafe {
                        transmute_copy::<[Vector<R, T, A>; 3], [Vector<R, T, A>; C]>(&arr)
                    })
                }
                4 => {
                    let arr = <[Vector<R, T, A>; 4]>::deserialize(deserializer)?;

                    Matrix::from_columns(unsafe {
                        transmute_copy::<[Vector<R, T, A>; 4], [Vector<R, T, A>; C]>(&arr)
                    })
                }

                _ => unreachable!("matrix column length is expected to be 2, 3, or 4"),
            },

            MatMajorAxisEnum::RowMajor => match R {
                2 => {
                    let arr = <[Vector<C, T, A>; 2]>::deserialize(deserializer)?;

                    Matrix::from_rows(unsafe {
                        transmute_copy::<[Vector<C, T, A>; 2], [Vector<C, T, A>; R]>(&arr)
                    })
                }
                3 => {
                    let arr = <[Vector<C, T, A>; 3]>::deserialize(deserializer)?;

                    Matrix::from_rows(unsafe {
                        transmute_copy::<[Vector<C, T, A>; 3], [Vector<C, T, A>; R]>(&arr)
                    })
                }
                4 => {
                    let arr = <[Vector<C, T, A>; 4]>::deserialize(deserializer)?;

                    Matrix::from_rows(unsafe {
                        transmute_copy::<[Vector<C, T, A>; 4], [Vector<C, T, A>; R]>(&arr)
                    })
                }

                _ => unreachable!("matrix column length is expected to be 2, 3, or 4"),
            },
        })
    }
}

// Aabb

#[cfg(feature = "aabb")]
impl<const N: usize, T: AabbScalar + Serialize, A: VecAlignment, R: AabbRepr> Serialize
    for Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.resolve() {
            ResolvedAabb::Cornered(a) => (a.min(), a.size()).serialize(serializer),
            ResolvedAabb::Centered(a) => (a.center(), a.extents()).serialize(serializer),
            ResolvedAabb::MinMaxed(a) => (a.min(), a.max()).serialize(serializer),
        }
    }
}

#[cfg(feature = "aabb")]
impl<'de, const N: usize, T: AabbScalar + Deserialize<'de>, A: VecAlignment, R: AabbRepr>
    Deserialize<'de> for Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match R::ENUM {
            AabbReprEnum::Centered => {
                let (center, extents) =
                    <(Vector<N, T, A>, Vector<N, T, A>)>::deserialize(deserializer)?;

                Ok(Self::from_center_extents(center, extents))
            }
            AabbReprEnum::Cornered => {
                let (min, size) = <(Vector<N, T, A>, Vector<N, T, A>)>::deserialize(deserializer)?;

                Ok(Self::from_min_size(min, size))
            }
            AabbReprEnum::MinMaxed => {
                let (min, max) = <(Vector<N, T, A>, Vector<N, T, A>)>::deserialize(deserializer)?;

                Ok(Self::from_min_max(min, max))
            }
        }
    }
}

// Quaternion

#[cfg(feature = "quaternion")]
impl<T: Scalar + Serialize, A: VecAlignment> Serialize for Quaternion<T, A> {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_array().serialize(serializer)
    }
}

#[cfg(feature = "quaternion")]
impl<'de, T: Scalar + Deserialize<'de>, A: VecAlignment> Deserialize<'de> for Quaternion<T, A> {
    #[inline(always)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_array(Deserialize::deserialize(deserializer)?))
    }
}
