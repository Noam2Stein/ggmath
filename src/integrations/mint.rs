use mint::IntoMint;

use crate::{
    Alignment, Mask, Matrix, Quaternion, Scalar, Vector, mat2, mat3, mat4, vec2, vec3, vec4,
};

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
        vec2!(value.x, value.y)
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
        vec2!(value.x, value.y)
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
        vec3!(value.x, value.y, value.z)
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
        vec3!(value.x, value.y, value.z)
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
        vec4!(value.x, value.y, value.z, value.w)
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
    type MintType = mint::ColumnMatrix2<T>;
}

impl<T, A: Alignment> From<mint::ColumnMatrix2<T>> for Matrix<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::ColumnMatrix2<T>) -> Self {
        mat2!(value.x.into(), value.y.into())
    }
}

impl<T, A: Alignment> From<Matrix<2, T, A>> for mint::ColumnMatrix2<T>
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

impl<T, A: Alignment> From<mint::RowMatrix2<T>> for Matrix<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::RowMatrix2<T>) -> Self {
        mat2!(vec2!(value.x.x, value.y.x), vec2!(value.x.y, value.y.y))
    }
}

impl<T, A: Alignment> From<Matrix<2, T, A>> for mint::RowMatrix2<T>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Matrix<2, T, A>) -> Self {
        Self {
            x: mint::Vector2 {
                x: value.x_axis.x,
                y: value.y_axis.x,
            },
            y: mint::Vector2 {
                x: value.x_axis.y,
                y: value.y_axis.y,
            },
        }
    }
}

impl<T, A: Alignment> IntoMint for Matrix<3, T, A>
where
    T: Scalar,
{
    type MintType = mint::ColumnMatrix3<T>;
}

impl<T, A: Alignment> From<mint::ColumnMatrix3<T>> for Matrix<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::ColumnMatrix3<T>) -> Self {
        mat3!(value.x.into(), value.y.into(), value.z.into())
    }
}

impl<T, A: Alignment> From<Matrix<3, T, A>> for mint::ColumnMatrix3<T>
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

impl<T, A: Alignment> From<mint::RowMatrix3<T>> for Matrix<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::RowMatrix3<T>) -> Self {
        mat3!(
            vec3!(value.x.x, value.y.x, value.z.x),
            vec3!(value.x.y, value.y.y, value.z.y),
            vec3!(value.x.z, value.y.z, value.z.z)
        )
    }
}

impl<T, A: Alignment> From<Matrix<3, T, A>> for mint::RowMatrix3<T>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Matrix<3, T, A>) -> Self {
        Self {
            x: mint::Vector3 {
                x: value.x_axis.x,
                y: value.y_axis.x,
                z: value.z_axis.x,
            },
            y: mint::Vector3 {
                x: value.x_axis.y,
                y: value.y_axis.y,
                z: value.z_axis.y,
            },
            z: mint::Vector3 {
                x: value.x_axis.z,
                y: value.y_axis.z,
                z: value.z_axis.z,
            },
        }
    }
}

impl<T, A: Alignment> IntoMint for Matrix<4, T, A>
where
    T: Scalar,
{
    type MintType = mint::ColumnMatrix4<T>;
}

impl<T, A: Alignment> From<mint::ColumnMatrix4<T>> for Matrix<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::ColumnMatrix4<T>) -> Self {
        mat4!(
            value.x.into(),
            value.y.into(),
            value.z.into(),
            value.w.into()
        )
    }
}

impl<T, A: Alignment> From<Matrix<4, T, A>> for mint::ColumnMatrix4<T>
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

impl<T, A: Alignment> From<mint::RowMatrix4<T>> for Matrix<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: mint::RowMatrix4<T>) -> Self {
        mat4!(
            vec4!(value.x.x, value.y.x, value.z.x, value.w.x),
            vec4!(value.x.y, value.y.y, value.z.y, value.w.y),
            vec4!(value.x.z, value.y.z, value.z.z, value.w.z),
            vec4!(value.x.w, value.y.w, value.z.w, value.w.w)
        )
    }
}

impl<T, A: Alignment> From<Matrix<4, T, A>> for mint::RowMatrix4<T>
where
    T: Scalar,
{
    #[inline]
    fn from(value: Matrix<4, T, A>) -> Self {
        Self {
            x: mint::Vector4 {
                x: value.x_axis.x,
                y: value.y_axis.x,
                z: value.z_axis.x,
                w: value.w_axis.x,
            },
            y: mint::Vector4 {
                x: value.x_axis.y,
                y: value.y_axis.y,
                z: value.z_axis.y,
                w: value.w_axis.y,
            },
            z: mint::Vector4 {
                x: value.x_axis.z,
                y: value.y_axis.z,
                z: value.z_axis.z,
                w: value.w_axis.z,
            },
            w: mint::Vector4 {
                x: value.x_axis.w,
                y: value.y_axis.w,
                z: value.z_axis.w,
                w: value.w_axis.w,
            },
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
        Self::new(value.v.x, value.v.y, value.v.z, value.s)
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
