//! A benchmark suite for [`ggmath`].
//!
//! Using the [`bench!`] macro, benchmarks compare the performance of
//! aligned vs unaligned, [`ggmath`] vs [`glam`] and AoS vs SoA.
//!
//! Benchmarks are seperated into [`operations`] and [`workloads`].
//! Note that benchmarks from both categories are micro-benchmarks, which can
//! have misleading results.
//!
//! The constants below can be modified to benchmark different scenarios.

mod bench;
mod operations;
mod workloads;

/// Controls the [`divan`] parameter [`sample_count`] for all benchmarks.
///
/// [`sample_count`]: https://docs.rs/divan/latest/divan/attr.bench.html#sample_count
const SAMPLE_COUNT: u32 = 100;

/// Controls the [`divan`] parameter [`sample_size`] for all benchmarks.
///
/// [`sample_size`]: https://docs.rs/divan/latest/divan/attr.bench.html#sample_size
const SAMPLE_SIZE: u32 = 100;

/// Benchmarks operate on arrays of inputs and outputs. This contols the default
/// number of elements in those arrays.
///
/// As this number grows, the bottleneck of benchmarks moves from computation
/// throughput to memory bandwidth. You can tune this number to find out when
/// SIMD-alignment stops improving performance.
const ARRAY_LEN: usize = 20_000;

/// The number of values batched together in batch-operation benchmarks.
///
/// For example, multiplying multiple vectors by the same matrix.
const BATCH_LEN: usize = 20;

fn main() {
    divan::main();
}
