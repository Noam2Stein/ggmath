use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, One, Scalar, Unaligned, Vector, Zero,
    length::TwoOrThree,
    utils::{specialize_23, transmute_mut, transmute_ref},
};

// The contents of the `generic` submodule *would* be simply put in this root
// module, but due to a rustdoc bug, that would cause functionality generic over
// `T` to be shown after all submodule functionality.

mod generic;

mod float;

#[cfg(feature = "wide")]
mod wide;

#[cfg(feature = "wide")]
mod wide_float;

/// A rotor used to represent rotation.
///
/// A rotor is a mathematical object used to represent rotations. In comparison
/// to rotation matrices and Euler angles, rotors are more compact and
/// efficient, and avoid common issues in 3D, such as the infamous gimbal lock.
///
/// If you are familiar with quaternions, you already know how to use rotors.
/// Rotors work the same way as quaternions, resolve to the same math, have
/// equal performance, etc. However rotors tend to be easier to understand, and
/// extend better to 2D.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use `rotor.normalize()` to maintain precision.
///
/// # Type aliases
///
/// - [`Rot2<T>`] for [`Rotor<2, T, Unaligned>`].
/// - [`Rot3<T>`] for [`Rotor<3, T, Unaligned>`].
/// - [`Rot2A<T>`] for [`Rotor<2, T, Aligned>`].
/// - [`Rot3A<T>`] for [`Rotor<3, T, Aligned>`].
///
/// # Representation and Fields
///
/// Unless you are familiar with rotor/quaternion math, avoid using raw rotor
/// elements directly. Instead, use higher level helper functions.
///
/// > You may have an easier time reading documentation specific to
/// > [2D](Rot2#representation-and-fields) and
/// > [3D](Rot3#representation-and-fields) first. This section explains the
/// > representation in a dimension agnostic manner.
///
/// A rotor stores one value for each basis plane of rotation, followed by a
/// scalar value `s`, which is always last in memory.
///
/// In 2D there is only one plane of rotation, `xy`. In 3D there are three
/// basis planes of rotation, `xy, xz, yz`. If additional dimensions are ever
/// supported, the pattern would continue with `N choose 2` basis planes, signs
/// determined by increasing index order, and planes stored in lexicographical
/// order. For example, in 4D this would be `xy, xz, xw, yz, yw, zw`.
///
/// Plane elements store `plane_of_rotation * sin(angle/2)`, and `s` stores
/// `cos(angle/2)`. Even though the half angle looks odd, its benefits are what
/// make rotors a useful and efficient object.
///
/// > In advanced Geometric Algebra terms, the precise definition of this type
/// > is `R = e^(B/2)`, with vector multiplication `R~vR`. This differs from the
/// > traditional rotor convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// # Memory layout
///
/// [`Rotor<2, T, A>`] is a transparent wrapper around [`Vector<2, T, A>`], and
/// [`Rotor<3, T, A>`] is a transparent wrapper around [`Vector<4, T, A>`].
///
/// If additional dimensions are ever supported, [`Rotor<N, T, A>`] would remain
/// a transparent wrapper around [`Vector<rotor_len(N), T, A>`], where
/// `rotor_len(N) = (N choose 2) + 1`.
#[expect(private_bounds)]
pub struct Rotor<const N: usize, T, A: Alignment>(
    pub(crate) <Length<N> as TwoOrThree>::Select<Vector<2, T, A>, Vector<4, T, A>>,
)
where
    Length<N>: TwoOrThree,
    T: Scalar;

/// A 2D rotor used to represent 2D rotation.
///
/// A rotor is a mathematical object used to represent rotations. In comparison
/// to rotation matrices and Euler angles, rotors are more compact and
/// efficient, and avoid common issues in 3D, such as the infamous gimbal lock.
///
/// If you are familiar with quaternions, you already know how to use rotors.
/// Rotors work the same way as quaternions, resolve to the same math, have
/// equal performance, etc. However rotors tend to be easier to understand, and
/// extend better to 2D.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use `rotor.normalize()` to maintain precision.
///
/// # No SIMD alignment
///
/// [`Rot2<T>`] does not have SIMD alignment, for that use [`Rot2A<T>`].
///
/// # Representation and Fields
///
/// Unless you are familiar with rotor/quaternion math, avoid using raw rotor
/// elements directly. Instead, use higher level helper functions.
///
/// This type stores two elements, with this order in memory:
///
/// - `xy: T = sin(angle/2)`
/// - `s: T = cos(angle/2)`
///
/// Even though the half angle looks odd, its benefits are what make rotors a
/// useful and efficient object.
///
/// > In advanced Geometric Algebra terms, the precise definition of this type
/// > is `R = e^(B/2)`, with vector multiplication `R~vR`. This differs from the
/// > traditional rotor convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Rot2<T> = Rotor<2, T, Unaligned>;

/// A 3D rotor used to represent 3D rotation.
///
/// A rotor is a mathematical object used to represent rotations. In comparison
/// to rotation matrices and Euler angles, rotors are more compact and
/// efficient, and avoid common issues in 3D, such as the infamous gimbal lock.
///
/// If you are familiar with quaternions, you already know how to use rotors.
/// Rotors work the same way as quaternions, resolve to the same math, have
/// equal performance, etc. However rotors tend to be easier to understand, and
/// extend better to 2D.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use `rotor.normalize()` to maintain precision.
///
/// # No SIMD alignment
///
/// [`Rot3<T>`] does not have SIMD alignment, for that use [`Rot3A<T>`].
///
/// # Representation and Fields
///
/// Unless you are familiar with rotor/quaternion math, avoid using raw rotor
/// elements directly. Instead, use higher level helper functions.
///
/// This type stores four elements, with this order in memory:
///
/// - `xy: T = plane_of_rotation.xy * sin(angle/2)`
/// - `xz: T = plane_of_rotation.xz * sin(angle/2)`
/// - `yz: T = plane_of_rotation.yz * sin(angle/2)`
/// - `s: T = cos(angle/2)`
///
/// Each plane element rotates one axis to another (e.g., `xy` rotates `+X` to
/// `+Y`). This representation uses lexicographical ordering `xy, xz, yz`, which
/// differs from right-hand rule conventions that might expect `yz, zx, xy`
/// (which is `x, y, z` in axis angle notation). Even though the half angle
/// looks odd, its benefits are what make rotors a useful and efficient object.
///
/// > In advanced Geometric Algebra terms, the precise definition of this type
/// > is `R = e^(B/2)`, with vector multiplication `R~vR`. This differs from the
/// > traditional rotor convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Rot3<T> = Rotor<3, T, Unaligned>;

/// A 2D rotor used to represent 2D rotation.
///
/// A rotor is a mathematical object used to represent rotations. In comparison
/// to rotation matrices and Euler angles, rotors are more compact and
/// efficient, and avoid common issues in 3D, such as the infamous gimbal lock.
///
/// If you are familiar with quaternions, you already know how to use rotors.
/// Rotors work the same way as quaternions, resolve to the same math, have
/// equal performance, etc. However rotors tend to be easier to understand, and
/// extend better to 2D.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use `rotor.normalize()` to maintain precision.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Rot2A<T>`] has SIMD alignment. For no SIMD use
/// [`Rot2<T>`].
///
/// # Representation and Fields
///
/// Unless you are familiar with rotor/quaternion math, avoid using raw rotor
/// elements directly. Instead, use higher level helper functions.
///
/// This type stores two elements, with this order in memory:
///
/// - `xy: T = sin(angle/2)`
/// - `s: T = cos(angle/2)`
///
/// Even though the half angle looks odd, its benefits are what make rotors a
/// useful and efficient object.
///
/// > In advanced Geometric Algebra terms, the precise definition of this type
/// > is `R = e^(B/2)`, with vector multiplication `R~vR`. This differs from the
/// > traditional rotor convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Rot2A<T> = Rotor<2, T, Aligned>;

/// A 3D rotor used to represent 3D rotation.
///
/// A rotor is a mathematical object used to represent rotations. In comparison
/// to rotation matrices and Euler angles, rotors are more compact and
/// efficient, and avoid common issues in 3D, such as the infamous gimbal lock.
///
/// If you are familiar with quaternions, you already know how to use rotors.
/// Rotors work the same way as quaternions, resolve to the same math, have
/// equal performance, etc. However rotors tend to be easier to understand, and
/// extend better to 2D.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use `rotor.normalize()` to maintain precision.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Rot3A<T>`] has SIMD alignment. For no SIMD use
/// [`Rot3<T>`].
///
/// # Representation and Fields
///
/// Unless you are familiar with rotor/quaternion math, avoid using raw rotor
/// elements directly. Instead, use higher level helper functions.
///
/// This type stores four elements, with this order in memory:
///
/// - `xy: T = plane_of_rotation.xy * sin(angle/2)`
/// - `xz: T = plane_of_rotation.xz * sin(angle/2)`
/// - `yz: T = plane_of_rotation.yz * sin(angle/2)`
/// - `s: T = cos(angle/2)`
///
/// Each plane element rotates one axis to another (e.g., `xy` rotates `+X` to
/// `+Y`). This representation uses lexicographical ordering `xy, xz, yz`, which
/// differs from right-hand rule conventions that might expect `yz, zx, xy`
/// (which is `x, y, z` in axis angle notation). Even though the half angle
/// looks odd, its benefits are what make rotors a useful and efficient object.
///
/// > In advanced Geometric Algebra terms, the precise definition of this type
/// > is `R = e^(B/2)`, with vector multiplication `R~vR`. This differs from the
/// > traditional rotor convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Rot3A<T> = Rotor<3, T, Aligned>;

impl<const N: usize, T, A: Alignment> Clone for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T, A: Alignment> Copy for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar,
{
}

#[doc(hidden)]
#[repr(C)]
pub struct Rot2Fields<T> {
    /// The first and only basis plane element, rotating `+X` to `+Y`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    ///
    /// Equal to `sin(angle/2)`.
    pub xy: T,
    /// The scalar part of a rotor.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    ///
    /// Equal to `cos(angle/2)`.
    pub s: T,
}

impl<T, A: Alignment> Deref for Rotor<2, T, A>
where
    T: Scalar,
{
    type Target = Rot2Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Rotor<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `T`, and so begin with `Rot2Fields<T>`.
        unsafe { transmute_ref::<Rotor<2, T, A>, Rot2Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Rotor<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Rotor<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `T`, and so begin with `Rot2Fields<T>`.
        unsafe { transmute_mut::<Rotor<2, T, A>, Rot2Fields<T>>(self) }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Rot3Fields<T> {
    /// The first basis plane element, rotating `+X` to `+Y`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    ///
    /// Equal to `plane_of_rotation.xy * sin(angle/2)`.
    pub xy: T,
    /// The second basis plane element, rotating `+X` to `+Z`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    ///
    /// Equal to `plane_of_rotation.xz * sin(angle/2)`.
    pub xz: T,
    /// The third basis plane element, rotating `+Y` to `+Z`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    ///
    /// Equal to `plane_of_rotation.yz * sin(angle/2)`.
    pub yz: T,
    /// The scalar part of a rotor.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    ///
    /// Equal to `cos(angle/2)`.
    pub s: T,
}

impl<T, A: Alignment> Deref for Rotor<3, T, A>
where
    T: Scalar,
{
    type Target = Rot3Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Rotor<3, T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Rot3Fields<T>`.
        unsafe { transmute_ref::<Rotor<3, T, A>, Rot3Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Rotor<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Rotor<3, T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Rot3Fields<T>`.
        unsafe { transmute_mut::<Rotor<3, T, A>, Rot3Fields<T>>(self) }
    }
}

impl<const N: usize, T, A: Alignment> Debug for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        specialize_23!(Rotor::<N, T, A>::debug_backend(self, f))
    }
}

impl<const N: usize, T, A: Alignment> Display for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Display,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        specialize_23!(Rotor::<N, T, A>::display_backend(self, f))
    }
}

impl<const N: usize, T, A: Alignment> PartialEq for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        specialize_23!(Rotor::<N, T, A>::eq_backend(self, other))
    }
}

impl<const N: usize, T, A: Alignment> Eq for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Eq,
{
}

impl<const N: usize, T, A: Alignment> Hash for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Hash,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        specialize_23!(Rotor::<N, T, A>::hash_backend(self, (state,)))
    }
}

impl<const N: usize, T, A: Alignment> Default for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Zero + One,
{
    /// Returns [`IDENTITY`].
    ///
    /// [`IDENTITY`]: Self::IDENTITY
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

macro_rules! impl_neg {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Neg for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::neg_backend(self))
            }
        }

        impl<const N: usize, T, A: Alignment> Neg for &Rotor<N, T, A>
        where
        Length<N>: TwoOrThree,
        T: Scalar + Neg<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                -*self
            }
        }
    };
}
impl_neg!(
    /// Negates the sign of each element.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
);

macro_rules! impl_add {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Add for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Self) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::add_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Add<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: &Self) -> Self::Output {
                self + *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Add<Rotor<N, T, A>> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Rotor<N, T, A>) -> Self::Output {
                *self + rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Add for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Self) -> Self::Output {
                *self + *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> AddAssign for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> AddAssign<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: &Self) {
                *self = *self + rhs;
            }
        }
    };
}
impl_add!(
    /// Adds each element of a rotor to another rotor.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
);

macro_rules! impl_sub {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Sub for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Self) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::sub_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Sub<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: &Self) -> Self::Output {
                self - *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Sub<Rotor<N, T, A>> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Rotor<N, T, A>) -> Self::Output {
                *self - rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Sub for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Self) -> Self::Output {
                *self - *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> SubAssign for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> SubAssign<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: &Self) {
                *self = *self - rhs;
            }
        }
    };
}
impl_sub!(
    /// Subtracts each element of a rotor from another rotor.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
);

macro_rules! impl_mul_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::mul_scalar_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<T> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                *self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                *self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: T) {
                *self = *self * rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &T) {
                *self = *self * *rhs;
            }
        }
    };
}
impl_mul_scalar!(
    /// Multiplies every element by a uniform scalar.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_vector_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<Rotor<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Rotor<N, T, A>) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::vector_mul_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotor<N, T, A>) -> Self::Output {
                self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Rotor<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Rotor<N, T, A>) -> Self::Output {
                *self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotor<N, T, A>) -> Self::Output {
                *self * *rhs
            }
        }
    };
}
impl_vector_mul!(
    /// Transforms a vector by a rotor.
    ///
    /// If the rotor is normalized, this just rotates the vector. If the rotor
    /// is not normalized, this also scales the vector by
    /// `rotor.length_squared()`.
    ///
    /// Because the library uses left multiplication, vectors always go on the
    /// left-hand side.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Self) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::mul_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotor<N, T, A>) -> Self::Output {
                self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Rotor<N, T, A>> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Rotor<N, T, A>) -> Self::Output {
                *self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotor<N, T, A>) -> Self::Output {
                *self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Rotor<N, T, A>) {
                *self = *self * *rhs;
            }
        }
    };
}
impl_mul!(
    /// Multiplies two rotors.
    ///
    /// The resulting rotor is equivalent to first applying the left rotor, then
    /// the right rotor.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_div_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Div<T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: T) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::div_scalar_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Div<&T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: &T) -> Self::Output {
                self / *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Div<T> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Div<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: T) -> Self::Output {
                *self / rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Div<&T> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Div<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: &T) -> Self::Output {
                *self / *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> DivAssign<T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Div<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, rhs: T) {
                *self = *self / rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> DivAssign<&T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Div<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, rhs: &T) {
                *self = *self / *rhs;
            }
        }
    };
}
impl_div_scalar!(
    /// Divides every element by a uniform scalar.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

// SAFETY: Rotors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Send` the list also is, and the padding is `Send` too.
unsafe impl<const N: usize, T, A: Alignment> Send for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Send,
{
}

// SAFETY: Rotors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Sync` the list also is, and the padding is `Sync` too.
unsafe impl<const N: usize, T, A: Alignment> Sync for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Sync,
{
}

impl<const N: usize, T, A: Alignment> Unpin for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Unpin,
{
}

impl<const N: usize, T, A: Alignment> UnwindSafe for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + UnwindSafe,
{
}

impl<const N: usize, T, A: Alignment> RefUnwindSafe for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + RefUnwindSafe,
{
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        Aligned, Mask, Matrix, Rot2, Rot2A, Rot3, Rot3A, Rotor, Unaligned, Vec2A, Vec4A, Vector,
        test_utils::{assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_layout() {
        for_types!(|T: PrimitiveNumber| {
            assert_eq!(size_of::<Rot2<T>>(), size_of::<T>() * 2);
            assert_eq!(align_of::<Rot2<T>>(), align_of::<T>());

            assert_eq!(size_of::<Rot3<T>>(), size_of::<T>() * 4);
            assert_eq!(align_of::<Rot3<T>>(), align_of::<T>());

            assert_eq!(size_of::<Rot2A<T>>(), size_of::<Vec2A<T>>());
            assert_eq!(align_of::<Rot2A<T>>(), align_of::<Vec2A<T>>());

            assert_eq!(size_of::<Rot3A<T>>(), size_of::<Vec4A<T>>());
            assert_eq!(align_of::<Rot3A<T>>(), align_of::<Vec4A<T>>());
        });
    }

    #[test]
    fn test_identity() {
        for_types!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Rotor::<2, T, A>::IDENTITY,
                Rotor::<2, T, A>::new(T::ZERO, T::ONE)
            );
            assert_eq!(
                Rotor::<3, T, A>::IDENTITY,
                Rotor::<3, T, A>::new(T::ZERO, T::ZERO, T::ZERO, T::ONE)
            );
        });
    }

    #[test]
    fn test_conjugate() {
        for_types!(|T: PrimitiveFloat, A| {
            for rotor in random_iter::<Rotor<2, T, A>>() {
                assert_test_eq!(rotor.conjugate(), Rotor::<2, T, A>::new(-rotor.xy, rotor.s));
            }
            for rotor in random_iter::<Rotor<3, T, A>>() {
                assert_test_eq!(
                    rotor.conjugate(),
                    Rotor::<3, T, A>::new(-rotor.xy, -rotor.xz, -rotor.yz, rotor.s)
                );
            }
        });
    }

    #[test]
    fn test_dot() {
        for_types!(|T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Rotor<2, T, A>; 2]>() {
                assert_test_eq!(a.dot(b), a.0.dot(b.0));
            }
            for [a, b] in random_iter::<[Rotor<3, T, A>; 2]>() {
                assert_test_eq!(a.dot(b), a.0.dot(b.0));
            }
        });
    }

    #[test]
    fn test_length_squared() {
        for_types!(|T: PrimitiveFloat, A| {
            for rotor in random_iter::<Rotor<2, T, A>>() {
                assert_test_eq!(rotor.length_squared(), rotor.0.length_squared());
            }
            for rotor in random_iter::<Rotor<3, T, A>>() {
                assert_test_eq!(rotor.length_squared(), rotor.0.length_squared());
            }
        });
    }

    #[test]
    fn test_to_alignment() {
        for_types!(|T: PrimitiveNumber, A| {
            let rotor = Rotor::<2, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                rotor.to_alignment(),
                Rotor::<2, T, Aligned>::from_vector(rotor.0.align())
            );
            assert_eq!(
                rotor.to_alignment(),
                Rotor::<2, T, Unaligned>::from_vector(rotor.0.unalign())
            );

            let rotor = Rotor::<3, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                rotor.to_alignment(),
                Rotor::<3, T, Aligned>::from_vector(rotor.0.align())
            );
            assert_eq!(
                rotor.to_alignment(),
                Rotor::<3, T, Unaligned>::from_vector(rotor.0.unalign())
            );
        });
    }

    #[test]
    fn test_align() {
        for_types!(|T: PrimitiveNumber, A| {
            let rotor = Rotor::<2, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                rotor.align(),
                Rotor::<2, T, Aligned>::from_vector(rotor.0.align())
            );

            let rotor = Rotor::<3, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                rotor.align(),
                Rotor::<3, T, Aligned>::from_vector(rotor.0.align())
            );
        });
    }

    #[test]
    fn test_unalign() {
        for_types!(|T: PrimitiveNumber, A| {
            let rotor = Rotor::<2, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                rotor.unalign(),
                Rotor::<2, T, Unaligned>::from_vector(rotor.0.unalign())
            );

            let rotor = Rotor::<3, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                rotor.unalign(),
                Rotor::<3, T, Unaligned>::from_vector(rotor.0.unalign())
            );
        });
    }

    #[test]
    fn test_deref() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            let rotor = Rotor::<2, T, A>::new(x, y);
            assert_eq!(rotor.xy, x);
            assert_eq!(rotor.s, y);

            let rotor = Rotor::<3, T, A>::new(x, y, z, w);
            assert_eq!(rotor.xy, x);
            assert_eq!(rotor.xz, y);
            assert_eq!(rotor.yz, z);
            assert_eq!(rotor.s, w);
        });
    }

    #[test]
    fn test_deref_mut() {
        for_types!(|T: PrimitiveNumber, A| {
            let [mut x, mut y, mut z, mut w] = std::array::from_fn(|i| T::as_from(i + 1));

            let mut rotor = Rotor::<2, T, A>::new(x, y);
            assert_eq!(&mut rotor.xy, &mut x);
            assert_eq!(&mut rotor.s, &mut y);

            let mut rotor = Rotor::<3, T, A>::new(x, y, z, w);
            assert_eq!(&mut rotor.xy, &mut x);
            assert_eq!(&mut rotor.xz, &mut y);
            assert_eq!(&mut rotor.yz, &mut z);
            assert_eq!(&mut rotor.s, &mut w);
        });
    }

    #[test]
    fn test_debug() {
        for_types!(|T: PrimitiveNumber, A| {
            let rotor = Rotor::<2, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(format!("{rotor:?}"), format!("{:?}", rotor.to_vector()));

            let rotor = Rotor::<3, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(format!("{rotor:?}"), format!("{:?}", rotor.to_vector()));
        });
    }

    #[test]
    fn test_display() {
        for_types!(|T: PrimitiveNumber, A| {
            let rotor = Rotor::<2, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(format!("{rotor}"), format!("{}", rotor.to_vector()));

            let rotor = Rotor::<3, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(format!("{rotor}"), format!("{}", rotor.to_vector()));
        });
    }

    #[test]
    fn test_eq() {
        for_types!(|T: PrimitiveNumber, A| {
            for ([rotor, other], mask) in random_iter::<([Rotor<2, T, A>; 2], Mask<2, T, A>)>() {
                let other = Rotor(mask.select(rotor.0, other.0));
                assert_eq!(rotor == other, rotor.as_vector() == other.as_vector());
            }
            for ([rotor, other], mask) in random_iter::<([Rotor<3, T, A>; 2], Mask<4, T, A>)>() {
                let other = Rotor(mask.select(rotor.0, other.0));
                assert_eq!(rotor == other, rotor.as_vector() == other.as_vector());
            }
        });
    }

    #[test]
    fn test_ne() {
        for_types!(|T: PrimitiveNumber, A| {
            for ([rotor, other], mask) in random_iter::<([Rotor<2, T, A>; 2], Mask<2, T, A>)>() {
                let other = Rotor(mask.select(rotor.0, other.0));
                assert_eq!(rotor != other, rotor.as_vector() != other.as_vector());
            }
            for ([rotor, other], mask) in random_iter::<([Rotor<3, T, A>; 2], Mask<4, T, A>)>() {
                let other = Rotor(mask.select(rotor.0, other.0));
                assert_eq!(rotor != other, rotor.as_vector() != other.as_vector());
            }
        });
    }

    #[test]
    fn test_default() {
        for_types!(|N: TwoOrThree, T: PrimitiveNumber, A| {
            assert_eq!(Rotor::<N, T, A>::default(), Rotor::IDENTITY);
        });
    }

    #[test]
    fn test_neg() {
        for_types!(|T: PrimitiveFloat, A| {
            for rotor in random_iter::<Rotor<2, T, A>>() {
                assert_test_eq!(-rotor, Rotor::<2, T, A>::new(-rotor.xy, -rotor.s));
            }
            for rotor in random_iter::<Rotor<3, T, A>>() {
                assert_test_eq!(
                    -rotor,
                    Rotor::<3, T, A>::new(-rotor.xy, -rotor.xz, -rotor.yz, -rotor.s)
                );
            }
        });
    }

    #[test]
    fn test_add() {
        for_types!(|T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Rotor<2, T, A>; 2]>() {
                assert_test_eq!(a + b, Rotor::<2, T, A>::new(a.xy + b.xy, a.s + b.s));
            }
            for [a, b] in random_iter::<[Rotor<3, T, A>; 2]>() {
                assert_test_eq!(
                    a + b,
                    Rotor::<3, T, A>::new(a.xy + b.xy, a.xz + b.xz, a.yz + b.yz, a.s + b.s)
                );
            }
        });
    }

    #[test]
    fn test_sub() {
        for_types!(|T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Rotor<2, T, A>; 2]>() {
                assert_test_eq!(a - b, Rotor::<2, T, A>::new(a.xy - b.xy, a.s - b.s));
            }
            for [a, b] in random_iter::<[Rotor<3, T, A>; 2]>() {
                assert_test_eq!(
                    a - b,
                    Rotor::<3, T, A>::new(a.xy - b.xy, a.xz - b.xz, a.yz - b.yz, a.s - b.s)
                );
            }
        });
    }

    #[test]
    fn test_mul_scalar() {
        for_types!(|T: PrimitiveFloat, A| {
            for (rotor, scalar) in random_iter::<(Rotor<2, T, A>, T)>() {
                assert_test_eq!(
                    rotor * scalar,
                    Rotor::<2, T, A>::new(rotor.xy * scalar, rotor.s * scalar)
                );
            }
            for (rotor, scalar) in random_iter::<(Rotor<3, T, A>, T)>() {
                assert_test_eq!(
                    rotor * scalar,
                    Rotor::<3, T, A>::new(
                        rotor.xy * scalar,
                        rotor.xz * scalar,
                        rotor.yz * scalar,
                        rotor.s * scalar
                    )
                );
            }
        });
    }

    #[test]
    fn test_vector_mul() {
        for_types!(|N: TwoOrThree, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                if vector.is_finite() {
                    assert_test_eq!(vector * Rotor::IDENTITY, vector, 0.0 = -0.0);
                }
            }
        });
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<2, T, A>, T)>() {
                let angle = angle % 6.0;
                let (half_sin, half_cos) = (angle / 2.0).sin_cos();
                assert_test_eq!(
                    vector * Rotor::<2, T, A>::new(half_sin, half_cos),
                    vector.rotate(angle),
                    abs <= vector.length() * 1e-5 + 1e-5,
                    0.0 = -0.0
                );
            }

            for (vector, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                let angle = angle % 6.0;
                let (half_sin, half_cos) = (angle / 2.0).sin_cos();
                if vector.is_finite() && half_sin.is_finite() && half_cos.is_finite() {
                    assert_test_eq!(
                        vector * Rotor::<3, T, A>::new(half_sin, 0.0, 0.0, half_cos),
                        vector.rotate_xy(angle),
                        abs <= vector.length() * 1e-5 + 1e-5,
                        0.0 = -0.0
                    );
                    assert_test_eq!(
                        vector * Rotor::<3, T, A>::new(0.0, half_sin, 0.0, half_cos),
                        vector.rotate_xz(angle),
                        abs <= vector.length() * 1e-5 + 1e-5,
                        0.0 = -0.0
                    );
                    assert_test_eq!(
                        vector * Rotor::<3, T, A>::new(0.0, 0.0, half_sin, half_cos),
                        vector.rotate_yz(angle),
                        abs <= vector.length() * 1e-5 + 1e-5,
                        0.0 = -0.0
                    );
                }
            }

            for (vector, rotor) in [
                (
                    Vector::<3, T, A>::new(-4.1, 3.3, 10.3),
                    Rotor::<3, T, A>::new(0.8, 0.4, 0.3, 0.1),
                ),
                (
                    Vector::<3, T, A>::new(-4.1, 3.3, 10.3),
                    Rotor::<3, T, A>::IDENTITY,
                ),
            ]
            .into_iter()
            .chain(random_iter())
            {
                if !vector.is_finite() {
                    continue;
                }

                let rotor = rotor.normalize_or(Rotor::<3, T, A>::IDENTITY).normalize();
                let matrix = Matrix::<3, T, A>::from_rotor(rotor);

                assert_test_eq!(
                    vector * rotor,
                    vector * matrix,
                    abs <= vector.abs().max_element() * 1e-6 + 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_mul() {
        for_types!(|N: TwoOrThree, T: PrimitiveFloat, A| {
            for (vector, [rotor_1, rotor_2]) in
                random_iter::<(Vector<N, T, A>, [Rotor<N, T, A>; 2])>()
            {
                if !vector.is_finite() || vector.length() > 1e5 {
                    continue;
                }

                let [rotor_1, rotor_2] =
                    [rotor_1, rotor_2].map(|r| r.normalize_or(Rotor::IDENTITY).normalize());

                assert_test_eq!(
                    vector * (rotor_1 * rotor_2),
                    vector * rotor_1 * rotor_2,
                    abs <= vector.length() * 1e-6 + 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_div_scalar() {
        for_types!(|T: PrimitiveFloat, A| {
            for (rotor, scalar) in random_iter::<(Rotor<2, T, A>, T)>() {
                assert_test_eq!(
                    rotor / scalar,
                    Rotor::<2, T, A>::new(rotor.xy / scalar, rotor.s / scalar)
                );
            }
            for (rotor, scalar) in random_iter::<(Rotor<3, T, A>, T)>() {
                assert_test_eq!(
                    rotor / scalar,
                    Rotor::<3, T, A>::new(
                        rotor.xy / scalar,
                        rotor.xz / scalar,
                        rotor.yz / scalar,
                        rotor.s / scalar
                    )
                );
            }
        });
    }
}
