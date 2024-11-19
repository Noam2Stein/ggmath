use crate::builder::*;

use super::*;

impl<const N: usize, T: Scalar, AInput: VecAlignment, AOutput: VecAlignment>
    Builder<Vector<N, T, AOutput>> for Vector<N, T, AInput>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn build(self) -> Vector<N, T, AOutput> {
        self.into_alignment()
    }
}

impl<T: Scalar, OutputA: VecAlignment> Builder<Vector<2, T, OutputA>> for (T, T) {
    #[inline(always)]
    fn build(self) -> Vector<2, T, OutputA> {
        Vector::from_array([self.0, self.1])
    }
}

impl<T: Scalar, OutputA: VecAlignment> Builder<Vector<3, T, OutputA>> for (T, T, T) {
    #[inline(always)]
    fn build(self) -> Vector<3, T, OutputA> {
        Vector::from_array([self.0, self.1, self.2])
    }
}
impl<T: Scalar, A: VecAlignment, OutputA: VecAlignment> Builder<Vector<3, T, OutputA>>
    for (Vector<2, T, A>, T)
{
    #[inline(always)]
    fn build(self) -> Vector<3, T, OutputA> {
        Vector::from_array([self.0.x(), self.0.y(), self.1])
    }
}
impl<T: Scalar, A: VecAlignment, OutputA: VecAlignment> Builder<Vector<3, T, OutputA>>
    for (T, Vector<2, T, A>)
{
    #[inline(always)]
    fn build(self) -> Vector<3, T, OutputA> {
        Vector::from_array([self.0, self.1.x(), self.1.y()])
    }
}

impl<T: Scalar, OutputA: VecAlignment> Builder<Vector<4, T, OutputA>> for (T, T, T, T) {
    fn build(self) -> Vector<4, T, OutputA> {
        Vector::from_array([self.0, self.1, self.2, self.3])
    }
}
impl<T: Scalar, A: VecAlignment, OutputA: VecAlignment> Builder<Vector<4, T, OutputA>>
    for (Vector<2, T, A>, T, T)
{
    fn build(self) -> Vector<4, T, OutputA> {
        Vector::from_array([self.0.x(), self.0.y(), self.1, self.2])
    }
}
impl<T: Scalar, A: VecAlignment, OutputA: VecAlignment> Builder<Vector<4, T, OutputA>>
    for (T, Vector<2, T, A>, T)
{
    fn build(self) -> Vector<4, T, OutputA> {
        Vector::from_array([self.0, self.1.x(), self.1.y(), self.2])
    }
}
impl<T: Scalar, A: VecAlignment, OutputA: VecAlignment> Builder<Vector<4, T, OutputA>>
    for (T, T, Vector<2, T, A>)
{
    fn build(self) -> Vector<4, T, OutputA> {
        Vector::from_array([self.0, self.1, self.2.x(), self.2.y()])
    }
}
impl<T: Scalar, A0: VecAlignment, A1: VecAlignment, OutputA: VecAlignment>
    Builder<Vector<4, T, OutputA>> for (Vector<2, T, A0>, Vector<2, T, A1>)
{
    fn build(self) -> Vector<4, T, OutputA> {
        Vector::from_array([self.0.x(), self.0.y(), self.1.x(), self.1.y()])
    }
}
impl<T: Scalar, A: VecAlignment, OutputA: VecAlignment> Builder<Vector<4, T, OutputA>>
    for (Vector<3, T, A>, T)
{
    fn build(self) -> Vector<4, T, OutputA> {
        Vector::from_array([self.0.x(), self.0.y(), self.0.z(), self.1])
    }
}
impl<T: Scalar, A: VecAlignment, OutputA: VecAlignment> Builder<Vector<4, T, OutputA>>
    for (T, Vector<3, T, A>)
{
    fn build(self) -> Vector<4, T, OutputA> {
        Vector::from_array([self.0, self.1.x(), self.1.y(), self.1.z()])
    }
}
