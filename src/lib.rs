#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod vector;
pub use vector::*;

mod dir;
mod primitive_aliases;
pub use dir::*;
pub use primitive_aliases::*;

////////////////////////////////////////////////////////////////////////////////
// Utilities
////////////////////////////////////////////////////////////////////////////////

use core::{
    any::TypeId,
    mem::{ManuallyDrop, transmute_copy},
};

macro_rules! assertion {
    ($e:expr, $m:literal $(, $arg:expr)*$(,)?) => {
        #[cfg(any(feature = "assertions", all(feature = "debug_assertions", debug_assertions)))]
        assert!($e, $m $(, $arg)*);
    };
}

use assertion;

#[inline(always)]
fn cast<T: 'static, U: 'static>(value: T) -> U {
    assert_eq!(TypeId::of::<T>(), TypeId::of::<U>());

    unsafe { transmute::<T, U>(value) }
}

/// Transmutes a value from one type to another.
///
/// ## Safety
///
/// The caller must ensure that `value` is valid for `U`. However, the caller
/// does not need to ensure layout compatibility, as the function automatically
/// checks that `T` and `U` have matching layouts at compile time.
#[inline(always)]
const unsafe fn transmute<T, U>(value: T) -> U {
    const {}
    assert!(size_of::<T>() == size_of::<U>());
    assert!(align_of::<T>() >= align_of::<U>());

    unsafe { transmute_copy::<ManuallyDrop<T>, U>(&ManuallyDrop::new(value)) }
}

/// Transmutes a value from one type to another and copies it if necessary to
/// ensure alignment.
///
/// ## Safety
///
/// The caller must ensure that `value` is valid for `U`. However, the caller
/// does not need to ensure layout compatibility, as the function automatically
/// checks that `T` and `U` have matching layouts at compile time.
#[inline(always)]
const unsafe fn transmute_align<T, U>(value: T) -> U {
    const {}
    assert!(size_of::<T>() == size_of::<U>());

    unsafe { transmute_copy::<ManuallyDrop<T>, U>(&ManuallyDrop::new(value)) }
}

/// Transmutes a reference from one type to another.
///
/// ## Safety
///
/// The caller must ensure that `value` is valid for `U`. However, the caller
/// does not need to ensure layout compatibility, as the function automatically
/// checks that `T` and `U` have matching layouts at compile time.
#[inline(always)]
const unsafe fn transmute_ref<T, U>(value: &T) -> &U {
    const {}
    assert!(size_of::<T>() >= size_of::<U>());
    assert!(align_of::<T>() >= align_of::<U>());

    unsafe { transmute_copy::<&T, &U>(&value) }
}

/// Transmutes a mutable reference from one type to another.
///
/// ## Safety
///
/// The caller must ensure that `value` is valid for `U`. However, the caller
/// does not need to ensure layout compatibility, as the function automatically
/// checks that `T` and `U` have matching layouts at compile time.
#[inline(always)]
const unsafe fn transmute_mut<T, U>(value: &mut T) -> &mut U {
    const {}
    assert!(size_of::<T>() >= size_of::<U>());
    assert!(align_of::<T>() >= align_of::<U>());

    unsafe { transmute_copy::<&mut T, &mut U>(&value) }
}
