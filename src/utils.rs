#[cfg(any(
    target_feature = "sse2",
    all(target_arch = "aarch64", target_feature = "neon")
))]
pub(crate) use crate::utils::safe_target_feature::*;
#[cfg(feature = "wide")]
pub(crate) use crate::utils::wide_ty::*;
pub(crate) use crate::utils::{math::*, primitive_traits::*, repr::*, specialize::*, transmute::*};

mod math;
mod primitive_traits;
mod repr;
#[cfg(any(
    target_feature = "sse2",
    all(target_arch = "aarch64", target_feature = "neon")
))]
mod safe_target_feature;
mod specialize;
mod transmute;
#[cfg(feature = "wide")]
mod wide_ty;
