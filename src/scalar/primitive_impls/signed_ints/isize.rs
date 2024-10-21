use crate as ggmath;

use crate::scalar::default_impl::scalar_default_impl;

#[cfg(target_pointer_width = "32")]
scalar_default_impl!(isize(4));

#[cfg(target_pointer_width = "64")]
scalar_default_impl!(isize(8));
