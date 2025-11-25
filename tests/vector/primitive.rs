use ggmath::Vector;

use crate::{assert_panic, vec2, vec3, vec4};

pub fn primitive_tests(a: T, b: T, c: T, d: T) {
    assert!(b > a);
    assert!(d > c);

    assert!(size_of::<Vec2>() == size_of::<[T; 2]>());
    assert!(size_of::<Vec3>() == size_of::<[T; 3]>() || size_of::<Vec3>() == size_of::<[T; 4]>());
    assert!(size_of::<Vec4>() == size_of::<[T; 4]>());

    assert!(align_of::<Vec2>() == align_of::<T>() || align_of::<Vec2>() == size_of::<Vec2>());
    assert!(align_of::<Vec3>() == align_of::<T>() || align_of::<Vec3>() == size_of::<Vec3>());
    assert!(align_of::<Vec4>() == align_of::<T>() || align_of::<Vec4>() == size_of::<Vec4>());

    assert_eq!(Vec2::from_array([a, b]).to_array(), [a, b]);
    assert_eq!(Vec3::from_array([a, b, c]).to_array(), [a, b, c]);
    assert_eq!(Vec4::from_array([a, b, c, d]).to_array(), [a, b, c, d]);

    assert_eq!(Vec2::splat(a).to_array(), [a; 2]);
    assert_eq!(Vec3::splat(a).to_array(), [a; 3]);
    assert_eq!(Vec4::splat(a).to_array(), [a; 4]);

    assert_eq!(Vec2::from_fn(|i| [a, b][i]).to_array(), [a, b]);
    assert_eq!(Vec3::from_fn(|i| [a, b, c][i]).to_array(), [a, b, c]);
    assert_eq!(Vec4::from_fn(|i| [a, b, c, d][i]).to_array(), [a, b, c, d]);

    assert_eq!(vec2!(a, b).to_array(), [a, b]);
    assert_eq!(vec3!(a, b, c).to_array(), [a, b, c]);
    assert_eq!(vec4!(a, b, c, d).to_array(), [a, b, c, d]);

    assert_eq!(vec2!(a).to_array(), [a; 2]);
    assert_eq!(vec3!(a).to_array(), [a; 3]);
    assert_eq!(vec4!(a).to_array(), [a; 4]);

    assert_eq!(vec2!(vec2!(a, b)).to_array(), [a, b]);
    assert_eq!(vec3!(vec3!(a, b, c)).to_array(), [a, b, c]);
    assert_eq!(vec4!(vec4!(a, b, c, d)).to_array(), [a, b, c, d]);

    assert_eq!(vec3!(a, vec2!(b, c)).to_array(), [a, b, c]);
    assert_eq!(vec3!(vec2!(a, b), c).to_array(), [a, b, c]);
    assert_eq!(vec4!(a, b, vec2!(c, d)).to_array(), [a, b, c, d]);
    assert_eq!(vec4!(a, vec2!(b, c), d).to_array(), [a, b, c, d]);
    assert_eq!(vec4!(a, vec3!(b, c, d)).to_array(), [a, b, c, d]);
    assert_eq!(vec4!(vec2!(a, b), vec2!(c, d)).to_array(), [a, b, c, d]);
    assert_eq!(vec4!(vec3!(a, b, c), d).to_array(), [a, b, c, d]);

    assert_eq!(vec2!(a).len(), 2);
    assert_eq!(vec3!(a).len(), 3);
    assert_eq!(vec4!(a).len(), 4);

    assert_eq!(vec2!(a, b).as_array_ref(), &[a, b]);
    assert_eq!(vec3!(a, b, c).as_array_ref(), &[a, b, c]);
    assert_eq!(vec4!(a, b, c, d).as_array_ref(), &[a, b, c, d]);

    assert_eq!(vec2!(a, b).as_array_mut(), &mut [a, b]);
    assert_eq!(vec3!(a, b, c).as_array_mut(), &mut [a, b, c]);
    assert_eq!(vec4!(a, b, c, d).as_array_mut(), &mut [a, b, c, d]);

    assert_eq!(vec2!(a, b).iter().collect::<Vec<T>>(), vec![a, b]);
    assert_eq!(vec3!(a, b, c).iter().collect::<Vec<T>>(), vec![a, b, c]);
    assert_eq!(
        vec4!(a, b, c, d).iter().collect::<Vec<T>>(),
        vec![a, b, c, d]
    );

    assert_eq!(
        vec2!(a, b).iter_mut().map(|x| *x).collect::<Vec<T>>(),
        vec![a, b]
    );
    assert_eq!(
        vec3!(a, b, c).iter_mut().map(|x| *x).collect::<Vec<T>>(),
        vec![a, b, c]
    );
    assert_eq!(
        vec4!(a, b, c, d).iter_mut().map(|x| *x).collect::<Vec<T>>(),
        vec![a, b, c, d]
    );

    assert_eq!(vec2!(a, b).map(|x| x == a).to_array(), [true, false]);
    assert_eq!(vec2!(a, b).map(|x| x == b).to_array(), [false, true]);
    assert_eq!(
        vec3!(a, b, c).map(|x| x == a).to_array(),
        [true, false, a == c]
    );
    assert_eq!(
        vec3!(a, b, c).map(|x| x == b).to_array(),
        [false, true, false]
    );
    assert_eq!(
        vec3!(a, b, c).map(|x| x == c).to_array(),
        [a == c, false, true]
    );
    assert_eq!(
        vec4!(a, b, c, d).map(|x| x == a).to_array(),
        [true, false, a == c, false]
    );
    assert_eq!(
        vec4!(a, b, c, d).map(|x| x == b).to_array(),
        [false, true, false, b == d]
    );
    assert_eq!(
        vec4!(a, b, c, d).map(|x| x == c).to_array(),
        [a == c, false, true, false]
    );
    assert_eq!(
        vec4!(a, b, c, d).map(|x| x == d).to_array(),
        [false, b == d, false, true]
    );

    assert_eq!(vec2!(a, b).get(0), Some(a));
    assert_eq!(vec2!(a, b).get(1), Some(b));
    assert_eq!(vec2!(a, b).get(2), None);
    assert_eq!(vec2!(a, b).get(3), None);
    assert_eq!(vec3!(a, b, c).get(0), Some(a));
    assert_eq!(vec3!(a, b, c).get(1), Some(b));
    assert_eq!(vec3!(a, b, c).get(2), Some(c));
    assert_eq!(vec3!(a, b, c).get(3), None);
    assert_eq!(vec3!(a, b, c).get(4), None);
    assert_eq!(vec4!(a, b, c, d).get(0), Some(a));
    assert_eq!(vec4!(a, b, c, d).get(1), Some(b));
    assert_eq!(vec4!(a, b, c, d).get(2), Some(c));
    assert_eq!(vec4!(a, b, c, d).get(3), Some(d));
    assert_eq!(vec4!(a, b, c, d).get(4), None);
    assert_eq!(vec4!(a, b, c, d).get(5), None);

    assert_eq!(vec2!(a, b).get_mut(0), Some(&mut a.clone()));
    assert_eq!(vec2!(a, b).get_mut(1), Some(&mut b.clone()));
    assert_eq!(vec2!(a, b).get_mut(2), None);
    assert_eq!(vec2!(a, b).get_mut(3), None);
    assert_eq!(vec3!(a, b, c).get_mut(0), Some(&mut a.clone()));
    assert_eq!(vec3!(a, b, c).get_mut(1), Some(&mut b.clone()));
    assert_eq!(vec3!(a, b, c).get_mut(2), Some(&mut c.clone()));
    assert_eq!(vec3!(a, b, c).get_mut(3), None);
    assert_eq!(vec3!(a, b, c).get_mut(4), None);
    assert_eq!(vec4!(a, b, c, d).get_mut(0), Some(&mut a.clone()));
    assert_eq!(vec4!(a, b, c, d).get_mut(1), Some(&mut b.clone()));
    assert_eq!(vec4!(a, b, c, d).get_mut(2), Some(&mut c.clone()));
    assert_eq!(vec4!(a, b, c, d).get_mut(3), Some(&mut d.clone()));
    assert_eq!(vec4!(a, b, c, d).get_mut(4), None);
    assert_eq!(vec4!(a, b, c, d).get_mut(5), None);

    assert_eq!(vec2!(a, b).swizzle2::<0, 1>(), vec2!(a, b));
    assert_eq!(vec3!(a, b, c).swizzle2::<0, 1>(), vec2!(a, b));
    assert_eq!(vec4!(a, b, c, d).swizzle2::<0, 1>(), vec2!(a, b));
    assert_eq!(vec2!(a, b).swizzle2::<0, 0>(), vec2!(a, a));
    assert_eq!(vec3!(a, b, c).swizzle2::<0, 0>(), vec2!(a, a));
    assert_eq!(vec4!(a, b, c, d).swizzle2::<0, 0>(), vec2!(a, a));
    assert_eq!(vec2!(a, b).swizzle2::<1, 1>(), vec2!(b, b));
    assert_eq!(vec3!(a, b, c).swizzle2::<1, 1>(), vec2!(b, b));
    assert_eq!(vec4!(a, b, c, d).swizzle2::<1, 1>(), vec2!(b, b));
    assert_eq!(vec2!(a, b).swizzle2::<1, 0>(), vec2!(b, a));
    assert_eq!(vec3!(a, b, c).swizzle2::<1, 0>(), vec2!(b, a));
    assert_eq!(vec4!(a, b, c, d).swizzle2::<1, 0>(), vec2!(b, a));

    assert_eq!(vec2!(a, b).swizzle3::<0, 1, 0>(), vec3!(a, b, a));
    assert_eq!(vec3!(a, b, c).swizzle3::<0, 1, 2>(), vec3!(a, b, c));
    assert_eq!(vec4!(a, b, c, d).swizzle3::<0, 1, 2>(), vec3!(a, b, c));
    assert_eq!(vec2!(a, b).swizzle3::<1, 0, 1>(), vec3!(b, a, b));
    assert_eq!(vec3!(a, b, c).swizzle3::<2, 1, 0>(), vec3!(c, b, a));
    assert_eq!(vec4!(a, b, c, d).swizzle3::<3, 2, 1>(), vec3!(d, c, b));
    assert_eq!(vec3!(a, b, c).swizzle3::<1, 1, 1>(), vec3!(b, b, b));
    assert_eq!(vec4!(a, b, c, d).swizzle3::<2, 2, 2>(), vec3!(c, c, c));
    assert_eq!(vec4!(a, b, c, d).swizzle3::<0, 2, 3>(), vec3!(a, c, d));
    assert_eq!(vec4!(a, b, c, d).swizzle3::<3, 0, 1>(), vec3!(d, a, b));

    assert_eq!(vec2!(a, b).swizzle4::<0, 1, 0, 1>(), vec4!(a, b, a, b));
    assert_eq!(vec3!(a, b, c).swizzle4::<0, 1, 2, 0>(), vec4!(a, b, c, a));
    assert_eq!(
        vec4!(a, b, c, d).swizzle4::<0, 1, 2, 3>(),
        vec4!(a, b, c, d)
    );
    assert_eq!(vec3!(a, b, c).swizzle4::<2, 1, 0, 0>(), vec4!(c, b, a, a));
    assert_eq!(
        vec4!(a, b, c, d).swizzle4::<3, 2, 1, 0>(),
        vec4!(d, c, b, a)
    );
    assert_eq!(
        vec4!(a, b, c, d).swizzle4::<1, 1, 1, 1>(),
        vec4!(b, b, b, b)
    );
    assert_eq!(
        vec4!(a, b, c, d).swizzle4::<2, 3, 2, 3>(),
        vec4!(c, d, c, d)
    );
    assert_eq!(
        vec4!(a, b, c, d).swizzle4::<0, 0, 2, 3>(),
        vec4!(a, a, c, d)
    );
    assert_eq!(
        vec4!(a, b, c, d).swizzle4::<3, 3, 0, 1>(),
        vec4!(d, d, a, b)
    );

    assert_eq!(vec2!(a, b).xy(), vec2!(a, b));
    assert_eq!(vec3!(a, b, c).xy(), vec2!(a, b));
    assert_eq!(vec4!(a, b, c, d).xy(), vec2!(a, b));
    assert_eq!(vec2!(a, b).xx(), vec2!(a, a));
    assert_eq!(vec3!(a, b, c).xx(), vec2!(a, a));
    assert_eq!(vec4!(a, b, c, d).xx(), vec2!(a, a));
    assert_eq!(vec2!(a, b).yy(), vec2!(b, b));
    assert_eq!(vec3!(a, b, c).yy(), vec2!(b, b));
    assert_eq!(vec4!(a, b, c, d).yy(), vec2!(b, b));
    assert_eq!(vec2!(a, b).yx(), vec2!(b, a));
    assert_eq!(vec3!(a, b, c).yx(), vec2!(b, a));
    assert_eq!(vec4!(a, b, c, d).yx(), vec2!(b, a));

    assert_eq!(vec2!(a, b).xyx(), vec3!(a, b, a));
    assert_eq!(vec3!(a, b, c).xyz(), vec3!(a, b, c));
    assert_eq!(vec4!(a, b, c, d).xyz(), vec3!(a, b, c));
    assert_eq!(vec2!(a, b).yxy(), vec3!(b, a, b));
    assert_eq!(vec3!(a, b, c).zyx(), vec3!(c, b, a));
    assert_eq!(vec4!(a, b, c, d).wzy(), vec3!(d, c, b));
    assert_eq!(vec3!(a, b, c).yyy(), vec3!(b, b, b));
    assert_eq!(vec4!(a, b, c, d).zzz(), vec3!(c, c, c));
    assert_eq!(vec4!(a, b, c, d).xzw(), vec3!(a, c, d));
    assert_eq!(vec4!(a, b, c, d).wxy(), vec3!(d, a, b));

    assert_eq!(vec2!(a, b).xyxy(), vec4!(a, b, a, b));
    assert_eq!(vec3!(a, b, c).xyzx(), vec4!(a, b, c, a));
    assert_eq!(vec4!(a, b, c, d).xyzw(), vec4!(a, b, c, d));
    assert_eq!(vec3!(a, b, c).zyxx(), vec4!(c, b, a, a));
    assert_eq!(vec4!(a, b, c, d).wzyx(), vec4!(d, c, b, a));
    assert_eq!(vec4!(a, b, c, d).yyyy(), vec4!(b, b, b, b));
    assert_eq!(vec4!(a, b, c, d).zwzw(), vec4!(c, d, c, d));
    assert_eq!(vec4!(a, b, c, d).xxzw(), vec4!(a, a, c, d));
    assert_eq!(vec4!(a, b, c, d).wwxy(), vec4!(d, d, a, b));

    assert_eq!(vec2!(a, b).reverse(), vec2!(b, a));
    assert_eq!(vec3!(a, b, c).reverse(), vec3!(c, b, a));
    assert_eq!(vec4!(a, b, c, d).reverse(), vec4!(d, c, b, a));

    assert_eq!(vec2!(a, b).x, a);
    assert_eq!(vec2!(a, b).y, b);
    assert_eq!(vec3!(a, b, c).x, a);
    assert_eq!(vec3!(a, b, c).y, b);
    assert_eq!(vec3!(a, b, c).z, c);
    assert_eq!(vec4!(a, b, c, d).x, a);
    assert_eq!(vec4!(a, b, c, d).y, b);
    assert_eq!(vec4!(a, b, c, d).z, c);
    assert_eq!(vec4!(a, b, c, d).w, d);

    assert_eq!(vec2!(a, b).with_x(c), vec2!(c, b));
    assert_eq!(vec2!(a, b).with_y(c), vec2!(a, c));
    assert_eq!(vec3!(a, b, c).with_x(d), vec3!(d, b, c));
    assert_eq!(vec3!(a, b, c).with_y(d), vec3!(a, d, c));
    assert_eq!(vec3!(a, b, c).with_z(d), vec3!(a, b, d));
    assert_eq!(vec4!(a, b, c, d).with_x(b), vec4!(b, b, c, d));
    assert_eq!(vec4!(a, b, c, d).with_y(c), vec4!(a, c, c, d));
    assert_eq!(vec4!(a, b, c, d).with_z(d), vec4!(a, b, d, d));
    assert_eq!(vec4!(a, b, c, d).with_w(a), vec4!(a, b, c, a));

    let mut v = vec2!(a, b);
    v.x = c;
    assert_eq!(v, vec2!(c, b));
    v.y = d;
    assert_eq!(v, vec2!(c, d));

    let mut v = vec3!(a, b, c);
    v.x = d;
    assert_eq!(v, vec3!(d, b, c));
    v.y = a;
    assert_eq!(v, vec3!(d, a, c));
    v.z = b;
    assert_eq!(v, vec3!(d, a, b));

    let mut v = vec4!(a, b, c, d);
    v.x = b;
    assert_eq!(v, vec4!(b, b, c, d));
    v.y = c;
    assert_eq!(v, vec4!(b, c, c, d));
    v.z = d;
    assert_eq!(v, vec4!(b, c, d, d));
    v.w = a;
    assert_eq!(v, vec4!(b, c, d, a));

    assert_eq!(Vec2::from([a, b]), vec2!(a, b));
    assert_eq!(Vec3::from([a, b, c]), vec3!(a, b, c));
    assert_eq!(Vec4::from([a, b, c, d]), vec4!(a, b, c, d));

    assert_eq!(<[T; 2]>::from(vec2!(a, b)), [a, b]);
    assert_eq!(<[T; 3]>::from(vec3!(a, b, c)), [a, b, c]);
    assert_eq!(<[T; 4]>::from(vec4!(a, b, c, d)), [a, b, c, d]);

    assert_eq!(vec2!(a, b)[0], a);
    assert_eq!(vec2!(a, b)[1], b);
    assert_eq!(vec3!(a, b, c)[0], a);
    assert_eq!(vec3!(a, b, c)[1], b);
    assert_eq!(vec3!(a, b, c)[2], c);
    assert_eq!(vec4!(a, b, c, d)[0], a);
    assert_eq!(vec4!(a, b, c, d)[1], b);
    assert_eq!(vec4!(a, b, c, d)[2], c);
    assert_eq!(vec4!(a, b, c, d)[3], d);

    assert_panic!(vec2!(a, b)[2]);
    assert_panic!(vec3!(a, b, c)[3]);
    assert_panic!(vec4!(a, b, c, d)[4]);

    assert_eq!(&mut vec2!(a, b)[0], &mut a.clone());
    assert_eq!(&mut vec2!(a, b)[1], &mut b.clone());
    assert_eq!(&mut vec3!(a, b, c)[0], &mut a.clone());
    assert_eq!(&mut vec3!(a, b, c)[1], &mut b.clone());
    assert_eq!(&mut vec3!(a, b, c)[2], &mut c.clone());
    assert_eq!(&mut vec4!(a, b, c, d)[0], &mut a.clone());
    assert_eq!(&mut vec4!(a, b, c, d)[1], &mut b.clone());
    assert_eq!(&mut vec4!(a, b, c, d)[2], &mut c.clone());
    assert_eq!(&mut vec4!(a, b, c, d)[3], &mut d.clone());

    assert_panic!(&mut vec2!(a, b)[2]);
    assert_panic!(&mut vec3!(a, b, c)[3]);
    assert_panic!(&mut vec4!(a, b, c, d)[4]);

    assert_eq!(vec2!(a, b).into_iter().collect::<Vec<T>>(), vec![a, b]);
    assert_eq!(
        vec3!(a, b, c).into_iter().collect::<Vec<T>>(),
        vec![a, b, c]
    );
    assert_eq!(
        vec4!(a, b, c, d).into_iter().collect::<Vec<T>>(),
        vec![a, b, c, d]
    );

    assert_eq!(
        (&mut vec2!(a, b))
            .into_iter()
            .map(|x: &mut T| *x)
            .collect::<Vec<T>>(),
        vec![a, b]
    );
    assert_eq!(
        (&mut vec3!(a, b, c))
            .into_iter()
            .map(|x: &mut T| *x)
            .collect::<Vec<T>>(),
        vec![a, b, c]
    );
    assert_eq!(
        (&mut vec4!(a, b, c, d))
            .into_iter()
            .map(|x: &mut T| *x)
            .collect::<Vec<T>>(),
        vec![a, b, c, d]
    );

    assert_eq!(format!("{:?}", vec2!(a, b)), format!("({a:?}, {b:?})"));
    assert_eq!(
        format!("{:?}", vec3!(a, b, c)),
        format!("({a:?}, {b:?}, {c:?})")
    );
    assert_eq!(
        format!("{:?}", vec4!(a, b, c, d)),
        format!("({a:?}, {b:?}, {c:?}, {d:?})")
    );

    assert_eq!(format!("{}", vec2!(a, b)), format!("({a}, {b})"));
    assert_eq!(format!("{}", vec3!(a, b, c)), format!("({a}, {b}, {c})"));
    assert_eq!(
        format!("{}", vec4!(a, b, c, d)),
        format!("({a}, {b}, {c}, {d})")
    );

    assert_eq!(Vec2::default(), vec2!(T::default()));
    assert_eq!(Vec3::default(), vec3!(T::default()));
    assert_eq!(Vec4::default(), vec4!(T::default()));

    assert_eq!(vec2!(a, b) == vec2!(a, b), true);
    assert_eq!(vec3!(a, b, c) == vec3!(a, b, c), true);
    assert_eq!(vec4!(a, b, c, d) == vec4!(a, b, c, d), true);
    assert_eq!(vec2!(b, b) == vec2!(a, b), false);
    assert_eq!(vec3!(b, b, c) == vec3!(a, b, c), false);
    assert_eq!(vec4!(b, b, c, d) == vec4!(a, b, c, d), false);
    assert_eq!(vec2!(a, a) == vec2!(a, b), false);
    assert_eq!(vec3!(a, a, c) == vec3!(a, b, c), false);
    assert_eq!(vec4!(a, a, c, d) == vec4!(a, b, c, d), false);
    assert_eq!(vec2!(b, a) == vec2!(a, b), false);
    assert_eq!(vec3!(b, d, a) == vec3!(a, b, c), false);
    assert_eq!(vec4!(b, d, d, c) == vec4!(a, b, c, d), false);

    assert_eq!(vec2!(a, b) != vec2!(a, b), false);
    assert_eq!(vec3!(a, b, c) != vec3!(a, b, c), false);
    assert_eq!(vec4!(a, b, c, d) != vec4!(a, b, c, d), false);
    assert_eq!(vec2!(b, b) != vec2!(a, b), true);
    assert_eq!(vec3!(b, b, c) != vec3!(a, b, c), true);
    assert_eq!(vec4!(b, b, c, d) != vec4!(a, b, c, d), true);
    assert_eq!(vec2!(a, a) != vec2!(a, b), true);
    assert_eq!(vec3!(a, a, c) != vec3!(a, b, c), true);
    assert_eq!(vec4!(a, a, c, d) != vec4!(a, b, c, d), true);
    assert_eq!(vec2!(b, a) != vec2!(a, b), true);
    assert_eq!(vec3!(b, d, a) != vec3!(a, b, c), true);
    assert_eq!(vec4!(b, d, d, c) != vec4!(a, b, c, d), true);
}

type Vec2 = Vector<2, T, A>;
type Vec3 = Vector<3, T, A>;
type Vec4 = Vector<4, T, A>;
