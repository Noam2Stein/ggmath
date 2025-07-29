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

#[test]
#[should_panic]
fn test_vector_index_out_of_bounds() {
    let vec = vec3!(1, 2, 3);
    let _ = vec.index(3); // Should panic - only indices 0, 1, 2 are valid
}

#[test]
#[should_panic]
fn test_vector_index_operator_out_of_bounds() {
    let vec = vec4!(1, 2, 3, 4);
    let _ = vec[4]; // Should panic - only indices 0, 1, 2, 3 are valid
}

#[test]
#[should_panic]
fn test_vector_index_mut_out_of_bounds() {
    let mut vec = vec2!(1, 2);
    vec[2] = 5; // Should panic - only indices 0, 1 are valid
}

#[test]
fn test_vector_get_bounds_checking() {
    let vec = vec3!(1, 2, 3);

    // Valid indices should return Some
    assert_eq!(vec.get(0), Some(1));
    assert_eq!(vec.get(1), Some(2));
    assert_eq!(vec.get(2), Some(3));

    // Invalid indices should return None
    assert_eq!(vec.get(3), None);
    assert_eq!(vec.get(10), None);
    assert_eq!(vec.get(usize::MAX), None);
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

// Alignment tests - VecAligned vs VecPacked behavior
#[test]
fn test_vector_alignment_differences() {
    let aligned = vec3!(1.0f32, 2.0, 3.0);
    let packed = vec3p!(1.0f32, 2.0, 3.0);

    // Values should be equal
    assert_eq!(aligned, packed);

    // But memory layout should be different
    assert_ne!(size_of_val(&aligned), size_of_val(&packed));

    // Aligned should be larger due to padding
    assert!(size_of_val(&aligned) >= size_of_val(&packed));

    // Can convert between alignments
    let aligned_to_packed = aligned.unalign();
    let packed_to_aligned = packed.align();
    assert_eq!(aligned_to_packed, packed);
    assert_eq!(packed_to_aligned, aligned);
}

// Type conversion tests
#[test]
fn test_vector_to_t_conversions() {
    // Test supported type conversions
    let vec_u8 = vec3!(1u8, 2, 3);
    let vec_u16: Vec3<u16> = vec_u8.to_t();
    assert_eq!(vec_u16, vec3!(1u16, 2, 3));

    let vec_i8 = vec2!(100i8, 50);
    let vec_f32: Vec2<f32> = vec_i8.to_t();
    assert_eq!(vec_f32, vec2!(100.0f32, 50.0));

    let vec_bool = vec4!(true, false, true, false);
    let vec_u8: Vec4<u8> = vec_bool.to_t();
    assert_eq!(vec_u8, vec4!(1u8, 0, 1, 0));
}

// Cross product edge cases
#[test]
fn test_vector_cross_product_edge_cases() {
    // Parallel vectors should give zero cross product
    let vec1 = vec3!(1.0f32, 2.0, 3.0);
    let vec2 = vec3!(2.0f32, 4.0, 6.0); // Parallel to vec1
    let cross = vec1.cross(vec2);
    assert!((cross.x()).abs() < 0.0001);
    assert!((cross.y()).abs() < 0.0001);
    assert!((cross.z()).abs() < 0.0001);

    // Zero vector cross product
    let zero = vec3!(0.0f32, 0.0, 0.0);
    let any = vec3!(1.0f32, 2.0, 3.0);
    let cross_zero = zero.cross(any);
    assert_eq!(cross_zero, vec3!(0.0, 0.0, 0.0));

    // Anti-commutativity: a × b = -(b × a)
    let a = vec3!(1.0f32, 0.0, 0.0);
    let b = vec3!(0.0f32, 1.0, 0.0);
    let cross_ab = a.cross(b);
    let cross_ba = b.cross(a);
    assert_eq!(cross_ab, -cross_ba);

    // Magnitude of cross product for unit vectors
    let unit_x = vec3!(1.0f32, 0.0, 0.0);
    let unit_y = vec3!(0.0f32, 1.0, 0.0);
    let cross_units = unit_x.cross(unit_y);
    assert!((cross_units.mag() - 1.0).abs() < f32::EPSILON);
}

// Comprehensive swizzle testing
#[test]
fn test_vector_swizzle_comprehensive() {
    let vec4 = vec4!(1, 2, 3, 4);

    // Test all 2-component swizzles for vec4
    assert_eq!(vec4.xx(), vec2!(1, 1));
    assert_eq!(vec4.xy(), vec2!(1, 2));
    assert_eq!(vec4.xz(), vec2!(1, 3));
    assert_eq!(vec4.xw(), vec2!(1, 4));
    assert_eq!(vec4.yx(), vec2!(2, 1));
    assert_eq!(vec4.yy(), vec2!(2, 2));
    assert_eq!(vec4.yz(), vec2!(2, 3));
    assert_eq!(vec4.yw(), vec2!(2, 4));
    assert_eq!(vec4.zx(), vec2!(3, 1));
    assert_eq!(vec4.zy(), vec2!(3, 2));
    assert_eq!(vec4.zz(), vec2!(3, 3));
    assert_eq!(vec4.zw(), vec2!(3, 4));
    assert_eq!(vec4.wx(), vec2!(4, 1));
    assert_eq!(vec4.wy(), vec2!(4, 2));
    assert_eq!(vec4.wz(), vec2!(4, 3));
    assert_eq!(vec4.ww(), vec2!(4, 4));

    // Test 3-component swizzles
    assert_eq!(vec4.xyz(), vec3!(1, 2, 3));
    assert_eq!(vec4.zyx(), vec3!(3, 2, 1));
    assert_eq!(vec4.www(), vec3!(4, 4, 4));

    // Test 4-component swizzles
    assert_eq!(vec4.xyzw(), vec4!(1, 2, 3, 4));
    assert_eq!(vec4.wzyx(), vec4!(4, 3, 2, 1));
}

#[test]
fn test_vector_swizzle_set_operations() {
    let mut vec4 = vec4!(1, 2, 3, 4);

    // Test swizzle set operations
    vec4.set_xy(vec2!(10, 20));
    assert_eq!(vec4, vec4!(10, 20, 3, 4));

    vec4.set_zw(vec2!(30, 40));
    assert_eq!(vec4, vec4!(10, 20, 30, 40));

    vec4.set_xyz(vec3!(100, 200, 300));
    assert_eq!(vec4, vec4!(100, 200, 300, 40));
}

#[test]
fn test_vector_swizzle_with_operations() {
    let vec4 = vec4!(1, 2, 3, 4);

    // Test with operations (non-mutating)
    let result = vec4.with_xy(vec2!(10, 20));
    assert_eq!(result, vec4!(10, 20, 3, 4));
    assert_eq!(vec4, vec4!(1, 2, 3, 4)); // Original unchanged

    let result = vec4.with_zw(vec2!(30, 40));
    assert_eq!(result, vec4!(1, 2, 30, 40));
}

#[test]
fn test_vector_division_by_zero() {
    let vec = vec2!(1.0f32, 2.0);
    let zero_vec = vec2!(0.0f32, 0.0);

    let result = vec / zero_vec;
    assert!(result.x().is_infinite());
    assert!(result.y().is_infinite());
}

// Test different scalar types
#[test]
fn test_vector_different_scalar_types() {
    // Boolean vectors
    let bool_vec = vec3!(true, false, true);
    assert_eq!(bool_vec.x(), true);
    assert_eq!(bool_vec.y(), false);
    assert_eq!(bool_vec.z(), true);

    // u8 vectors (test byte-sized operations)
    let u8_vec = vec4!(1u8, 2, 3, 4);
    assert_eq!(u8_vec.sum(), 10);

    // Large integer types
    let u64_vec = vec2!(u64::MAX, u64::MAX / 2);
    assert!(u64_vec.x() > u64_vec.y());
}

// Memory layout and size assertions
#[test]
fn test_vector_memory_layout() {
    // Ensure packed vectors have expected sizes
    assert_eq!(size_of::<Vec2P<f32>>(), 2 * size_of::<f32>());
    assert_eq!(size_of::<Vec3P<f32>>(), 3 * size_of::<f32>());
    assert_eq!(size_of::<Vec4P<f32>>(), 4 * size_of::<f32>());

    // Aligned vectors should be at least as large as packed
    assert!(size_of::<Vec2<f32>>() >= size_of::<Vec2P<f32>>());
    assert!(size_of::<Vec3<f32>>() >= size_of::<Vec3P<f32>>());
    assert!(size_of::<Vec4<f32>>() >= size_of::<Vec4P<f32>>());

    // Test specific sizes mentioned in the README
    // Vec3 aligned should typically be 16 bytes (4 * f32), not 12
    assert_eq!(size_of::<Vec3<f32>>(), 16);
    assert_eq!(size_of::<Vec3P<f32>>(), 12);
}

// Test iterator functionality
#[test]
fn test_vector_iterator() {
    let vec = vec3!(1, 2, 3);
    let mut iter = vec.into_iter();

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);

    // Test collect back to vector
    let collected: Vec<i32> = vec4!(10, 20, 30, 40).into_iter().collect();
    assert_eq!(collected, vec![10, 20, 30, 40]);
}

// Test splat functions
#[test]
fn test_splat_functions() {
    // Test module-level splat functions
    assert_eq!(splat2(5), vec2!(5, 5));
    assert_eq!(splat3(7.0f32), vec3!(7.0, 7.0, 7.0));
    assert_eq!(splat4(true), vec4!(true, true, true, true));
}

// Test from_fn functionality
#[test]
fn test_vector_from_fn_edge_cases() {
    // Test with different functions
    let squares = Vec4::from_fn(|i| (i * i) as f32);
    assert_eq!(squares, vec4!(0.0, 1.0, 4.0, 9.0));

    let alternating = Vec3::from_fn(|i| if i % 2 == 0 { 1 } else { -1 });
    assert_eq!(alternating, vec3!(1, -1, 1));
}

// Test map variants
#[test]
fn test_vector_map_variants() {
    let vec = vec3!(1, 2, 3);

    // Test map with different operations
    let doubled = vec.map(|x| x * 2);
    assert_eq!(doubled, vec3!(2, 4, 6));

    let negated = vec.map(|x| -x);
    assert_eq!(negated, vec3!(-1, -2, -3));

    let as_f32 = vec3!(1i8, 2, 3).map(|x| x as f32);
    assert_eq!(as_f32, vec3!(1.0f32, 2.0, 3.0));
}

// Test fold operations
#[test]
fn test_vector_fold_operations() {
    let vec = vec4!(1, 2, 3, 4);

    // Test different fold operations
    assert_eq!(vec.fold(|a, b| a + b), 10); // sum
    assert_eq!(vec.fold(|a, b| a * b), 24); // product
    assert_eq!(vec.fold(|a, b| a.max(b)), 4); // max
    assert_eq!(vec.fold(|a, b| a.min(b)), 1); // min
}

// Test constants for different types
#[test]
fn test_vector_constants() {
    // Test that constants work for different numeric types
    assert_eq!(Vec3::<f64>::ZERO, vec3!(0.0f64, 0.0, 0.0));
    assert_eq!(Vec4::<u8>::ZERO, vec4!(0u8, 0, 0, 0));
    assert_eq!(Vec4::<u8>::ONE, vec4!(1u8, 1, 1, 1));
    assert_eq!(Vec2::<i16>::ZERO, vec2!(0i16, 0));
    assert_eq!(Vec2::<i16>::ONE, vec2!(1i16, 1));

    // Test negative one for signed types
    assert_eq!(Vec3::<i64>::NEG_ONE, vec3!(-1i64, -1, -1));
    assert_eq!(Vec2::<f32>::NEG_ONE, vec2!(-1.0f32, -1.0));
}
