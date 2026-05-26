use serde::{Deserialize, Serialize};

use crate::{
    Affine, Alignment, Length, Mask, Matrix, Quaternion, Scalar, SupportedLength, Vector,
    utils::{transmute_generic, transmute_ref},
};

impl<const N: usize, T, A: Alignment> Serialize for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_array().serialize(serializer)
    }
}

impl<'de, const N: usize, T, A: Alignment> Deserialize<'de> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_array(match N {
            // SAFETY: Because `N == 2`, `[T; N]` and `[T; 2]` are the same
            // type.
            2 => unsafe {
                transmute_generic::<[T; 2], [T; N]>(Deserialize::deserialize(deserializer)?)
            },
            // SAFETY: Because `N == 3`, `[T; N]` and `[T; 3]` are the same
            // type.
            3 => unsafe {
                transmute_generic::<[T; 3], [T; N]>(Deserialize::deserialize(deserializer)?)
            },
            // SAFETY: Because `N == 4`, `[T; N]` and `[T; 4]` are the same
            // type.
            4 => unsafe {
                transmute_generic::<[T; 4], [T; N]>(Deserialize::deserialize(deserializer)?)
            },
            _ => unreachable!(),
        }))
    }
}

impl<const N: usize, T, A: Alignment> Serialize for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_rows().serialize(serializer)
    }
}

impl<'de, const N: usize, T, A: Alignment> Deserialize<'de> for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_rows(&match N {
            // SAFETY: Because `N == 2`, `[Vector<N, T, A>; N]` and
            // `[Vector<N, T, A>; 2]` are the same type.
            2 => unsafe {
                transmute_generic::<[Vector<N, T, A>; 2], [Vector<N, T, A>; N]>(
                    Deserialize::deserialize(deserializer)?,
                )
            },
            // SAFETY: Because `N == 3`, `[Vector<N, T, A>; N]` and
            // `[Vector<N, T, A>; 3]` are the same type.
            3 => unsafe {
                transmute_generic::<[Vector<N, T, A>; 3], [Vector<N, T, A>; N]>(
                    Deserialize::deserialize(deserializer)?,
                )
            },
            // SAFETY: Because `N == 4`, `[Vector<N, T, A>; N]` and
            // `[Vector<N, T, A>; 4]` are the same type.
            4 => unsafe {
                transmute_generic::<[Vector<N, T, A>; 4], [Vector<N, T, A>; N]>(
                    Deserialize::deserialize(deserializer)?,
                )
            },
            _ => unreachable!(),
        }))
    }
}

impl<T, A: Alignment> Serialize for Quaternion<T, A>
where
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_array().serialize(serializer)
    }
}

impl<'de, T, A: Alignment> Deserialize<'de> for Quaternion<T, A>
where
    T: Scalar + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_array(Deserialize::deserialize(deserializer)?))
    }
}

impl<const N: usize, T, A: Alignment> Serialize for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match N {
            // SAFETY: Because `N == 2`, `Affine<N, T, A>` and `Affine<2, T, A>`
            // are the same type.
            2 => unsafe {
                transmute_ref::<Affine<N, T, A>, Affine<2, T, A>>(self)
                    .as_rows()
                    .serialize(serializer)
            },

            // SAFETY: Because `N == 3`, `Affine<N, T, A>` and `Affine<3, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_ref::<Affine<N, T, A>, Affine<3, T, A>>(self)
                    .as_rows()
                    .serialize(serializer)
            },

            // SAFETY: Because `N == 4`, `Affine<N, T, A>` and `Affine<4, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_ref::<Affine<N, T, A>, Affine<4, T, A>>(self)
                    .as_rows()
                    .serialize(serializer)
            },

            _ => unreachable!(),
        }
    }
}

impl<'de, const N: usize, T, A: Alignment> Deserialize<'de> for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match N {
            // SAFETY: Because `N == 2`, `Affine<2, T, A>` and `Affine<N, T, A>`
            // are the same type.
            2 => unsafe {
                transmute_generic::<Affine<2, T, A>, Affine<N, T, A>>(Affine::<2, T, A>::from_rows(
                    &Deserialize::deserialize(deserializer)?,
                ))
            },

            // SAFETY: Because `N == 3`, `Affine<3, T, A>` and `Affine<N, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_generic::<Affine<3, T, A>, Affine<N, T, A>>(Affine::<3, T, A>::from_rows(
                    &Deserialize::deserialize(deserializer)?,
                ))
            },

            // SAFETY: Because `N == 4`, `Affine<4, T, A>` and `Affine<N, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_generic::<Affine<4, T, A>, Affine<N, T, A>>(Affine::<4, T, A>::from_rows(
                    &Deserialize::deserialize(deserializer)?,
                ))
            },

            _ => unreachable!(),
        })
    }
}

impl<const N: usize, T, A: Alignment> Serialize for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_array().serialize(serializer)
    }
}

impl<'de, const N: usize, T, A: Alignment> Deserialize<'de> for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_array(match N {
            // SAFETY: Because `N == 2`, `[bool; N]` and `[bool; 2]` are the
            // same type.
            2 => unsafe {
                transmute_generic::<[bool; 2], [bool; N]>(Deserialize::deserialize(deserializer)?)
            },
            // SAFETY: Because `N == 3`, `[bool; N]` and `[bool; 3]` are the
            // same type.
            3 => unsafe {
                transmute_generic::<[bool; 3], [bool; N]>(Deserialize::deserialize(deserializer)?)
            },
            // SAFETY: Because `N == 4`, `[bool; N]` and `[bool; 4]` are the
            // same type.
            4 => unsafe {
                transmute_generic::<[bool; 4], [bool; N]>(Deserialize::deserialize(deserializer)?)
            },
            _ => unreachable!(),
        }))
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::{boxed::Box, error::Error};

    use serde_json::{from_str, to_string};

    use crate::{
        Affine, Affine2, Affine2A, Affine3, Affine3A, Aligned, Mask2, Mask2A, Mask3, Mask3A, Mask4,
        Mask4A, Mat2, Mat2A, Mat3, Mat3A, Mat4, Mat4A, Quat, QuatA, Unaligned, Vec2, Vec2A, Vec3,
        Vec3A, Vec4, Vec4A,
    };

    #[test]
    fn test_vector() -> Result<(), Box<dyn Error>> {
        let vector = Vec2A::<i32>::new(1, 2);
        assert_eq!(vector, from_str(&to_string(&vector)?)?);
        assert_eq!(vector.unalign(), from_str(&to_string(&vector)?)?);
        assert!(from_str::<Vec3A<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec4A<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec3<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec4<i32>>(&to_string(&vector)?).is_err());

        let vector = Vec3A::<i32>::new(1, 2, 3);
        assert_eq!(vector, from_str(&to_string(&vector)?)?);
        assert_eq!(vector.unalign(), from_str(&to_string(&vector)?)?);
        assert!(from_str::<Vec2A<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec4A<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec2<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec4<i32>>(&to_string(&vector)?).is_err());

        let vector = Vec4A::<i32>::new(1, 2, 3, 4);
        assert_eq!(vector, from_str(&to_string(&vector)?)?);
        assert_eq!(vector.unalign(), from_str(&to_string(&vector)?)?);
        assert!(from_str::<Vec2A<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec3A<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec2<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec3<i32>>(&to_string(&vector)?).is_err());

        let vector = Vec2::<i32>::new(1, 2);
        assert_eq!(vector, from_str(&to_string(&vector)?)?);
        assert_eq!(vector.align(), from_str(&to_string(&vector)?)?);
        assert!(from_str::<Vec3A<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec4A<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec3<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec4<i32>>(&to_string(&vector)?).is_err());

        let vector = Vec3::<i32>::new(1, 2, 3);
        assert_eq!(vector, from_str(&to_string(&vector)?)?);
        assert_eq!(vector.align(), from_str(&to_string(&vector)?)?);
        assert!(from_str::<Vec2A<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec4A<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec2<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec4<i32>>(&to_string(&vector)?).is_err());

        let vector = Vec4::<i32>::new(1, 2, 3, 4);
        assert_eq!(vector, from_str(&to_string(&vector)?)?);
        assert_eq!(vector.align(), from_str(&to_string(&vector)?)?);
        assert!(from_str::<Vec2A<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec3A<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec2<i32>>(&to_string(&vector)?).is_err());
        assert!(from_str::<Vec3<i32>>(&to_string(&vector)?).is_err());

        Ok(())
    }

    #[test]
    fn test_matrix() -> Result<(), Box<dyn Error>> {
        let matrix = Mat2A::<i32>::from_rows(&[Vec2A::new(1, 2), Vec2A::new(3, 4)]);
        assert_eq!(matrix, from_str(&to_string(&matrix)?)?);
        assert_eq!(matrix.unalign(), from_str(&to_string(&matrix)?)?);
        assert!(from_str::<Mat3A<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat4A<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat3<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat4<i32>>(&to_string(&matrix)?).is_err());

        let matrix = Mat3A::<i32>::from_rows(&[
            Vec3A::new(1, 2, 3),
            Vec3A::new(4, 5, 6),
            Vec3A::new(7, 8, 9),
        ]);
        assert_eq!(matrix, from_str(&to_string(&matrix)?)?);
        assert_eq!(matrix.unalign(), from_str(&to_string(&matrix)?)?);
        assert!(from_str::<Mat2A<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat4A<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat2<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat4<i32>>(&to_string(&matrix)?).is_err());

        let matrix = Mat4A::<i32>::from_rows(&[
            Vec4A::new(1, 2, 3, 4),
            Vec4A::new(5, 6, 7, 8),
            Vec4A::new(9, 10, 11, 12),
            Vec4A::new(13, 14, 15, 16),
        ]);
        assert_eq!(matrix, from_str(&to_string(&matrix)?)?);
        assert_eq!(matrix.unalign(), from_str(&to_string(&matrix)?)?);
        assert!(from_str::<Mat2A<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat3A<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat2<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat3<i32>>(&to_string(&matrix)?).is_err());

        let matrix = Mat2::<i32>::from_rows(&[Vec2::new(1, 2), Vec2::new(3, 4)]);
        assert_eq!(matrix, from_str(&to_string(&matrix)?)?);
        assert_eq!(matrix.align(), from_str(&to_string(&matrix)?)?);
        assert!(from_str::<Mat3A<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat4A<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat3<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat4<i32>>(&to_string(&matrix)?).is_err());

        let matrix =
            Mat3::<i32>::from_rows(&[Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(7, 8, 9)]);
        assert_eq!(matrix, from_str(&to_string(&matrix)?)?);
        assert_eq!(matrix.align(), from_str(&to_string(&matrix)?)?);
        assert!(from_str::<Mat2A<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat4A<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat2<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat4<i32>>(&to_string(&matrix)?).is_err());

        let matrix = Mat4::<i32>::from_rows(&[
            Vec4::new(1, 2, 3, 4),
            Vec4::new(5, 6, 7, 8),
            Vec4::new(9, 10, 11, 12),
            Vec4::new(13, 14, 15, 16),
        ]);
        assert_eq!(matrix, from_str(&to_string(&matrix)?)?);
        assert_eq!(matrix.align(), from_str(&to_string(&matrix)?)?);
        assert!(from_str::<Mat2A<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat3A<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat2<i32>>(&to_string(&matrix)?).is_err());
        assert!(from_str::<Mat3<i32>>(&to_string(&matrix)?).is_err());

        Ok(())
    }

    #[test]
    fn test_quaternion() -> Result<(), Box<dyn Error>> {
        let quat = QuatA::<i32>::from_xyzw(1, 2, 3, 4);
        assert_eq!(quat, from_str(&to_string(&quat)?)?);
        assert_eq!(quat.unalign(), from_str(&to_string(&quat)?)?);

        let quat = Quat::<i32>::from_xyzw(1, 2, 3, 4);
        assert_eq!(quat, from_str(&to_string(&quat)?)?);
        assert_eq!(quat.align(), from_str(&to_string(&quat)?)?);

        Ok(())
    }

    #[test]
    fn test_affine() -> Result<(), Box<dyn Error>> {
        let affine = Affine2A::<i32>::from_submatrix_translation(
            Mat2A::from_rows(&[Vec2A::new(1, 2), Vec2A::new(3, 4)]),
            Vec2A::new(5, 6),
        );
        assert_eq!(affine, from_str(&to_string(&affine)?)?);
        assert_eq!(affine.unalign(), from_str(&to_string(&affine)?)?);
        assert!(from_str::<Affine3A<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Aligned>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine3<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Unaligned>>(&to_string(&affine)?).is_err());

        let affine = Affine3A::<i32>::from_submatrix_translation(
            Mat3A::from_rows(&[
                Vec3A::new(1, 2, 3),
                Vec3A::new(4, 5, 6),
                Vec3A::new(97, 8, 9),
            ]),
            Vec3A::new(10, 11, 12),
        );
        assert_eq!(affine, from_str(&to_string(&affine)?)?);
        assert_eq!(affine.unalign(), from_str(&to_string(&affine)?)?);
        assert!(from_str::<Affine2A<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Aligned>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine2<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Unaligned>>(&to_string(&affine)?).is_err());

        let affine = Affine::<4, i32, Aligned>::from_submatrix_translation(
            Mat4A::from_rows(&[
                Vec4A::new(1, 2, 3, 4),
                Vec4A::new(5, 6, 7, 8),
                Vec4A::new(9, 10, 11, 12),
                Vec4A::new(13, 14, 15, 16),
            ]),
            Vec4A::new(17, 18, 19, 20),
        );
        assert_eq!(affine, from_str(&to_string(&affine)?)?);
        assert_eq!(affine.unalign(), from_str(&to_string(&affine)?)?);
        assert!(from_str::<Affine2A<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine3A<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine2<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine3<i32>>(&to_string(&affine)?).is_err());

        let affine = Affine2::<i32>::from_submatrix_translation(
            Mat2::from_rows(&[Vec2::new(1, 2), Vec2::new(3, 4)]),
            Vec2::new(5, 6),
        );
        assert_eq!(affine, from_str(&to_string(&affine)?)?);
        assert_eq!(affine.align(), from_str(&to_string(&affine)?)?);
        assert!(from_str::<Affine3A<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Aligned>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine3<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Unaligned>>(&to_string(&affine)?).is_err());

        let affine = Affine3::<i32>::from_submatrix_translation(
            Mat3::from_rows(&[Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(97, 8, 9)]),
            Vec3::new(10, 11, 12),
        );
        assert_eq!(affine, from_str(&to_string(&affine)?)?);
        assert_eq!(affine.align(), from_str(&to_string(&affine)?)?);
        assert!(from_str::<Affine2A<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Aligned>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine2<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Unaligned>>(&to_string(&affine)?).is_err());

        let affine = Affine::<4, i32, Unaligned>::from_submatrix_translation(
            Mat4::from_rows(&[
                Vec4::new(1, 2, 3, 4),
                Vec4::new(5, 6, 7, 8),
                Vec4::new(9, 10, 11, 12),
                Vec4::new(13, 14, 15, 16),
            ]),
            Vec4::new(17, 18, 19, 20),
        );
        assert_eq!(affine, from_str(&to_string(&affine)?)?);
        assert_eq!(affine.align(), from_str(&to_string(&affine)?)?);
        assert!(from_str::<Affine2A<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine3A<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine2<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine3<i32>>(&to_string(&affine)?).is_err());

        Ok(())
    }

    #[test]
    fn test_mask() -> Result<(), Box<dyn Error>> {
        let mask = Mask2A::<i32>::new(false, true);
        assert_eq!(mask, from_str(&to_string(&mask)?)?);
        assert_eq!(mask.unalign(), from_str(&to_string(&mask)?)?);
        assert!(from_str::<Mask3A<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask4A<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask3<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask4<i32>>(&to_string(&mask)?).is_err());

        let mask = Mask3A::<i32>::new(false, true, false);
        assert_eq!(mask, from_str(&to_string(&mask)?)?);
        assert_eq!(mask.unalign(), from_str(&to_string(&mask)?)?);
        assert!(from_str::<Mask2A<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask4A<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask2<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask4<i32>>(&to_string(&mask)?).is_err());

        let mask = Mask4A::<i32>::new(false, true, false, true);
        assert_eq!(mask, from_str(&to_string(&mask)?)?);
        assert_eq!(mask.unalign(), from_str(&to_string(&mask)?)?);
        assert!(from_str::<Mask2A<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask3A<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask2<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask3<i32>>(&to_string(&mask)?).is_err());

        let mask = Mask2::<i32>::new(false, true);
        assert_eq!(mask, from_str(&to_string(&mask)?)?);
        assert_eq!(mask.align(), from_str(&to_string(&mask)?)?);
        assert!(from_str::<Mask3A<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask4A<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask3<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask4<i32>>(&to_string(&mask)?).is_err());

        let mask = Mask3::<i32>::new(false, true, false);
        assert_eq!(mask, from_str(&to_string(&mask)?)?);
        assert_eq!(mask.align(), from_str(&to_string(&mask)?)?);
        assert!(from_str::<Mask2A<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask4A<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask2<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask4<i32>>(&to_string(&mask)?).is_err());

        let mask = Mask4::<i32>::new(false, true, false, true);
        assert_eq!(mask, from_str(&to_string(&mask)?)?);
        assert_eq!(mask.align(), from_str(&to_string(&mask)?)?);
        assert!(from_str::<Mask2A<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask3A<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask2<i32>>(&to_string(&mask)?).is_err());
        assert!(from_str::<Mask3<i32>>(&to_string(&mask)?).is_err());

        Ok(())
    }
}
