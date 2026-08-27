//! Based on `https://github.com/bitshifter/mathbench-rs/blob/master/benches/ray_sphere_intersect.rs`.

use std::hint::black_box;

use ggmath::{Vec3, Vec3A};
use wide::{f32x4, f32x8};

use crate::{ARRAY_LEN, bench};

macro_rules! function {
    ($Vec3:ty) => {
        |ray_direction: $Vec3| {
            let sphere_center = black_box(<$Vec3>::ZERO);
            let sphere_radius_squared = black_box(100.0);
            let ray_origin = black_box(<$Vec3>::new(0.0, 0.0, -11.0));

            let oc = ray_origin - sphere_center;
            let b = oc.dot(ray_direction);
            let c = oc.length_squared() - sphere_radius_squared;
            let discriminant = b * b - c;

            if discriminant > 0.0 {
                let discriminant_sqrt = discriminant.sqrt();
                let t1 = -b - discriminant_sqrt;

                if t1 > 0.0 {
                    t1
                } else {
                    let t2 = -b + discriminant_sqrt;

                    if t2 > 0.0 { t2 } else { f32::MAX }
                }
            } else {
                f32::MAX
            }
        }
    };
}

macro_rules! soa_function {
    ($Vec3:ty, $Simd:ty) => {
        |ray_direction: $Vec3| {
            let sphere_center = black_box(<$Vec3>::ZERO);
            let sphere_radius_squared = black_box(100.0);
            let ray_origin = black_box(<$Vec3>::new(
                <$Simd>::ZERO,
                <$Simd>::ZERO,
                <$Simd>::splat(-11.0),
            ));

            let oc = ray_origin - sphere_center;
            let b = oc.dot(ray_direction);
            let c = oc.length_squared() - sphere_radius_squared;
            let discriminant = b * b - c;

            let discriminant_sqrt = discriminant.sqrt();
            let t1 = -b - discriminant_sqrt;
            let t2 = -b + discriminant_sqrt;

            let discriminant_is_positive = discriminant.simd_gt(<$Simd>::ZERO);
            (discriminant_is_positive & t1.simd_gt(<$Simd>::ZERO)).select(
                t1,
                (discriminant_is_positive & t2.simd_gt(<$Simd>::ZERO)).select(t2, <$Simd>::MAX),
            )
        }
    };
}

bench!(
    vec3f32,
    ARRAY_LEN,
    (unaligned, function!(Vec3<f32>)),
    (unaligned_glam, function!(glam::Vec3)),
    (aligned, function!(Vec3A<f32>)),
    (aligned_glam, function!(glam::Vec3A)),
    (x4_unaligned, soa_function!(Vec3<f32x4>, f32x4)),
    (x8_unaligned, soa_function!(Vec3<f32x8>, f32x8)),
);
