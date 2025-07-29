use ggmath::*;

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

// Construction and conversion tests
#[test]
fn test_vector_construction() {
    let tested = vec4!(5, vec2!(1, 2), 6);
    let manual = Vec4::from_array([5, 1, 2, 6]);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_from_fn() {
    let tested = Vec3::from_fn(|i| i as f32);
    let manual = Vec3::from_array([0.0, 1.0, 2.0]);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_splat() {
    let tested = Vec3::splat(5.0);
    let manual = Vec3::from_array([5.0, 5.0, 5.0]);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_splat_functions() {
    let tested = splat3(7.0);
    let manual = Vec3::from_array([7.0, 7.0, 7.0]);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_to_array() {
    let vec = vec3!(1, 2, 3);
    let tested = vec.to_array();
    let manual = [1, 2, 3];
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_as_array_ref() {
    let vec = vec3!(1, 2, 3);
    let tested = vec.as_array_ref();
    let manual = &[1, 2, 3];
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_as_array_mut() {
    let mut vec = vec3!(1, 2, 3);
    let tested = vec.as_array_mut();
    tested[1] = 5;
    let manual = vec3!(1, 5, 3);
    assert_eq!(vec, manual);
}

#[test]
fn test_vector_as_ptr() {
    let vec = vec3!(1, 2, 3);
    let tested = vec.as_ptr();
    let manual = vec.as_array_ref().as_ptr();
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_as_mut_ptr() {
    let mut vec = vec3!(1, 2, 3);
    let tested = vec.as_mut_ptr();
    let manual = vec.as_array_mut().as_mut_ptr();
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_from_array_ref() {
    let array = [1, 2, 3];
    let tested = Vec3P::from_array_ref(&array);
    let manual = &Vec3P::from_array([1, 2, 3]);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_from_array_mut() {
    let mut array = [1, 2, 3];
    let tested = Vec3P::from_array_mut(&mut array);
    tested[1] = 5;
    let manual = &mut Vec3P::from_array([1, 5, 3]);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_as_bytes_ref() {
    let vec = vec3!(1u8, 2, 3);
    let tested = vec.as_bytes_ref();
    let manual = &[1u8, 2, 3];
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_as_bytes_ref_padded() {
    let vec = vec3!(1u8, 2, 3);
    let tested = vec.as_bytes_ref_padded();
    // This will include padding, so we just check it's not empty
    assert!(!tested.is_empty());
}

#[test]
fn test_vector_to_t() {
    let vec = vec3!(1u8, 2, 3);
    let tested = vec.to_t::<u32>();
    let manual = vec3!(1u32, 2, 3);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_align_conversion() {
    let vec = vec3!(1, 2, 3);
    let tested = vec.align();
    let manual = Vec3::from_array([1, 2, 3]);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_unalign_conversion() {
    let vec = vec3!(1, 2, 3);
    let tested = vec.unalign();
    let manual = Vec3P::from_array([1, 2, 3]);
    assert_eq!(tested, manual);
}

// Get and access tests
#[test]
fn test_vector_get() {
    let vec = vec4!(1, 2, 3, 4);
    let tested = vec.get(1);
    let manual = Some(2);
    assert_eq!(tested, manual);

    let tested = vec.get(5);
    let manual = None;
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_index_method() {
    let vec = vec4!(1, 2, 3, 4);
    let tested = vec.index(2);
    let manual = 3;
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_get_ref() {
    let vec = vec4!(1, 2, 3, 4);
    let tested = vec.get_ref(1);
    let manual = Some(&2);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_index_ref() {
    let vec = vec4!(1, 2, 3, 4);
    let tested = vec.index_ref(2);
    let manual = &3;
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_get_2_ref() {
    let tested = vec4!(1, 2, 3, 4).get_2_ref(1).map(|r| *r);
    let manual = Some(vec2p!(2, 3));
    assert_eq!(tested, manual);

    let tested = vec4!(1, 2, 3, 4).get_2_ref(3).map(|r| *r);
    let manual = None;
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_get_3_ref() {
    let tested = vec4!(1, 2, 3, 4).get_3_ref(1).map(|r| *r);
    let manual = Some(vec3p!(2, 3, 4));
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_get_4_ref() {
    let tested = vec4!(1, 2, 3, 4).get_4_ref(0).map(|r| *r);
    let manual = Some(vec4p!(1, 2, 3, 4));
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_get_mut() {
    let mut vec = vec4!(1, 2, 3, 4);
    let tested = vec.get_mut(1);
    *tested.unwrap() = 5;
    let manual = vec4!(1, 5, 3, 4);
    assert_eq!(vec, manual);
}

#[test]
fn test_vector_index_mut_method() {
    let mut vec = vec4!(1, 2, 3, 4);
    let tested = vec.index_mut(2);
    *tested = 6;
    let manual = vec4!(1, 2, 6, 4);
    assert_eq!(vec, manual);
}

#[test]
fn test_vector_get_2_mut() {
    let mut tested_owned = vec4!(1, 2, 3, 4);
    let tested = tested_owned.get_2_mut(1).map(|r| *r);
    let manual = Some(vec2p!(2, 3));
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_get_3_mut() {
    let mut tested_owned = vec4!(1, 2, 3, 4);
    let tested = tested_owned.get_3_mut(1).map(|r| *r);
    let manual = Some(vec3p!(2, 3, 4));
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_get_4_mut() {
    let mut tested_owned = vec4!(1, 2, 3, 4);
    let tested = tested_owned.get_4_mut(0).map(|r| *r);
    let manual = Some(vec4p!(1, 2, 3, 4));
    assert_eq!(tested, manual);
}

// Map and transform tests
#[test]
fn test_vector_map() {
    let vec = vec3!(1, 2, 3);
    let tested = vec.map(|x| x * 2);
    let manual = vec3!(2, 4, 6);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_map_ref() {
    let vec = vec3!(1, 2, 3);
    let tested = vec.map_ref(|x| x * 2);
    let manual = vec3!(2, 4, 6);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_map_rhs() {
    let vec1 = vec3!(1, 2, 3);
    let vec2 = vec3!(4, 5, 6);
    let tested = vec1.map_rhs(vec2, |a, b| a + b);
    let manual = vec3!(5, 7, 9);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_map_ref_rhs() {
    let vec1 = vec3!(1, 2, 3);
    let vec2 = vec3!(4, 5, 6);
    let tested = vec1.map_ref_rhs(&vec2, |a, b| a + b);
    let manual = vec3!(5, 7, 9);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_fold() {
    let vec = vec3!(1, 2, 3);
    let tested = vec.fold(|a, b| a + b);
    let manual = 6;
    assert_eq!(tested, manual);
}

// Swizzle tests
#[test]
fn test_vector_swizzle() {
    let tested = vec4!(1, 2, 3, 4).wzyx();
    let manual = vec4!(4, 3, 2, 1);
    assert_eq!(tested, manual);

    let tested = vec2p!(1, 2).yy();
    let manual = vec2p!(2, 2);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_swizzle_mut() {
    let mut tested_owned = vec4!(1, 2, 3, 4);
    let tested = tested_owned.xy_zw_mut();
    let manual = (&mut vec2p!(1, 2), &mut vec2p!(3, 4));
    assert_eq!(tested, manual);

    let mut tested_owned = vec4!(1, 2, 3, 4);
    let tested = tested_owned.x_y_zw_mut();
    let manual = (&mut 1, &mut 2, &mut vec2p!(3, 4));
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_swizzle_set() {
    let tested = {
        let mut initial = vec4!(1, 2, 3, 4);
        initial.set_xz(vec2!(5, 6));
        initial
    };
    let manual = vec4!(5, 2, 6, 4);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_swizzle_with() {
    let tested = vec4!(1, 2, 3, 4).with_yz(vec2!(5, 6));
    let manual = vec4!(1, 5, 6, 4);
    assert_eq!(tested, manual);
}

// Extended operations tests
#[test]
fn test_vector_sum() {
    let vec = vec3!(1, 2, 3);
    let tested = vec.sum();
    let manual = 6;
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_dot() {
    let vec1 = vec3!(1, 2, 3);
    let vec2 = vec3!(4, 5, 6);
    let tested = vec1.dot(vec2);
    let manual = 1 * 4 + 2 * 5 + 3 * 6;
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_cross() {
    let vec1 = vec3!(1, 0, 0);
    let vec2 = vec3!(0, 1, 0);
    let tested = vec1.cross(vec2);
    let manual = vec3!(0, 0, 1);
    assert_eq!(tested, manual);
}

// Primitive-specific tests (testing one type per macro group)
#[test]
fn test_vector_int_constants() {
    let tested = Vec3::<i32>::ZERO;
    let manual = vec3!(0, 0, 0);
    assert_eq!(tested, manual);

    let tested = Vec3::<i32>::ONE;
    let manual = vec3!(1, 1, 1);
    assert_eq!(tested, manual);

    let tested = Vec3::<i32>::NEG_ONE;
    let manual = vec3!(-1, -1, -1);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_uint_constants() {
    let tested = Vec3::<u32>::ZERO;
    let manual = vec3!(0, 0, 0);
    assert_eq!(tested, manual);

    let tested = Vec3::<u32>::ONE;
    let manual = vec3!(1, 1, 1);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_float_constants() {
    let tested = Vec3::<f32>::ZERO;
    let manual = vec3!(0.0, 0.0, 0.0);
    assert_eq!(tested, manual);

    let tested = Vec3::<f32>::ONE;
    let manual = vec3!(1.0, 1.0, 1.0);
    assert_eq!(tested, manual);

    let tested = Vec3::<f32>::NEG_ONE;
    let manual = vec3!(-1.0, -1.0, -1.0);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_int_methods() {
    let vec: Vec3<i32> = vec3!(-2, 0, 3);
    let tested = vec.is_negative();
    let manual = vec3!(true, false, false);
    assert_eq!(tested, manual);

    let tested = vec.is_bin_positive();
    let manual = vec3!(false, true, true);
    assert_eq!(tested, manual);

    let tested = vec.abs();
    let manual = vec3!(2, 0, 3);
    assert_eq!(tested, manual);

    let tested = vec.neg_abs();
    let manual = vec3!(-2, 0, -3);
    assert_eq!(tested, manual);

    let tested = vec.signumt();
    let manual = vec3!(-1, 0, 1);
    assert_eq!(tested, manual);

    let tested = vec.bin_signum();
    let manual = vec3!(-1, 1, 1);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_uint_methods() {
    let vec: Vec3<u32> = vec3!(0, 5, 10);
    let tested = vec.is_positive();
    let manual = vec3!(false, true, true);
    assert_eq!(tested, manual);

    let tested = vec.is_zero();
    let manual = vec3!(true, false, false);
    assert_eq!(tested, manual);

    let tested = vec.signumt();
    let manual = vec3!(0, 1, 1);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_float_methods() {
    let vec: Vec3<f32> = vec3!(-2.0, 0.0, 3.0);
    let tested = vec.is_positive();
    let manual = vec3!(false, false, true);
    assert_eq!(tested, manual);

    let tested = vec.is_negative();
    let manual = vec3!(true, false, false);
    assert_eq!(tested, manual);

    let tested = vec.is_zero();
    let manual = vec3!(false, true, false);
    assert_eq!(tested, manual);

    let tested = vec.is_bin_positive();
    let manual = vec3!(false, true, true);
    assert_eq!(tested, manual);

    let tested = vec.is_bin_negative();
    let manual = vec3!(true, false, false);
    assert_eq!(tested, manual);

    let tested = vec.abs();
    let manual = vec3!(2.0, 0.0, 3.0);
    assert_eq!(tested, manual);

    let tested = vec.neg_abs();
    let manual = vec3!(-2.0, 0.0, -3.0);
    assert_eq!(tested, manual);

    let tested = vec.signumt();
    let manual = vec3!(-1.0, 0.0, 1.0);
    assert_eq!(tested, manual);

    let tested = vec.bin_signum();
    let manual = vec3!(-1.0, 1.0, 1.0);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_min_max() {
    let vec1: Vec3<i32> = vec3!(1, 5, 3);
    let vec2: Vec3<i32> = vec3!(4, 2, 6);
    let tested = vec1.min(vec2);
    let manual = vec3!(1, 2, 3);
    assert_eq!(tested, manual);

    let tested = vec1.max(vec2);
    let manual = vec3!(4, 5, 6);
    assert_eq!(tested, manual);

    let tested = vec1.clamp(vec3!(2, 2, 2), vec3!(4, 4, 4));
    let manual = vec3!(2, 4, 3);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_cmin_cmax() {
    let vec: Vec3<i32> = vec3!(1, 5, 3);
    let tested = vec.fold(i32::min);
    let manual = 1;
    assert_eq!(tested, manual);

    let vec2: Vec3<i32> = vec3!(1, 5, 3);
    let tested2 = vec2.fold(i32::max);
    let manual2 = 5;
    assert_eq!(tested2, manual2);
}

#[test]
fn test_vector_abs_diff() {
    let vec1: Vec3<u32> = vec3!(5, 2, 8);
    let vec2: Vec3<u32> = vec3!(3, 7, 4);
    let tested = vec1.abs_diff(vec2);
    let manual = vec3!(2, 5, 4);
    assert_eq!(tested, manual);
}

// Standard trait implementations
#[test]
fn test_vector_default() {
    let tested: Vec3<i32> = Vec3::default();
    let manual = vec3!(0, 0, 0);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_clone() {
    let vec = vec3!(1, 2, 3);
    let tested = vec.clone();
    let manual = vec3!(1, 2, 3);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_copy() {
    let vec = vec3!(1, 2, 3);
    let tested = vec;
    let manual = vec3!(1, 2, 3);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_partial_eq() {
    let vec1 = vec3!(1, 2, 3);
    let vec2 = vec3!(1, 2, 3);
    let vec3 = vec3!(1, 2, 4);
    assert_eq!(vec1, vec2);
    assert_ne!(vec1, vec3);
}

#[test]
fn test_vector_eq() {
    let vec1 = vec3!(1, 2, 3);
    let vec2 = vec3!(1, 2, 3);
    assert_eq!(vec1, vec2);
}

#[test]
fn test_vector_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let vec1 = vec3!(1, 2, 3);
    let vec2 = vec3!(1, 2, 3);

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    vec1.hash(&mut hasher1);
    vec2.hash(&mut hasher2);

    assert_eq!(hasher1.finish(), hasher2.finish());
}

#[test]
fn test_vector_debug() {
    let vec = vec3!(1, 2, 3);
    let tested = format!("{:?}", vec);
    let manual = "(1, 2, 3)";
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_display() {
    let vec = vec3!(1, 2, 3);
    let tested = vec.to_string();
    let manual = "(1, 2, 3)";
    assert_eq!(tested, manual);
}

// Index tests
#[test]
fn test_vector_index() {
    let vec = vec3!(1, 2, 3);
    let tested = vec[1];
    let manual = 2;
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_index_mut() {
    let mut vec = vec3!(1, 2, 3);
    vec[1] = 5;
    let tested = vec;
    let manual = vec3!(1, 5, 3);
    assert_eq!(tested, manual);
}

// Operator tests (testing a few key operators)
#[test]
fn test_vector_add() {
    let vec1 = vec3!(1, 2, 3);
    let vec2 = vec3!(4, 5, 6);
    let tested = vec1 + vec2;
    let manual = vec3!(5, 7, 9);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_sub() {
    let vec1 = vec3!(5, 7, 9);
    let vec2 = vec3!(1, 2, 3);
    let tested = vec1 - vec2;
    let manual = vec3!(4, 5, 6);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_mul() {
    let vec1 = vec3!(2, 3, 4);
    let vec2 = vec3!(5, 6, 7);
    let tested = vec1 * vec2;
    let manual = vec3!(10, 18, 28);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_div() {
    let vec1 = vec3!(10, 18, 28);
    let vec2 = vec3!(2, 3, 4);
    let tested = vec1 / vec2;
    let manual = vec3!(5, 6, 7);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_neg() {
    let vec = vec3!(1, -2, 3);
    let tested = -vec;
    let manual = vec3!(-1, 2, -3);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_add_assign() {
    let mut vec = vec3!(1, 2, 3);
    vec += vec3!(4, 5, 6);
    let tested = vec;
    let manual = vec3!(5, 7, 9);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_sub_assign() {
    let mut vec = vec3!(5, 7, 9);
    vec -= vec3!(1, 2, 3);
    let tested = vec;
    let manual = vec3!(4, 5, 6);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_mul_assign() {
    let mut vec = vec3!(2, 3, 4);
    vec *= vec3!(5, 6, 7);
    let tested = vec;
    let manual = vec3!(10, 18, 28);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_div_assign() {
    let mut vec = vec3!(10, 18, 28);
    vec /= vec3!(2, 3, 4);
    let tested = vec;
    let manual = vec3!(5, 6, 7);
    assert_eq!(tested, manual);
}
