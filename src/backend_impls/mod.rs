mod default_impls;

#[cfg(target_feature = "sse")]
mod sse;

#[cfg(not(target_feature = "sse"))]
mod fallback;
