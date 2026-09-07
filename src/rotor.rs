use core::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, One, Scalar, Unaligned, Vector, Zero,
    backend::RotorBackend,
    length::Three,
    utils::{specialize_3, transmute_mut, transmute_ref},
};

// These submodules have empty lines between them so that rustfmt does not
// incorrectly reorder them. The order is important since it impacts the order
// of `impl` blocks in rustdoc's output.
//
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
/// A rotor is a mathematical object used to represent rotations. You may be
/// familiar with quaternions, which are mathematically identical to 3D rotors,
/// however rotors tend to be easier to understand, and extend better to
/// dimensions other than 3D.
///
/// In comparison to rotation matrices, rotors are more compact, are faster to
/// chain, and can be properly interpolated, making them the best type for
/// manipulating rotations. Applying a rotor on a vector is slower than applying
/// a matrix, so consider converting your rotor to a matrix before rotating a
/// lot of vectors.
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
/// - [`Rotor3<T>`] for [`Rotor<3, T, Unaligned>`].
/// - [`Rotor3A<T>`] for [`Rotor<3, T, Aligned>`].
///
/// # Representation
///
/// Unless you are familiar with rotor/quaternion math, avoid using rotor
/// elements directly. Instead, use higher level helper functions. You may find
/// it easier to read documentation specific to
/// [3D](Rotor3#representation-and-fields) first. This section explains the
/// representation in a dimension agnostic manner.
///
/// This type stores all multivector elements that have an even grade.
///
/// - In 2D, this is: `s, xy`
/// - In 3D, this is: `s, xy, xz, yz`
/// - In 4D, this is: `s, xy, xz, xw, yz, yw, zw, xyzw`
///
/// This type uses the rotor convention `R = e^(B/2)`, with vector
/// multiplication `R~vR`. This differs from the traditional convention,
/// `R = e^(-B/2)` and `RvR~`.
///
/// Note that the order of elements in memory and plane orientations (`xy`
/// versus `yx`) do not necessarily follow lexicographical ordering. They may
/// vary with `N` to enable dimension-specific optimizations. Currently, only
/// 3D rotors are supported, and their representation is `yz, zx, xy, s` rather
/// than the natural `s, xy, xz, yz`.
///
/// Fields are exposed by implementing [`Deref`] and [`DerefMut`].
///
/// # Memory Layout
///
/// [`Rotor<3, T, A>`] is a transparent wrapper around [`Vector<4, T, A>`].
///
/// If additional dimensions are ever supported, [`Rotor<N, T, A>`] would remain
/// a transparent wrapper around [`Vector<rotor_len(N), T, A>`], where
/// `rotor_len(n) = sum((0..=n).step_by(2).map(|k| (n choose k)))`.
///
/// [`rotor.normalize()`]: Rotor#method.normalize
#[repr(transparent)]
#[expect(private_bounds)]
pub struct Rotor<const N: usize, T, A: Alignment>(
    /// Currently, since only 3D rotors are supported, the internal field is
    /// always a vector4.
    ///
    /// If additional dimensions are ever supported, this will need to be
    /// changed.
    pub(crate) Vector<4, T, A>,
)
where
    Length<N>: Three,
    T: Scalar;

/// A 3D rotor representing 3D rotation.
///
/// A rotor is a mathematical object used to represent rotations. You may be
/// familiar with quaternions, which are mathematically identical to 3D rotors,
/// however rotors tend to be easier to understand, and extend better to
/// dimensions other than 3D.
///
/// In comparison to rotation matrices, rotors are more compact, are faster to
/// chain, and can be properly interpolated, making them the best type for
/// manipulating rotations. Applying a rotor on a vector is slower than applying
/// a matrix, so consider converting your rotor to a matrix before rotating a
/// lot of vectors.
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
/// # Representation
///
/// Unless you are familiar with rotor/quaternion math, avoid using rotor
/// elements directly. Instead, use higher level helper functions.
///
/// This type stores four elements, with this order in memory:
///
/// - `yz: T = plane_of_rotation.yz * sin(angle/2)`
/// - `zx: T = plane_of_rotation.zx * sin(angle/2)`
/// - `xy: T = plane_of_rotation.xy * sin(angle/2)`
/// - `s: T = cos(angle/2)`
///
/// Each plane element rotates one axis to another (e.g., `xy` rotates `+X` to
/// `+Y`).
///
/// Although the natural representation would be `s, xy, xz, yz`, this type
/// uses the right-handed dual representation `yz, zx, xy, s`. This ordering
/// and choice of signs is tailored for SIMD, allowing some operations to avoid
/// element rearrangement.
///
/// > In advanced Geometric Algebra terms, this type uses the rotor convention
/// > `R = e^(B/2)` with vector multiplication `R~vR`. This differs from the
/// > traditional convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that the fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`rotor.normalize()`]: Rotor#method.normalize
pub type Rotor3<T> = Rotor<3, T, Unaligned>;

/// A 3D rotor representing 3D rotation.
///
/// A rotor is a mathematical object used to represent rotations. You may be
/// familiar with quaternions, which are mathematically identical to 3D rotors,
/// however rotors tend to be easier to understand, and extend better to
/// dimensions other than 3D.
///
/// In comparison to rotation matrices, rotors are more compact, are faster to
/// chain, and can be properly interpolated, making them the best type for
/// manipulating rotations. Applying a rotor on a vector is slower than applying
/// a matrix, so consider converting your rotor to a matrix before rotating a
/// lot of vectors.
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
/// # Representation
///
/// Unless you are familiar with rotor/quaternion math, avoid using rotor
/// elements directly. Instead, use higher level helper functions.
///
/// This type stores four elements, with this order in memory:
///
/// - `yz: T = plane_of_rotation.yz * sin(angle/2)`
/// - `zx: T = plane_of_rotation.zx * sin(angle/2)`
/// - `xy: T = plane_of_rotation.xy * sin(angle/2)`
/// - `s: T = cos(angle/2)`
///
/// Each plane element rotates one axis to another (e.g., `xy` rotates `+X` to
/// `+Y`).
///
/// Although the natural representation would be `s, xy, xz, yz`, this type
/// uses the right-handed dual representation `yz, zx, xy, s`. This ordering
/// and choice of signs is tailored for SIMD, allowing some operations to avoid
/// element rearrangement.
///
/// > In advanced Geometric Algebra terms, this type uses the rotor convention
/// > `R = e^(B/2)` with vector multiplication `R~vR`. This differs from the
/// > traditional convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that the fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`rotor.normalize()`]: Rotor#method.normalize
pub type Rotor3A<T> = Rotor<3, T, Aligned>;

impl<T, A: Alignment> Rotor<3, T, A>
where
    T: Scalar,
{
    #[inline(always)]
    #[track_caller]
    fn vector_mul_backend(vector: Vector<3, T, A>, rhs: Rotor<3, T, A>) -> Vector<3, T, A>
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let bivector = rhs.0.xyz();
        let bivector_length = bivector.dot(bivector);
        let self_dot_bivector = vector.dot(bivector);

        (vector * (rhs.s * rhs.s - bivector_length))
            + (bivector * (self_dot_bivector + self_dot_bivector))
            + (bivector.cross(vector) * (rhs.s + rhs.s))
    }
}

impl<const N: usize, T, A: Alignment> Clone for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T, A: Alignment> Copy for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar,
{
}

#[doc(hidden)]
#[repr(C)]
pub struct Rotor3Fields<T> {
    /// The plane element rotating `+Y` to `+Z`.
    ///
    /// Equal to `plane_of_rotation.yz * sin(angle/2)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    pub yz: T,
    /// The plane element rotating `+Z` to `+X`.
    ///
    /// Equal to `plane_of_rotation.zx * sin(angle/2)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    pub zx: T,
    /// The plane element rotating `+X` to `+Y`.
    ///
    /// Equal to `plane_of_rotation.xy * sin(angle/2)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    pub xy: T,
    /// The scalar element.
    ///
    /// Equal to `cos(angle/2)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    pub s: T,
}

impl<T, A: Alignment> Deref for Rotor<3, T, A>
where
    T: Scalar,
{
    type Target = Rotor3Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Rotor<3, T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Rotor3Fields<T>`.
        unsafe { transmute_ref::<Rotor<3, T, A>, Rotor3Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Rotor<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Rotor<3, T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Rotor3Fields<T>`.
        unsafe { transmute_mut::<Rotor<3, T, A>, Rotor3Fields<T>>(self) }
    }
}

impl<const N: usize, T, A: Alignment> Debug for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Rotor3")
            .field("yz", &self.0.x)
            .field("zx", &self.0.y)
            .field("xy", &self.0.z)
            .field("s", &self.0.w)
            .finish()
    }
}

impl<const N: usize, T, A: Alignment> PartialEq for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<const N: usize, T, A: Alignment> Eq for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Eq,
{
}

impl<const N: usize, T, A: Alignment> Hash for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Hash,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<const N: usize, T, A: Alignment> Default for Rotor<N, T, A>
where
    Length<N>: Three,
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
            Length<N>: Three,
            T: Scalar + Neg<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl<const N: usize, T, A: Alignment> Neg for &Rotor<N, T, A>
        where
        Length<N>: Three,
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
    /// The resulting rotor still represents the same rotation. To invert the
    /// rotation, use [`rotor.inverse()`].
    ///
    /// [`rotor.inverse()`]: Rotor#method.inverse
);

macro_rules! impl_add {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Add for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Add<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl<const N: usize, T, A: Alignment> Add<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
    /// Adds the elements of two rotors.
    ///
    /// This usually does not result in a valid rotation. Only use this if you
    /// know what you are doing! To chain two rotations, use
    /// `rotor_1 * rotor_2`.
);

macro_rules! impl_sub {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Sub for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl<const N: usize, T, A: Alignment> Sub<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
    /// Subtracts the elements of two rotors.
    ///
    /// This usually does not result in a valid rotation. Only use this if you
    /// know what you are doing!
);

macro_rules! impl_mul_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<T> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for Rotor<N, T, A>
        where
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
    /// Multiplies the elements of a rotor by a scalar.
    ///
    /// This usually does not result in a valid rotation. Only use this if you
    /// know what you are doing!
);

macro_rules! impl_vector_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<Rotor<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Rotor<N, T, A>) -> Self::Output {
                specialize_3!(Rotor::<N, T, A>::vector_mul_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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

        impl<const N: usize, T, A: Alignment> MulAssign<Rotor<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Rotor<N, T, A>) {
                *self = *self * rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&Rotor<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: Three,
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
impl_vector_mul!(
    /// Transforms a vector by a rotor.
    ///
    /// If the rotor is not normalized, this scales the vector by the rotor's
    /// squared length.
);

macro_rules! impl_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Self) -> Self::Output {
                specialize_3!(<T as RotorBackend<N, A>>::rotor_mul(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
    /// Chains two rotors, resulting in a rotor equivalent to applying the left
    /// rotor then the right rotor.
);

macro_rules! impl_div_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Div<T> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: T) -> Self::Output {
                Self(self.0 / rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Div<&T> for Rotor<N, T, A>
        where
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
            Length<N>: Three,
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
    /// Divides the elements of a rotor by a scalar.
    ///
    /// This usually does not result in a valid rotation. Only use this if you
    /// know what you are doing!
);

// SAFETY: Rotors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Send` the list also is, and the padding is `Send` too.
unsafe impl<const N: usize, T, A: Alignment> Send for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Send,
{
}

// SAFETY: Rotors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Sync` the list also is, and the padding is `Sync` too.
unsafe impl<const N: usize, T, A: Alignment> Sync for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Sync,
{
}

impl<const N: usize, T, A: Alignment> Unpin for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Unpin,
{
}

impl<const N: usize, T, A: Alignment> UnwindSafe for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + UnwindSafe,
{
}

impl<const N: usize, T, A: Alignment> RefUnwindSafe for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + RefUnwindSafe,
{
}

#[cfg(test)]
mod tests {
    extern crate std;

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
