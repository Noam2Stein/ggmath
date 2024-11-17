use super::*;

pub trait IntoVector<const N: usize, T: Scalar>: Sized
where
    ScalarCount<N>: VecLen<N>,
{
    fn into_vec_array(self) -> [T; N];

    fn into_vec<A: VecAlignment>(self) -> Vector<N, T, A> {
        Vector::from_array(self.into_vec_array())
    }
}
impl<const N: usize, T: Scalar, A: VecAlignment> IntoVector<N, T> for Vector<N, T, A>
where
    ScalarCount<N>: VecLen<N>,
{
    fn into_vec_array(self) -> [T; N] {
        self.into_array()
    }
}

impl<T: Scalar> IntoVector<2, T> for (T, T) {
    fn into_vec_array(self) -> [T; 2] {
        [self.0, self.1]
    }
}

impl<T: Scalar> IntoVector<3, T> for (T, T, T) {
    fn into_vec_array(self) -> [T; 3] {
        [self.0, self.1, self.2]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<3, T> for (Vector<2, T, A>, T) {
    fn into_vec_array(self) -> [T; 3] {
        [self.0.x(), self.0.y(), self.1]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<3, T> for (T, Vector<2, T, A>) {
    fn into_vec_array(self) -> [T; 3] {
        [self.0, self.1.x(), self.1.y()]
    }
}

impl<T: Scalar> IntoVector<4, T> for (T, T, T, T) {
    fn into_vec_array(self) -> [T; 4] {
        [self.0, self.1, self.2, self.3]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (Vector<2, T, A>, T, T) {
    fn into_vec_array(self) -> [T; 4] {
        [self.0.x(), self.0.y(), self.1, self.2]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (T, Vector<2, T, A>, T) {
    fn into_vec_array(self) -> [T; 4] {
        [self.0, self.1.x(), self.1.y(), self.2]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (T, T, Vector<2, T, A>) {
    fn into_vec_array(self) -> [T; 4] {
        [self.0, self.1, self.2.x(), self.2.y()]
    }
}
impl<T: Scalar, A0: VecAlignment, A1: VecAlignment> IntoVector<4, T>
    for (Vector<2, T, A0>, Vector<2, T, A1>)
{
    fn into_vec_array(self) -> [T; 4] {
        [self.0.x(), self.0.y(), self.1.x(), self.1.y()]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (Vector<3, T, A>, T) {
    fn into_vec_array(self) -> [T; 4] {
        [self.0.x(), self.0.y(), self.0.z(), self.1]
    }
}
impl<T: Scalar, A: VecAlignment> IntoVector<4, T> for (T, Vector<3, T, A>) {
    fn into_vec_array(self) -> [T; 4] {
        [self.0, self.1.x(), self.1.y(), self.1.z()]
    }
}
