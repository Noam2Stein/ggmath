//! the [VectorOrScalar] type alias is either a ```Vector```, or ```T``` if ```N = 1```.

use inner::InnerVector;

use super::*;

pub type VectorOrScalar<const N: usize, T, A> =
    <ScalarCount<N> as VecLenOrOne>::VectorOrScalar<T, A>;

pub type VecNOrScalar<const N: usize, T> =
    <ScalarCount<N> as VecLenOrOne>::VectorOrScalar<T, VecAligned>;

pub type VecNPOrScalar<const N: usize, T> =
    <ScalarCount<N> as VecLenOrOne>::VectorOrScalar<T, VecPacked>;

pub type InnerVectorOrScalar<const N: usize, T, A> =
    <ScalarCount<N> as VecLenOrOne>::InnerVectorOrScalar<T, A>;

/// Sealed trait for ```ScalarCount```s that are valid as vector lengths or 1 for the scalar.
/// Used by [VectorOrScalar].
///
/// - Any ```ScalarCount``` that implements [VecLen] also implements this trait.
///
/// # Examples
/// ```
/// use ggmath::vec::{or_scalar::*, *};
///
/// // Be generic over N
/// struct RangeN<const N: usize>
/// where
///     ScalarCount<N>: VecLenOrOne, // Not VecLen!
/// {
///     start: VecNOrScalar<N, f32>,
///     end: VecNOrScalar<N, f32>,
/// }
///
/// // If we know that N is a VecLen, we know that RangeN<N> contains Vecs
/// fn get_vec<const N: usize>(range: RangeN<N>) -> VecN<N, f32> where ScalarCount<N>: VecLen<N> {
///     range.start
/// }
/// ```
#[allow(private_bounds)]
pub trait VecLenOrOne: Seal {
    type VectorOrScalar<T: Scalar, A: VecAlignment>: Construct;
    type InnerVectorOrScalar<T: Scalar, A: VecAlignment>: InnerConstruct;
}

impl Seal for ScalarCount<1> {}
impl<const N: usize> Seal for ScalarCount<N> where ScalarCount<N>: VecLen<N> {}

impl VecLenOrOne for ScalarCount<1> {
    type InnerVectorOrScalar<T: Scalar, A: VecAlignment> = T;
    type VectorOrScalar<T: Scalar, A: VecAlignment> = T;
}
impl<const N: usize> VecLenOrOne for ScalarCount<N>
where
    ScalarCount<N>: VecLen<N>,
{
    type VectorOrScalar<T: Scalar, A: VecAlignment> = Vector<N, T, A>;
    type InnerVectorOrScalar<T: Scalar, A: VecAlignment> = InnerVector<N, T, A>;
}

trait Seal {}
