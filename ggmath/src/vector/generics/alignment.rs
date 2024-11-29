use std::{
    any::{type_name, TypeId},
    mem::{transmute, transmute_copy},
};

use super::*;

/// Sealed trait for the alignment rules of a vector.
/// - Doesn't affect the outer vector API, just the inner implementation.
/// - Use the [```Vec2```], [```Vec3```], [```Vec4```] type aliases for the default alignment.
///
/// Implemented by ```VecAligned``` and ```VecPacked```, each have different uses and advantages.
/// To understand them first understand the [Rust type-layout](<https://doc.rust-lang.org/reference/type-layout.html>).
///
/// ### [VecPacked]
///
/// ensures that the vector has the same type-layout as ```[T; N]```.
/// ```
/// assert_eq!(
///     size_of::<Vector<N, T, VecPacked>>(),
///     size_of::<T>() * N
/// );
/// assert_eq!(
///     align_of::<Vector<N, T, VecPacked>>(),
///     align_of::<T>()
/// );
/// ```
///
/// - [```Vec2P```], [```Vec3P```], and [```Vec4P```] are type aliases for packed vectors.
///
/// ### [VecAligned]
///
/// ensures that the vector is aligned to ```[T; N]```'s size.
/// ```
/// assert_eq!(
///     size_of::<Vector<N, T, VecAligned>>(),
///     (size_of::<T>() * N).next_power_of_two()
/// );
/// assert_eq!(
///     align_of::<Vector<N, T, VecAligned>>(),
///     (size_of::<T>() * N).next_power_of_two()
/// );
/// ```
///
/// - This means that the size and alignment of ```Vec3<T>``` is the same as ```Vec4<T>```'s.
/// - This means that ```size/align_of<Vec2> = size_of<T> * 2```, and ```size/align_of<Vec3> = size/align_of<Vec4> = size_of<T> * 4```.
///
/// - [```Vec2```], [```Vec3```], and [```Vec4```] are type aliases for aligned vectors.
///
/// ## How to pick
///
/// Sometimes a specific type-layout is required.
/// For example GPU uniform-structs have strict type-layout requirements, which include vectors following the ```VecAligned``` type-layout.
/// When neither alignment is required, choose based on their performance advantages/disadvantages:
///
/// - ```VecAligned``` results in faster computations because of SIMD registers which require the extra alignment.
/// - ```VecAligned``` may take more space because of the larger alignment, like where ```Vec3``` always takes the space of a ```Vec4```.
/// - ```VecPacked``` takes less space because of the minimal alignment and the lack of padding.
/// - ```VecPacked``` may result in slower computations because of the SIMD register's requirements.
///
/// Basically only use ```VecPacked``` for vectors that you don't perform much computation on.
/// On any other case use ```VecAligned```.
#[allow(private_bounds)]
pub trait VecAlignment: Seal + Sized + 'static + Send + Sync {
    /// Used by [```Vector```] to know its inner type.
    ///
    /// In ```VecPacked```: ```InnerVector = [T; N]```.
    /// for a packed vector, the compiler knows that ```packed_vector.0``` (the inner value) is always a ```[T; N]```.
    ///
    /// in ```VecAligned```: ```InnerVector = <ScalarCount<N> as VecLen>::InnerAlignedVector<T>```.
    /// for an aligned vector, the compiler doesn't know the inner type unless ```N``` is known.
    /// This is because Rust doesn't have a single generic type capable of representing different sizes and alignments.
    ///
    /// Basically ```Vector``` calls ```VecAlignment``` for its inner type,
    /// which calls ```VecLen``` for the final result.
    /// Why is the order ```Vector``` => ```VecAlignment``` => ```VecLen```?
    /// Why not ```Vector``` => ```VecLen``` => ```VecAlignment```?
    /// So that the compiler knows a packed vector is always an array.
    type InnerVector<const N: usize, T: Scalar>: Construct
    where
        ScalarCount<N>: VecLen;
}

/// See [```VecAlignment```].
/// ensures that the vector is aligned to ```[T; N]```'s size.
/// ```
/// assert_eq!(
///     size_of::<Vector<N, T, VecAligned>>(),
///     (size_of::<T>() * N).next_power_of_two()
/// );
/// assert_eq!(
///     align_of::<Vector<N, T, VecAligned>>(),
///     (size_of::<T>() * N).next_power_of_two()
/// );
/// ```
///
/// - This means that the size and alignment of ```Vec3<T>``` is the same as ```Vec4<T>```'s.
/// - This means that ```size/align_of<Vec2> = size_of<T> * 2```, and ```size/align_of<Vec3> = size/align_of<Vec4> = size_of<T> * 4```.
///
/// - [```Vec2```], [```Vec3```], and [```Vec4```] are type aliases for aligned vectors.
///
pub struct VecAligned;

impl VecAlignment for VecAligned {
    type InnerVector<const N: usize, T: Scalar>
        = <ScalarCount<N> as VecLen>::InnerAlignedVector<T>
    where
        ScalarCount<N>: VecLen;
}

/// See [```VecAlignment```].
/// ensures that the vector has the same type-layout as ```[T; N]```.
/// ```
/// assert_eq!(
///     size_of::<Vector<N, T, VecPacked>>(),
///     size_of::<T>() * N
/// );
/// assert_eq!(
///     align_of::<Vector<N, T, VecPacked>>(),
///     align_of::<T>()
/// );
/// ```
///
/// - [```Vec2P```], [```Vec3P```], and [```Vec4P```] are type aliases for packed vectors.
pub struct VecPacked;

impl VecAlignment for VecPacked {
    type InnerVector<const N: usize, T: Scalar> = [T; N]
    where
        ScalarCount<N>: VecLen;
}

pub enum AlignmentResolvedVector<const N: usize, T: Scalar>
where
    ScalarCount<N>: VecLen,
{
    Aligned(Vector<N, T, VecAligned>),
    Packed(Vector<N, T, VecPacked>),
}
pub enum AlignmentResolvedVectorRef<'a, const N: usize, T: Scalar>
where
    ScalarCount<N>: VecLen,
{
    Aligned(&'a Vector<N, T, VecAligned>),
    Packed(&'a Vector<N, T, VecPacked>),
}
pub enum AlignmentResolvedVectorMut<'a, const N: usize, T: Scalar>
where
    ScalarCount<N>: VecLen,
{
    Aligned(&'a mut Vector<N, T, VecAligned>),
    Packed(&'a mut Vector<N, T, VecPacked>),
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn into_aligned(self) -> Vector<N, T, VecAligned> {
        Vector::from_array(self.into_array())
    }
    #[inline(always)]
    pub fn into_packed(self) -> Vector<N, T, VecPacked> {
        Vector::from_array(self.into_array())
    }
    #[inline(always)]
    pub fn into_alignment<AOutput: VecAlignment>(self) -> Vector<N, T, AOutput> {
        Vector::from_array(self.into_array())
    }

    #[inline(always)]
    pub fn resolve_alignment(self) -> AlignmentResolvedVector<N, T> {
        unsafe {
            if TypeId::of::<A>() == TypeId::of::<VecAligned>() {
                AlignmentResolvedVector::Aligned(transmute_copy(&self))
            } else if TypeId::of::<A>() == TypeId::of::<VecPacked>() {
                AlignmentResolvedVector::Packed(transmute_copy(&self))
            } else {
                panic!("invalid VecAlignment: {}", type_name::<A>())
            }
        }
    }
    #[inline(always)]
    pub fn resolve_alignment_ref(&self) -> AlignmentResolvedVectorRef<N, T> {
        unsafe {
            if TypeId::of::<A>() == TypeId::of::<VecAligned>() {
                AlignmentResolvedVectorRef::Aligned(transmute(self))
            } else if TypeId::of::<A>() == TypeId::of::<VecPacked>() {
                AlignmentResolvedVectorRef::Packed(transmute(self))
            } else {
                panic!("invalid VecAlignment: {}", type_name::<A>())
            }
        }
    }
    #[inline(always)]
    pub fn resolve_alignment_mut(&mut self) -> AlignmentResolvedVectorMut<N, T> {
        unsafe {
            if TypeId::of::<A>() == TypeId::of::<VecAligned>() {
                AlignmentResolvedVectorMut::Aligned(transmute(self))
            } else if TypeId::of::<A>() == TypeId::of::<VecPacked>() {
                AlignmentResolvedVectorMut::Packed(transmute(self))
            } else {
                panic!("invalid VecAlignment: {}", type_name::<A>())
            }
        }
    }

    #[inline(always)]
    pub fn from_resolved_alignment_fns(
        f_aligned: impl FnOnce() -> Vector<N, T, VecAligned>,
        f_packed: impl FnOnce() -> Vector<N, T, VecPacked>,
    ) -> Self {
        unsafe {
            if TypeId::of::<A>() == TypeId::of::<VecAligned>() {
                transmute_copy(&f_aligned())
            } else if TypeId::of::<A>() == TypeId::of::<VecPacked>() {
                transmute_copy(&f_packed())
            } else {
                panic!("invalid VecAlignment: {}", type_name::<A>())
            }
        }
    }
}

trait Seal {}
impl Seal for VecAligned {}
impl Seal for VecPacked {}
