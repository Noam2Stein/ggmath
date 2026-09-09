use core::marker::PhantomData;

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
};

use crate::{
    Affine, Alignment, Length, Mask, Matrix, Projective, Rotation2, Rotor, Scalar, SupportedLength,
    Vector,
    length::{Three, TwoOrThree},
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

impl<const N: usize, T, A: Alignment> Serialize for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match N {
            // SAFETY: Because `N == 2`, `Projective<N, T, A>` and
            // `Projective<2, T, A>` are the same type.
            2 => unsafe {
                transmute_ref::<Projective<N, T, A>, Projective<2, T, A>>(self)
                    .as_rows()
                    .serialize(serializer)
            },

            // SAFETY: Because `N == 3`, `Projective<N, T, A>` and
            // `Projective<3, T, A>` are the same type.
            3 => unsafe {
                transmute_ref::<Projective<N, T, A>, Projective<3, T, A>>(self)
                    .as_rows()
                    .serialize(serializer)
            },

            _ => unreachable!(),
        }
    }
}

impl<'de, const N: usize, T, A: Alignment> Deserialize<'de> for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match N {
            // SAFETY: Because `N == 2`, `Projective<2, T, A>` and
            // `Projective<N, T, A>` are the same type.
            2 => unsafe {
                transmute_generic::<Projective<2, T, A>, Projective<N, T, A>>(
                    Projective::<2, T, A>::from_rows(&Deserialize::deserialize(deserializer)?),
                )
            },

            // SAFETY: Because `N == 3`, `Projective<3, T, A>` and
            // `Projective<N, T, A>` are the same type.
            3 => unsafe {
                transmute_generic::<Projective<3, T, A>, Projective<N, T, A>>(
                    Projective::<3, T, A>::from_rows(&Deserialize::deserialize(deserializer)?),
                )
            },

            _ => unreachable!(),
        })
    }
}

impl<T, A: Alignment> Serialize for Rotation2<T, A>
where
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Rot2", 2)?;
        state.serialize_field("cos", &self.cos)?;
        state.serialize_field("sin", &self.sin)?;
        state.end()
    }
}

impl<'de, T, A: Alignment> Deserialize<'de> for Rotation2<T, A>
where
    T: Scalar + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["cos", "sin"];

        enum Field {
            Cos,
            Sin,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                        formatter.write_str("`cos` or `sin`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cos" => Ok(Field::Cos),
                            "sin" => Ok(Field::Sin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Rot2Visitor<T, A: Alignment>(PhantomData<(T, A)>);

        impl<'de, T, A: Alignment> Visitor<'de> for Rot2Visitor<T, A>
        where
            T: Scalar + Deserialize<'de>,
        {
            type Value = Rotation2<T, A>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("struct Rot2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let cos = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let sin = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(Rotation2::from_cos_sin(cos, sin))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut cos = None;
                let mut sin = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Cos => {
                            if cos.is_some() {
                                return Err(serde::de::Error::duplicate_field("cos"));
                            }
                            cos = Some(map.next_value()?);
                        }
                        Field::Sin => {
                            if sin.is_some() {
                                return Err(serde::de::Error::duplicate_field("sin"));
                            }
                            sin = Some(map.next_value()?);
                        }
                    }
                }
                let cos = cos.ok_or_else(|| serde::de::Error::missing_field("cos"))?;
                let sin = sin.ok_or_else(|| serde::de::Error::missing_field("sin"))?;
                Ok(Rotation2::from_cos_sin(cos, sin))
            }
        }

        deserializer.deserialize_struct("Rot2", FIELDS, Rot2Visitor(PhantomData))
    }
}

impl<const N: usize, T, A: Alignment> Serialize for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Rotor3", 4)?;
        state.serialize_field("yz", &self.0.x)?;
        state.serialize_field("zx", &self.0.y)?;
        state.serialize_field("xy", &self.0.z)?;
        state.serialize_field("s", &self.0.w)?;
        state.end()
    }
}

impl<'de, const N: usize, T, A: Alignment> Deserialize<'de> for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["yz", "zx", "xy", "s"];

        enum Field {
            Yz,
            Zx,
            Xy,
            S,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                        formatter.write_str("`yz`, `zx`, `xy` or `s`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "yz" => Ok(Field::Yz),
                            "zx" => Ok(Field::Zx),
                            "xy" => Ok(Field::Xy),
                            "s" => Ok(Field::S),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RotorVisitor<const N: usize, T, A: Alignment>(PhantomData<(T, A)>);

        impl<'de, const N: usize, T, A: Alignment> Visitor<'de> for RotorVisitor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Deserialize<'de>,
        {
            type Value = Rotor<N, T, A>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("struct Rotor3")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let yz = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let zx = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let xy = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                let s = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;
                Ok(Rotor(Vector::<4, T, A>::new(yz, zx, xy, s)))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut yz = None;
                let mut zx = None;
                let mut xy = None;
                let mut s = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Yz => {
                            if yz.is_some() {
                                return Err(serde::de::Error::duplicate_field("yz"));
                            }
                            yz = Some(map.next_value()?);
                        }
                        Field::Zx => {
                            if zx.is_some() {
                                return Err(serde::de::Error::duplicate_field("zx"));
                            }
                            zx = Some(map.next_value()?);
                        }
                        Field::Xy => {
                            if xy.is_some() {
                                return Err(serde::de::Error::duplicate_field("xy"));
                            }
                            xy = Some(map.next_value()?);
                        }
                        Field::S => {
                            if s.is_some() {
                                return Err(serde::de::Error::duplicate_field("s"));
                            }
                            s = Some(map.next_value()?);
                        }
                    }
                }
                let yz = yz.ok_or_else(|| serde::de::Error::missing_field("yz"))?;
                let zx = zx.ok_or_else(|| serde::de::Error::missing_field("zx"))?;
                let xy = xy.ok_or_else(|| serde::de::Error::missing_field("xy"))?;
                let s = s.ok_or_else(|| serde::de::Error::missing_field("s"))?;
                Ok(Rotor(Vector::<4, T, A>::new(yz, zx, xy, s)))
            }
        }

        deserializer.deserialize_struct("Rotor3", FIELDS, RotorVisitor(PhantomData))
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
        Mask4A, Mat2, Mat2A, Mat3, Mat3A, Mat4, Mat4A, Proj2, Proj2A, Proj3, Proj3A, Rot2, Rotor3,
        Rotor3A, Unaligned, Vec2, Vec2A, Vec3, Vec3A, Vec4, Vec4A,
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
    fn test_affine() -> Result<(), Box<dyn Error>> {
        let affine = Affine2A::<i32>::from_matrix_translation(
            &Mat2A::from_rows(&[Vec2A::new(1, 2), Vec2A::new(3, 4)]),
            Vec2A::new(5, 6),
        );
        assert_eq!(affine, from_str(&to_string(&affine)?)?);
        assert_eq!(affine.unalign(), from_str(&to_string(&affine)?)?);
        assert!(from_str::<Affine3A<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Aligned>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine3<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Unaligned>>(&to_string(&affine)?).is_err());

        let affine = Affine3A::<i32>::from_matrix_translation(
            &Mat3A::from_rows(&[
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

        let affine = Affine::<4, i32, Aligned>::from_matrix_translation(
            &Mat4A::from_rows(&[
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

        let affine = Affine2::<i32>::from_matrix_translation(
            &Mat2::from_rows(&[Vec2::new(1, 2), Vec2::new(3, 4)]),
            Vec2::new(5, 6),
        );
        assert_eq!(affine, from_str(&to_string(&affine)?)?);
        assert_eq!(affine.align(), from_str(&to_string(&affine)?)?);
        assert!(from_str::<Affine3A<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Aligned>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine3<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Unaligned>>(&to_string(&affine)?).is_err());

        let affine = Affine3::<i32>::from_matrix_translation(
            &Mat3::from_rows(&[Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(97, 8, 9)]),
            Vec3::new(10, 11, 12),
        );
        assert_eq!(affine, from_str(&to_string(&affine)?)?);
        assert_eq!(affine.align(), from_str(&to_string(&affine)?)?);
        assert!(from_str::<Affine2A<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Aligned>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine2<i32>>(&to_string(&affine)?).is_err());
        assert!(from_str::<Affine<4, i32, Unaligned>>(&to_string(&affine)?).is_err());

        let affine = Affine::<4, i32, Unaligned>::from_matrix_translation(
            &Mat4::from_rows(&[
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
    fn test_projective() -> Result<(), Box<dyn Error>> {
        let projective = Proj2A::<i32>::from_rows(&[
            Vec3A::new(1, 2, 3),
            Vec3A::new(4, 5, 6),
            Vec3A::new(7, 8, 9),
        ]);
        assert_eq!(projective, from_str(&to_string(&projective)?)?);
        assert_eq!(projective.unalign(), from_str(&to_string(&projective)?)?);
        assert!(from_str::<Proj3A<i32>>(&to_string(&projective)?).is_err());
        assert!(from_str::<Proj3<i32>>(&to_string(&projective)?).is_err());

        let projective = Proj3A::<i32>::from_rows(&[
            Vec4A::new(1, 2, 3, 4),
            Vec4A::new(5, 6, 7, 8),
            Vec4A::new(9, 10, 11, 12),
            Vec4A::new(13, 14, 15, 16),
        ]);
        assert_eq!(projective, from_str(&to_string(&projective)?)?);
        assert_eq!(projective.unalign(), from_str(&to_string(&projective)?)?);
        assert!(from_str::<Proj2A<i32>>(&to_string(&projective)?).is_err());
        assert!(from_str::<Proj2<i32>>(&to_string(&projective)?).is_err());

        let projective =
            Proj2::<i32>::from_rows(&[Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(7, 8, 9)]);
        assert_eq!(projective, from_str(&to_string(&projective)?)?);
        assert_eq!(projective.align(), from_str(&to_string(&projective)?)?);
        assert!(from_str::<Proj3A<i32>>(&to_string(&projective)?).is_err());
        assert!(from_str::<Proj3<i32>>(&to_string(&projective)?).is_err());

        let projective = Proj3::<i32>::from_rows(&[
            Vec4::new(1, 2, 3, 4),
            Vec4::new(5, 6, 7, 8),
            Vec4::new(9, 10, 11, 12),
            Vec4::new(13, 14, 15, 16),
        ]);
        assert_eq!(projective, from_str(&to_string(&projective)?)?);
        assert_eq!(projective.align(), from_str(&to_string(&projective)?)?);
        assert!(from_str::<Proj2A<i32>>(&to_string(&projective)?).is_err());
        assert!(from_str::<Proj2<i32>>(&to_string(&projective)?).is_err());

        Ok(())
    }

    #[test]
    fn test_rotation2() -> Result<(), Box<dyn Error>> {
        let rotation = Rot2::<i32>::from_cos_sin(5, 6);
        assert_eq!(rotation, from_str(&to_string(&rotation)?)?);
        assert_eq!(rotation.align(), from_str(&to_string(&rotation)?)?);

        Ok(())
    }

    #[test]
    fn test_rotor() -> Result<(), Box<dyn Error>> {
        let rotor = Rotor3::<i32>::from_elements(1, 2, 3, 4);
        assert_eq!(rotor, from_str(&to_string(&rotor)?)?);
        assert_eq!(rotor.align(), from_str(&to_string(&rotor)?)?);

        let rotor = Rotor3A::<i32>::from_elements(1, 2, 3, 4);
        assert_eq!(rotor, from_str(&to_string(&rotor)?)?);
        assert_eq!(rotor.unalign(), from_str(&to_string(&rotor)?)?);

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
