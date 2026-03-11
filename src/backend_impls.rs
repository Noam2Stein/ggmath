mod default_impls;

#[cfg(target_feature = "sse2")]
mod sse2;

#[cfg(not(target_feature = "sse2"))]
mod fallback;

/*
When `portable_simd` is stabilized, target-architecture specific implementations
should be deleted.
*/
