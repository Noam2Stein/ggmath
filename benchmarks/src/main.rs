#![allow(unused_variables)]

use std::{
    fmt::{self, Display, Formatter},
    time::{Duration, Instant},
};

const REPEATS: usize = 1000;
const BENCHMARK_REPEATS: usize = 1000;

fn main() {
    print_benchmark(
        "xzy funny udi test",
        || {
            use ggmath::primitive_aliases::f32::*;

            let a = fvec3((1.0, 2.0, 3.0));
            let b = a.xzy();
        },
        || {
            use glam::{f32::*, Vec3Swizzles};

            let a = vec3a(1.0, 2.0, 3.0);
            let b = a.xzy();
        },
    );
}

fn print_benchmark(name: impl Display, f_ggmath: fn(), f_glam: fn()) {
    let result = run_benchmark(f_ggmath, f_glam);

    println!("{name}: {result}")
}

struct BenchmarkResult {
    ggmath_speed: f64,
}
impl Display for BenchmarkResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.ggmath_speed > 1.0 {
            // GG 😎
            write!(f, "\x1b[32mggmath > glam\x1b[0m")
        } else {
            // GG 😔
            write!(f, "\x1b[31mggmath < glam\x1b[0m")
        }?;

        write!(f, ". ggmath speed / glam speed = {}", self.ggmath_speed)?;

        Ok(())
    }
}

fn run_benchmark(f_ggmath: fn(), f_glam: fn()) -> BenchmarkResult {
    let results = [(); BENCHMARK_REPEATS].map(move |_| {
        let duration_ggmath = run_repeated(f_ggmath);
        let duration_glam = run_repeated(f_glam);

        duration_glam.as_secs_f64() / duration_ggmath.as_secs_f64()
    });

    BenchmarkResult {
        ggmath_speed: results.into_iter().sum::<f64>() / BENCHMARK_REPEATS as f64,
    }
}
#[inline(never)]
fn run_repeated(f: fn()) -> Duration {
    let start = Instant::now();

    for _ in 0..REPEATS {
        f();
    }

    let end = Instant::now();

    end - start
}
