macro_rules! impl_wide_float {
    ($Wide:ident) => {
        // MISSING: from_rotation_x
        // MISSING: from_rotation_y
        // MISSING: from_rotation_z
        // MISSING: from_axis_angle
        // MISSING: from_scaled_axis
        // MISSING: from_rotation_arc
        // MISSING: from_rotation_arc_colinear
        // MISSING: from_euler
        // MISSING: from_matrix
        // MISSING: look_to_lh
        // MISSING: look_to_rh
        // MISSING: look_at_lh
        // MISSING: look_at_rh
        // MISSING: to_axis_angle
        // MISSING: to_scaled_axis
        // MISSING: to_euler
        // MISSING: is_nan
        // MISSING: is_finite
        // MISSING: inverse
        // MISSING: angle_between
        // MISSING: lerp
        // MISSING: slerp
        // MISSING: rotate_towards
        // MISSING: length
        // MISSING: normalize
        // MISSING: try_normalize
        // MISSING: normalize_or
        // MISSING: normalize_and_length
        // MISSING: is_normalized
        // MISSING: abs_diff_eq
    };
}
impl_wide_float!(f32x4);
impl_wide_float!(f32x8);
impl_wide_float!(f32x16);
impl_wide_float!(f64x2);
impl_wide_float!(f64x4);
impl_wide_float!(f64x8);
