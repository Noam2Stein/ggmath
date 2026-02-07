use mint::IntoMint;

use crate::{Alignment, Mask, Quaternion, Scalar, Vector, vec2, vec3, vec4};

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
