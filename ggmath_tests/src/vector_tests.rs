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

// Thanks ChatGPT for the tests!

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

// Magnitude and normalization tests
#[test]
fn test_vector_mag() {
    // Test simple 3-4-5 right triangle
    let vec = vec2!(3.0f32, 4.0);
    let tested = vec.mag();
    let expected = 5.0;
    assert!((tested - expected).abs() < f32::EPSILON);

    // Test unit vector
    let vec = vec3!(1.0f32, 0.0, 0.0);
    let tested = vec.mag();
    let expected = 1.0;
    assert!((tested - expected).abs() < f32::EPSILON);

    // Test zero vector
    let vec = vec3!(0.0f32, 0.0, 0.0);
    let tested = vec.mag();
    let expected = 0.0;
    assert!((tested - expected).abs() < f32::EPSILON);

    // Test negative components
    let vec = vec2!(-3.0f32, -4.0);
    let tested = vec.mag();
    let expected = 5.0;
    assert!((tested - expected).abs() < f32::EPSILON);

    // Test f64 precision
    let vec = vec3!(1.0f64, 1.0, 1.0);
    let tested = vec.mag();
    let expected = (3.0f64).sqrt();
    assert!((tested - expected).abs() < f64::EPSILON);
}

#[test]
fn test_vector_square_mag() {
    // Test simple 3-4-5 right triangle
    let vec = vec2!(3.0f32, 4.0);
    let tested = vec.square_mag();
    let expected = 25.0;
    assert!((tested - expected).abs() < f32::EPSILON);

    // Test unit vector
    let vec = vec3!(1.0f32, 0.0, 0.0);
    let tested = vec.square_mag();
    let expected = 1.0;
    assert!((tested - expected).abs() < f32::EPSILON);

    // Test zero vector
    let vec = vec3!(0.0f32, 0.0, 0.0);
    let tested = vec.square_mag();
    let expected = 0.0;
    assert!((tested - expected).abs() < f32::EPSILON);

    // Test negative components (should be same as positive)
    let vec = vec2!(-3.0f32, -4.0);
    let tested = vec.square_mag();
    let expected = 25.0;
    assert!((tested - expected).abs() < f32::EPSILON);

    // Verify square_mag is faster than mag squared (same result)
    let vec = vec4!(2.0f32, 3.0, 6.0, 1.0);
    let mag_squared = vec.mag() * vec.mag();
    let square_mag = vec.square_mag();
    assert!((mag_squared - square_mag).abs() < f32::EPSILON);
}

#[test]
fn test_vector_normalize() {
    // Test normalization of simple vector
    let vec = vec2!(3.0f32, 4.0);
    let tested = vec.normalize();
    let expected = vec2!(0.6, 0.8); // 3/5, 4/5
    assert!((tested.x() - expected.x()).abs() < 0.0001);
    assert!((tested.y() - expected.y()).abs() < 0.0001);

    // Verify normalized vector has magnitude 1
    let magnitude = tested.mag();
    assert!((magnitude - 1.0).abs() < 0.0001);

    // Test normalizing unit vector (should remain unchanged)
    let vec = vec3!(1.0f32, 0.0, 0.0);
    let tested = vec.normalize();
    let expected = vec3!(1.0, 0.0, 0.0);
    assert!((tested.x() - expected.x()).abs() < f32::EPSILON);
    assert!((tested.y() - expected.y()).abs() < f32::EPSILON);
    assert!((tested.z() - expected.z()).abs() < f32::EPSILON);

    // Test with negative components
    let vec = vec2!(-6.0f32, -8.0);
    let tested = vec.normalize();
    let expected = vec2!(-0.6, -0.8);
    assert!((tested.x() - expected.x()).abs() < 0.0001);
    assert!((tested.y() - expected.y()).abs() < 0.0001);

    // Verify normalized magnitude is 1
    let magnitude = tested.mag();
    assert!((magnitude - 1.0).abs() < 0.0001);
}

#[test]
fn test_vector_with_mag() {
    // Test scaling to specific magnitude
    let vec = vec2!(3.0f32, 4.0);
    let tested = vec.with_mag(10.0);
    let expected_mag = 10.0;
    let actual_mag = tested.mag();
    assert!((actual_mag - expected_mag).abs() < 0.0001);

    // Verify direction is preserved (should be 6, 8 for mag 10)
    let expected = vec2!(6.0, 8.0);
    assert!((tested.x() - expected.x()).abs() < 0.0001);
    assert!((tested.y() - expected.y()).abs() < 0.0001);

    // Test with unit vector
    let vec = vec3!(1.0f32, 0.0, 0.0);
    let tested = vec.with_mag(5.0);
    let expected = vec3!(5.0, 0.0, 0.0);
    assert!((tested.x() - expected.x()).abs() < f32::EPSILON);
    assert!((tested.y() - expected.y()).abs() < f32::EPSILON);
    assert!((tested.z() - expected.z()).abs() < f32::EPSILON);

    // Test magnitude 0
    let vec = vec2!(3.0f32, 4.0);
    let tested = vec.with_mag(0.0);
    let expected = vec2!(0.0, 0.0);
    assert!((tested.x() - expected.x()).abs() < f32::EPSILON);
    assert!((tested.y() - expected.y()).abs() < f32::EPSILON);
}

#[test]
fn test_vector_with_square_mag() {
    // Test scaling to specific square magnitude
    let vec = vec2!(3.0f32, 4.0); // original square_mag = 25
    let tested = vec.with_square_mag(100.0); // target square_mag = 100
    let expected_square_mag = 100.0;
    let actual_square_mag = tested.square_mag();
    assert!((actual_square_mag - expected_square_mag).abs() < 0.001);

    // Verify magnitude is sqrt(100) = 10
    let actual_mag = tested.mag();
    assert!((actual_mag - 10.0).abs() < 0.001);

    // Test with unit vector
    let vec = vec3!(1.0f32, 0.0, 0.0);
    let tested = vec.with_square_mag(25.0);
    let expected = vec3!(5.0, 0.0, 0.0);
    assert!((tested.x() - expected.x()).abs() < 0.001);
    assert!((tested.y() - expected.y()).abs() < f32::EPSILON);
    assert!((tested.z() - expected.z()).abs() < f32::EPSILON);
}

#[test]
fn test_vector_with_min_mag() {
    // Test vector already above minimum
    let vec = vec2!(3.0f32, 4.0);
    let tested = vec.with_min_mag(3.0);
    assert!((tested.x() - vec.x()).abs() < f32::EPSILON);
    assert!((tested.y() - vec.y()).abs() < f32::EPSILON);

    // Test vector below minimum
    let vec = vec2!(1.0f32, 0.0);
    let tested = vec.with_min_mag(5.0);
    let expected = vec2!(5.0, 0.0);
    assert!((tested.x() - expected.x()).abs() < 0.0001);
    assert!((tested.y() - expected.y()).abs() < f32::EPSILON);

    // Verify new magnitude
    let actual_mag = tested.mag();
    assert!((actual_mag - 5.0).abs() < 0.0001);

    // Test with very small vector
    let vec = vec3!(0.1f32, 0.1, 0.1);
    let tested = vec.with_min_mag(2.0);
    let actual_mag = tested.mag();
    assert!((actual_mag - 2.0).abs() < 0.001);
}

#[test]
fn test_vector_with_max_mag() {
    // Test vector already below maximum
    let vec = vec2!(3.0f32, 4.0);
    let tested = vec.with_max_mag(10.0);
    assert!((tested.x() - vec.x()).abs() < f32::EPSILON);
    assert!((tested.y() - vec.y()).abs() < f32::EPSILON);

    // Test vector above maximum
    let vec = vec2!(6.0f32, 8.0);
    let tested = vec.with_max_mag(5.0);
    let expected = vec2!(3.0, 4.0);
    assert!((tested.x() - expected.x()).abs() < 0.0001);
    assert!((tested.y() - expected.y()).abs() < 0.0001);

    // Verify new magnitude
    let actual_mag = tested.mag();
    assert!((actual_mag - 5.0).abs() < 0.0001);

    // Test with very large vector
    let vec = vec3!(100.0f32, 100.0, 100.0);
    let tested = vec.with_max_mag(1.0);
    let actual_mag = tested.mag();
    assert!((actual_mag - 1.0).abs() < 0.001);
}

#[test]
fn test_vector_clamp_mag() {
    // Test vector within range
    let vec = vec2!(3.0f32, 4.0);
    let tested = vec.clamp_mag(2.0, 10.0);
    assert!((tested.x() - vec.x()).abs() < f32::EPSILON);
    assert!((tested.y() - vec.y()).abs() < f32::EPSILON);

    // Test vector below minimum
    let vec = vec2!(0.6f32, 0.8);
    let tested = vec.clamp_mag(3.0, 10.0);
    let actual_mag = tested.mag();
    assert!((actual_mag - 3.0).abs() < 0.001);

    // Test vector above maximum
    let vec = vec2!(12.0f32, 16.0);
    let tested = vec.clamp_mag(3.0, 10.0);
    let actual_mag = tested.mag();
    assert!((actual_mag - 10.0).abs() < 0.001);

    // Test edge case where min and max are the same
    let vec = vec3!(1.0f32, 1.0, 1.0);
    let tested = vec.clamp_mag(5.0, 5.0);
    let actual_mag = tested.mag();
    assert!((actual_mag - 5.0).abs() < 0.001);
}

#[test]
fn test_vector_with_min_square_mag() {
    // Test vector already above minimum
    let vec = vec2!(3.0f32, 4.0);
    let tested = vec.with_min_square_mag(9.0);
    assert!((tested.x() - vec.x()).abs() < f32::EPSILON);
    assert!((tested.y() - vec.y()).abs() < f32::EPSILON);

    // Test vector below minimum
    let vec = vec2!(1.0f32, 0.0);
    let tested = vec.with_min_square_mag(25.0);
    let expected_square_mag = 25.0;
    let actual_square_mag = tested.square_mag();
    assert!((actual_square_mag - expected_square_mag).abs() < 0.001);

    // Verify direction is preserved
    let expected = vec2!(5.0, 0.0);
    assert!((tested.x() - expected.x()).abs() < 0.001);
    assert!((tested.y() - expected.y()).abs() < f32::EPSILON);
}

#[test]
fn test_vector_with_max_square_mag() {
    // Test vector already below maximum
    let vec = vec2!(3.0f32, 4.0);
    let tested = vec.with_max_square_mag(100.0);
    assert!((tested.x() - vec.x()).abs() < f32::EPSILON);
    assert!((tested.y() - vec.y()).abs() < f32::EPSILON);

    // Test vector above maximum
    let vec = vec2!(6.0f32, 8.0);
    let tested = vec.with_max_square_mag(25.0);
    let expected_square_mag = 25.0;
    let actual_square_mag = tested.square_mag();
    assert!((actual_square_mag - expected_square_mag).abs() < 0.001);

    // Verify direction is preserved
    let expected = vec2!(3.0, 4.0);
    assert!((tested.x() - expected.x()).abs() < 0.001);
    assert!((tested.y() - expected.y()).abs() < 0.001);
}

#[test]
fn test_vector_clamp_square_mag() {
    // Test vector within range
    let vec = vec2!(3.0f32, 4.0);
    let tested = vec.clamp_square_mag(4.0, 100.0);
    assert!((tested.x() - vec.x()).abs() < f32::EPSILON);
    assert!((tested.y() - vec.y()).abs() < f32::EPSILON);

    // Test vector below minimum (should be scaled up)
    let vec = vec2!(1.0f32, 0.0); // square_mag = 1
    let tested = vec.clamp_square_mag(9.0, 100.0);
    let actual_square_mag = tested.square_mag();
    assert!((actual_square_mag - 9.0).abs() < 0.001);

    // Test vector above maximum (should be scaled down)
    let vec = vec2!(10.0f32, 0.0); // square_mag = 100
    let tested = vec.clamp_square_mag(4.0, 25.0);
    let actual_square_mag = tested.square_mag();
    assert!((actual_square_mag - 25.0).abs() < 0.001);

    // Test edge case where min and max are the same
    let vec = vec3!(1.0f32, 1.0, 1.0);
    let tested = vec.clamp_square_mag(25.0, 25.0);
    let actual_square_mag = tested.square_mag();
    assert!((actual_square_mag - 25.0).abs() < 0.001);
}

#[test]
fn test_magnitude_functions_with_f64() {
    // Test f64 precision for magnitude functions
    let vec = vec2!(3.0f64, 4.0);

    // Test mag
    let mag = vec.mag();
    assert!((mag - 5.0).abs() < f64::EPSILON);

    // Test square_mag
    let square_mag = vec.square_mag();
    assert!((square_mag - 25.0).abs() < f64::EPSILON);

    // Test normalize
    let normalized = vec.normalize();
    let normalized_mag = normalized.mag();
    assert!((normalized_mag - 1.0).abs() < 1e-15);

    // Test with_mag
    let scaled = vec.with_mag(10.0);
    let scaled_mag = scaled.mag();
    assert!((scaled_mag - 10.0).abs() < 1e-14);
}

#[test]
fn test_magnitude_edge_cases() {
    // Test with very small numbers near floating point precision limits
    let tiny_vec = vec2!(f32::EPSILON, f32::EPSILON);
    let mag = tiny_vec.mag();
    assert!(mag > 0.0);
    assert!(mag.is_finite());

    // Test normalize of very small vector
    let normalized = tiny_vec.normalize();
    let normalized_mag = normalized.mag();
    assert!((normalized_mag - 1.0).abs() < 0.001);

    // Test with large numbers (but not so large they overflow when squared)
    let large_vec = vec2!(1e15f32, 1e15f32);
    let mag = large_vec.mag();
    assert!(mag.is_finite());
    assert!(mag > 0.0);

    // Test magnitude consistency across vector dimensions
    let vec2 = vec2!(3.0f32, 4.0);
    let vec3 = vec3!(3.0f32, 4.0, 0.0);
    let vec4 = vec4!(3.0f32, 4.0, 0.0, 0.0);

    assert!((vec2.mag() - vec3.mag()).abs() < f32::EPSILON);
    assert!((vec2.mag() - vec4.mag()).abs() < f32::EPSILON);
}
