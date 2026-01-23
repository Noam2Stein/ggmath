use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
    panic::{RefUnwindSafe, UnwindSafe},
};

use assert_impl_trait::assert_impl;
use ggmath::{Aligned, Alignment, Length, Scalar, SupportedLength, Unaligned, Vector};

use crate::assert_panic;

assert_impl!(
    for<const N: usize, T: Scalar, A: Alignment>
    where
        Length<N>: SupportedLength,
    {
        Vector<N, T, A>: Clone,
        Vector<N, T, A>: Copy,
        Vector<N, T, A>: Index<usize>,
        Vector<N, T, A>: IndexMut<usize>,
        Vector<N, T, A>: IntoIterator<Item = T>,
        for<'a> where T: 'a {
            &'a mut Vector<N, T, A>: IntoIterator<Item = &'a mut T>,
        }

        for<'a> where T: 'a {
            Vector<N, T, A>: 'a,
        }
        where T: Send {
            Vector<N, T, A>: Send,
        }
        where T: Sync {
            Vector<N, T, A>: Sync,
        }
        where T: Unpin {
            Vector<N, T, A>: Unpin,
        }
        where T: UnwindSafe {
            Vector<N, T, A>: UnwindSafe,
        }
        where T: RefUnwindSafe {
            Vector<N, T, A>: RefUnwindSafe,
        }
        where T: Debug {
            Vector<N, T, A>: Debug,
        }
        where T: Display {
            Vector<N, T, A>: Display,
        }
        where T: PartialEq {
            Vector<N, T, A>: PartialEq,
        }
        where T: Eq {
            Vector<N, T, A>: Eq,
        }
        where T: Hash {
            Vector<N, T, A>: Hash,
        }
        where T: Default {
            Vector<N, T, A>: Default,
        }

        where T: Neg<Output = T> {
            Vector<N, T, A>: Neg,
        }
        where T: Not<Output = T> {
            Vector<N, T, A>: Not,
        }

        where T: Add<Output = T> {
            Vector<N, T, A>: Add,
            Vector<N, T, A>: Add<T>,
            Vector<N, T, A>: AddAssign,
            Vector<N, T, A>: AddAssign<T>,
        }
        where T: Sub<Output = T> {
            Vector<N, T, A>: Sub,
            Vector<N, T, A>: Sub<T>,
            Vector<N, T, A>: SubAssign,
            Vector<N, T, A>: SubAssign<T>,
        }
        where T: Mul<Output = T> {
            Vector<N, T, A>: Mul,
            Vector<N, T, A>: Mul<T>,
            Vector<N, T, A>: MulAssign,
            Vector<N, T, A>: MulAssign<T>,
        }
        where T: Div<Output = T> {
            Vector<N, T, A>: Div,
            Vector<N, T, A>: Div<T>,
            Vector<N, T, A>: DivAssign,
            Vector<N, T, A>: DivAssign<T>,
        }
        where T: Rem<Output = T> {
            Vector<N, T, A>: Rem,
            Vector<N, T, A>: Rem<T>,
            Vector<N, T, A>: RemAssign,
            Vector<N, T, A>: RemAssign<T>,
        }
        where T: Shl<Output = T> {
            Vector<N, T, A>: Shl,
            Vector<N, T, A>: Shl<T>,
            Vector<N, T, A>: ShlAssign,
            Vector<N, T, A>: ShlAssign<T>,
        }
        where T: Shr<Output = T> {
            Vector<N, T, A>: Shr,
            Vector<N, T, A>: Shr<T>,
            Vector<N, T, A>: ShrAssign,
            Vector<N, T, A>: ShrAssign<T>,
        }
        where T: BitAnd<Output = T> {
            Vector<N, T, A>: BitAnd,
            Vector<N, T, A>: BitAnd<T>,
            Vector<N, T, A>: BitAndAssign,
            Vector<N, T, A>: BitAndAssign<T>,
        }
        where T: BitOr<Output = T> {
            Vector<N, T, A>: BitOr,
            Vector<N, T, A>: BitOr<T>,
            Vector<N, T, A>: BitOrAssign,
            Vector<N, T, A>: BitOrAssign<T>,
        }
        where T: BitXor<Output = T> {
            Vector<N, T, A>: BitXor,
            Vector<N, T, A>: BitXor<T>,
            Vector<N, T, A>: BitXorAssign,
            Vector<N, T, A>: BitXorAssign<T>,
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

fn primitive_tests<
    T: Scalar + Debug + Display + PartialEq + PartialOrd + Default + UnwindSafe + RefUnwindSafe,
    A: Alignment,
>(
    x: T,
    y: T,
    z: T,
    w: T,
) {
    macro_rules! vec2 {
        ($($arg:expr),*$(,)?) => {
            ggmath::Vector::<2, T, A>::from(($($arg,)*))
        };
    }

    macro_rules! vec3 {
        ($($arg:expr),*$(,)?) => {
            ggmath::Vector::<3, T, A>::from(($($arg,)*))
        };
    }

    macro_rules! vec4 {
        ($($arg:expr),*$(,)?) => {
            ggmath::Vector::<4, T, A>::from(($($arg,)*))
        };
    }

    assert!(y > x && w > z);

    assert!(size_of::<Vector<2, T, A>>() == size_of::<T>() * 2);
    assert!(size_of::<Vector<4, T, A>>() == size_of::<T>() * 4);
    assert!(
        size_of::<Vector<3, T, A>>() == size_of::<T>() * 3
            || size_of::<Vector<3, T, A>>() == size_of::<T>() * 4
    );

    assert!(
        align_of::<Vector<2, T, A>>() == align_of::<T>()
            || align_of::<Vector<2, T, A>>() == size_of::<T>() * 2
    );
    assert!(
        align_of::<Vector<3, T, A>>() == align_of::<T>()
            || align_of::<Vector<3, T, A>>() == size_of::<T>() * 4
    );
    assert!(
        align_of::<Vector<4, T, A>>() == align_of::<T>()
            || align_of::<Vector<4, T, A>>() == size_of::<T>() * 4
    );

    assert_eq!(Vector::<2, T, A>::from_array([x, y]).to_array(), [x, y]);
    assert_eq!(
        Vector::<3, T, A>::from_array([x, y, z]).to_array(),
        [x, y, z]
    );
    assert_eq!(
        Vector::<4, T, A>::from_array([x, y, z, w]).to_array(),
        [x, y, z, w]
    );

    assert_eq!(Vector::<2, T, A>::splat(x).to_array(), [x; 2]);
    assert_eq!(Vector::<3, T, A>::splat(x).to_array(), [x; 3]);
    assert_eq!(Vector::<4, T, A>::splat(x).to_array(), [x; 4]);

    assert_eq!(Vector::<2, T, A>::from_fn(|i| [x, y][i]).to_array(), [x, y]);
    assert_eq!(
        Vector::<3, T, A>::from_fn(|i| [x, y, z][i]).to_array(),
        [x, y, z]
    );
    assert_eq!(
        Vector::<4, T, A>::from_fn(|i| [x, y, z, w][i]).to_array(),
        [x, y, z, w]
    );

    assert_eq!(vec2!(x, y).to_array(), [x, y]);
    assert_eq!(vec2!(vec2!(x, y)).to_array(), [x, y]);

    assert_eq!(vec3!(x, y, z).to_array(), [x, y, z]);
    assert_eq!(vec3!(x, vec2!(y, z)).to_array(), [x, y, z]);
    assert_eq!(vec3!(vec2!(x, y), z).to_array(), [x, y, z]);
    assert_eq!(vec3!(vec3!(x, y, z)).to_array(), [x, y, z]);

    assert_eq!(vec4!(x, y, z, w).to_array(), [x, y, z, w]);
    assert_eq!(vec4!(x, y, vec2!(z, w)).to_array(), [x, y, z, w]);
    assert_eq!(vec4!(x, vec2!(y, z), w).to_array(), [x, y, z, w]);
    assert_eq!(vec4!(x, vec3!(y, z, w)).to_array(), [x, y, z, w]);
    assert_eq!(vec4!(vec2!(x, y), vec2!(z, w)).to_array(), [x, y, z, w]);
    assert_eq!(vec4!(vec3!(x, y, z), w).to_array(), [x, y, z, w]);
    assert_eq!(vec4!(vec4!(x, y, z, w)).to_array(), [x, y, z, w]);

    assert_eq!(vec2!(x).to_array(), [x; 2]);
    assert_eq!(vec3!(x).to_array(), [x; 3]);
    assert_eq!(vec4!(x).to_array(), [x; 4]);

    assert_eq!(vec2!(x).align().to_alignment::<A>(), vec2!(x));
    assert_eq!(vec3!(x).align().to_alignment::<A>(), vec3!(x));
    assert_eq!(vec4!(x).align().to_alignment::<A>(), vec4!(x));

    assert_eq!(vec2!(x).unalign().to_alignment::<A>(), vec2!(x));
    assert_eq!(vec3!(x).unalign().to_alignment::<A>(), vec3!(x));
    assert_eq!(vec4!(x).unalign().to_alignment::<A>(), vec4!(x));

    assert_eq!(vec2!(x, y).as_array_ref(), &[x, y]);
    assert_eq!(vec3!(x, y, z).as_array_ref(), &[x, y, z]);
    assert_eq!(vec4!(x, y, z, w).as_array_ref(), &[x, y, z, w]);

    assert_eq!(vec2!(x, y).as_array_mut(), &mut [x, y]);
    assert_eq!(vec3!(x, y, z).as_array_mut(), &mut [x, y, z]);
    assert_eq!(vec4!(x, y, z, w).as_array_mut(), &mut [x, y, z, w]);

    assert_eq!(vec2!(x, y).iter().collect::<Vec<T>>(), vec![x, y]);
    assert_eq!(vec3!(x, y, z).iter().collect::<Vec<T>>(), vec![x, y, z]);
    assert_eq!(
        vec4!(x, y, z, w).iter().collect::<Vec<T>>(),
        vec![x, y, z, w]
    );

    assert_eq!(
        vec2!(x, y).iter_mut().map(|val| *val).collect::<Vec<T>>(),
        vec![x, y]
    );
    assert_eq!(
        vec3!(x, y, z)
            .iter_mut()
            .map(|val| *val)
            .collect::<Vec<T>>(),
        vec![x, y, z]
    );
    assert_eq!(
        vec4!(x, y, z, w)
            .iter_mut()
            .map(|val| *val)
            .collect::<Vec<T>>(),
        vec![x, y, z, w]
    );

    assert_eq!(vec2!(x, y).map(|val| val == x).to_array(), [true, false]);
    assert_eq!(vec2!(x, y).map(|val| val == y).to_array(), [false, true]);
    assert_eq!(
        vec3!(x, y, z).map(|val| val == x).to_array(),
        [true, false, x == z]
    );
    assert_eq!(
        vec3!(x, y, z).map(|val| val == y).to_array(),
        [false, true, false]
    );
    assert_eq!(
        vec3!(x, y, z).map(|val| val == z).to_array(),
        [x == z, false, true]
    );
    assert_eq!(
        vec4!(x, y, z, w).map(|val| val == x).to_array(),
        [true, false, x == z, false]
    );
    assert_eq!(
        vec4!(x, y, z, w).map(|val| val == y).to_array(),
        [false, true, false, y == w]
    );
    assert_eq!(
        vec4!(x, y, z, w).map(|val| val == z).to_array(),
        [x == z, false, true, false]
    );
    assert_eq!(
        vec4!(x, y, z, w).map(|val| val == w).to_array(),
        [false, y == w, false, true]
    );

    assert_eq!(vec2!(x, y).get(0), Some(x));
    assert_eq!(vec2!(x, y).get(1), Some(y));
    assert_eq!(vec2!(x, y).get(2), None);
    assert_eq!(vec2!(x, y).get(3), None);
    assert_eq!(vec3!(x, y, z).get(0), Some(x));
    assert_eq!(vec3!(x, y, z).get(1), Some(y));
    assert_eq!(vec3!(x, y, z).get(2), Some(z));
    assert_eq!(vec3!(x, y, z).get(3), None);
    assert_eq!(vec3!(x, y, z).get(4), None);
    assert_eq!(vec4!(x, y, z, w).get(0), Some(x));
    assert_eq!(vec4!(x, y, z, w).get(1), Some(y));
    assert_eq!(vec4!(x, y, z, w).get(2), Some(z));
    assert_eq!(vec4!(x, y, z, w).get(3), Some(w));
    assert_eq!(vec4!(x, y, z, w).get(4), None);
    assert_eq!(vec4!(x, y, z, w).get(5), None);

    assert_eq!(vec2!(x, y).get_mut(0), Some(&mut x.clone()));
    assert_eq!(vec2!(x, y).get_mut(1), Some(&mut y.clone()));
    assert_eq!(vec2!(x, y).get_mut(2), None);
    assert_eq!(vec2!(x, y).get_mut(3), None);
    assert_eq!(vec3!(x, y, z).get_mut(0), Some(&mut x.clone()));
    assert_eq!(vec3!(x, y, z).get_mut(1), Some(&mut y.clone()));
    assert_eq!(vec3!(x, y, z).get_mut(2), Some(&mut z.clone()));
    assert_eq!(vec3!(x, y, z).get_mut(3), None);
    assert_eq!(vec3!(x, y, z).get_mut(4), None);
    assert_eq!(vec4!(x, y, z, w).get_mut(0), Some(&mut x.clone()));
    assert_eq!(vec4!(x, y, z, w).get_mut(1), Some(&mut y.clone()));
    assert_eq!(vec4!(x, y, z, w).get_mut(2), Some(&mut z.clone()));
    assert_eq!(vec4!(x, y, z, w).get_mut(3), Some(&mut w.clone()));
    assert_eq!(vec4!(x, y, z, w).get_mut(4), None);
    assert_eq!(vec4!(x, y, z, w).get_mut(5), None);

    assert_eq!(vec2!(x, y).xy(), vec2!(x, y));
    assert_eq!(vec3!(x, y, z).xy(), vec2!(x, y));
    assert_eq!(vec4!(x, y, z, w).xy(), vec2!(x, y));
    assert_eq!(vec2!(x, y).xx(), vec2!(x, x));
    assert_eq!(vec3!(x, y, z).xx(), vec2!(x, x));
    assert_eq!(vec4!(x, y, z, w).xx(), vec2!(x, x));
    assert_eq!(vec2!(x, y).yy(), vec2!(y, y));
    assert_eq!(vec3!(x, y, z).yy(), vec2!(y, y));
    assert_eq!(vec4!(x, y, z, w).yy(), vec2!(y, y));
    assert_eq!(vec2!(x, y).yx(), vec2!(y, x));
    assert_eq!(vec3!(x, y, z).yx(), vec2!(y, x));
    assert_eq!(vec4!(x, y, z, w).yx(), vec2!(y, x));

    assert_eq!(vec2!(x, y).xyx(), vec3!(x, y, x));
    assert_eq!(vec3!(x, y, z).xyz(), vec3!(x, y, z));
    assert_eq!(vec4!(x, y, z, w).xyz(), vec3!(x, y, z));
    assert_eq!(vec2!(x, y).yxy(), vec3!(y, x, y));
    assert_eq!(vec3!(x, y, z).zyx(), vec3!(z, y, x));
    assert_eq!(vec4!(x, y, z, w).wzy(), vec3!(w, z, y));
    assert_eq!(vec3!(x, y, z).yyy(), vec3!(y, y, y));
    assert_eq!(vec4!(x, y, z, w).zzz(), vec3!(z, z, z));
    assert_eq!(vec4!(x, y, z, w).xzw(), vec3!(x, z, w));
    assert_eq!(vec4!(x, y, z, w).wxy(), vec3!(w, x, y));

    assert_eq!(vec2!(x, y).xyxy(), vec4!(x, y, x, y));
    assert_eq!(vec3!(x, y, z).xyzx(), vec4!(x, y, z, x));
    assert_eq!(vec4!(x, y, z, w).xyzw(), vec4!(x, y, z, w));
    assert_eq!(vec3!(x, y, z).zyxx(), vec4!(z, y, x, x));
    assert_eq!(vec4!(x, y, z, w).wzyx(), vec4!(w, z, y, x));
    assert_eq!(vec4!(x, y, z, w).yyyy(), vec4!(y, y, y, y));
    assert_eq!(vec4!(x, y, z, w).zwzw(), vec4!(z, w, z, w));
    assert_eq!(vec4!(x, y, z, w).xxzw(), vec4!(x, x, z, w));
    assert_eq!(vec4!(x, y, z, w).wwxy(), vec4!(w, w, x, y));

    assert_eq!(vec2!(x, y).reverse(), vec2!(y, x));
    assert_eq!(vec3!(x, y, z).reverse(), vec3!(z, y, x));
    assert_eq!(vec4!(x, y, z, w).reverse(), vec4!(w, z, y, x));

    assert_eq!(vec2!(x, y).x, x);
    assert_eq!(vec2!(x, y).y, y);
    assert_eq!(vec3!(x, y, z).x, x);
    assert_eq!(vec3!(x, y, z).y, y);
    assert_eq!(vec3!(x, y, z).z, z);
    assert_eq!(vec4!(x, y, z, w).x, x);
    assert_eq!(vec4!(x, y, z, w).y, y);
    assert_eq!(vec4!(x, y, z, w).z, z);
    assert_eq!(vec4!(x, y, z, w).w, w);

    assert_eq!(vec2!(x, y).with_x(z), vec2!(z, y));
    assert_eq!(vec2!(x, y).with_y(z), vec2!(x, z));
    assert_eq!(vec3!(x, y, z).with_x(w), vec3!(w, y, z));
    assert_eq!(vec3!(x, y, z).with_y(w), vec3!(x, w, z));
    assert_eq!(vec3!(x, y, z).with_z(w), vec3!(x, y, w));
    assert_eq!(vec4!(x, y, z, w).with_x(y), vec4!(y, y, z, w));
    assert_eq!(vec4!(x, y, z, w).with_y(z), vec4!(x, z, z, w));
    assert_eq!(vec4!(x, y, z, w).with_z(w), vec4!(x, y, w, w));
    assert_eq!(vec4!(x, y, z, w).with_w(x), vec4!(x, y, z, x));

    let mut v = vec2!(x, y);
    v.x = z;
    assert_eq!(v, vec2!(z, y));
    v.y = w;
    assert_eq!(v, vec2!(z, w));

    let mut v = vec3!(x, y, z);
    v.x = w;
    assert_eq!(v, vec3!(w, y, z));
    v.y = x;
    assert_eq!(v, vec3!(w, x, z));
    v.z = y;
    assert_eq!(v, vec3!(w, x, y));

    let mut v = vec4!(x, y, z, w);
    v.x = y;
    assert_eq!(v, vec4!(y, y, z, w));
    v.y = z;
    assert_eq!(v, vec4!(y, z, z, w));
    v.z = w;
    assert_eq!(v, vec4!(y, z, w, w));
    v.w = x;
    assert_eq!(v, vec4!(y, z, w, x));

    assert_eq!(vec2!(x, y)[0], x);
    assert_eq!(vec2!(x, y)[1], y);
    assert_eq!(vec3!(x, y, z)[0], x);
    assert_eq!(vec3!(x, y, z)[1], y);
    assert_eq!(vec3!(x, y, z)[2], z);
    assert_eq!(vec4!(x, y, z, w)[0], x);
    assert_eq!(vec4!(x, y, z, w)[1], y);
    assert_eq!(vec4!(x, y, z, w)[2], z);
    assert_eq!(vec4!(x, y, z, w)[3], w);

    assert_panic!(vec2!(x, y)[2]);
    assert_panic!(vec3!(x, y, z)[3]);
    assert_panic!(vec4!(x, y, z, w)[4]);

    assert_eq!(&mut vec2!(x, y)[0], &mut x.clone());
    assert_eq!(&mut vec2!(x, y)[1], &mut y.clone());
    assert_eq!(&mut vec3!(x, y, z)[0], &mut x.clone());
    assert_eq!(&mut vec3!(x, y, z)[1], &mut y.clone());
    assert_eq!(&mut vec3!(x, y, z)[2], &mut z.clone());
    assert_eq!(&mut vec4!(x, y, z, w)[0], &mut x.clone());
    assert_eq!(&mut vec4!(x, y, z, w)[1], &mut y.clone());
    assert_eq!(&mut vec4!(x, y, z, w)[2], &mut z.clone());
    assert_eq!(&mut vec4!(x, y, z, w)[3], &mut w.clone());

    assert_panic!(&mut vec2!(x, y)[2]);
    assert_panic!(&mut vec3!(x, y, z)[3]);
    assert_panic!(&mut vec4!(x, y, z, w)[4]);

    assert_eq!(vec2!(x, y).into_iter().collect::<Vec<T>>(), vec![x, y]);
    assert_eq!(
        vec3!(x, y, z).into_iter().collect::<Vec<T>>(),
        vec![x, y, z]
    );
    assert_eq!(
        vec4!(x, y, z, w).into_iter().collect::<Vec<T>>(),
        vec![x, y, z, w]
    );

    assert_eq!(
        (&mut vec2!(x, y))
            .into_iter()
            .map(|val| *val)
            .collect::<Vec<T>>(),
        vec![x, y]
    );
    assert_eq!(
        (&mut vec3!(x, y, z))
            .into_iter()
            .map(|val| *val)
            .collect::<Vec<T>>(),
        vec![x, y, z]
    );
    assert_eq!(
        (&mut vec4!(x, y, z, w))
            .into_iter()
            .map(|val| *val)
            .collect::<Vec<T>>(),
        vec![x, y, z, w]
    );

    assert_eq!(format!("{:?}", vec2!(x, y)), format!("({x:?}, {y:?})"));
    assert_eq!(
        format!("{:?}", vec3!(x, y, z)),
        format!("({x:?}, {y:?}, {z:?})")
    );
    assert_eq!(
        format!("{:?}", vec4!(x, y, z, w)),
        format!("({x:?}, {y:?}, {z:?}, {w:?})")
    );

    assert_eq!(format!("{}", vec2!(x, y)), format!("({x}, {y})"));
    assert_eq!(format!("{}", vec3!(x, y, z)), format!("({x}, {y}, {z})"));
    assert_eq!(
        format!("{}", vec4!(x, y, z, w)),
        format!("({x}, {y}, {z}, {w})")
    );

    assert_eq!(Vector::<2, T, A>::default(), vec2!(T::default()));
    assert_eq!(Vector::<3, T, A>::default(), vec3!(T::default()));
    assert_eq!(Vector::<4, T, A>::default(), vec4!(T::default()));

    assert_eq!(vec2!(x, y) == vec2!(x, y), true);
    assert_eq!(vec3!(x, y, z) == vec3!(x, y, z), true);
    assert_eq!(vec4!(x, y, z, w) == vec4!(x, y, z, w), true);
    assert_eq!(vec2!(y, y) == vec2!(x, y), false);
    assert_eq!(vec3!(y, y, z) == vec3!(x, y, z), false);
    assert_eq!(vec4!(y, y, z, w) == vec4!(x, y, z, w), false);
    assert_eq!(vec2!(x, x) == vec2!(x, y), false);
    assert_eq!(vec3!(x, x, z) == vec3!(x, y, z), false);
    assert_eq!(vec4!(x, x, z, w) == vec4!(x, y, z, w), false);
    assert_eq!(vec2!(y, x) == vec2!(x, y), false);
    assert_eq!(vec3!(y, w, x) == vec3!(x, y, z), false);
    assert_eq!(vec4!(y, w, w, z) == vec4!(x, y, z, w), false);

    assert_eq!(vec2!(x, y) != vec2!(x, y), false);
    assert_eq!(vec3!(x, y, z) != vec3!(x, y, z), false);
    assert_eq!(vec4!(x, y, z, w) != vec4!(x, y, z, w), false);
    assert_eq!(vec2!(y, y) != vec2!(x, y), true);
    assert_eq!(vec3!(y, y, z) != vec3!(x, y, z), true);
    assert_eq!(vec4!(y, y, z, w) != vec4!(x, y, z, w), true);
    assert_eq!(vec2!(x, x) != vec2!(x, y), true);
    assert_eq!(vec3!(x, x, z) != vec3!(x, y, z), true);
    assert_eq!(vec4!(x, x, z, w) != vec4!(x, y, z, w), true);
    assert_eq!(vec2!(y, x) != vec2!(x, y), true);
    assert_eq!(vec3!(y, w, x) != vec3!(x, y, z), true);
    assert_eq!(vec4!(y, w, w, z) != vec4!(x, y, z, w), true);
}
