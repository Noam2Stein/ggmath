use ggmath::{Simdness, vec2, vec2g, vec2s, vec3, vec3g, vec3s, vec4, vec4g, vec4s};

use crate::{TestScalar, assert_panic, assert_val, test};

test! {
    primitive_api for T, S in [f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, bool], [Simd, NonSimd]:

    let [a, b, c, d] = T::TEST_VALUES;

    assert_val!(Vec2T::from_array([a, b]).as_array(), [a, b], "Vec2::from_array([{a}, {b}])");
    assert_val!(Vec3T::from_array([a, b, c]).as_array(), [a, b, c], "Vec3::from_array([{a}, {b}, {c}])");
    assert_val!(Vec4T::from_array([a, b, c, d]).as_array(), [a, b, c, d], "Vec4::from_array([{a}, {b}, {c}, {d}])");

    assert_val!(Vec2T::splat(a).as_array(), [a, a], "Vec2::splat({a})");
    assert_val!(Vec3T::splat(a).as_array(), [a, a, a], "Vec3::splat({a})");
    assert_val!(Vec4T::splat(a).as_array(), [a, a, a, a], "Vec4::splat({a})");

    assert_val!(Vec2T::from_fn(|i| [a, b][i]).as_array(), [a, b], "Vec2::from_fn(|i| [{a}, {b}][i])");
    assert_val!(Vec3T::from_fn(|i| [a, b, c][i]).as_array(), [a, b, c], "Vec3::from_fn(|i| [{a}, {b}, {c}][i])");
    assert_val!(Vec4T::from_fn(|i| [a, b, c, d][i]).as_array(), [a, b, c, d], "Vec4::from_fn(|i| [{a}, {b}, {c}, {d}][i])");

    assert_val!(Vec2T::const_from_array([a, b]).as_array(), [a, b], "Vec2::const_from_array([{a}, {b}])");
    assert_val!(Vec3T::const_from_array([a, b, c]).as_array(), [a, b, c], "Vec3::const_from_array([{a}, {b}, {c}])");
    assert_val!(Vec4T::const_from_array([a, b, c, d]).as_array(), [a, b, c, d], "Vec4::const_from_array([{a}, {b}, {c}, {d}])");

    assert_val!(vec2t!(a, b).len(), 2, "({a}, {b}).len()");
    assert_val!(vec3t!(a, b, c).len(), 3, "({a}, {b}, {c}).len()");
    assert_val!(vec4t!(a, b, c, d).len(), 4, "({a}, {b}, {c}, {d}).len()");

    assert_val!(vec2t!(a, b).is_simd(), S::IS_SIMD, "({a}, {b}).is_simd()");
    assert_val!(vec3t!(a, b, c).is_simd(), S::IS_SIMD, "({a}, {b}, {c}).is_simd()");
    assert_val!(vec4t!(a, b, c, d).is_simd(), S::IS_SIMD, "({a}, {b}, {c}, {d}).is_simd()");

    assert_val!(vec2t!(a, b).as_array(), [a, b], "vec2g!({a}, {b})");
    assert_val!(vec2t!(vec2t!(a, b)).as_array(), [a, b], "vec2g!(vec2g!({a}, {b}))");
    assert_val!(vec3t!(a, b, c).as_array(), [a, b, c], "vec3g!({a}, {b}, {c})");
    assert_val!(vec3t!(a, vec2t!(b, c)).as_array(), [a, b, c], "vec3g!({a}, vec2g!({b}, {c}))");
    assert_val!(vec4t!(a, b, c, d).as_array(), [a, b, c, d], "vec4g!({a}, {b}, {c}, {d})");
    assert_val!(vec4t!(a, vec2t!(b, c), d).as_array(), [a, b, c, d], "vec4g!({a}, vec2g!({b}, {c}), {d})");

    assert_val!(vec2t!(a).as_array(), [a; 2], "vec2g!({a})");
    assert_val!(vec3t!(a).as_array(), [a; 3], "vec3g!({a})");
    assert_val!(vec4t!(a).as_array(), [a; 4], "vec4g!({a})");

    assert_val!(*vec2t!(a, b).as_array_ref(), [a, b], "({a}, {b}).as_array_ref()");
    assert_val!(*vec3t!(a, b, c).as_array_ref(), [a, b, c], "({a}, {b}, {c}).as_array_ref()");
    assert_val!(*vec4t!(a, b, c, d).as_array_ref(), [a, b, c, d], "({a}, {b}, {c}, {d}).as_array_ref()");

    assert_val!(*vec2t!(a, b).as_mut_array(), [a, b], "({a}, {b}).as_mut_array()");
    assert_val!(*vec3t!(a, b, c).as_mut_array(), [a, b, c], "({a}, {b}, {c}).as_mut_array()");
    assert_val!(*vec4t!(a, b, c, d).as_mut_array(), [a, b, c, d], "({a}, {b}, {c}, {d}).as_mut_array()");

    assert_val!(vec2t!(a, b).as_simd(), vec2!(a, b), "({a}, {b}).as_simd()");
    assert_val!(vec3t!(a, b, c).as_simd(), vec3!(a, b, c), "({a}, {b}, {c}).as_simd()");
    assert_val!(vec4t!(a, b, c, d).as_simd(), vec4!(a, b, c, d), "({a}, {b}, {c}, {d}).as_simd()");

    assert_val!(vec2t!(a, b).as_nonsimd(), vec2s!(a, b), "({a}, {b}).as_nonsimd()");
    assert_val!(vec3t!(a, b, c).as_nonsimd(), vec3s!(a, b, c), "({a}, {b}, {c}).as_nonsimd()");
    assert_val!(vec4t!(a, b, c, d).as_nonsimd(), vec4s!(a, b, c, d), "({a}, {b}, {c}, {d}).as_nonsimd()");

    assert_val!(vec2t!(a, b).get(0), Some(a), "({a}, {b}).get(0)");
    assert_val!(vec2t!(a, b).get(1), Some(b), "({a}, {b}).get(1)");
    assert_val!(vec2t!(a, b).get(2), None::<T>, "({a}, {b}).get(2)");
    assert_val!(vec2t!(a, b).get(3), None::<T>, "({a}, {b}).get(3)");

    assert_val!(vec3t!(a, b, c).get(0), Some(a), "({a}, {b}, {c}).get(0)");
    assert_val!(vec3t!(a, b, c).get(1), Some(b), "({a}, {b}, {c}).get(1)");
    assert_val!(vec3t!(a, b, c).get(2), Some(c), "({a}, {b}, {c}).get(2)");
    assert_val!(vec3t!(a, b, c).get(3), None::<T>, "({a}, {b}, {c}).get(3)");
    assert_val!(vec3t!(a, b, c).get(4), None::<T>, "({a}, {b}, {c}).get(4)");

    assert_val!(vec4t!(a, b, c, d).get(0), Some(a), "({a}, {b}, {c}, {d}).get(0)");
    assert_val!(vec4t!(a, b, c, d).get(1), Some(b), "({a}, {b}, {c}, {d}).get(1)");
    assert_val!(vec4t!(a, b, c, d).get(2), Some(c), "({a}, {b}, {c}, {d}).get(2)");
    assert_val!(vec4t!(a, b, c, d).get(3), Some(d), "({a}, {b}, {c}, {d}).get(3)");
    assert_val!(vec4t!(a, b, c, d).get(4), None::<T>, "({a}, {b}, {c}, {d}).get(4)");
    assert_val!(vec4t!(a, b, c, d).get(5), None::<T>, "({a}, {b}, {c}, {d}).get(5)");

    assert_val!(vec2t!(a, b).get_mut(0).map(|x| *x), Some(a), "({a}, {b}).get_mut(0)");
    assert_val!(vec2t!(a, b).get_mut(1).map(|x| *x), Some(b), "({a}, {b}).get_mut(1)");
    assert_val!(vec2t!(a, b).get_mut(2).map(|x| *x), None, "({a}, {b}).get_mut(2)");
    assert_val!(vec2t!(a, b).get_mut(3).map(|x| *x), None, "({a}, {b}).get_mut(3)");

    assert_val!(vec3t!(a, b, c).get_mut(0).map(|x| *x), Some(a), "({a}, {b}, {c}).get_mut(0)");
    assert_val!(vec3t!(a, b, c).get_mut(1).map(|x| *x), Some(b), "({a}, {b}, {c}).get_mut(1)");
    assert_val!(vec3t!(a, b, c).get_mut(2).map(|x| *x), Some(c), "({a}, {b}, {c}).get_mut(2)");
    assert_val!(vec3t!(a, b, c).get_mut(3).map(|x| *x), None, "({a}, {b}, {c}).get_mut(3)");
    assert_val!(vec3t!(a, b, c).get_mut(4).map(|x| *x), None, "({a}, {b}, {c}).get_mut(4)");

    assert_val!(vec4t!(a, b, c, d).get_mut(0).map(|x| *x), Some(a), "({a}, {b}, {c}, {d}).get_mut(0)");
    assert_val!(vec4t!(a, b, c, d).get_mut(1).map(|x| *x), Some(b), "({a}, {b}, {c}, {d}).get_mut(1)");
    assert_val!(vec4t!(a, b, c, d).get_mut(2).map(|x| *x), Some(c), "({a}, {b}, {c}, {d}).get_mut(2)");
    assert_val!(vec4t!(a, b, c, d).get_mut(3).map(|x| *x), Some(d), "({a}, {b}, {c}, {d}).get_mut(3)");
    assert_val!(vec4t!(a, b, c, d).get_mut(4).map(|x| *x), None, "({a}, {b}, {c}, {d}).get_mut(4)");
    assert_val!(vec4t!(a, b, c, d).get_mut(5).map(|x| *x), None, "({a}, {b}, {c}, {d}).get_mut(5)");

    assert_val!(vec2t!(a, b).iter().collect::<Vec<T>>(), vec![a, b], "({a}, {b}).iter()");
    assert_val!(vec3t!(a, b, c).iter().collect::<Vec<T>>(), vec![a, b, c], "({a}, {b}, {c}).iter()");
    assert_val!(vec4t!(a, b, c, d).iter().collect::<Vec<T>>(), vec![a, b, c, d], "({a}, {b}, {c}, {d}).iter()");

    assert_val!(vec2t!(a, b).iter_mut().map(|x| *x).collect::<Vec<T>>(), vec![a, b], "({a}, {b}).iter_mut()");
    assert_val!(vec3t!(a, b, c).iter_mut().map(|x| *x).collect::<Vec<T>>(), vec![a, b, c], "({a}, {b}, {c}).iter_mut()");
    assert_val!(vec4t!(a, b, c, d).iter_mut().map(|x| *x).collect::<Vec<T>>(), vec![a, b, c, d], "({a}, {b}, {c}, {d}).iter_mut()");

    assert_val!(vec2t!(a, b).map(|x| x == b), vec2g!(a == b, b == b), "({a}, {b}).map(|x| x == {b})");
    assert_val!(vec3t!(a, b, c).map(|x| x == b), vec3g!(a == b, b == b, c == b), "({a}, {b}, {c}).map(|x| x == {b})");
    assert_val!(vec4t!(a, b, c, d).map(|x| x == b), vec4g!(a == b, b == b, c == b, d == b), "({a}, {b}, {c}, {d}).map(|x| x == {b})");

    assert_val!(vec2t!(a, b).reverse(), vec2t!(b, a), "({a}, {b}).reverse()");
    assert_val!(vec3t!(a, b, c).reverse(), vec3t!(c, b, a), "({a}, {b}, {c}).reverse()");
    assert_val!(vec4t!(a, b, c, d).reverse(), vec4t!(d, c, b, a), "({a}, {b}, {c}, {d}).reverse()");

    assert_val!(vec2t!(a, b).swizzle2::<0, 1>(), vec2t!(a, b), "({a}, {b}).swizzle2::<0, 1>()");
    assert_val!(vec2t!(a, b).swizzle2::<1, 0>(), vec2t!(b, a), "({a}, {b}).swizzle2::<1, 0>()");
    assert_val!(vec2t!(a, b).swizzle2::<1, 1>(), vec2t!(b, b), "({a}, {b}).swizzle2::<1, 1>()");

    assert_val!(vec3t!(a, b, c).swizzle2::<0, 1>(), vec2t!(a, b), "({a}, {b}, {c}).swizzle2::<0, 1>()");
    assert_val!(vec3t!(a, b, c).swizzle2::<0, 2>(), vec2t!(a, c), "({a}, {b}, {c}).swizzle2::<0, 2>()");
    assert_val!(vec3t!(a, b, c).swizzle2::<2, 1>(), vec2t!(c, b), "({a}, {b}, {c}).swizzle2::<2, 1>()");

    assert_val!(vec4t!(a, b, c, d).swizzle2::<0, 1>(), vec2t!(a, b), "({a}, {b}, {c}, {d}).swizzle2::<0, 1>()");
    assert_val!(vec4t!(a, b, c, d).swizzle2::<1, 3>(), vec2t!(b, d), "({a}, {b}, {c}, {d}).swizzle2::<1, 3>()");
    assert_val!(vec4t!(a, b, c, d).swizzle2::<3, 1>(), vec2t!(d, b), "({a}, {b}, {c}, {d}).swizzle2::<3, 1>()");

    assert_val!(vec2t!(a, b).swizzle3::<0, 1, 1>(), vec3t!(a, b, b), "({a}, {b}).swizzle3::<0, 1, 1>()");
    assert_val!(vec2t!(a, b).swizzle3::<1, 0, 1>(), vec3t!(b, a, b), "({a}, {b}).swizzle3::<1, 0, 1>()");

    assert_val!(vec3t!(a, b, c).swizzle3::<0, 1, 2>(), vec3t!(a, b, c), "({a}, {b}, {c}).swizzle3::<0, 1, 2>()");
    assert_val!(vec3t!(a, b, c).swizzle3::<1, 0, 2>(), vec3t!(b, a, c), "({a}, {b}, {c}).swizzle3::<1, 0, 2>()");
    assert_val!(vec3t!(a, b, c).swizzle3::<2, 1, 0>(), vec3t!(c, b, a), "({a}, {b}, {c}).swizzle3::<2, 1, 0>()");

    assert_val!(vec4t!(a, b, c, d).swizzle3::<0, 1, 2>(), vec3t!(a, b, c), "({a}, {b}, {c}, {d}).swizzle3::<0, 1, 2>()");
    assert_val!(vec4t!(a, b, c, d).swizzle3::<1, 0, 2>(), vec3t!(b, a, c), "({a}, {b}, {c}, {d}).swizzle3::<1, 0, 2>()");
    assert_val!(vec4t!(a, b, c, d).swizzle3::<2, 1, 0>(), vec3t!(c, b, a), "({a}, {b}, {c}, {d}).swizzle3::<2, 1, 0>()");

    assert_val!(vec2t!(a, b).swizzle4::<0, 1, 1, 0>(), vec4t!(a, b, b, a), "({a}, {b}).swizzle4::<0, 1, 1, 0>()");
    assert_val!(vec2t!(a, b).swizzle4::<1, 0, 1, 0>(), vec4t!(b, a, b, a), "({a}, {b}).swizzle4::<1, 0, 1, 0>()");
    assert_val!(vec2t!(a, b).swizzle4::<1, 1, 0, 0>(), vec4t!(b, b, a, a), "({a}, {b}).swizzle4::<1, 1, 0, 0>()");

    assert_val!(vec3t!(a, b, c).swizzle4::<0, 1, 2, 0>(), vec4t!(a, b, c, a), "({a}, {b}, {c}).swizzle4::<0, 1, 2, 0>()");
    assert_val!(vec3t!(a, b, c).swizzle4::<1, 0, 2, 0>(), vec4t!(b, a, c, a), "({a}, {b}, {c}).swizzle4::<1, 0, 2, 0>()");
    assert_val!(vec3t!(a, b, c).swizzle4::<2, 1, 0, 0>(), vec4t!(c, b, a, a), "({a}, {b}, {c}).swizzle4::<2, 1, 0, 0>()");

    assert_val!(vec4t!(a, b, c, d).swizzle4::<0, 1, 2, 3>(), vec4t!(a, b, c, d), "({a}, {b}, {c}, {d}).swizzle4::<0, 1, 2, 3>()");
    assert_val!(vec4t!(a, b, c, d).swizzle4::<1, 0, 2, 3>(), vec4t!(b, a, c, d), "({a}, {b}, {c}, {d}).swizzle4::<1, 0, 2, 3>()");
    assert_val!(vec4t!(a, b, c, d).swizzle4::<2, 1, 0, 3>(), vec4t!(c, b, a, d), "({a}, {b}, {c}, {d}).swizzle4::<2, 1, 0, 3>()");
    assert_val!(vec4t!(a, b, c, d).swizzle4::<3, 1, 2, 0>(), vec4t!(d, b, c, a), "({a}, {b}, {c}, {d}).swizzle4::<3, 1, 2, 0>()");

    assert_val!(vec2t!(a, b)[0], a, "({a}, {b})[0]");
    assert_val!(vec2t!(a, b)[1], b, "({a}, {b})[1]");

    assert_val!(vec3t!(a, b, c)[0], a, "({a}, {b}, {c})[0]");
    assert_val!(vec3t!(a, b, c)[1], b, "({a}, {b}, {c})[1]");
    assert_val!(vec3t!(a, b, c)[2], c, "({a}, {b}, {c})[2]");

    assert_val!(vec4t!(a, b, c, d)[0], a, "({a}, {b}, {c}, {d})[0]");
    assert_val!(vec4t!(a, b, c, d)[1], b, "({a}, {b}, {c}, {d})[1]");
    assert_val!(vec4t!(a, b, c, d)[2], c, "({a}, {b}, {c}, {d})[2]");
    assert_val!(vec4t!(a, b, c, d)[3], d, "({a}, {b}, {c}, {d})[3]");

    assert_panic!(vec2t!(a, b)[2], "({a}, {b})[2]");
    assert_panic!(vec3t!(a, b, c)[3], "({a}, {b}, {c})[3]");
    assert_panic!(vec4t!(a, b, c, d)[4], "({a}, {b}, {c}, {d})[4]");

    assert_val!(&mut vec2t!(a, b)[0], &mut a.clone(), "&mut ({a}, {b})[0]");
    assert_val!(&mut vec2t!(a, b)[1], &mut b.clone(), "&mut ({a}, {b})[1]");

    assert_val!(&mut vec3t!(a, b, c)[0], &mut a.clone(), "&mut ({a}, {b}, {c})[0]");
    assert_val!(&mut vec3t!(a, b, c)[1], &mut b.clone(), "&mut ({a}, {b}, {c})[1]");
    assert_val!(&mut vec3t!(a, b, c)[2], &mut c.clone(), "&mut ({a}, {b}, {c})[2]");

    assert_val!(&mut vec4t!(a, b, c, d)[0], &mut a.clone(), "&mut ({a}, {b}, {c}, {d})[0]");
    assert_val!(&mut vec4t!(a, b, c, d)[1], &mut b.clone(), "&mut ({a}, {b}, {c}, {d})[1]");
    assert_val!(&mut vec4t!(a, b, c, d)[2], &mut c.clone(), "&mut ({a}, {b}, {c}, {d})[2]");
    assert_val!(&mut vec4t!(a, b, c, d)[3], &mut d.clone(), "&mut ({a}, {b}, {c}, {d})[3]");

    assert_panic!(&mut vec2t!(a, b)[2], "(&mut {a}, {b})[2]");
    assert_panic!(&mut vec3t!(a, b, c)[3], "(&mut {a}, {b}, {c})[3]");
    assert_panic!(&mut vec4t!(a, b, c, d)[4], "(&mut {a}, {b}, {c}, {d})[4]");

    assert_val!(vec2t!(a, b) == vec2t!(a, b), true, "({a}, {b}) == ({a}, {b})");
    assert_val!(vec2t!(a, b) == vec2t!(b, a), false, "({a}, {b}) == ({b}, {a})");
    assert_val!(vec2t!(a, b) == vec2t!(b, b), false, "({a}, {b}) == ({b}, {b})");

    assert_val!(vec3t!(a, b, c) == vec3t!(a, b, c), true, "({a}, {b}, {c}) == ({a}, {b}, {c})");
    assert_val!(vec3t!(a, b, c) == vec3t!(d, c, b), false, "({a}, {b}, {c}) == ({d}, {c}, {b})");
    assert_val!(vec3t!(a, b, c) == vec3t!(b, b, c), false, "({a}, {b}, {c}) == ({b}, {b}, {c})");

    assert_val!(vec4t!(a, b, c, d) == vec4t!(a, b, c, d), true, "({a}, {b}, {c}, {d}) == ({a}, {b}, {c}, {d})");
    assert_val!(vec4t!(a, b, c, d) == vec4t!(d, c, b, a), false, "({a}, {b}, {c}, {d}) == ({d}, {c}, {b}, {a})");
    assert_val!(vec4t!(a, b, c, d) == vec4t!(b, b, b, d), false, "({a}, {b}, {c}, {d}) == ({b}, {b}, {b}, {d})");

    assert_val!(vec2t!(a, b) != vec2t!(a, b), false, "({a}, {b}) != ({a}, {b})");
    assert_val!(vec2t!(a, b) != vec2t!(b, a), true, "({a}, {b}) != ({b}, {a})");
    assert_val!(vec2t!(a, b) != vec2t!(b, b), true, "({a}, {b}) != ({b}, {b})");

    assert_val!(vec3t!(a, b, c) != vec3t!(a, b, c), false, "({a}, {b}, {c}) != ({a}, {b}, {c})");
    assert_val!(vec3t!(a, b, c) != vec3t!(d, c, b), true, "({a}, {b}, {c}) != ({d}, {c}, {b})");
    assert_val!(vec3t!(a, b, c) != vec3t!(b, b, c), true, "({a}, {b}, {c}) != ({b}, {b}, {c})");

    assert_val!(vec4t!(a, b, c, d) != vec4t!(a, b, c, d), false, "({a}, {b}, {c}, {d}) != ({a}, {b}, {c}, {d})");
    assert_val!(vec4t!(a, b, c, d) != vec4t!(d, c, b, a), true, "({a}, {b}, {c}, {d}) != ({d}, {c}, {b}, {a})");
    assert_val!(vec4t!(a, b, c, d) != vec4t!(b, b, b, d), true, "({a}, {b}, {c}, {d}) != ({b}, {b}, {b}, {d})");

    assert_val!(vec2t!(a, b).to_string(), format!("({a}, {b})"), "({a}, {b}).to_string()");
    assert_val!(vec3t!(a, b, c).to_string(), format!("({a}, {b}, {c})"), "({a}, {b}, {c}).to_string()");
    assert_val!(vec4t!(a, b, c, d).to_string(), format!("({a}, {b}, {c}, {d})"), "({a}, {b}, {c}, {d}).to_string()");

    assert_val!(format!("{:?}", vec2t!(a, b)), format!("({a:?}, {b:?})"), "format!(\"{{:?}}\", ({a}, {b}))");
    assert_val!(format!("{:?}", vec3t!(a, b, c)), format!("({a:?}, {b:?}, {c:?})"), "format!(\"{{:?}}\", ({a}, {b}, {c}))");
    assert_val!(format!("{:?}", vec4t!(a, b, c, d)), format!("({a:?}, {b:?}, {c:?}, {d:?})"), "format!(\"{{:?}}\", ({a}, {b}, {c}, {d}))");

    assert_val!(vec2t!(a, b).x, a, "({a}, {b}).x");
    assert_val!(vec2t!(a, b).y, b, "({a}, {b}).y");

    assert_val!(vec3t!(a, b, c).x, a, "({a}, {b}, {c}).x");
    assert_val!(vec3t!(a, b, c).y, b, "({a}, {b}, {c}).y");
    assert_val!(vec3t!(a, b, c).z, c, "({a}, {b}, {c}).z");

    assert_val!(vec4t!(a, b, c, d).x, a, "({a}, {b}, {c}, {d}).x");
    assert_val!(vec4t!(a, b, c, d).y, b, "({a}, {b}, {c}, {d}).y");
    assert_val!(vec4t!(a, b, c, d).z, c, "({a}, {b}, {c}, {d}).z");
    assert_val!(vec4t!(a, b, c, d).w, d, "({a}, {b}, {c}, {d}).w");

    assert_val!(*&mut vec2t!(a, b).x, a, "&mut ({a}, {b}).x");
    assert_val!(*&mut vec2t!(a, b).y, b, "&mut ({a}, {b}).y");

    assert_val!(*&mut vec3t!(a, b, c).x, a, "&mut ({a}, {b}, {c}).x");
    assert_val!(*&mut vec3t!(a, b, c).y, b, "&mut ({a}, {b}, {c}).y");
    assert_val!(*&mut vec3t!(a, b, c).z, c, "&mut ({a}, {b}, {c}).z");

    assert_val!(*&mut vec4t!(a, b, c, d).x, a, "&mut ({a}, {b}, {c}, {d}).x");
    assert_val!(*&mut vec4t!(a, b, c, d).y, b, "&mut ({a}, {b}, {c}, {d}).y");
    assert_val!(*&mut vec4t!(a, b, c, d).z, c, "&mut ({a}, {b}, {c}, {d}).z");
    assert_val!(*&mut vec4t!(a, b, c, d).w, d, "&mut ({a}, {b}, {c}, {d}).w");

    #[cfg(feature = "swizzle")]
    {
        assert_val!(vec2t!(a, b).xy(), vec2t!(a, b), "({a}, {b}).xy()");
        assert_val!(vec2t!(a, b).yx(), vec2t!(b, a), "({a}, {b}).yx()");
        assert_val!(vec2t!(a, b).yy(), vec2t!(b, b), "({a}, {b}).yy()");

        assert_val!(vec3t!(a, b, c).xy(), vec2t!(a, b), "({a}, {b}, {c}).xy()");
        assert_val!(vec3t!(a, b, c).xz(), vec2t!(a, c), "({a}, {b}, {c}).xz()");
        assert_val!(vec3t!(a, b, c).zy(), vec2t!(c, b), "({a}, {b}, {c}).zy()");

        assert_val!(vec4t!(a, b, c, d).xy(), vec2t!(a, b), "({a}, {b}, {c}, {d}).xy()");
        assert_val!(vec4t!(a, b, c, d).yw(), vec2t!(b, d), "({a}, {b}, {c}, {d}).yw()");
        assert_val!(vec4t!(a, b, c, d).wy(), vec2t!(d, b), "({a}, {b}, {c}, {d}).wy()");

        assert_val!(vec2t!(a, b).xyy(), vec3t!(a, b, b), "({a}, {b}).xyy()");
        assert_val!(vec2t!(a, b).yxy(), vec3t!(b, a, b), "({a}, {b}).yxy()");

        assert_val!(vec3t!(a, b, c).xyz(), vec3t!(a, b, c), "({a}, {b}, {c}).xyz()");
        assert_val!(vec3t!(a, b, c).yxz(), vec3t!(b, a, c), "({a}, {b}, {c}).yxz()");
        assert_val!(vec3t!(a, b, c).zyx(), vec3t!(c, b, a), "({a}, {b}, {c}).zyx()");

        assert_val!(vec4t!(a, b, c, d).xyz(), vec3t!(a, b, c), "({a}, {b}, {c}, {d}).xyz()");
        assert_val!(vec4t!(a, b, c, d).yxz(), vec3t!(b, a, c), "({a}, {b}, {c}, {d}).yxz()");
        assert_val!(vec4t!(a, b, c, d).zyx(), vec3t!(c, b, a), "({a}, {b}, {c}, {d}).zyx()");

        assert_val!(vec2t!(a, b).xyyx(), vec4t!(a, b, b, a), "({a}, {b}).xyyx()");
        assert_val!(vec2t!(a, b).yxyx(), vec4t!(b, a, b, a), "({a}, {b}).yxyx()");
        assert_val!(vec2t!(a, b).yyxx(), vec4t!(b, b, a, a), "({a}, {b}).yyxx()");

        assert_val!(vec3t!(a, b, c).xyzx(), vec4t!(a, b, c, a), "({a}, {b}, {c}).xyzx()");
        assert_val!(vec3t!(a, b, c).yxzx(), vec4t!(b, a, c, a), "({a}, {b}, {c}).yxzx()");
        assert_val!(vec3t!(a, b, c).zyxx(), vec4t!(c, b, a, a), "({a}, {b}, {c}).zyxx()");

        assert_val!(vec4t!(a, b, c, d).xyzw(), vec4t!(a, b, c, d), "({a}, {b}, {c}, {d}).xyzw()");
        assert_val!(vec4t!(a, b, c, d).yxzw(), vec4t!(b, a, c, d), "({a}, {b}, {c}, {d}).yxzw()");
        assert_val!(vec4t!(a, b, c, d).zyxw(), vec4t!(c, b, a, d), "({a}, {b}, {c}, {d}).zyxw()");
        assert_val!(vec4t!(a, b, c, d).wyzx(), vec4t!(d, b, c, a), "({a}, {b}, {c}, {d}).wyzx()");
    }
}
