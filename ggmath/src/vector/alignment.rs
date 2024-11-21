use super::*;

/// Sealed trait for the alignment rules of a vector.
/// - Doesn't affect the outer vector API, just the inner implementation.
/// - Use the [```VecN```]```<N, T>``` type alias to use the default storage.
///
/// Implemented by ```VecAligned``` and ```VecPacked```, each have different uses and advantages.
/// To understand them first understand [Rust type-layout](<https://doc.rust-lang.org/reference/type-layout.html>).
///
/// ### [VecPacked]
///
/// ensures that the vector has the same type-layout as ```[T; N]```.
/// ```
/// // VecNP<N, T> = Vector<N, T, VecPacked>
/// assert_eq!(
///     size_of::<VecNP<N, T>>(),
///     size_of::<T>() * N
/// );
/// assert_eq!(
///     align_of::<VecNP<N, T>>(),
///     align_of::<T>()
/// );
/// ```
///
/// - use [```VecNP```]<N, T>
///
/// ### [VecAligned]
///
/// ensures that the vector has the next alignment from ```[T; N]```'s size, and a size equal to the alignment.
/// ```
/// // VecN<N, T> = Vector<N, T, VecAligned>
/// assert_eq!(
///     size_of::<VecN<N, T>>(),
///     (size_of::<T>() * N).next_power_of_two()
/// );
/// assert_eq!(
///     align_of::<VecN<N, T>>(),
///     (size_of::<T>() * N).next_power_of_two()
/// );
/// ```
///
/// - This means that the size and alignment of ```Vec3<T>``` is the same as ```Vec4<T>```'s.
/// - This means that ```size/align_of<Vec2> = size_of<T> * 2```, and ```size/align_of<Vec3> = size/align_of<Vec4> = size_of<T> * 4```.
///
/// - use [```VecN```]<N, T>
///
/// ## How to pick
///
/// Sometimes the ```VecAligned``` type-layout is required.
/// For example in GPU uniform-structs that have strict type-layout requirements, which include vectors following the ```VecAligned``` type-layout.
/// When neither storage is required, choose based on their performance advantages/disadvantages:
///
/// - ```VecAligned``` results in faster computations because of SIMD registers which require the extra alignment.
/// - ```VecAligned``` may take more space because of the larger alignment, and that a ```Vec3``` always takes the space of a ```Vec4```.
/// - ```VecPacked``` takes less space because of the minimal alignment and the lack of padding.
/// - ```VecPacked``` may result in slower computations because of the SIMD register's requirements.
///
/// Basically only use ```VecPacked``` (```VecNP```) when storing large arrays of vectors that you don't perform much computation on.
/// On any other case use ```VecAligned``` (```VecN```, The default).
#[allow(private_bounds)]
pub trait VecAlignment: Seal + Sized + 'static + Send + Sync {
    type InnerVector<const N: usize, T: Scalar>: Construct
    where
        ScalarCount<N>: VecLen;
}

/// Vector inner storage that ensures that the vector has the next alignment from ```[T; N]```'s size, and a size equal to the alignment.
/// ```
/// // VecN<N, T> = Vector<N, T, VecAligned>
/// assert_eq!(
///     size_of::<VecN<N, T>>(),
///     (size_of::<T>() * N).next_power_of_two()
/// );
/// assert_eq!(
///     align_of::<VecN<N, T>>(),
///     (size_of::<T>() * N).next_power_of_two()
/// );
/// ```
///
/// - This means that the size and alignment of ```Vec3<T>``` is the same as ```Vec4<T>```'s.
/// - This means that ```size/align_of<Vec2> = size_of<T> * 2```, and ```size/align_of<Vec3> = size/align_of<Vec4> = size_of<T> * 4```.
///
/// ## When to use
///
/// Sometimes the ```VecAligned``` type-layout is required.
/// For example in GPU uniform-structs that have strict type-layout requirements, which include vectors following the ```VecAligned``` type-layout.
/// When not required, choose based on performance advantages/disadvantages:
///
/// - results in faster computations than ```VecPacked``` because of SIMD registers which require the extra alignment.
/// - may take more space than ```VecPacked``` because of the larger alignment, and that a ```Vec3``` always takes the space of a ```Vec4```.
///
/// Always recommended except for when storing large arrays of vectors that you don't perform much computation on.
pub struct VecAligned;

/// Vector inner storage that ensures that the vector has the same type-layout as ```[T; N]```.
/// ```
/// // VecNP<N, T> = Vector<N, T, VecPacked>
/// assert_eq!(
///     size_of::<VecNP<N, T>>(),
///     size_of::<T>() * N
/// );
/// assert_eq!(
///     align_of::<VecNP<N, T>>(),
///     align_of::<T>()
/// );
/// ```
///
/// ## When to use
///
/// Sometimes the ```VecAligned``` type-layout is required.
/// For example in GPU uniform-structs that have strict type-layout requirements, which include vectors following the ```VecAligned``` type-layout.
/// When ```VecAligned``` isn't required, choose based on performance advantages/disadvantages:
///
/// - takes less space than ```VecAligned``` because of the minimal alignment and the lack of padding.
/// - may result in slower computations than ```VecAligned``` because of the SIMD register's requirements.
///
/// Only recommended when storing large arrays of vectors that you don't perform much computation on.
pub struct VecPacked;

impl VecAlignment for VecAligned {
    type InnerVector<const N: usize, T: Scalar>
    = <ScalarCount<N> as VecLen>::InnerAlignedVector<T> where
    ScalarCount<N>: VecLen;
}

impl VecAlignment for VecPacked {
    type InnerVector<const N: usize, T: Scalar> = [T; N]where
    ScalarCount<N>: VecLen;
}

trait Seal {}
impl Seal for VecAligned {}
impl Seal for VecPacked {}
