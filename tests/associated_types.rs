use ggmath::Vec3;

#[test]
fn test_to_bits() {
    let vector = Vec3::<f32>::new(1.0, 2.0, 3.0);
    assert_eq!(vector.to_bits(), vector.map(f32::to_bits));
}

#[test]
fn test_cast_unsigned() {
    let vector = Vec3::<i32>::new(1, 2, 3);
    assert_eq!(vector.cast_unsigned(), vector.map(i32::cast_unsigned));
}

#[test]
fn test_cast_signed() {
    let vector = Vec3::<u32>::new(1, 2, 3);
    assert_eq!(vector.cast_signed(), vector.map(u32::cast_signed));
}
