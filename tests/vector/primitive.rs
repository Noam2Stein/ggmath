use std::{
    fmt::{Debug, Display},
    panic::{RefUnwindSafe, UnwindSafe},
};

use ggmath::{Alignment, Scalar, Vector};

use crate::assert_panic;

pub fn test<T, A: Alignment>(x: T, y: T, z: T, w: T)
where
    T: Scalar + Debug + Display + PartialEq + PartialOrd + Default + UnwindSafe + RefUnwindSafe,
{
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

    assert_eq!(Vector::<2, T, A>::new(x, y).to_array(), [x, y]);
    assert_eq!(Vector::<3, T, A>::new(x, y, z).to_array(), [x, y, z]);
    assert_eq!(Vector::<4, T, A>::new(x, y, z, w).to_array(), [x, y, z, w]);

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

    assert_eq!(
        Vector::<2, T, A>::new(x, y).align().to_alignment::<A>(),
        Vector::<2, T, A>::new(x, y)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).align().to_alignment::<A>(),
        Vector::<3, T, A>::new(x, y, z)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, x)
            .align()
            .to_alignment::<A>(),
        Vector::<4, T, A>::new(x, y, z, x)
    );

    assert_eq!(
        Vector::<2, T, A>::new(x, y).unalign().to_alignment::<A>(),
        Vector::<2, T, A>::new(x, y)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z)
            .unalign()
            .to_alignment::<A>(),
        Vector::<3, T, A>::new(x, y, z)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, x)
            .unalign()
            .to_alignment::<A>(),
        Vector::<4, T, A>::new(x, y, z, x)
    );

    assert_eq!(Vector::<2, T, A>::new(x, y).as_array_ref(), &[x, y]);
    assert_eq!(Vector::<3, T, A>::new(x, y, z).as_array_ref(), &[x, y, z]);
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).as_array_ref(),
        &[x, y, z, w]
    );

    assert_eq!(Vector::<2, T, A>::new(x, y).as_array_mut(), &mut [x, y]);
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).as_array_mut(),
        &mut [x, y, z]
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).as_array_mut(),
        &mut [x, y, z, w]
    );

    assert_eq!(
        Vector::<2, T, A>::new(x, y).iter().collect::<Vec<T>>(),
        vec![x, y]
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).iter().collect::<Vec<T>>(),
        vec![x, y, z]
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w)
            .iter()
            .collect::<Vec<T>>(),
        vec![x, y, z, w]
    );

    assert_eq!(
        Vector::<2, T, A>::new(x, y)
            .iter_mut()
            .map(|val| *val)
            .collect::<Vec<T>>(),
        vec![x, y]
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z)
            .iter_mut()
            .map(|val| *val)
            .collect::<Vec<T>>(),
        vec![x, y, z]
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w)
            .iter_mut()
            .map(|val| *val)
            .collect::<Vec<T>>(),
        vec![x, y, z, w]
    );

    assert_eq!(
        Vector::<2, T, A>::new(x, y).map(|val| val == x).to_array(),
        [true, false]
    );
    assert_eq!(
        Vector::<2, T, A>::new(x, y).map(|val| val == y).to_array(),
        [false, true]
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z)
            .map(|val| val == x)
            .to_array(),
        [true, false, x == z]
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z)
            .map(|val| val == y)
            .to_array(),
        [false, true, false]
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z)
            .map(|val| val == z)
            .to_array(),
        [x == z, false, true]
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w)
            .map(|val| val == x)
            .to_array(),
        [true, false, x == z, false]
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w)
            .map(|val| val == y)
            .to_array(),
        [false, true, false, y == w]
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w)
            .map(|val| val == z)
            .to_array(),
        [x == z, false, true, false]
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w)
            .map(|val| val == w)
            .to_array(),
        [false, y == w, false, true]
    );

    assert_eq!(
        Vector::<2, T, A>::new(x, y).xy(),
        Vector::<2, T, A>::new(x, y)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).xy(),
        Vector::<2, T, A>::new(x, y)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).xy(),
        Vector::<2, T, A>::new(x, y)
    );
    assert_eq!(
        Vector::<2, T, A>::new(x, y).xx(),
        Vector::<2, T, A>::new(x, x)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).xx(),
        Vector::<2, T, A>::new(x, x)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).xx(),
        Vector::<2, T, A>::new(x, x)
    );
    assert_eq!(
        Vector::<2, T, A>::new(x, y).yy(),
        Vector::<2, T, A>::new(y, y)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).yy(),
        Vector::<2, T, A>::new(y, y)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).yy(),
        Vector::<2, T, A>::new(y, y)
    );
    assert_eq!(
        Vector::<2, T, A>::new(x, y).yx(),
        Vector::<2, T, A>::new(y, x)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).yx(),
        Vector::<2, T, A>::new(y, x)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).yx(),
        Vector::<2, T, A>::new(y, x)
    );

    assert_eq!(
        Vector::<2, T, A>::new(x, y).xyx(),
        Vector::<3, T, A>::new(x, y, x)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).xyz(),
        Vector::<3, T, A>::new(x, y, z)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).xyz(),
        Vector::<3, T, A>::new(x, y, z)
    );
    assert_eq!(
        Vector::<2, T, A>::new(x, y).yxy(),
        Vector::<3, T, A>::new(y, x, y)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).zyx(),
        Vector::<3, T, A>::new(z, y, x)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).wzy(),
        Vector::<3, T, A>::new(w, z, y)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).yyy(),
        Vector::<3, T, A>::new(y, y, y)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).zzz(),
        Vector::<3, T, A>::new(z, z, z)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).xzw(),
        Vector::<3, T, A>::new(x, z, w)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).wxy(),
        Vector::<3, T, A>::new(w, x, y)
    );

    assert_eq!(
        Vector::<2, T, A>::new(x, y).xyxy(),
        Vector::<4, T, A>::new(x, y, x, y)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).xyzx(),
        Vector::<4, T, A>::new(x, y, z, x)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).xyzw(),
        Vector::<4, T, A>::new(x, y, z, w)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).zyxx(),
        Vector::<4, T, A>::new(z, y, x, x)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).wzyx(),
        Vector::<4, T, A>::new(w, z, y, x)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).yyyy(),
        Vector::<4, T, A>::new(y, y, y, y)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).zwzw(),
        Vector::<4, T, A>::new(z, w, z, w)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).xxzw(),
        Vector::<4, T, A>::new(x, x, z, w)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).wwxy(),
        Vector::<4, T, A>::new(w, w, x, y)
    );

    assert_eq!(
        Vector::<2, T, A>::new(x, y).reverse(),
        Vector::<2, T, A>::new(y, x)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).reverse(),
        Vector::<3, T, A>::new(z, y, x)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).reverse(),
        Vector::<4, T, A>::new(w, z, y, x)
    );

    assert_eq!(Vector::<2, T, A>::new(x, y).x, x);
    assert_eq!(Vector::<2, T, A>::new(x, y).y, y);
    assert_eq!(Vector::<3, T, A>::new(x, y, z).x, x);
    assert_eq!(Vector::<3, T, A>::new(x, y, z).y, y);
    assert_eq!(Vector::<3, T, A>::new(x, y, z).z, z);
    assert_eq!(Vector::<4, T, A>::new(x, y, z, w).x, x);
    assert_eq!(Vector::<4, T, A>::new(x, y, z, w).y, y);
    assert_eq!(Vector::<4, T, A>::new(x, y, z, w).z, z);
    assert_eq!(Vector::<4, T, A>::new(x, y, z, w).w, w);

    assert_eq!(
        Vector::<2, T, A>::new(x, y).with_x(z),
        Vector::<2, T, A>::new(z, y)
    );
    assert_eq!(
        Vector::<2, T, A>::new(x, y).with_y(z),
        Vector::<2, T, A>::new(x, z)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).with_x(w),
        Vector::<3, T, A>::new(w, y, z)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).with_y(w),
        Vector::<3, T, A>::new(x, w, z)
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z).with_z(w),
        Vector::<3, T, A>::new(x, y, w)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).with_x(y),
        Vector::<4, T, A>::new(y, y, z, w)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).with_y(z),
        Vector::<4, T, A>::new(x, z, z, w)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).with_z(w),
        Vector::<4, T, A>::new(x, y, w, w)
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w).with_w(x),
        Vector::<4, T, A>::new(x, y, z, x)
    );

    let mut v = Vector::<2, T, A>::new(x, y);
    v.x = z;
    assert_eq!(v, Vector::<2, T, A>::new(z, y));
    v.y = w;
    assert_eq!(v, Vector::<2, T, A>::new(z, w));

    let mut v = Vector::<3, T, A>::new(x, y, z);
    v.x = w;
    assert_eq!(v, Vector::<3, T, A>::new(w, y, z));
    v.y = x;
    assert_eq!(v, Vector::<3, T, A>::new(w, x, z));
    v.z = y;
    assert_eq!(v, Vector::<3, T, A>::new(w, x, y));

    let mut v = Vector::<4, T, A>::new(x, y, z, w);
    v.x = y;
    assert_eq!(v, Vector::<4, T, A>::new(y, y, z, w));
    v.y = z;
    assert_eq!(v, Vector::<4, T, A>::new(y, z, z, w));
    v.z = w;
    assert_eq!(v, Vector::<4, T, A>::new(y, z, w, w));
    v.w = x;
    assert_eq!(v, Vector::<4, T, A>::new(y, z, w, x));

    assert_eq!(Vector::<2, T, A>::new(x, y)[0], x);
    assert_eq!(Vector::<2, T, A>::new(x, y)[1], y);
    assert_eq!(Vector::<3, T, A>::new(x, y, z)[0], x);
    assert_eq!(Vector::<3, T, A>::new(x, y, z)[1], y);
    assert_eq!(Vector::<3, T, A>::new(x, y, z)[2], z);
    assert_eq!(Vector::<4, T, A>::new(x, y, z, w)[0], x);
    assert_eq!(Vector::<4, T, A>::new(x, y, z, w)[1], y);
    assert_eq!(Vector::<4, T, A>::new(x, y, z, w)[2], z);
    assert_eq!(Vector::<4, T, A>::new(x, y, z, w)[3], w);

    assert_panic!(Vector::<2, T, A>::new(x, y)[2]);
    assert_panic!(Vector::<3, T, A>::new(x, y, z)[3]);
    assert_panic!(Vector::<4, T, A>::new(x, y, z, w)[4]);

    assert_eq!(&mut Vector::<2, T, A>::new(x, y)[0], &mut x.clone());
    assert_eq!(&mut Vector::<2, T, A>::new(x, y)[1], &mut y.clone());
    assert_eq!(&mut Vector::<3, T, A>::new(x, y, z)[0], &mut x.clone());
    assert_eq!(&mut Vector::<3, T, A>::new(x, y, z)[1], &mut y.clone());
    assert_eq!(&mut Vector::<3, T, A>::new(x, y, z)[2], &mut z.clone());
    assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w)[0], &mut x.clone());
    assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w)[1], &mut y.clone());
    assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w)[2], &mut z.clone());
    assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w)[3], &mut w.clone());

    assert_panic!(&mut Vector::<2, T, A>::new(x, y)[2]);
    assert_panic!(&mut Vector::<3, T, A>::new(x, y, z)[3]);
    assert_panic!(&mut Vector::<4, T, A>::new(x, y, z, w)[4]);

    assert_eq!(
        Vector::<2, T, A>::new(x, y).into_iter().collect::<Vec<T>>(),
        vec![x, y]
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z)
            .into_iter()
            .collect::<Vec<T>>(),
        vec![x, y, z]
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w)
            .into_iter()
            .collect::<Vec<T>>(),
        vec![x, y, z, w]
    );

    assert_eq!(
        (&mut Vector::<2, T, A>::new(x, y))
            .into_iter()
            .map(|val| *val)
            .collect::<Vec<T>>(),
        vec![x, y]
    );
    assert_eq!(
        (&mut Vector::<3, T, A>::new(x, y, z))
            .into_iter()
            .map(|val| *val)
            .collect::<Vec<T>>(),
        vec![x, y, z]
    );
    assert_eq!(
        (&mut Vector::<4, T, A>::new(x, y, z, w))
            .into_iter()
            .map(|val| *val)
            .collect::<Vec<T>>(),
        vec![x, y, z, w]
    );

    assert_eq!(
        format!("{:?}", Vector::<2, T, A>::new(x, y)),
        format!("({x:?}, {y:?})")
    );
    assert_eq!(
        format!("{:?}", Vector::<3, T, A>::new(x, y, z)),
        format!("({x:?}, {y:?}, {z:?})")
    );
    assert_eq!(
        format!("{:?}", Vector::<4, T, A>::new(x, y, z, w)),
        format!("({x:?}, {y:?}, {z:?}, {w:?})")
    );

    assert_eq!(
        format!("{}", Vector::<2, T, A>::new(x, y)),
        format!("({x}, {y})")
    );
    assert_eq!(
        format!("{}", Vector::<3, T, A>::new(x, y, z)),
        format!("({x}, {y}, {z})")
    );
    assert_eq!(
        format!("{}", Vector::<4, T, A>::new(x, y, z, w)),
        format!("({x}, {y}, {z}, {w})")
    );

    assert_eq!(
        Vector::<2, T, A>::default(),
        Vector::<2, T, A>::splat(T::default())
    );
    assert_eq!(
        Vector::<3, T, A>::default(),
        Vector::<3, T, A>::splat(T::default())
    );
    assert_eq!(
        Vector::<4, T, A>::default(),
        Vector::<4, T, A>::splat(T::default())
    );

    assert_eq!(
        Vector::<2, T, A>::new(x, y) == Vector::<2, T, A>::new(x, y),
        true
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z) == Vector::<3, T, A>::new(x, y, z),
        true
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w) == Vector::<4, T, A>::new(x, y, z, w),
        true
    );
    assert_eq!(
        Vector::<2, T, A>::new(y, y) == Vector::<2, T, A>::new(x, y),
        false
    );
    assert_eq!(
        Vector::<3, T, A>::new(y, y, z) == Vector::<3, T, A>::new(x, y, z),
        false
    );
    assert_eq!(
        Vector::<4, T, A>::new(y, y, z, w) == Vector::<4, T, A>::new(x, y, z, w),
        false
    );
    assert_eq!(
        Vector::<2, T, A>::new(x, x) == Vector::<2, T, A>::new(x, y),
        false
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, x, z) == Vector::<3, T, A>::new(x, y, z),
        false
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, x, z, w) == Vector::<4, T, A>::new(x, y, z, w),
        false
    );
    assert_eq!(
        Vector::<2, T, A>::new(y, x) == Vector::<2, T, A>::new(x, y),
        false
    );
    assert_eq!(
        Vector::<3, T, A>::new(y, w, x) == Vector::<3, T, A>::new(x, y, z),
        false
    );
    assert_eq!(
        Vector::<4, T, A>::new(y, w, w, z) == Vector::<4, T, A>::new(x, y, z, w),
        false
    );

    assert_eq!(
        Vector::<2, T, A>::new(x, y) != Vector::<2, T, A>::new(x, y),
        false
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, y, z) != Vector::<3, T, A>::new(x, y, z),
        false
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, y, z, w) != Vector::<4, T, A>::new(x, y, z, w),
        false
    );
    assert_eq!(
        Vector::<2, T, A>::new(y, y) != Vector::<2, T, A>::new(x, y),
        true
    );
    assert_eq!(
        Vector::<3, T, A>::new(y, y, z) != Vector::<3, T, A>::new(x, y, z),
        true
    );
    assert_eq!(
        Vector::<4, T, A>::new(y, y, z, w) != Vector::<4, T, A>::new(x, y, z, w),
        true
    );
    assert_eq!(
        Vector::<2, T, A>::new(x, x) != Vector::<2, T, A>::new(x, y),
        true
    );
    assert_eq!(
        Vector::<3, T, A>::new(x, x, z) != Vector::<3, T, A>::new(x, y, z),
        true
    );
    assert_eq!(
        Vector::<4, T, A>::new(x, x, z, w) != Vector::<4, T, A>::new(x, y, z, w),
        true
    );
    assert_eq!(
        Vector::<2, T, A>::new(y, x) != Vector::<2, T, A>::new(x, y),
        true
    );
    assert_eq!(
        Vector::<3, T, A>::new(y, w, x) != Vector::<3, T, A>::new(x, y, z),
        true
    );
    assert_eq!(
        Vector::<4, T, A>::new(y, w, w, z) != Vector::<4, T, A>::new(x, y, z, w),
        true
    );
}
