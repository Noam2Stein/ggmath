use mint::IntoMint;

use crate::{Alignment, Mask, Matrix, Quaternion, Scalar, Vector};

impl<T, A: Alignment> IntoMint for Vector<2, T, A>
where
    T: Scalar,
{
    type MintType = mint::Vector2<T>;
}

impl<T, A: Alignment> From<mint::Point2<T>> for Vector<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::Point2<T>) -> Self {
        Vector::<2, T, A>::new(value.x, value.y)
    }
}

impl<T, A: Alignment> From<Vector<2, T, A>> for mint::Point2<T>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Vector<2, T, A>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl<T, A: Alignment> From<mint::Vector2<T>> for Vector<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::Vector2<T>) -> Self {
        Vector::<2, T, A>::new(value.x, value.y)
    }
}

impl<T, A: Alignment> From<Vector<2, T, A>> for mint::Vector2<T>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Vector<2, T, A>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl<T, A: Alignment> IntoMint for Vector<3, T, A>
where
    T: Scalar,
{
    type MintType = mint::Vector3<T>;
}

impl<T, A: Alignment> From<mint::Point3<T>> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::Point3<T>) -> Self {
        Vector::<3, T, A>::new(value.x, value.y, value.z)
    }
}

impl<T, A: Alignment> From<Vector<3, T, A>> for mint::Point3<T>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Vector<3, T, A>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl<T, A: Alignment> From<mint::Vector3<T>> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::Vector3<T>) -> Self {
        Vector::<3, T, A>::new(value.x, value.y, value.z)
    }
}

impl<T, A: Alignment> From<Vector<3, T, A>> for mint::Vector3<T>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Vector<3, T, A>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl<T, A: Alignment> IntoMint for Vector<4, T, A>
where
    T: Scalar,
{
    type MintType = mint::Vector4<T>;
}

impl<T, A: Alignment> From<mint::Vector4<T>> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::Vector4<T>) -> Self {
        Vector::<4, T, A>::new(value.x, value.y, value.z, value.w)
    }
}

impl<T, A: Alignment> From<Vector<4, T, A>> for mint::Vector4<T>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Vector<4, T, A>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl<T, A: Alignment> IntoMint for Matrix<2, T, A>
where
    T: Scalar,
{
    type MintType = mint::RowMatrix2<T>;
}

impl<T, A: Alignment> From<mint::RowMatrix2<T>> for Matrix<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::RowMatrix2<T>) -> Self {
        Matrix::<2, T, A>::from_rows(&[value.x.into(), value.y.into()])
    }
}

impl<T, A: Alignment> From<Matrix<2, T, A>> for mint::RowMatrix2<T>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Matrix<2, T, A>) -> Self {
        Self {
            x: value.x_axis.into(),
            y: value.y_axis.into(),
        }
    }
}

impl<T, A: Alignment> IntoMint for Matrix<3, T, A>
where
    T: Scalar,
{
    type MintType = mint::RowMatrix3<T>;
}

impl<T, A: Alignment> From<mint::RowMatrix3<T>> for Matrix<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::RowMatrix3<T>) -> Self {
        Matrix::<3, T, A>::from_rows(&[value.x.into(), value.y.into(), value.z.into()])
    }
}

impl<T, A: Alignment> From<Matrix<3, T, A>> for mint::RowMatrix3<T>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Matrix<3, T, A>) -> Self {
        Self {
            x: value.x_axis.into(),
            y: value.y_axis.into(),
            z: value.z_axis.into(),
        }
    }
}

impl<T, A: Alignment> IntoMint for Matrix<4, T, A>
where
    T: Scalar,
{
    type MintType = mint::RowMatrix4<T>;
}

impl<T, A: Alignment> From<mint::RowMatrix4<T>> for Matrix<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::RowMatrix4<T>) -> Self {
        Matrix::<4, T, A>::from_rows(&[
            value.x.into(),
            value.y.into(),
            value.z.into(),
            value.w.into(),
        ])
    }
}

impl<T, A: Alignment> From<Matrix<4, T, A>> for mint::RowMatrix4<T>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Matrix<4, T, A>) -> Self {
        Self {
            x: value.x_axis.into(),
            y: value.y_axis.into(),
            z: value.z_axis.into(),
            w: value.w_axis.into(),
        }
    }
}

impl<T, A: Alignment> IntoMint for Quaternion<T, A>
where
    T: Scalar,
{
    type MintType = mint::Quaternion<T>;
}

impl<T, A: Alignment> From<mint::Quaternion<T>> for Quaternion<T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::Quaternion<T>) -> Self {
        Self::from_xyzw(value.v.x, value.v.y, value.v.z, value.s)
    }
}

impl<T, A: Alignment> From<Quaternion<T, A>> for mint::Quaternion<T>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Quaternion<T, A>) -> Self {
        Self {
            v: mint::Vector3 {
                x: value.x,
                y: value.y,
                z: value.z,
            },
            s: value.w,
        }
    }
}

impl<T, A: Alignment> IntoMint for Mask<2, T, A>
where
    T: Scalar,
{
    type MintType = mint::Vector2<bool>;
}

impl<T, A: Alignment> From<mint::Vector2<bool>> for Mask<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::Vector2<bool>) -> Self {
        Self::new(value.x, value.y)
    }
}

impl<T, A: Alignment> From<Mask<2, T, A>> for mint::Vector2<bool>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Mask<2, T, A>) -> Self {
        Self {
            x: value.get(0),
            y: value.get(1),
        }
    }
}

impl<T, A: Alignment> IntoMint for Mask<3, T, A>
where
    T: Scalar,
{
    type MintType = mint::Vector3<bool>;
}

impl<T, A: Alignment> From<mint::Vector3<bool>> for Mask<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::Vector3<bool>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl<T, A: Alignment> From<Mask<3, T, A>> for mint::Vector3<bool>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Mask<3, T, A>) -> Self {
        Self {
            x: value.get(0),
            y: value.get(1),
            z: value.get(2),
        }
    }
}

impl<T, A: Alignment> IntoMint for Mask<4, T, A>
where
    T: Scalar,
{
    type MintType = mint::Vector4<bool>;
}

impl<T, A: Alignment> From<mint::Vector4<bool>> for Mask<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::Vector4<bool>) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}

impl<T, A: Alignment> From<Mask<4, T, A>> for mint::Vector4<bool>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Mask<4, T, A>) -> Self {
        Self {
            x: value.get(0),
            y: value.get(1),
            z: value.get(2),
            w: value.get(3),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Mask2, Mask2U, Mask3, Mask3U, Mask4, Mask4U, Mat2, Mat2U, Mat3, Mat3U, Mat4, Mat4U, Quat,
        QuatU, Vec2, Vec2U, Vec3, Vec3U, Vec4, Vec4U,
    };

    #[test]
    fn test_vector() {
        let vector = Vec2::new(1, 2);
        assert_eq!(vector, mint::Point2::from(vector).into());
        assert_eq!(vector, mint::Vector2::from(vector).into());

        let vector = Vec3::new(1, 2, 3);
        assert_eq!(vector, mint::Point3::from(vector).into());
        assert_eq!(vector, mint::Vector3::from(vector).into());

        let vector = Vec4::new(1, 2, 3, 4);
        assert_eq!(vector, mint::Vector4::from(vector).into());

        let vector = Vec2U::new(1, 2);
        assert_eq!(vector, mint::Point2::from(vector).into());
        assert_eq!(vector, mint::Vector2::from(vector).into());

        let vector = Vec3U::new(1, 2, 3);
        assert_eq!(vector, mint::Point3::from(vector).into());
        assert_eq!(vector, mint::Vector3::from(vector).into());

        let vector = Vec4U::new(1, 2, 3, 4);
        assert_eq!(vector, mint::Vector4::from(vector).into());
    }

    #[test]
    fn test_matrix() {
        let matrix = Mat2::from_rows(&[Vec2::new(1, 2), Vec2::new(3, 4)]);
        assert_eq!(matrix, mint::RowMatrix2::from(matrix).into());

        let matrix = Mat3::from_rows(&[Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(7, 8, 9)]);
        assert_eq!(matrix, mint::RowMatrix3::from(matrix).into());

        let matrix = Mat4::from_rows(&[
            Vec4::new(1, 2, 3, 4),
            Vec4::new(5, 6, 7, 8),
            Vec4::new(9, 10, 11, 12),
            Vec4::new(13, 14, 15, 16),
        ]);
        assert_eq!(matrix, mint::RowMatrix4::from(matrix).into());

        let matrix = Mat2U::from_rows(&[Vec2U::new(1, 2), Vec2U::new(3, 4)]);
        assert_eq!(matrix, mint::RowMatrix2::from(matrix).into());

        let matrix = Mat3U::from_rows(&[
            Vec3U::new(1, 2, 3),
            Vec3U::new(4, 5, 6),
            Vec3U::new(7, 8, 9),
        ]);
        assert_eq!(matrix, mint::RowMatrix3::from(matrix).into());

        let matrix = Mat4U::from_rows(&[
            Vec4U::new(1, 2, 3, 4),
            Vec4U::new(5, 6, 7, 8),
            Vec4U::new(9, 10, 11, 12),
            Vec4U::new(13, 14, 15, 16),
        ]);
        assert_eq!(matrix, mint::RowMatrix4::from(matrix).into());
    }

    #[test]
    fn test_quaternion() {
        let quat = Quat::from_xyzw(1, 2, 3, 4);
        assert_eq!(quat, mint::Quaternion::from(quat).into());

        let quat = QuatU::from_xyzw(1, 2, 3, 4);
        assert_eq!(quat, mint::Quaternion::from(quat).into());
    }

    #[test]
    fn test_mask() {
        let mask = Mask2::<i32>::new(false, true);
        assert_eq!(mask, mint::Vector2::<bool>::from(mask).into());

        let mask = Mask3::<i32>::new(false, true, false);
        assert_eq!(mask, mint::Vector3::<bool>::from(mask).into());

        let mask = Mask4::<i32>::new(false, true, false, true);
        assert_eq!(mask, mint::Vector4::<bool>::from(mask).into());

        let mask = Mask2U::<i32>::new(false, true);
        assert_eq!(mask, mint::Vector2::<bool>::from(mask).into());

        let mask = Mask3U::<i32>::new(false, true, false);
        assert_eq!(mask, mint::Vector3::<bool>::from(mask).into());

        let mask = Mask4U::<i32>::new(false, true, false, true);
        assert_eq!(mask, mint::Vector4::<bool>::from(mask).into());
    }
}
