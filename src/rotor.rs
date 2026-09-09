use core::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Dim, Element, One, Unaligned, Vector, Zero,
    backend::RotorBackend,
    dim::Three,
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
    Dim<N>: Three,
    T: Element;

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
    T: Element,
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
    Dim<N>: Three,
    T: Element,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T, A: Alignment> Copy for Rotor<N, T, A>
where
    Dim<N>: Three,
    T: Element,
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
    T: Element,
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
    T: Element,
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
    Dim<N>: Three,
    T: Element + Debug,
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
    Dim<N>: Three,
    T: Element + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<const N: usize, T, A: Alignment> Eq for Rotor<N, T, A>
where
    Dim<N>: Three,
    T: Element + Eq,
{
}

impl<const N: usize, T, A: Alignment> Hash for Rotor<N, T, A>
where
    Dim<N>: Three,
    T: Element + Hash,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<const N: usize, T, A: Alignment> Default for Rotor<N, T, A>
where
    Dim<N>: Three,
    T: Element + Zero + One,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T>,
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
        Dim<N>: Three,
        T: Element + Neg<Output = T>,
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
            Dim<N>: Three,
            T: Element + Add<Output = T>,
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
            Dim<N>: Three,
            T: Element + Add<Output = T>,
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
            Dim<N>: Three,
            T: Element + Add<Output = T>,
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
            Dim<N>: Three,
            T: Element + Add<Output = T>,
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
            Dim<N>: Three,
            T: Element + Add<Output = T>,
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
            Dim<N>: Three,
            T: Element + Add<Output = T>,
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
            Dim<N>: Three,
            T: Element + Sub<Output = T>,
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
            Dim<N>: Three,
            T: Element + Sub<Output = T>,
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
            Dim<N>: Three,
            T: Element + Sub<Output = T>,
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
            Dim<N>: Three,
            T: Element + Sub<Output = T>,
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
            Dim<N>: Three,
            T: Element + Sub<Output = T>,
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
            Dim<N>: Three,
            T: Element + Sub<Output = T>,
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
            Dim<N>: Three,
            T: Element + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
            Dim<N>: Three,
            T: Element + Div<Output = T>,
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
            Dim<N>: Three,
            T: Element + Div<Output = T>,
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
            Dim<N>: Three,
            T: Element + Div<Output = T>,
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
            Dim<N>: Three,
            T: Element + Div<Output = T>,
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
            Dim<N>: Three,
            T: Element + Div<Output = T>,
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
            Dim<N>: Three,
            T: Element + Div<Output = T>,
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
    Dim<N>: Three,
    T: Element + Send,
{
}

// SAFETY: Rotors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Sync` the list also is, and the padding is `Sync` too.
unsafe impl<const N: usize, T, A: Alignment> Sync for Rotor<N, T, A>
where
    Dim<N>: Three,
    T: Element + Sync,
{
}

impl<const N: usize, T, A: Alignment> Unpin for Rotor<N, T, A>
where
    Dim<N>: Three,
    T: Element + Unpin,
{
}

impl<const N: usize, T, A: Alignment> UnwindSafe for Rotor<N, T, A>
where
    Dim<N>: Three,
    T: Element + UnwindSafe,
{
}

impl<const N: usize, T, A: Alignment> RefUnwindSafe for Rotor<N, T, A>
where
    Dim<N>: Three,
    T: Element + RefUnwindSafe,
{
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        Matrix, Rotor, Vector,
        test_utils::{assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_identity() {
        for_types!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Rotor::<3, T, A>::IDENTITY,
                Rotor::<3, T, A>::from_elements(T::ZERO, T::ZERO, T::ZERO, T::ONE)
            );
        });
    }

    #[test]
    fn test_conjugate() {
        for_types!(|T: PrimitiveFloat, A| {
            for rotor in random_iter::<Rotor<3, T, A>>() {
                assert_test_eq!(
                    rotor.conjugate(),
                    Rotor::<3, T, A>::from_elements(-rotor.yz, -rotor.zx, -rotor.xy, rotor.s)
                );
            }
        });
    }

    #[test]
    fn test_deref() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            let rotor = Rotor::<3, T, A>::from_elements(x, y, z, w);
            assert_eq!(rotor.yz, x);
            assert_eq!(rotor.zx, y);
            assert_eq!(rotor.xy, z);
            assert_eq!(rotor.s, w);
        });
    }

    #[test]
    fn test_deref_mut() {
        for_types!(|T: PrimitiveNumber, A| {
            let [mut x, mut y, mut z, mut w] = std::array::from_fn(|i| T::as_from(i + 1));

            let mut rotor = Rotor::<3, T, A>::from_elements(x, y, z, w);
            assert_eq!(&mut rotor.yz, &mut x);
            assert_eq!(&mut rotor.zx, &mut y);
            assert_eq!(&mut rotor.xy, &mut z);
            assert_eq!(&mut rotor.s, &mut w);
        });
    }

    #[test]
    fn test_debug() {
        for_types!(|T: PrimitiveNumber, A| {
            let rotor = Rotor::<3, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                format!("{rotor:?}"),
                format!(
                    "Rotor3 {{ yz: {:?}, zx: {:?}, xy: {:?}, s: {:?} }}",
                    rotor.yz, rotor.zx, rotor.xy, rotor.s
                )
            );
        });
    }

    #[test]
    fn test_vector_mul() {
        for_types!(|T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<3, T, A>>() {
                if vector.is_finite() {
                    assert_test_eq!(vector * Rotor::IDENTITY, vector, 0.0 = -0.0);
                }
            }
            for (vector, rotor) in random_iter::<(Vector<3, T, A>, Rotor<3, T, A>)>() {
                assert_test_eq!(vector * -rotor, vector * rotor, 0.0 = -0.0);

                let rotor = rotor.normalize_or(Rotor::IDENTITY) * rotor.length().clamp(0.2, 5.0);
                if !vector.is_finite() || !rotor.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    vector * rotor,
                    vector * rotor.normalize() * rotor.length_squared(),
                    abs <= (vector * rotor).length() * 1e-5
                );
                assert_test_eq!(
                    (vector * rotor.normalize()).length(),
                    vector.length(),
                    abs <= vector.length() * 1e-5
                );
            }

            for (vector, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                let angle = angle % 6.0;
                let (sin, s) = (angle / 2.0).sin_cos();

                if !vector.is_finite() || !sin.is_finite() || !s.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    vector * Rotor::<3, T, A>::from_elements(sin, 0.0, 0.0, s),
                    vector.rotate_yz(angle),
                    abs <= vector.length() * 1e-5 + 1e-5,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    vector * Rotor::<3, T, A>::from_elements(0.0, sin, 0.0, s),
                    vector.rotate_xz(-angle),
                    abs <= vector.length() * 1e-5 + 1e-5,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    vector * Rotor::<3, T, A>::from_elements(0.0, 0.0, sin, s),
                    vector.rotate_xy(angle),
                    abs <= vector.length() * 1e-5 + 1e-5,
                    0.0 = -0.0
                );
            }

            for (vector, rotor) in [
                (
                    Vector::<3, T, A>::new(-4.1, 3.3, 10.3),
                    Rotor::<3, T, A>::from_elements(0.8, 0.4, 0.3, 0.1),
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
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, [rotor_1, rotor_2]) in
                random_iter::<(Vector<3, T, A>, [Rotor<3, T, A>; 2])>()
            {
                if !vector.is_finite() || vector.length() > 1e5 {
                    continue;
                }

                let [rotor_1, rotor_2] = [rotor_1, rotor_2]
                    .map(|r| r.normalize_or(Rotor::IDENTITY) * r.length().clamp(0.2, 5.0));

                assert_test_eq!(
                    vector * (rotor_1 * rotor_2),
                    vector * rotor_1 * rotor_2,
                    abs <= (vector * rotor_1 * rotor_2).length() * 1e-5 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }
}
