use ggmath::*;

#[test]
fn test_vector_construction() {
    let tested = vec4!(5, vec2!(1, 2), 6);
    let manual = Vec4::from_array([5, 1, 2, 6]);
    assert_eq!(tested, manual);
}

#[test]
fn test_vector_get() {
    let tested = vec4!(1, 2, 3, 4).get_2_ref(1).map(|r| *r);
    let manual = Some(vec2p!(2, 3));
    assert_eq!(tested, manual);

    let tested = vec4!(1, 2, 3, 4).get_2_mut(1).map(|r| *r);
    let manual = Some(vec2p!(2, 3));
    assert_eq!(tested, manual);

    let tested = vec4!(1, 2, 3, 4).get_2_ref(3).map(|r| *r);
    let manual = None;
    assert_eq!(tested, manual);
}

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
