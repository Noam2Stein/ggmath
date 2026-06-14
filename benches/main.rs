mod bench;
mod f32;

/// Controls the [`divan`] parameter [`sample_count`] for all benchmarks.
///
/// [`sample_count`]: https://docs.rs/divan/latest/divan/attr.bench.html#sample_count
const SAMPLE_COUNT: u32 = 100;

/// Controls the [`divan`] parameter [`sample_size`] for all benchmarks.
///
/// [`sample_size`]: https://docs.rs/divan/latest/divan/attr.bench.html#sample_size
const SAMPLE_SIZE: u32 = 100;

/// The number of values micro-benchmarks use.
///
/// For small values, the bottleneck is computation throughput. For large enough
/// values, the bottleneck is memory.
///
/// As this number grows, the bottleneck of benchmarks moves from computation
/// throughput to memory bandwidth. You can tune this number to find out when
/// SIMD-alignment stops improving performance.
const MICROBENCH_ARRAY_LEN: usize = 20_000;

/// The number of vectors used for benchmarks that apply the same transformation
/// to multiple vectors in a batched manner.
const TRANSFORM_BATCH: usize = 20;

fn main() {
    divan::main();
}
