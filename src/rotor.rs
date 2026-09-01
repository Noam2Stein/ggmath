use core::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, One, Scalar, Unaligned, Vector, Zero,
    backend::RotorBackend,
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

/// A rotor representing rotation.
///
/// A rotor is a mathematical object used to represent rotations. In comparison
/// to rotation matrices, rotors are more compact and are easier to blend.
/// Rotors are basically identical to quaternions, which you may be familiar
/// with, but tend to be easier to understand, and extend better to dimensions
/// other than 3D.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use [`rotor.normalize()`] to maintain precision.
///
/// # Type aliases
///
/// - [`Rotor2<T>`] for [`Rotor<2, T, Unaligned>`].
/// - [`Rotor3<T>`] for [`Rotor<3, T, Unaligned>`].
/// - [`Rotor2A<T>`] for [`Rotor<2, T, Aligned>`].
/// - [`Rotor3A<T>`] for [`Rotor<3, T, Aligned>`].
///
/// # Representation and Fields
///
/// Unless you are familiar with rotor/quaternion math, avoid using raw rotor
/// elements directly. Instead, use higher level helper functions. You may have
/// an easier time reading documentation specific to
/// [2D](Rotor2#representation-and-fields) and
/// [3D](Rotor3#representation-and-fields) first. This section explains rotor
/// representation in a dimension agnostic manner, and uses advance Geometric
/// Algebra terms.
///
/// This type stores all multivector elements that have an even grade, with
/// signs determined by increasing index order, with the scalar element being
/// last, and other elements ordered by increasing grade then lexicographically.
///
/// - In 2D, this is: `xy, s`
/// - In 3D, this is: `xy, xz, yz, s`
/// - In 4D, this is: `xy, xz, xw, yz, yw, zw, xyzw, s`
///
/// > The scalar element being last enables an optimization where getting the
/// > bivector of a [`Rotor3A`] is a no-op.
///
/// This type uses the sign convention `R = e^(B/2)`, with vector multiplication
/// `R~vR`. This differs from the traditional convention, `R = e^(-B/2)` and
/// `RvR~`.
///
/// Fields are exposed by implementing [`Deref`] and [`DerefMut`].
///
/// # Memory layout
///
/// [`Rotor<2, T, A>`] is a transparent wrapper around [`Vector<2, T, A>`], and
/// [`Rotor<3, T, A>`] is a transparent wrapper around [`Vector<4, T, A>`].
///
/// If additional dimensions are ever supported, [`Rotor<N, T, A>`] would remain
/// a transparent wrapper around [`Vector<rotor_len(N), T, A>`], where
/// `rotor_len(n) = sum((0..=n).step_by(2).map(|k| (n choose k)))`.
///
/// [`rotor.normalize()`]: Rotor#method.normalize
#[expect(private_bounds)]
pub struct Rotor<const N: usize, T, A: Alignment>(
    pub(crate) <Length<N> as TwoOrThree>::Select<Vector<2, T, A>, Vector<4, T, A>>,
)
where
    Length<N>: TwoOrThree,
    T: Scalar;

/// A 2D rotor representing 2D rotation.
///
/// A rotor is a mathematical object used to represent rotations. In comparison
/// to rotation matrices, rotors are more compact and are easier to blend.
/// Rotors are basically identical to quaternions, which you may be familiar
/// with, but tend to be easier to understand, and extend better to dimensions
/// other than 3D.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use [`rotor.normalize()`] to maintain precision.
///
/// # No SIMD alignment
///
/// [`Rotor2<T>`] does not have SIMD alignment, for that use [`Rotor2A<T>`].
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
/// Even though we could instead store `sin(angle), cos(angle)`, the half-angle
/// has benefits even in 2D that make it worth the slight performance overhead.
///
/// > In advanced Geometric Algebra terms, the precise definition of this type
/// > is `R = e^(B/2)`, with vector multiplication `R~vR`. This differs from the
/// > traditional rotor convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`rotor.normalize()`]: Rotor#method.normalize
pub type Rotor2<T> = Rotor<2, T, Unaligned>;

/// A 3D rotor representing 3D rotation.
///
/// A rotor is a mathematical object used to represent rotations. In comparison
/// to rotation matrices, rotors are more compact and are easier to blend.
/// Rotors are basically identical to quaternions, which you may be familiar
/// with, but tend to be easier to understand, and extend better to dimensions
/// other than 3D.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use [`rotor.normalize()`] to maintain precision.
///
/// # No SIMD alignment
///
/// [`Rotor3<T>`] does not have SIMD alignment, for that use [`Rotor3A<T>`].
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
/// (which is `x, y, z` in axis-angle notation). Even though the half-angle
/// looks odd, its benefits are what make rotors a useful and efficient object.
///
/// > In advanced Geometric Algebra terms, the precise definition of this type
/// > is `R = e^(B/2)`, with vector multiplication `R~vR`. This differs from the
/// > traditional rotor convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`rotor.normalize()`]: Rotor#method.normalize
pub type Rotor3<T> = Rotor<3, T, Unaligned>;

/// A 2D rotor representing 2D rotation.
///
/// A rotor is a mathematical object used to represent rotations. In comparison
/// to rotation matrices, rotors are more compact and are easier to blend.
/// Rotors are basically identical to quaternions, which you may be familiar
/// with, but tend to be easier to understand, and extend better to dimensions
/// other than 3D.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use [`rotor.normalize()`] to maintain precision.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Rotor2A<T>`] has SIMD alignment. For no SIMD
/// use [`Rotor2<T>`].
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
/// Even though we could instead store `sin(angle), cos(angle)`, the half-angle
/// has benefits even in 2D that make it worth the slight performance overhead.
///
/// > In advanced Geometric Algebra terms, the precise definition of this type
/// > is `R = e^(B/2)`, with vector multiplication `R~vR`. This differs from the
/// > traditional rotor convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`rotor.normalize()`]: Rotor#method.normalize
pub type Rotor2A<T> = Rotor<2, T, Aligned>;

/// A 3D rotor representing 3D rotation.
///
/// A rotor is a mathematical object used to represent rotations. In comparison
/// to rotation matrices, rotors are more compact and are easier to blend.
/// Rotors are basically identical to quaternions, which you may be familiar
/// with, but tend to be easier to understand, and extend better to dimensions
/// other than 3D.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use [`rotor.normalize()`] to maintain precision.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Rotor3A<T>`] has SIMD alignment. For no SIMD
/// use [`Rotor3<T>`].
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
/// (which is `x, y, z` in axis-angle notation). Even though the half-angle
/// looks odd, its benefits are what make rotors a useful and efficient object.
///
/// > In advanced Geometric Algebra terms, the precise definition of this type
/// > is `R = e^(B/2)`, with vector multiplication `R~vR`. This differs from the
/// > traditional rotor convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`rotor.normalize()`]: Rotor#method.normalize
pub type Rotor3A<T> = Rotor<3, T, Aligned>;

impl<T, A: Alignment> Rotor<2, T, A>
where
    T: Scalar,
{
    #[inline(always)]
    fn debug_backend(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    where
        T: Debug,
    {
        f.debug_struct("Rotor")
            .field("xy", &self.xy)
            .field("s", &self.s)
            .finish()
    }

    #[inline(always)]
    fn eq_backend(&self, other: &Self) -> bool
    where
        T: PartialEq,
    {
        self.0 == other.0
    }

    #[inline(always)]
    fn hash_backend<H: core::hash::Hasher>(&self, (state,): (&mut H,))
    where
        T: Hash,
    {
        self.0.hash(state);
    }

    #[inline(always)]
    #[track_caller]
    fn neg_backend(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self(-self.0)
    }

    #[inline(always)]
    #[track_caller]
    fn add_backend(self, rhs: Self) -> Self
    where
        T: Add<Output = T>,
    {
        Self(self.0 + rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    fn sub_backend(self, rhs: Self) -> Self
    where
        T: Sub<Output = T>,
    {
        Self(self.0 - rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    fn mul_scalar_backend(self, rhs: T) -> Self
    where
        T: Mul<Output = T>,
    {
        Self(self.0 * rhs)
    }

    #[inline(always)]
    #[track_caller]
    fn div_scalar_backend(self, rhs: T) -> Self
    where
        T: Div<Output = T>,
    {
        Self(self.0 / rhs)
    }
}

impl<T, A: Alignment> Rotor<3, T, A>
where
    T: Scalar,
{
    #[inline(always)]
    fn debug_backend(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    where
        T: Debug,
    {
        f.debug_struct("Rotor")
            .field("xy", &self.xy)
            .field("xz", &self.xz)
            .field("yz", &self.yz)
            .field("s", &self.s)
            .finish()
    }

    #[inline(always)]
    fn eq_backend(&self, other: &Self) -> bool
    where
        T: PartialEq,
    {
        self.0 == other.0
    }

    #[inline(always)]
    fn hash_backend<H: core::hash::Hasher>(&self, (state,): (&mut H,))
    where
        T: Hash,
    {
        self.0.hash(state);
    }

    #[inline(always)]
    #[track_caller]
    fn neg_backend(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self(-self.0)
    }

    #[inline(always)]
    #[track_caller]
    fn add_backend(self, rhs: Self) -> Self
    where
        T: Add<Output = T>,
    {
        Self(self.0 + rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    fn sub_backend(self, rhs: Self) -> Self
    where
        T: Sub<Output = T>,
    {
        Self(self.0 - rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    fn mul_scalar_backend(self, rhs: T) -> Self
    where
        T: Mul<Output = T>,
    {
        Self(self.0 * rhs)
    }

    #[inline(always)]
    #[track_caller]
    fn div_scalar_backend(self, rhs: T) -> Self
    where
        T: Div<Output = T>,
    {
        Self(self.0 / rhs)
    }
}

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
    /// The basis plane element of a rotor, rotating `+X` to `+Y`.
    ///
    /// Equal to `sin(angle/2)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    pub xy: T,
    /// The scalar part of a rotor.
    ///
    /// Equal to `cos(angle/2)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
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
    /// The first basis plane element of a rotor, rotating `+X` to `+Y`.
    ///
    /// Equal to `plane_of_rotation.xy * sin(angle/2)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    pub xy: T,
    /// The second basis plane element of a rotor, rotating `+X` to `+Z`.
    ///
    /// Equal to `plane_of_rotation.xz * sin(angle/2)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    pub xz: T,
    /// The third basis plane element of a rotor, rotating `+Y` to `+Z`.
    ///
    /// Equal to `plane_of_rotation.yz * sin(angle/2)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    pub yz: T,
    /// The scalar part of a rotor.
    ///
    /// Equal to `cos(angle/2)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
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
    /// Negates the elements of a rotor.
    ///
    /// The resulting rotor still represents the same rotation. To invert a
    /// rotation, use [`rotor.inverse()`].
    ///
    /// [`rotor.inverse()`]: Rotor#method.inverse
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
    /// Adds together the elements of two rotors.
    ///
    /// The resulting rotor most likely does not represent a valid rotation.
    /// Only use this if you know what you are doing. If you want to "chain"
    /// two rotations, use `rotor * rotor` instead.
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
    /// Performs subtraction for the elements of two rotors.
    ///
    /// The resulting rotor most likely does not represent a valid rotation.
    /// Only use this if you know what you are doing.
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
    /// Multiplies each element of a rotor by a scalar.
    ///
    /// The resulting rotor most likely does not represent a valid rotation.
    /// Only use this if you know what you are doing.
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
                specialize_23!(<T as RotorBackend<N, A>>::rotor_vector_mul(self, rhs))
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
    /// If the rotor is normalized, this applies rotation. If the rotor is not
    /// normalized, this also scales the vector by `rotor.length_squared()`.
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
                specialize_23!(<T as RotorBackend<N, A>>::rotor_mul(self, rhs))
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
    /// Multiplies two rotors together, resulting in a rotor equivalent to first
    /// applying the left rotor then applying the right rotor.
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
    /// Divides each element of a rotor by a scalar.
    ///
    /// The resulting rotor most likely does not represent a valid rotation.
    /// Only use this if you know what you are doing.
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

    use crate::{Rotor2, Rotor2A, Rotor3, Rotor3A, Vec2A, Vec4A, test_utils::for_types};

    #[test]
    fn test_layout() {
        for_types!(|T: PrimitiveNumber| {
            assert_eq!(size_of::<Rotor2<T>>(), size_of::<T>() * 2);
            assert_eq!(align_of::<Rotor2<T>>(), align_of::<T>());

            assert_eq!(size_of::<Rotor3<T>>(), size_of::<T>() * 4);
            assert_eq!(align_of::<Rotor3<T>>(), align_of::<T>());

            assert_eq!(size_of::<Rotor2A<T>>(), size_of::<Vec2A<T>>());
            assert_eq!(align_of::<Rotor2A<T>>(), align_of::<Vec2A<T>>());

            assert_eq!(size_of::<Rotor3A<T>>(), size_of::<Vec4A<T>>());
            assert_eq!(align_of::<Rotor3A<T>>(), align_of::<Vec4A<T>>());
        });
    }

    #[test]
    fn test_identity() {
        todo!()
    }

    #[test]
    fn test_conjugate() {
        todo!()
    }

    #[test]
    fn test_dot() {
        todo!()
    }

    #[test]
    fn test_length_squared() {
        todo!()
    }

    #[test]
    fn test_to_alignment() {
        todo!()
    }

    #[test]
    fn test_align() {
        todo!()
    }

    #[test]
    fn test_unalign() {
        todo!()
    }

    #[test]
    fn test_deref() {
        todo!()
    }

    #[test]
    fn test_deref_mut() {
        todo!()
    }

    #[test]
    fn test_debug() {
        todo!()
    }

    #[test]
    fn test_eq() {
        todo!()
    }

    #[test]
    fn test_ne() {
        todo!()
    }

    #[test]
    fn test_default() {
        todo!()
    }

    #[test]
    fn test_neg() {
        todo!()
    }

    #[test]
    fn test_add() {
        todo!()
    }

    #[test]
    fn test_sub() {
        todo!()
    }

    #[test]
    fn test_mul_scalar() {
        todo!()
    }

    #[test]
    fn test_vector_mul() {
        todo!()
    }

    #[test]
    fn test_mul() {
        todo!()
    }

    #[test]
    fn test_div_scalar() {
        todo!()
    }
}
