use crate::{Scalar, Scalar, Simdness, Vector};

mod f32;
mod f64;
pub(crate) use f32::*;
pub(crate) use f64::*;

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! vector_specialization_hack {
    (<$T:ty as $VectorApi:ident<N, S>>::$method:ident($($arg:expr),* $(,)?)) => {
        const {
            match S::IS_SIMD {
                false => ||,
                true => match N {

                }
            }
        }
        ($($arg),*)
    };
}

pub(crate) unsafe fn transmute_vector_length<T: TransmuteVectorLength<N2>, const N2: usize>(
    t: T,
) -> T::Output {
    unsafe { t.transmute_vector_length() }
}

pub(crate) fn as_vector_simdness<T: AsVectorSimdness<S2>, S2: Simdness>(t: T) -> T::Output {
    t.as_vector_simdness::<S2>()
}

trait TransmuteVectorLength<const N2: usize> {
    type Output;

    unsafe fn transmute_vector_length(self) -> Self::Output;
}

trait AsVectorSimdness<S2: Simdness> {
    type Output;

    fn as_vector_simdness(self) -> Self::Output;
}

impl<T: Scalar, const N2: usize> TransmuteVectorLength<N2> for T {
    type Output = T;

    unsafe fn transmute_vector_length(self) -> Self::Output {
        self
    }
}

impl<T: Scalar, S2: Simdness> AsVectorSimdness<S2> for T {
    type Output = T;

    fn as_vector_simdness(self) -> Self::Output {
        self
    }
}

impl<const N: usize, T: Scalar<N, S> + Scalar<N2, S>, S: Simdness, const N2: usize>
    TransmuteVectorLength<N2> for Vector<N, T, S>
{
    type Output = Vector<N2, T, S>;

    unsafe fn transmute_vector_length(self) -> Self::Output {
        unsafe { core::mem::transmute_copy::<Vector<N, T, S>, Vector<N2, T, S>>(&self) }
    }
}

impl<const N: usize, T: Scalar<N, S> + Scalar<N, S2>, S: Simdness, S2: Simdness>
    AsVectorSimdness<S2> for Vector<N, T, S>
{
    type Output = Vector<N, T, S2>;

    fn as_vector_simdness(self) -> Self::Output {
        self.as_storage()
    }
}
