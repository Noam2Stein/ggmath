#[cfg(feature = "bytemuck")]
mod bytemuck;

#[cfg(feature = "mint")]
mod mint;

#[cfg(feature = "serde")]
mod serde;

/*
Integration with `zerocopy` is blocked on:
https://github.com/rust-lang/rust/issues/95174

`zerocopy` requires using derive macros which `ggmath` doesn't support because
of the generic parameter `A: Alignment` which adds a breaking bound to the trait
implementation.

`adt_const_params` would make it possible to make `A` a const parameter that
doesn't add extra bounds in derives.
*/
