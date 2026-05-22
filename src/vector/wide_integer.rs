macro_rules! impl_wide_integer {
    ($Wide:ident) => {
        // MISSING: all
        // MISSING: any
        // MISSING: blend
        // MISSING: simd_eq
        // MISSING: simd_ne
        // MISSING: simd_eq_mask
        // MISSING: simd_ne_mask
        // MISSING: simd_lt_mask
        // MISSING: simd_gt_mask
        // MISSING: simd_le_mask
        // MISSING: simd_ge_mask
        // MISSING: max
        // MISSING: min
        // MISSING: clamp
        // MISSING: max_element
        // MISSING: min_element
        // MISSING: checked_add
        // MISSING: checked_sub
        // MISSING: checked_mul
        // MISSING: checked_div
        // MISSING: checked_rem
        // MISSING: saturating_add
        // MISSING: saturating_sub
        // MISSING: saturating_mul
        // MISSING: wrapping_add
        // MISSING: wrapping_sub
        // MISSING: wrapping_mul
    };
}
impl_wide_integer!(i8x16);
impl_wide_integer!(i8x32);
impl_wide_integer!(i16x8);
impl_wide_integer!(i16x16);
impl_wide_integer!(i16x32);
impl_wide_integer!(i32x4);
impl_wide_integer!(i32x8);
impl_wide_integer!(i32x16);
impl_wide_integer!(i64x2);
impl_wide_integer!(i64x4);
impl_wide_integer!(i64x8);
impl_wide_integer!(u8x16);
impl_wide_integer!(u8x32);
impl_wide_integer!(u16x8);
impl_wide_integer!(u16x16);
impl_wide_integer!(u16x32);
impl_wide_integer!(u32x4);
impl_wide_integer!(u32x8);
impl_wide_integer!(u32x16);
impl_wide_integer!(u64x2);
impl_wide_integer!(u64x4);
impl_wide_integer!(u64x8);
