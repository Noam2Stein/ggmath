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
