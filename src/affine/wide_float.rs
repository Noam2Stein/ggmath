macro_rules! impl_wide_float {
    ($Wide:ident) => {
        // MISSING: is_nan
        // MISSING: is_finite
        // MISSING: inverse
        // MISSING: try_inverse
        // MISSING: inverse_or
        // MISSING: inverse_or_zero
        // MISSING: abs_diff_eq
        // MISSING: from_angle
        // MISSING: from_angle_translation
        // MISSING: from_scale_angle
        // MISSING: from_scale_angle_translation
        // MISSING: to_scale_angle_translation
        // MISSING: from_rotation_x
        // MISSING: from_rotation_y
        // MISSING: from_rotation_z
        // MISSING: from_quat
        // MISSING: from_axis_angle
        // MISSING: from_euler
        // MISSING: from_scale_rotation
        // MISSING: from_rotation_translation
        // MISSING: from_scale_rotation_translation
        // MISSING: look_to_lh
        // MISSING: look_to_rh
        // MISSING: look_at_lh
        // MISSING: look_to_rh
        // MISSING: to_euler
        // MISSING: to_scale_rotation_translation
    };
}
impl_wide_float!(f32x4);
impl_wide_float!(f32x8);
impl_wide_float!(f32x16);
impl_wide_float!(f64x2);
impl_wide_float!(f64x4);
impl_wide_float!(f64x8);
