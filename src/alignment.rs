use core::{fmt::Debug, hash::Hash};

/// A marker type used to enable SIMD alignment.
///
/// See [`Alignment`] for more details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Aligned;

/// A marker type used to disable SIMD alignment.
///
/// See [`Alignment`] for more details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Unaligned;

/// A marker trait used to enable and disable SIMD alignment.
///
/// Math types (e.g., [`Vector<N, T, A>`]) are generic over `A: Alignment`,
/// which can be either [`Unaligned`] or [`Aligned`]. Types with `A = Unaligned`
/// have type aliases without a suffix (e.g., [`Vec3<T>`]), and types with
/// `A = Aligned` have type aliases with an `A` suffix (e.g., [`Vec3A<T>`]).
///
/// Types with `A = Unaligned` are stored as arrays of type `T`, without
/// padding or additional alignment. Types with `A = Aligned` are considered
/// "SIMD-aligned types".
///
/// For supported `T` types and target configurations, SIMD-aligned types use
/// SIMD-compatible representations, and appropriate operations use specialized SIMD
/// implementations. Exact representations are documented on each SIMD type. For
/// unsupported types, SIMD-aligned types are identical to their non-SIMD variants.
///
/// SIMD-aligned types tend to improve arithmetic throughput, but have higher
/// alignment and may have padding. For example, [`Vec3<f32>`] has a size of 12
/// bytes and an alignment of 4 bytes, while [`Vec3A<f32>`] has a size and alignment
/// of 16 bytes.
///
/// SIMD-aligned types tend to improve performance when the bottleneck is arithmetic
/// throughput, and tend to hurt performance when the bottleneck is memory
/// bandwidth.
///
/// [`Vector<N, T, A>`]: crate::Vector
/// [`Vec3<T>`]: crate::Vec3
/// [`Vec3A<T>`]: crate::Vec3A
/// [`Vec3<f32>`]: crate::Vec3
/// [`Vec3A<f32>`]: crate::Vec3A
#[expect(private_bounds)]
pub trait Alignment: Sealed {
    #[doc(hidden)]
    const IS_ALIGNED: bool;

    #[doc(hidden)]
    type Select<A: Copy, U: Copy>: Copy;
}

impl Alignment for Aligned {
    const IS_ALIGNED: bool = true;

    type Select<A: Copy, U: Copy> = A;
}

impl Alignment for Unaligned {
    const IS_ALIGNED: bool = false;

    type Select<A: Copy, U: Copy> = U;
}

trait Sealed:
    'static + Send + Sync + Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash + Default
{
}

impl Sealed for Aligned {}
impl Sealed for Unaligned {}
