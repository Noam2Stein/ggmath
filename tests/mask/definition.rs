use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
    panic::{RefUnwindSafe, UnwindSafe},
};

use assert_impl_trait::assert_impl;
use ggmath::{
    Aligned, Alignment, Length, Mask, Scalar, SupportedLength, Unaligned, vec2, vec3, vec4,
};
use itertools::iproduct;

use crate::assert_panic;

assert_impl!(
    for<const N: usize, T, A: Alignment>
    where
        Length<N>: SupportedLength,
        T: Scalar,
    {
        Mask<N, T, A>: Send,
        Mask<N, T, A>: Sync,
        Mask<N, T, A>: Unpin,
        Mask<N, T, A>: UnwindSafe,
        Mask<N, T, A>: RefUnwindSafe,
        Mask<N, T, A>: Debug,
        Mask<N, T, A>: Clone,
        Mask<N, T, A>: Copy,
        Mask<N, T, A>: PartialEq,
        Mask<N, T, A>: Eq,
        Mask<N, T, A>: Hash,
        Mask<N, T, A>: Default,
        Mask<N, T, A>: Display,
        Mask<N, T, A>: IntoIterator<Item = bool>,
        Mask<N, T, A>: Not,
        Mask<N, T, A>: BitAnd,
        Mask<N, T, A>: BitAnd<bool>,
        Mask<N, T, A>: BitAndAssign,
        Mask<N, T, A>: BitAndAssign<bool>,
        Mask<N, T, A>: BitOr,
        Mask<N, T, A>: BitOr<bool>,
        Mask<N, T, A>: BitOrAssign,
        Mask<N, T, A>: BitOrAssign<bool>,
        Mask<N, T, A>: BitXor,
        Mask<N, T, A>: BitXor<bool>,
        Mask<N, T, A>: BitXorAssign,
        Mask<N, T, A>: BitXorAssign<bool>,

        for<'a> where T: 'a {
            Mask<N, T, A>: 'a,
            &'a Mask<N, T, A>: IntoIterator<Item = bool>,
        }
    }
);

#[test]
fn f32_aligned() {
    primitive_tests::<f32, Aligned>(1.0, 2.0, 3.0, 4.0);
}

#[test]
fn f32_unaligned() {
    primitive_tests::<f32, Unaligned>(1.0, 2.0, 3.0, 4.0);
}

#[test]
fn f64_aligned() {
    primitive_tests::<f64, Aligned>(1.0, 2.0, 3.0, 4.0);
}

#[test]
fn f64_unaligned() {
    primitive_tests::<f64, Unaligned>(1.0, 2.0, 3.0, 4.0);
}

#[test]
fn i8_aligned() {
    primitive_tests::<i8, Aligned>(1, 2, 3, 4);
}

#[test]
fn i8_unaligned() {
    primitive_tests::<i8, Unaligned>(1, 2, 3, 4);
}

#[test]
fn i16_aligned() {
    primitive_tests::<i16, Aligned>(1, 2, 3, 4);
}

#[test]
fn i16_unaligned() {
    primitive_tests::<i16, Unaligned>(1, 2, 3, 4);
}

#[test]
fn i32_aligned() {
    primitive_tests::<i32, Aligned>(1, 2, 3, 4);
}

#[test]
fn i32_unaligned() {
    primitive_tests::<i32, Unaligned>(1, 2, 3, 4);
}

#[test]
fn i64_aligned() {
    primitive_tests::<i64, Aligned>(1, 2, 3, 4);
}

#[test]
fn i64_unaligned() {
    primitive_tests::<i64, Unaligned>(1, 2, 3, 4);
}

#[test]
fn i128_aligned() {
    primitive_tests::<i128, Aligned>(1, 2, 3, 4);
}

#[test]
fn i128_unaligned() {
    primitive_tests::<i128, Unaligned>(1, 2, 3, 4);
}

#[test]
fn isize_aligned() {
    primitive_tests::<isize, Aligned>(1, 2, 3, 4);
}

#[test]
fn isize_unaligned() {
    primitive_tests::<isize, Unaligned>(1, 2, 3, 4);
}

#[test]
fn u8_aligned() {
    primitive_tests::<u8, Aligned>(1, 2, 3, 4);
}

#[test]
fn u8_unaligned() {
    primitive_tests::<u8, Unaligned>(1, 2, 3, 4);
}

#[test]
fn u16_aligned() {
    primitive_tests::<u16, Aligned>(1, 2, 3, 4);
}

#[test]
fn u16_unaligned() {
    primitive_tests::<u16, Unaligned>(1, 2, 3, 4);
}

#[test]
fn u32_aligned() {
    primitive_tests::<u32, Aligned>(1, 2, 3, 4);
}

#[test]
fn u32_unaligned() {
    primitive_tests::<u32, Unaligned>(1, 2, 3, 4);
}

#[test]
fn u64_aligned() {
    primitive_tests::<u64, Aligned>(1, 2, 3, 4);
}

#[test]
fn u64_unaligned() {
    primitive_tests::<u64, Unaligned>(1, 2, 3, 4);
}

#[test]
fn u128_aligned() {
    primitive_tests::<u128, Aligned>(1, 2, 3, 4);
}

#[test]
fn u128_unaligned() {
    primitive_tests::<u128, Unaligned>(1, 2, 3, 4);
}

#[test]
fn usize_aligned() {
    primitive_tests::<usize, Aligned>(1, 2, 3, 4);
}

#[test]
fn usize_unaligned() {
    primitive_tests::<usize, Unaligned>(1, 2, 3, 4);
}

#[test]
fn bool_aligned() {
    primitive_tests::<bool, Aligned>(false, true, false, true);
}

#[test]
fn bool_unaligned() {
    primitive_tests::<bool, Unaligned>(false, true, false, true);
}

fn primitive_tests<T, A: Alignment>(a: T, b: T, c: T, d: T)
where
    T: Scalar + Debug + PartialEq,
{
    assert!(size_of::<Mask<2, T, A>>() == 2);
    assert!(size_of::<Mask<3, T, A>>() == 3);
    assert!(size_of::<Mask<4, T, A>>() == 4);

    assert!(align_of::<Mask<2, T, A>>() == 1);
    assert!(align_of::<Mask<3, T, A>>() == 1);
    assert!(align_of::<Mask<4, T, A>>() == 1);

    for (x, y, z, w) in iproduct!([false, true], [false, true], [false, true], [false, true]) {
        assert_eq!(Mask::<2, T, A>::from_array([x, y]).to_array(), [x, y]);
        assert_eq!(Mask::<3, T, A>::from_array([x, y, z]).to_array(), [x, y, z]);
        assert_eq!(
            Mask::<4, T, A>::from_array([x, y, z, w]).to_array(),
            [x, y, z, w]
        );

        assert_eq!(Mask::<2, T, A>::splat(x).to_array(), [x; 2]);
        assert_eq!(Mask::<3, T, A>::splat(x).to_array(), [x; 3]);
        assert_eq!(Mask::<4, T, A>::splat(x).to_array(), [x; 4]);

        assert_eq!(Mask::<2, T, A>::from_fn(|i| [x, y][i]).to_array(), [x, y]);
        assert_eq!(
            Mask::<3, T, A>::from_fn(|i| [x, y, z][i]).to_array(),
            [x, y, z]
        );
        assert_eq!(
            Mask::<4, T, A>::from_fn(|i| [x, y, z, w][i]).to_array(),
            [x, y, z, w]
        );

        assert_eq!(Mask::<2, T, A>::new(x, y).to_array(), [x, y]);
        assert_eq!(Mask::<3, T, A>::new(x, y, z).to_array(), [x, y, z]);
        assert_eq!(Mask::<4, T, A>::new(x, y, z, w).to_array(), [x, y, z, w]);

        assert_eq!(Mask::<2, T, A>::new(x, y).align().to_array(), [x, y]);
        assert_eq!(Mask::<3, T, A>::new(x, y, z).align().to_array(), [x, y, z]);
        assert_eq!(
            Mask::<4, T, A>::new(x, y, z, w).align().to_array(),
            [x, y, z, w]
        );

        assert_eq!(Mask::<2, T, A>::new(x, y).unalign().to_array(), [x, y]);
        assert_eq!(
            Mask::<3, T, A>::new(x, y, z).unalign().to_array(),
            [x, y, z]
        );
        assert_eq!(
            Mask::<4, T, A>::new(x, y, z, w).unalign().to_array(),
            [x, y, z, w]
        );

        assert_eq!(Mask::<2, T, A>::new(x, y).all(), x && y);
        assert_eq!(Mask::<3, T, A>::new(x, y, z).all(), x && y && z);
        assert_eq!(Mask::<4, T, A>::new(x, y, z, w).all(), x && y && z && w);

        assert_eq!(Mask::<2, T, A>::new(x, y).any(), x || y);
        assert_eq!(Mask::<3, T, A>::new(x, y, z).any(), x || y || z);
        assert_eq!(Mask::<4, T, A>::new(x, y, z, w).any(), x || y || z || w);

        assert_eq!(
            Mask::<2, T, A>::new(x, y).select(vec2!(a, b), vec2!(c, d)),
            vec2!(if x { a } else { c }, if y { b } else { d })
        );
        assert_eq!(
            Mask::<3, T, A>::new(x, y, z).select(vec3!(a, b, c), vec3!(c, d, a)),
            vec3!(
                if x { a } else { c },
                if y { b } else { d },
                if z { c } else { a }
            )
        );
        assert_eq!(
            Mask::<4, T, A>::new(x, y, z, w).select(vec4!(a, b, c, d), vec4!(c, d, a, b)),
            vec4!(
                if x { a } else { c },
                if y { b } else { d },
                if z { c } else { a },
                if w { d } else { b },
            )
        );

        assert_eq!(
            Mask::<2, T, A>::new(x, y).iter().collect::<Vec<bool>>(),
            vec![x, y]
        );
        assert_eq!(
            Mask::<3, T, A>::new(x, y, z).iter().collect::<Vec<bool>>(),
            vec![x, y, z]
        );
        assert_eq!(
            Mask::<4, T, A>::new(x, y, z, w)
                .iter()
                .collect::<Vec<bool>>(),
            vec![x, y, z, w]
        );

        assert_eq!(Mask::<2, T, A>::new(x, y).get(0), x);
        assert_eq!(Mask::<2, T, A>::new(x, y).get(1), y);
        assert_panic!(Mask::<2, T, A>::new(x, y).get(2));
        assert_panic!(Mask::<2, T, A>::new(x, y).get(3));
        assert_eq!(Mask::<3, T, A>::new(x, y, z).get(0), x);
        assert_eq!(Mask::<3, T, A>::new(x, y, z).get(1), y);
        assert_eq!(Mask::<3, T, A>::new(x, y, z).get(2), z);
        assert_panic!(Mask::<3, T, A>::new(x, y, z).get(3));
        assert_panic!(Mask::<3, T, A>::new(x, y, z).get(4));
        assert_eq!(Mask::<4, T, A>::new(x, y, z, w).get(0), x);
        assert_eq!(Mask::<4, T, A>::new(x, y, z, w).get(1), y);
        assert_eq!(Mask::<4, T, A>::new(x, y, z, w).get(2), z);
        assert_eq!(Mask::<4, T, A>::new(x, y, z, w).get(3), w);
        assert_panic!(Mask::<4, T, A>::new(x, y, z, w).get(4));
        assert_panic!(Mask::<4, T, A>::new(x, y, z, w).get(5));

        let mut m = Mask::<2, T, A>::new(x, y);
        m.set(0, y);
        assert_eq!(m.to_array(), [y, y]);
        m.set(1, x);
        assert_eq!(m.to_array(), [y, x]);
        assert_panic!(m.clone().set(2, x));
        assert_panic!(m.clone().set(3, z));

        let mut m = Mask::<3, T, A>::new(x, y, z);
        m.set(0, y);
        assert_eq!(m.to_array(), [y, y, z]);
        m.set(1, x);
        assert_eq!(m.to_array(), [y, x, z]);
        m.set(2, x);
        assert_eq!(m.to_array(), [y, x, x]);
        assert_panic!(m.clone().set(3, w));
        assert_panic!(m.clone().set(4, w));

        let mut m = Mask::<4, T, A>::new(x, y, z, w);
        m.set(0, y);
        assert_eq!(m.to_array(), [y, y, z, w]);
        m.set(1, x);
        assert_eq!(m.to_array(), [y, x, z, w]);
        m.set(2, x);
        assert_eq!(m.to_array(), [y, x, x, w]);
        m.set(3, x);
        assert_eq!(m.to_array(), [y, x, x, x]);
        assert_panic!(m.clone().set(4, w));
        assert_panic!(m.clone().set(5, w));

        assert_eq!(
            Mask::<2, T, A>::new(x, y)
                .into_iter()
                .collect::<Vec<bool>>(),
            vec![x, y]
        );
        assert_eq!(
            Mask::<3, T, A>::new(x, y, z)
                .into_iter()
                .collect::<Vec<bool>>(),
            vec![x, y, z]
        );
        assert_eq!(
            Mask::<4, T, A>::new(x, y, z, w)
                .into_iter()
                .collect::<Vec<bool>>(),
            vec![x, y, z, w]
        );

        assert_eq!(
            format!("{:?}", Mask::<2, T, A>::new(x, y)),
            format!("({x}, {y})")
        );
        assert_eq!(
            format!("{:?}", Mask::<3, T, A>::new(x, y, z)),
            format!("({x}, {y}, {z})")
        );
        assert_eq!(
            format!("{:?}", Mask::<4, T, A>::new(x, y, z, w)),
            format!("({x}, {y}, {z}, {w})")
        );

        assert_eq!(
            format!("{}", Mask::<2, T, A>::new(x, y)),
            format!("({x}, {y})")
        );
        assert_eq!(
            format!("{}", Mask::<3, T, A>::new(x, y, z)),
            format!("({x}, {y}, {z})")
        );
        assert_eq!(
            format!("{}", Mask::<4, T, A>::new(x, y, z, w)),
            format!("({x}, {y}, {z}, {w})")
        );

        assert_eq!(Mask::<2, T, A>::default().to_array(), [bool::default(); 2]);
        assert_eq!(Mask::<3, T, A>::default().to_array(), [bool::default(); 3]);
        assert_eq!(Mask::<4, T, A>::default().to_array(), [bool::default(); 4]);

        assert_eq!(
            Mask::<2, T, A>::new(x, y) == Mask::<2, T, A>::new(x, y),
            true
        );
        assert_eq!(
            Mask::<3, T, A>::new(x, y, z) == Mask::<3, T, A>::new(x, y, z),
            true
        );
        assert_eq!(
            Mask::<4, T, A>::new(x, y, z, w) == Mask::<4, T, A>::new(x, y, z, w),
            true
        );
        assert_eq!(
            Mask::<2, T, A>::new(y, y) == Mask::<2, T, A>::new(x, y),
            x == y
        );
        assert_eq!(
            Mask::<3, T, A>::new(y, y, z) == Mask::<3, T, A>::new(x, y, z),
            x == y
        );
        assert_eq!(
            Mask::<4, T, A>::new(y, y, z, w) == Mask::<4, T, A>::new(x, y, z, w),
            x == y
        );
        assert_eq!(
            Mask::<2, T, A>::new(x, x) == Mask::<2, T, A>::new(x, y),
            x == y
        );
        assert_eq!(
            Mask::<3, T, A>::new(x, x, z) == Mask::<3, T, A>::new(x, y, z),
            x == y
        );
        assert_eq!(
            Mask::<4, T, A>::new(x, x, z, w) == Mask::<4, T, A>::new(x, y, z, w),
            x == y
        );
        assert_eq!(
            Mask::<2, T, A>::new(y, x) == Mask::<2, T, A>::new(x, y),
            x == y
        );
        assert_eq!(
            Mask::<3, T, A>::new(y, w, x) == Mask::<3, T, A>::new(x, y, z),
            x == y && x == z && y == w
        );
        assert_eq!(
            Mask::<4, T, A>::new(y, w, w, z) == Mask::<4, T, A>::new(x, y, z, w),
            x == y && x == z && y == w && z == w
        );

        assert_eq!(
            Mask::<2, T, A>::new(x, y) != Mask::<2, T, A>::new(x, y),
            false
        );
        assert_eq!(
            Mask::<3, T, A>::new(x, y, z) != Mask::<3, T, A>::new(x, y, z),
            false
        );
        assert_eq!(
            Mask::<4, T, A>::new(x, y, z, w) != Mask::<4, T, A>::new(x, y, z, w),
            false
        );
        assert_eq!(
            Mask::<2, T, A>::new(y, y) != Mask::<2, T, A>::new(x, y),
            x != y
        );
        assert_eq!(
            Mask::<3, T, A>::new(y, y, z) != Mask::<3, T, A>::new(x, y, z),
            x != y
        );
        assert_eq!(
            Mask::<4, T, A>::new(y, y, z, w) != Mask::<4, T, A>::new(x, y, z, w),
            x != y
        );
        assert_eq!(
            Mask::<2, T, A>::new(x, x) != Mask::<2, T, A>::new(x, y),
            x != y
        );
        assert_eq!(
            Mask::<3, T, A>::new(x, x, z) != Mask::<3, T, A>::new(x, y, z),
            x != y
        );
        assert_eq!(
            Mask::<4, T, A>::new(x, x, z, w) != Mask::<4, T, A>::new(x, y, z, w),
            x != y
        );
        assert_eq!(
            Mask::<2, T, A>::new(y, x) != Mask::<2, T, A>::new(x, y),
            x != y
        );
        assert_eq!(
            Mask::<3, T, A>::new(y, w, x) != Mask::<3, T, A>::new(x, y, z),
            x != y || x != z || y != w
        );
        assert_eq!(
            Mask::<4, T, A>::new(y, w, w, z) != Mask::<4, T, A>::new(x, y, z, w),
            x != y || x != z || y != w || z != w
        );

        assert_eq!(!Mask::<2, T, A>::new(x, y), Mask::<2, T, A>::new(!x, !y));
        assert_eq!(
            !Mask::<3, T, A>::new(x, y, z),
            Mask::<3, T, A>::new(!x, !y, !z)
        );
        assert_eq!(
            !Mask::<4, T, A>::new(x, y, z, x),
            Mask::<4, T, A>::new(!x, !y, !z, !x)
        );

        assert_eq!(
            Mask::<2, T, A>::new(x, y) & Mask::<2, T, A>::new(z, x),
            Mask::<2, T, A>::new(x & z, y & x)
        );
        assert_eq!(
            Mask::<3, T, A>::new(x, y, z) & Mask::<3, T, A>::new(z, x, y),
            Mask::<3, T, A>::new(x & z, y & x, z & y)
        );
        assert_eq!(
            Mask::<4, T, A>::new(x, y, z, x) & Mask::<4, T, A>::new(z, x, y, z),
            Mask::<4, T, A>::new(x & z, y & x, z & y, x & z)
        );

        assert_eq!(
            Mask::<2, T, A>::new(x, y) | Mask::<2, T, A>::new(z, x),
            Mask::<2, T, A>::new(x | z, y | x)
        );
        assert_eq!(
            Mask::<3, T, A>::new(x, y, z) | Mask::<3, T, A>::new(z, x, y),
            Mask::<3, T, A>::new(x | z, y | x, z | y)
        );
        assert_eq!(
            Mask::<4, T, A>::new(x, y, z, x) | Mask::<4, T, A>::new(z, x, y, z),
            Mask::<4, T, A>::new(x | z, y | x, z | y, x | z)
        );

        assert_eq!(
            Mask::<2, T, A>::new(x, y) ^ Mask::<2, T, A>::new(z, x),
            Mask::<2, T, A>::new(x ^ z, y ^ x)
        );
        assert_eq!(
            Mask::<3, T, A>::new(x, y, z) ^ Mask::<3, T, A>::new(z, x, y),
            Mask::<3, T, A>::new(x ^ z, y ^ x, z ^ y)
        );
        assert_eq!(
            Mask::<4, T, A>::new(x, y, z, x) ^ Mask::<4, T, A>::new(z, x, y, z),
            Mask::<4, T, A>::new(x ^ z, y ^ x, z ^ y, x ^ z)
        );
    }
}
