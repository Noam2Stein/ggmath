#![expect(dead_code)]

use std::{fmt::Debug, hash::Hash};

use assert_impl_trait::assert_impl;
use ggmath::{Aligned, Alignment, Length, Scalar, SupportedLength, Unaligned, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Aabb<const N: usize, T, A: Alignment>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    center: Vector<N, T, A>,
    extents: Vector<N, T, A>,
}

type Aabb2<T> = Aabb<2, T, Aligned>;
type Aabb3<T> = Aabb<3, T, Aligned>;
type Aabb4<T> = Aabb<4, T, Aligned>;
type Aabb2U<T> = Aabb<2, T, Unaligned>;
type Aabb3U<T> = Aabb<3, T, Unaligned>;
type Aabb4U<T> = Aabb<4, T, Unaligned>;

assert_impl!(
    for<const N: usize, T, A: Alignment>
    where
        Length<N>: SupportedLength,
        T: Scalar,
    {
        where T: Debug {
            Aabb<N, T, A>: Debug,
        }
        where T: Clone {
            Aabb<N, T, A>: Clone,
        }
        where T: Copy {
            Aabb<N, T, A>: Copy,
        }
        where T: PartialEq {
            Aabb<N, T, A>: PartialEq,
        }
        where T: Eq {
            Aabb<N, T, A>: Eq,
        }
        where T: Hash {
            Aabb<N, T, A>: Hash,
        }
        where T: Default {
            Aabb<N, T, A>: Default,
        }
    }
);
