use std::mem::transmute;

use ggmath::*;

// verify size
const _: () = assert!(size_of::<FVec2>() == 2 * size_of::<f32>());
const _: () = assert!(size_of::<FVec3>() == 4 * size_of::<f32>());
const _: () = assert!(size_of::<FVec4>() == 4 * size_of::<f32>());
const _: () = assert!(size_of::<FVec2P>() == 2 * size_of::<f32>());
const _: () = assert!(size_of::<FVec3P>() == 3 * size_of::<f32>());
const _: () = assert!(size_of::<FVec4P>() == 4 * size_of::<f32>());

const _: () = assert!(size_of::<IVec2>() == 2 * size_of::<i32>());
const _: () = assert!(size_of::<IVec3>() == 4 * size_of::<i32>());
const _: () = assert!(size_of::<IVec4>() == 4 * size_of::<i32>());
const _: () = assert!(size_of::<IVec2P>() == 2 * size_of::<i32>());
const _: () = assert!(size_of::<IVec3P>() == 3 * size_of::<i32>());
const _: () = assert!(size_of::<IVec4P>() == 4 * size_of::<i32>());

// verify alignment
const _: () = assert!(align_of::<FVec2>() == 8);
const _: () = assert!(align_of::<FVec3>() == 16);
const _: () = assert!(align_of::<FVec4>() == 16);
const _: () = assert!(align_of::<FVec2P>() == align_of::<f32>());
const _: () = assert!(align_of::<FVec3P>() == align_of::<f32>());
const _: () = assert!(align_of::<FVec4P>() == align_of::<f32>());

#[test]
fn test_new() {
    assert!(vec4!(1, vec2p!(2, 3), 4) == Vec4::from_array([1, 2, 3, 4]));
    assert!(vec4!(1, vec3p!(2, 3, 4)) == Vec4::from_array([1, 2, 3, 4]));
    assert!(vec4!(vec2p!(1, 2), vec2p!(3, 4)) == Vec4::from_array([1, 2, 3, 4]));
    assert!(vec4!(vec4!(1, 2, 3, 4)) == Vec4::from_array([1, 2, 3, 4]));

    assert!(splat2(1) == Vec2::from_array([1; 2]));
    assert!(splat3(-1) == Vec3::from_array([-1; 3]));
    assert!(splat4(u8::MAX) == Vec4::from_array([u8::MAX; 4]));
}

#[test]
fn test_fmt() {
    assert_eq!(format!("{:?}", vec2!(1, 2)), "(1, 2)");
    assert_eq!(format!("{:?}", vec3!(1, 2, 3)), "(1, 2, 3)");
    assert_eq!(format!("{}", vec4!(1, 2, 3, 4)), "(1, 2, 3, 4)");
    assert_eq!(format!("{}", vec2p!(1, 2)), "(1, 2)");
    assert_eq!(format!("{}", vec3p!(1, 2, 3)), "(1, 2, 3)");
    assert_eq!(format!("{:?}", vec4p!(1, 2, 3, 4)), "(1, 2, 3, 4)");

    assert_eq!(format!("{:?}", vec2!(1.0, 2.0)), "(1.0, 2.0)");
    assert_eq!(format!("{:?}", vec3!(1.0, 2.0, 3.0)), "(1.0, 2.0, 3.0)");
    assert_eq!(format!("{}", vec4!(1.0, 2.0, 3.0, 4.0)), "(1, 2, 3, 4)");
    assert_eq!(format!("{:?}", vec2p!(1.0, 2.0)), "(1.0, 2.0)");
    assert_eq!(format!("{}", vec3p!(1.0, 2.0, 3.0)), "(1, 2, 3)");
    assert_eq!(format!("{}", vec4p!(1.0, 2.0, 3.0, 4.0)), "(1, 2, 3, 4)");
}

#[test]
fn test_default() {
    assert_eq!(FVec2::default(), splat2(Default::default()));
    assert_eq!(FVec4P::default(), splat4p(Default::default()));
}

#[test]
fn test_swizzle() {
    // Get

    assert_eq!(vec2!(1, 2).xxy(), vec3!(1, 1, 2));
    assert_eq!(vec3!(1, 2, 3).xyzx(), vec4!(1, 2, 3, 1));
    assert_eq!(vec4!(1, 2, 3, 4).xyzw(), vec4!(1, 2, 3, 4));
    assert_eq!(vec2!(1, 2).y(), 2);

    assert_eq!(vec2p!(false, true).xyx(), vec3p!(false, true, false));
    assert_eq!(vec2p!(false, true).y(), true);

    // Get Ref

    assert_eq!(vec2!(1, 2).x_ref(), &1);
    assert_eq!(vec3p!(1, 2, 3).yz_ref(), &vec2p!(2, 3));

    // Get Mut

    assert_eq!(vec2!(1, 2).x_mut(), &mut 1);
    assert_eq!(vec3p!(1, 2, 3).yz_mut(), &mut vec2p!(2, 3));
    assert_eq!(vec4p!(1, 2, 3, 4).xy_z_mut(), (&mut vec2p!(1, 2), &mut 3));

    // Set

    let mut vec = vec2!(1, 2);
    vec.set_x(3);
    assert_eq!(vec, vec2!(3, 2));

    let mut vec = vec3p!(1, 2, 3);
    vec.set_zx(vec2!(4, 5));
    assert_eq!(vec, vec3p!(5, 2, 4));

    let mut vec = vec4p!(1, 2, 3, 4);
    vec.set_xzyw(vec4p!(5, 6, 7, 8));
    assert_eq!(vec, vec4p!(5, 7, 6, 8));

    // With

    assert_eq!(vec2!(1, 2).with_x(3), vec2!(3, 2));
    assert_eq!(vec3p!(1, 2, 3).with_y(4), vec3p!(1, 4, 3));
    assert_eq!(vec4!(1, 2, 3, 4).with_zx(vec2!(5, 6)), vec4p!(6, 2, 5, 4));
}

#[test]
fn test_conversion() {
    // T

    assert_eq!(vec2!(1, 2).to_t::<f64>(), vec2!(1.0, 2.0));
    assert_eq!(vec3p!(1, 2, 3).to_t::<f64>(), vec3!(1.0, 2.0, 3.0));

    // A

    assert_eq!(vec2!(1, 2).align(), vec2!(1, 2));
    assert_eq!(vec3p!(1, 2, 3).align(), vec3!(1, 2, 3));
    assert_eq!(vec4!(1, 2, 3, 4).unalign(), vec4!(1, 2, 3, 4));
    assert_eq!(vec2p!(1, 2).unalign(), vec2p!(1, 2));

    assert_eq!(Vec2::from(vec2p!(1, 2)), vec2!(1, 2));
    assert_eq!(Vec3P::from(vec3!(1, 2, 3)), vec3!(1, 2, 3));

    // Layout

    assert_eq!(vec2p!(1, 2).to_layout::<VecAligned>(), vec2!(1, 2));
    assert_eq!(vec3!(1, 2, 3).to_layout::<VecAligned>(), vec3!(1, 2, 3));
    assert_eq!(vec3!(1, 2, 3).to_layout::<VecPacked>(), vec3!(1, 2, 3));

    // Array

    assert_eq!(vec2!(1, 2).to_array(), [1, 2]);
    assert_eq!(vec3p!(1, 2, 3).to_array(), [1, 2, 3]);

    assert_eq!(vec2!(1, 2).as_array_ref(), &[1, 2]);
    assert_eq!(vec3p!(1, 2, 3).as_array_ref(), &[1, 2, 3]);

    assert_eq!(vec2!(1, 2).as_array_mut(), &mut [1, 2]);
    assert_eq!(vec3p!(1, 2, 3).as_array_mut(), &mut [1, 2, 3]);

    unsafe {
        assert_eq!(vec2!(1, 2).as_ptr().as_ref(), [1, 2].as_ptr().as_ref());
        assert_eq!(
            vec3p!(1, 2, 3).as_ptr().as_ref(),
            [1, 2, 3].as_ptr().as_ref()
        );

        assert_eq!(
            vec2!(1, 2).as_mut_ptr().as_mut(),
            [1, 2].as_mut_ptr().as_mut()
        );
        assert_eq!(
            vec3p!(1, 2, 3).as_mut_ptr().as_mut(),
            [1, 2, 3].as_mut_ptr().as_mut()
        );
    }

    assert_eq!(Vec2P::from_array_ref(&[1, 2]), &vec2p!(1, 2));
    assert_eq!(Vec2P::from_array_mut(&mut [1, 2]), &mut vec2!(1, 2));

    assert_eq!(Vec2::from([1, 2]), vec2!(1, 2));
    assert_eq!(Vec3P::from([1, 2, 3]), vec3p!(1, 2, 3));
    assert_eq!(<Vec2<_> as Into<[_; 2]>>::into(vec2!(1, 2)), [1, 2]);

    assert_eq!(<Vec2<_> as AsRef<[_; 2]>>::as_ref(&vec2!(1, 2)), &[1, 2]);

    // Bytes

    unsafe {
        assert_eq!(
            vec2!(1u16, 2u16).as_bytes_ref(),
            &transmute::<[u16; 2], [u8; 4]>([1, 2])
        );
        assert_eq!(
            vec3p!(1u16, 2u16, 3u16).as_bytes_ref(),
            &transmute::<[u16; 3], [u8; 6]>([1, 2, 3])
        );

        assert_eq!(
            vec2!(1u16, 2u16).as_bytes_mut(),
            &mut transmute::<[u16; 2], [u8; 4]>([1, 2])
        );
        assert_eq!(
            vec3p!(1u16, 2u16, 3u16).as_bytes_mut(),
            &mut transmute::<[u16; 3], [u8; 6]>([1, 2, 3])
        );

        assert_eq!(
            vec2!(1u16, 2u16).as_bytes_ref_padded().len(),
            size_of::<U16Vec2>()
        );
        assert_eq!(
            vec3!(1u16, 2u16, 3u16).as_bytes_ref_padded().len(),
            size_of::<U16Vec3>()
        );
        assert_eq!(
            vec4!(1u16, 2u16, 3u16, 4u16).as_bytes_ref_padded().len(),
            size_of::<U16Vec4>()
        );

        assert_eq!(
            vec2p!(1u16, 2u16).as_bytes_ref_padded(),
            vec2p!(1u16, 2u16).as_bytes_ref()
        );
        assert_eq!(
            vec3p!(1u16, 2u16, 3u16).as_bytes_ref_padded(),
            vec3p!(1u16, 2u16, 3u16).as_bytes_ref()
        );
        assert_eq!(
            vec4p!(1u16, 2u16, 3u16, 4u16).as_bytes_ref_padded(),
            vec4p!(1u16, 2u16, 3u16, 4u16).as_bytes_ref()
        );

        assert_eq!(
            vec2!(1u16, 2u16).as_bytes_mut_padded().len(),
            size_of::<U16Vec2>()
        );
        assert_eq!(
            vec3!(1u16, 2u16, 3u16).as_bytes_mut_padded().len(),
            size_of::<U16Vec3>()
        );
        assert_eq!(
            vec4!(1u16, 2u16, 3u16, 4u16).as_bytes_mut_padded().len(),
            size_of::<U16Vec4>()
        );

        assert_eq!(
            vec2p!(1u16, 2u16).as_bytes_mut_padded(),
            vec2p!(1u16, 2u16).as_bytes_mut()
        );
        assert_eq!(
            vec3p!(1u16, 2u16, 3u16).as_bytes_mut_padded(),
            vec3p!(1u16, 2u16, 3u16).as_bytes_mut()
        );
        assert_eq!(
            vec4p!(1u16, 2u16, 3u16, 4u16).as_bytes_mut_padded(),
            vec4p!(1u16, 2u16, 3u16, 4u16).as_bytes_mut()
        );
    }
}

#[test]
fn test_resolve() {
    assert_eq!(vec2!(1, 2).resolve(), ResolvedVector::Aligned(vec2!(1, 2)));
    assert_eq!(
        vec3p!(1, 2, 3).resolve(),
        ResolvedVector::Packed(vec3p!(1, 2, 3))
    );

    assert_eq!(
        vec2!(1, 2).resolve_ref(),
        ResolvedVectorRef::Aligned(&vec2!(1, 2))
    );
    assert_eq!(
        vec3p!(1, 2, 3).resolve_ref(),
        ResolvedVectorRef::Packed(&vec3p!(1, 2, 3))
    );

    assert_eq!(
        vec2!(1, 2).resolve_mut(),
        ResolvedVectorMut::Aligned(&mut vec2!(1, 2))
    );
    assert_eq!(
        vec3p!(1, 2, 3).resolve_mut(),
        ResolvedVectorMut::Packed(&mut vec3p!(1, 2, 3))
    );

    assert_eq!(Vec2::resolved(vec2!(1, 2), vec2p!(3, 4)), vec2!(1, 2));
    assert_eq!(
        Vec3P::resolved(vec3!(1, 2, 3), vec3p!(3, 4, 5)),
        vec3!(3, 4, 5)
    );
}

#[test]
fn test_index() {
    assert_eq!(vec2!(1, 2).get(0), Some(1));
    assert_eq!(vec4!(1, 2, 3, 4).get(3), Some(4));
    assert_eq!(vec2!(1, 2).get(2), None);
    assert_eq!(vec2!(1, 2).index(0), 1);

    unsafe {
        assert_eq!(vec2!(1, 2).get_unchecked(0), 1);
        assert_eq!(vec4!(1, 2, 3, 4).get_unchecked(3), 4);
    }

    assert_eq!(vec2!(1, 2).get_ref(0), Some(&1));
    assert_eq!(vec3p!(1, 2, 3).get_ref(1), Some(&2));
    assert_eq!(vec4!(1, 2, 3, 4).get_ref(4), None);
    assert_eq!(vec2!(1, 2).index_ref(0), &1);
    assert_eq!(vec2!(1, 2).get_2_ref(0), Some(&vec2p!(1, 2)));
    assert_eq!(vec4p!(1, 2, 3, 4).get_2_ref(1), Some(&vec2p!(2, 3)));
    assert_eq!(vec4!(1, 2, 3, 4).get_4_ref(1), None);
    assert_eq!(vec4p!(1, 2, 3, 4).get_2_ref(3), None);

    unsafe {
        assert_eq!(vec2!(1, 2).get_ref_unchecked(0), &1);
        assert_eq!(vec3p!(1, 2, 3).get_ref_unchecked(1), &2);
        assert_eq!(vec4!(1, 2, 3, 4).get_ref_unchecked(3), &4);
        assert_eq!(vec2!(1, 2).get_2_ref_unchecked(0), &vec2p!(1, 2));
        assert_eq!(vec4p!(1, 2, 3, 4).get_2_ref_unchecked(1), &vec2p!(2, 3));
    }

    assert_eq!(vec2!(1, 2).get_mut(0), Some(&mut 1));
    assert_eq!(vec3p!(1, 2, 3).get_mut(1), Some(&mut 2));
    assert_eq!(vec4!(1, 2, 3, 4).get_mut(4), None);
    assert_eq!(vec2!(1, 2).index_mut(0), &mut 1);
    assert_eq!(vec2!(1, 2).get_2_mut(0), Some(&mut vec2p!(1, 2)));
    assert_eq!(vec4p!(1, 2, 3, 4).get_2_mut(1), Some(&mut vec2p!(2, 3)));
    assert_eq!(vec4!(1, 2, 3, 4).get_4_mut(1), None);
    assert_eq!(vec4p!(1, 2, 3, 4).get_2_mut(3), None);

    unsafe {
        assert_eq!(vec2!(1, 2).get_mut_unchecked(0), &mut 1);
        assert_eq!(vec3p!(1, 2, 3).get_mut_unchecked(1), &mut 2);
        assert_eq!(vec4!(1, 2, 3, 4).get_mut_unchecked(3), &mut 4);
        assert_eq!(vec2!(1, 2).get_2_mut_unchecked(0), &mut vec2p!(1, 2));
        assert_eq!(vec4p!(1, 2, 3, 4).get_2_mut_unchecked(1), &mut vec2p!(2, 3));
    }
}

#[test]
fn test_iterator() {
    assert_eq!(Vec2::from_fn(|i| i + 1), vec2!(1, 2));
    assert_eq!(
        vec4!(1, 2, 3, 4).map(|x| (x + 1) as f64),
        vec4!(2.0, 3.0, 4.0, 5.0)
    );
    assert_eq!(
        vec4!(1, 2, 3, 4).map_ref(|x| (*x + 1) as f64),
        vec4!(2.0, 3.0, 4.0, 5.0)
    );
    assert_eq!(
        vec4!(1, 2, 3, 4).map_rhs(vec4!(5, 6, 7, 8), |x, y| x + y),
        vec4!(6, 8, 10, 12)
    );
    assert_eq!(
        vec4!(1, 2, 3, 4).map_ref_rhs(&vec4!(5, 6, 7, 8), |x, y| x + y),
        vec4!(6, 8, 10, 12)
    );

    assert_eq!(vec2!(1, 2).into_iter().collect::<Vec<_>>(), vec![1, 2]);
    assert_eq!(vec2!(1, 2).iter_ref().collect::<Vec<_>>(), vec![&1, &2]);
    assert_eq!(
        vec2!(1, 2).iter_mut().collect::<Vec<_>>(),
        vec![&mut 1, &mut 2]
    );
    assert_eq!((&vec2!(1, 2)).into_iter().collect::<Vec<_>>(), vec![&1, &2]);
    assert_eq!(
        (&mut vec2!(1, 2)).into_iter().collect::<Vec<_>>(),
        vec![&mut 1, &mut 2]
    );

    assert_eq!(vec2!(1, 2).fold(|x, y| x + y), 3);
    assert_eq!(vec4!(1, 2, 3, 4).fold(|x, y| (x + y) / 2), 3);
}

#[test]
fn test_ops() {
    assert_eq!(-vec2!(1, 2), vec2!(-1, -2));
    assert_eq!(!vec2!(1, 2), vec2!(!1, !2));
    assert_eq!(vec2!(1, 2) + vec2!(3, 4), vec2!(4, 6));
    assert_eq!(vec2!(1, 2) - vec2!(3, 4), vec2!(-2, -2));
    assert_eq!(vec2!(1, 2) * vec2!(3, 4), vec2!(3, 8));
    assert_eq!(vec2!(1, 2) * 2, vec2!(2, 4));
    assert_eq!(vec2!(3, 4) / vec2!(1, 2), vec2!(3, 2));
    assert_eq!(vec2!(1, 2) / 2, vec2!(0, 1));
    assert_eq!(vec2!(1, 2) % vec2!(3, 4), vec2!(1, 2));
    assert_eq!(vec2!(1, 2) % 2, vec2!(1, 0));
    assert_eq!(vec2!(1, 2) & vec2!(3, 4), vec2!(1, 0));
    assert_eq!(vec2!(1, 2) | vec2!(3, 4), vec2!(3, 6));
    assert_eq!(vec2!(1, 2) ^ vec2!(3, 4), vec2!(2, 6));
    assert_eq!(vec2!(1, 2) << vec2!(3, 4), vec2!(8, 32));
    assert_eq!(vec2!(1, 2) >> vec2!(3, 4), vec2!(0, 0));

    assert_eq!(vec2!(1, 2) == vec2!(1, 2), true);
    assert_eq!(vec2!(1, 3) == vec2!(1, 2), false);
    assert_eq!(vec2!(1, 2) != vec2!(1, 2), false);
    assert_eq!(vec2!(1, 3) != vec2!(1, 2), true);

    // vec3.
    // needs more tests because of padding.
    repetitive! {
        // bool
        let lhs = vec3!(true, true, false);
        let rhs = vec3!(true, false, true);
        assert_eq!(!lhs, vec3!(false, false, true));
        assert_eq!(lhs & rhs, vec3!(true, false, false));
        assert_eq!(lhs | rhs, vec3!(true, true, true));
        assert_eq!(lhs ^ rhs, vec3!(false, true, true));

        assert_eq!((lhs == lhs), true);
        assert_eq!((lhs == rhs), false);
        assert_eq!((lhs != lhs), false);
        assert_eq!((lhs != rhs), true);

        // numbers
        @for prim in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize, 'i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64] {
            let lhs = vec3!(4 as @prim, 5 as @prim, 6 as @prim);
            let rhs = vec3!(1 as @prim, 2 as @prim, 3 as @prim);

            assert_eq!(
                lhs.with_padding(@prim::MAX) + rhs.with_padding(1 as @prim),
                vec3!(5 as @prim, 7 as @prim, 9 as @prim),
            );
            assert_eq!(
                lhs.with_padding(@prim::MIN) - rhs.with_padding(1 as @prim),
                vec3!(3 as @prim, 3 as @prim, 3 as @prim),
            );
            assert_eq!(
                lhs.with_padding(@prim::MAX) * rhs.with_padding(2 as @prim),
                vec3!(4 as @prim, 10 as @prim, 18 as @prim),
            );
            assert_eq!(
                lhs.with_padding(@prim::MAX) % rhs.with_padding(1 as @prim),
                vec3!(0 as @prim, 1 as @prim, 0 as @prim),
            );

            assert_eq!((lhs.with_padding(1 as @prim) == lhs.with_padding(3 as @prim)), true);
            assert_eq!((lhs == rhs), false);
            assert_eq!((lhs != lhs), false);
            assert_eq!((lhs != rhs), true);
        }

        // signed numbers
        @for prim in ['i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64] {
            let lhs = vec3!(4 as @prim, 5 as @prim, 6 as @prim);

            assert_eq!(-lhs.with_padding(@prim::MIN), vec3!(-4 as @prim, -5 as @prim, -6 as @prim));
        }

        // ints
        @for prim in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize, 'i8, 'i16, 'i32, 'i64, 'i128, 'isize] {
            let lhs = vec3!(4 as @prim, 5 as @prim, 6 as @prim);
            let rhs = vec3!(1 as @prim, 2 as @prim, 3 as @prim);

            assert_eq!(lhs.with_padding(2 as @prim) / rhs.with_padding(0 as @prim), vec3!(4 as @prim, 2 as @prim, 2 as @prim));
            assert_eq!(!lhs, vec3!(!4 as @prim, !5 as @prim, !6 as @prim));
            assert_eq!(lhs & rhs, vec3!(0 as @prim, 0 as @prim, 2 as @prim));
            assert_eq!(lhs | rhs, vec3!(5 as @prim, 7 as @prim, 7 as @prim));
            assert_eq!(lhs ^ rhs, vec3!(5 as @prim, 7 as @prim, 5 as @prim));
            assert_eq!(lhs << rhs, vec3!(8 as @prim, 20 as @prim, 48 as @prim));
            assert_eq!(lhs >> rhs, vec3!(2 as @prim, 1 as @prim, 0 as @prim));
        }

        // floats
        @for prim in ['f32, 'f64] {
            let lhs = vec3!(4 as @prim, 5 as @prim, 6 as @prim);
            let rhs = vec3!(1 as @prim, 2 as @prim, 3 as @prim);

            assert_eq!(lhs / rhs, vec3!(4.0, 2.5, 2.0));
            assert_eq!(lhs / (2 as @prim), vec3!(2.0, 2.5, 3.0));
        }
    }

    assert_eq!(vec3!(1, 2, 3).sum(), 6);
    assert_eq!(vec3!(1, 2, 3).dot(vec3!(4, 5, 6)), 32);
    assert_eq!(vec3!(1, 2, 3).cross(vec3!(4, 5, 6)), vec3!(-3, 6, -3));
}

#[test]
fn test_bool() {
    assert_eq!(vec3!(true, true, false).as_u8(), vec3!(1, 1, 0));
    assert_eq!(vec3!(true, true, false).as_u16(), vec3!(1, 1, 0));
    assert_eq!(vec3!(true, true, false).as_u32(), vec3!(1, 1, 0));
    assert_eq!(vec3!(true, true, false).as_i128(), vec3!(1, 1, 0));
}

#[test]
fn test_num() {
    repetitive! {
        @for prim in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize, 'i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64] {
            assert_eq!(
                vec3!(1 as @prim, 5 as @prim, 3 as @prim).min(
                    vec3!(4 as @prim, 2 as @prim, 6 as @prim),
                ),
                vec3!(1 as @prim, 2 as @prim, 3 as @prim),
            );
            assert_eq!(
                vec3!(1 as @prim, 5 as @prim, 3 as @prim).max(
                    vec3!(4 as @prim, 2 as @prim, 6 as @prim),
                ),
                vec3!(4 as @prim, 5 as @prim, 6 as @prim),
            );
            assert_eq!(
                vec3!(1 as @prim, 5 as @prim, 3 as @prim).clamp(
                    vec3!(4 as @prim, 2 as @prim, 6 as @prim),
                    vec3!(7 as @prim, 8 as @prim, 9 as @prim),
                ),
                vec3!(4 as @prim, 5 as @prim, 6 as @prim),
            );

            assert_eq!(
                vec3!(1 as @prim, 5 as @prim, 6 as @prim).abs_diff(
                    vec3!(4 as @prim, 5 as @prim, 3 as @prim),
                ),
                vec3!(3 as @prim, 0 as @prim, 3 as @prim),
            );

            assert_eq!(
                vec3!(1 as @prim, 0 as @prim, 3 as @prim).is_positive(),
                vec3!(true, false, true),
            );
            assert_eq!(
                vec3!(1 as @prim, 0 as @prim, 3 as @prim).is_zero(),
                vec3!(false, true, false),
            );

            assert_eq!(
                vec3!(1 as @prim, 5 as @prim, 3 as @prim).square_mag(),
                35 as @prim,
            );

            assert_eq!(
                vec3!(1 as @prim, 0 as @prim, 3 as @prim).signumt(),
                vec3!(1 as @prim, 0 as @prim, 1 as @prim),
            );

            @for other_prim in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize, 'i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64] {
                @if other_prim != prim {
                    assert_eq!(
                        vec3!(1 as @prim, 5 as @prim, 3 as @prim).@['as_ other_prim](),
                        vec3!(1 as @other_prim, 5 as @other_prim, 3 as @other_prim),
                    );
                }
            }
        }
    }
}

#[test]
fn test_signed_num() {
    repetitive! {
        @for prim in ['i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64] {
            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, -3 as @prim).is_positive(),
                vec3!(false, true, false),
            );
            assert_eq!(
                vec3!(-1 as @prim, 0 as @prim, -3 as @prim).is_zero(),
                vec3!(false, true, false),
            );
            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, 0 as @prim).is_negative(),
                vec3!(true, false, false),
            );

            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, 0 as @prim).bin_signum(),
                vec3!(-1 as @prim, 1 as @prim, 1 as @prim),
            );
            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, 0 as @prim).is_bin_positive(),
                vec3!(false, true, true),
            );

            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, -0 as @prim).abs(),
                vec3!(1 as @prim, 5 as @prim, 0 as @prim),
            );
            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, 0 as @prim).neg_abs(),
                vec3!(-1 as @prim, -5 as @prim, -0 as @prim),
            );

            assert_eq!(
                vec3!(-1 as @prim, 5 as @prim, -3 as @prim).square_mag(),
                35 as @prim,
            );

            @for other_prim in ['i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64] {
                @if other_prim != prim {
                    assert_eq!(
                        vec3!(-1 as @prim, 5 as @prim, -3 as @prim).@['as_ other_prim](),
                        vec3!(-1 as @other_prim, 5 as @other_prim, -3 as @other_prim),
                    );
                }
            }
        }
    }
}

#[test]
fn test_floats() {
    repetitive! {
        @for prim in ['f32, 'f64] {
            assert_eq!(
                vec4!(5.6, 5.1, -3.3, -3.7 as @prim).round(),
                vec4!(6.0, 5.0, -3.0, -4.0),
                "round",
            );
            assert_eq!(
                vec4!(5.6, 5.1, -3.3, -3.7 as @prim).floor(),
                vec4!(5.0, 5.0, -4.0, -4.0),
                "floor",
            );
            assert_eq!(
                vec4!(5.6, 5.1, -3.3, -3.7 as @prim).ceil(),
                vec4!(6.0, 6.0, -3.0, -3.0),
                "ceil",
            );
            assert_eq!(
                vec4!(5.6, 5.1, -3.3, -3.7 as @prim).trunc(),
                vec4!(5.0, 5.0, -3.0, -3.0),
                "trunc",
            );
            assert_eq!(
                vec4!(5.6, 5.1, -3.3, -3.7 as @prim).atrunc(),
                vec4!(6.0, 6.0, -4.0, -4.0),
                "atrunc",
            );

            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).sin(),
                vec3!((5.6 as @prim).sin(), (5.1 as @prim).sin(), (-3.3 as @prim).sin()),
                "sin",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).cos(),
                vec3!((5.6 as @prim).cos(), (5.1 as @prim).cos(), (-3.3 as @prim).cos()),
                "cos",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).tan(),
                vec3!((5.6 as @prim).tan(), (5.1 as @prim).tan(), (-3.3 as @prim).tan()),
                "tan",
            );
            assert_eq!(
                vec3!(0.5, 0.0, -0.5 as @prim).asin(),
                vec3!((0.5 as @prim).asin(), (0.0 as @prim).asin(), (-0.5 as @prim).asin()),
                "asin",
            );
            assert_eq!(
                vec3!(0.5, 0.0, -0.5 as @prim).acos(),
                vec3!((0.5 as @prim).acos(), (0.0 as @prim).acos(), (-0.5 as @prim).acos()),
                "acos",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).atan(),
                vec3!((5.6 as @prim).atan(), (5.1 as @prim).atan(), (-3.3 as @prim).atan()),
                "atan",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).sinh(),
                vec3!((5.6 as @prim).sinh(), (5.1 as @prim).sinh(), (-3.3 as @prim).sinh()),
                "sinh",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).cosh(),
                vec3!((5.6 as @prim).cosh(), (5.1 as @prim).cosh(), (-3.3 as @prim).cosh()),
                "cosh",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).tanh(),
                vec3!((5.6 as @prim).tanh(), (5.1 as @prim).tanh(), (-3.3 as @prim).tanh()),
                "tanh",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).asinh(),
                vec3!((5.6 as @prim).asinh(), (5.1 as @prim).asinh(), (-3.3 as @prim).asinh()),
                "asinh",
            );
            assert_eq!(
                vec3!(1.5, 2.0, 3.0 as @prim).acosh(),
                vec3!((1.5 as @prim).acosh(), (2.0 as @prim).acosh(), (3.0 as @prim).acosh()),
                "acosh",
            );
            assert_eq!(
                vec3!(0.5, 0.0, -0.5 as @prim).atanh(),
                vec3!((0.5 as @prim).atanh(), (0.0 as @prim).atanh(), (-0.5 as @prim).atanh()),
                "atanh",
            );

            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).mag(),
                vec3!(5.6, 5.1, -3.3 as @prim).square_mag().sqrt(),
                "mag",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).normalize(),
                vec3!(5.6, 5.1, -3.3 as @prim) / vec3!(5.6, 5.1, -3.3 as @prim).mag(),
                "normalize",
            );
            assert_eq!(
                vec3!(1.0, 0.0, 0.0 as @prim).normalize(),
                vec3!(1.0, 0.0, 0.0 as @prim),
                "normalize",
            );
            assert_eq!(
                vec3!(-1.0, 0.0, 0.0 as @prim).normalize(),
                vec3!(-1.0, 0.0, 0.0 as @prim),
                "normalize",
            );
            assert_eq!(
                vec3!(3.0, 0.0, 0.0 as @prim).with_mag(2.0),
                vec3!(2.0, 0.0, 0.0 as @prim),
                "with_mag",
            );
            assert_eq!(
                vec3!(-5.0, 0.0, 0.0 as @prim).with_mag(2.0),
                vec3!(-2.0, 0.0, 0.0 as @prim),
                "with_mag",
            );
            assert_eq!(
                vec3!(3.0, 0.0, 0.0 as @prim).with_square_mag(4.0),
                vec3!(2.0, 0.0, 0.0 as @prim),
                "with_square_mag",
            );
            assert_eq!(
                vec3!(-5.0, 0.0, 0.0 as @prim).with_square_mag(4.0),
                vec3!(-2.0, 0.0, 0.0 as @prim),
                "with_max_square_mag",
            );

            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).with_min_mag(2.0),
                vec3!(5.6, 5.1, -3.3 as @prim),
                "with_min_mag",
            );
            assert_eq!(
                vec3!(3.0, 0.0, 0.0 as @prim).with_max_mag(2.0),
                vec3!(2.0, 0.0, 0.0 as @prim),
                "with_max_mag",
            );
            assert_eq!(
                vec3!(-5.0, 0.0, 0.0 as @prim).with_max_mag(2.0),
                vec3!(-2.0, 0.0, 0.0 as @prim),
                "with_max_mag",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).clamp_mag(7.0, 9.0),
                vec3!(5.6, 5.1, -3.3 as @prim),
                "clamp_mag",
            );
            assert_eq!(
                vec3!(6.0, 0.0, 0.0 as @prim).clamp_mag(2.0, 3.0),
                vec3!(3.0, 0.0, 0.0 as @prim),
                "clamp_mag",
            );
            assert_eq!(
                vec3!(-6.0, 0.0, 0.0 as @prim).clamp_mag(2.0, 3.0),
                vec3!(-3.0, 0.0, 0.0 as @prim),
                "clamp_mag",
            );
            assert_eq!(
                vec3!(1.0, 0.0, 0.0 as @prim).clamp_mag(2.0, 3.0),
                vec3!(2.0, 0.0, 0.0 as @prim),
                "clamp_mag",
            );
            assert_eq!(
                vec3!(-1.0, 0.0, 0.0 as @prim).clamp_mag(2.0, 3.0),
                vec3!(-2.0, 0.0, 0.0 as @prim),
                "clamp_mag",
            );

            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).with_min_square_mag(4.0),
                vec3!(5.6, 5.1, -3.3 as @prim),
                "with_min_square_mag",
            );
            assert_eq!(
                vec3!(3.0, 0.0, 0.0 as @prim).with_max_square_mag(4.0),
                vec3!(2.0, 0.0, 0.0 as @prim),
                "with_max_square_mag",
            );
            assert_eq!(
                vec3!(-5.0, 0.0, 0.0 as @prim).with_max_square_mag(4.0),
                vec3!(-2.0, 0.0, 0.0 as @prim),
                "with_max_square_mag",
            );
            assert_eq!(
                vec3!(5.6, 5.1, -3.3 as @prim).clamp_square_mag(49.0, 81.0),
                vec3!(5.6, 5.1, -3.3 as @prim),
                "clamp_square_mag",
            );
            assert_eq!(
                vec3!(6.0, 0.0, 0.0 as @prim).clamp_square_mag(4.0, 9.0),
                vec3!(3.0, 0.0, 0.0 as @prim),
                "clamp_square_mag",
            );
            assert_eq!(
                vec3!(-6.0, 0.0, 0.0 as @prim).clamp_square_mag(4.0, 9.0),
                vec3!(-3.0, 0.0, 0.0 as @prim),
                "clamp_square_mag",
            );
            assert_eq!(
                vec3!(1.0, 0.0, 0.0 as @prim).clamp_square_mag(4.0, 9.0),
                vec3!(2.0, 0.0, 0.0 as @prim),
                "clamp_square_mag",
            );
            assert_eq!(
                vec3!(-1.0, 0.0, 0.0 as @prim).clamp_square_mag(4.0, 9.0),
                vec3!(-2.0, 0.0, 0.0 as @prim),
                "clamp_square_mag",
            );
        }
    }
}
