use std::process::Command;

use cfg_aliases::cfg_aliases;

fn main() {
    codegen::codegen();

    Command::new("cargo").arg("fmt").status().unwrap();

    cfg_aliases! {
        f32x4_simd: {
            any(
                all(target_arch = "x86", target_feature = "sse"),
                all(target_arch = "x86_64", target_feature = "sse"),
                all(target_arch = "arm", target_feature = "neon"),
                all(target_arch = "aarch64", target_feature = "neon"),
                all(target_arch = "wasm32", target_feature = "simd128")
            )
        },
        f64x2_simd: {
            any(
                all(target_arch = "x86", target_feature = "sse2"),
                all(target_arch = "x86_64", target_feature = "sse2"),
                all(target_arch = "arm", target_feature = "neon"),
                all(target_arch = "aarch64", target_feature = "neon"),
                all(target_arch = "wasm32", target_feature = "simd128")
            )
        },
        f64x4_simd: {
            any(
                all(target_arch = "x86", target_feature = "avx"),
                all(target_arch = "x86_64", target_feature = "avx")
            )
        },
        i32x4_simd: {
            any(
                all(target_arch = "x86", target_feature = "sse2"),
                all(target_arch = "x86_64", target_feature = "sse2"),
                all(target_arch = "arm", target_feature = "neon"),
                all(target_arch = "aarch64", target_feature = "neon"),
                all(target_arch = "wasm32", target_feature = "simd128")
            )
        },
        i64x2_simd: {
            any(
                all(target_arch = "x86", target_feature = "sse2"),
                all(target_arch = "x86_64", target_feature = "sse2"),
                all(target_arch = "arm", target_feature = "neon"),
                all(target_arch = "aarch64", target_feature = "neon"),
                all(target_arch = "wasm32", target_feature = "simd128")
            )
        },
        i64x4_simd: {
            any(
                all(target_arch = "x86", target_feature = "avx2"),
                all(target_arch = "x86_64", target_feature = "avx2")
            )
        },
    }
}
