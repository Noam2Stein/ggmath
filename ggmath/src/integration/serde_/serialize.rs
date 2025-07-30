use serde::Serialize;

use super::*;

// Vector

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

// Matrix

impl<const C: usize, const R: usize, T: Scalar + Serialize, A: VecAlignment, M: MatrixMajorAxis>
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

// Aabb

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

// Quaternion

impl<T: Scalar + Serialize, A: VecAlignment> Serialize for Quaternion<T, A> {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_array().serialize(serializer)
    }
}
