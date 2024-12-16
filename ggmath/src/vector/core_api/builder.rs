use super::*;

pub use ggmath_proc_macros::{vec2, vec2p, vec3, vec3p, vec4, vec4p};

/// For types that can be turned into a vector through the ```vecn!()``` macros.
///
/// For example ```(T, T)``` implements ```VectorBuilder<2>``` so you can perform ```vec2!(x, y)```,
/// and ```(Vector<2, T, impl VecAlignment>, T, T)``` implements ```VectorBuilder<4>``` so you can perform ```vec4!(xy, z, w)```.
pub trait VectorBuilder<const N: usize>
where
    ScalarCount<N>: VecLen,
{
    type T: Scalar;

    fn build<A: VecAlignment>(self) -> Vector<N, Self::T, A>;
}

impl<const N: usize, T: Scalar, AInput: VecAlignment> VectorBuilder<N> for Vector<N, T, AInput>
where
    ScalarCount<N>: VecLen,
{
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment>(self) -> Vector<N, Self::T, A> {
        self.into_alignment()
    }
}

// N = 2

impl<T: Scalar> VectorBuilder<2> for (T, T) {
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment>(self) -> Vector<2, Self::T, A> {
        Vector::from_array([self.0, self.1])
    }
}

// N = 3

impl<T: Scalar> VectorBuilder<3> for (T, T, T) {
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment>(self) -> Vector<3, Self::T, A> {
        Vector::from_array([self.0, self.1, self.2])
    }
}

impl<T: Scalar, AInput: VecAlignment> VectorBuilder<3> for (Vector<2, T, AInput>, T) {
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment>(self) -> Vector<3, Self::T, A> {
        Vector::from_array([self.0.x(), self.0.y(), self.1])
    }
}

impl<T: Scalar, AInput: VecAlignment> VectorBuilder<3> for (T, Vector<2, T, AInput>) {
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment>(self) -> Vector<3, Self::T, A> {
        Vector::from_array([self.0, self.1.x(), self.1.y()])
    }
}

// N = 4

impl<T: Scalar> VectorBuilder<4> for (T, T, T, T) {
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment>(self) -> Vector<4, Self::T, A> {
        Vector::from_array([self.0, self.1, self.2, self.3])
    }
}

impl<T: Scalar, AInput: VecAlignment> VectorBuilder<4> for (Vector<2, T, AInput>, T, T) {
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment>(self) -> Vector<4, Self::T, A> {
        Vector::from_array([self.0.x(), self.0.y(), self.1, self.2])
    }
}

impl<T: Scalar, AAInput: VecAlignment> VectorBuilder<4> for (T, Vector<2, T, AAInput>, T) {
    type T = T;
    fn build<A: VecAlignment>(self) -> Vector<4, Self::T, A> {
        Vector::from_array([self.0, self.1.x(), self.1.y(), self.2])
    }
}

impl<T: Scalar, AAInput: VecAlignment> VectorBuilder<4> for (T, T, Vector<2, T, AAInput>) {
    type T = T;
    fn build<A: VecAlignment>(self) -> Vector<4, Self::T, A> {
        Vector::from_array([self.0, self.1, self.2.x(), self.2.y()])
    }
}

impl<T: Scalar, AAInput0: VecAlignment, AAInput1: VecAlignment> VectorBuilder<4>
    for (Vector<2, T, AAInput0>, Vector<2, T, AAInput1>)
{
    type T = T;
    fn build<A: VecAlignment>(self) -> Vector<4, Self::T, A> {
        Vector::from_array([self.0.x(), self.0.y(), self.1.x(), self.1.y()])
    }
}

impl<T: Scalar, AAInput: VecAlignment> VectorBuilder<4> for (Vector<3, T, AAInput>, T) {
    type T = T;
    fn build<A: VecAlignment>(self) -> Vector<4, Self::T, A> {
        Vector::from_array([self.0.x(), self.0.y(), self.0.z(), self.1])
    }
}

impl<T: Scalar, AInput: VecAlignment> VectorBuilder<4> for (T, Vector<3, T, AInput>) {
    type T = T;
    fn build<A: VecAlignment>(self) -> Vector<4, Self::T, A> {
        Vector::from_array([self.0, self.1.x(), self.1.y(), self.1.z()])
    }
}
