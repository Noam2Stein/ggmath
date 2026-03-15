use core::mem::{ManuallyDrop, transmute, transmute_copy};

/// Variant of [`transmute`] that checks memory-layout compatibility at runtime
/// instead of at compile time.
///
/// This makes it possible to transmute between generic types and dependently
/// sized types.
///
/// # Safety
///
/// Exactly like [`transmute`].
///
/// [`transmute`]: core::mem::transmute
#[inline]
#[track_caller]
pub const unsafe fn transmute_generic<T, U>(value: T) -> U {
    assert!(size_of::<T>() == size_of::<U>());

    // SAFETY: The caller must ensure that the bits of the input are valid for
    // the output type.
    unsafe { transmute_copy::<ManuallyDrop<T>, U>(&ManuallyDrop::new(value)) }
}

/// Transmutes a reference to a reference of a different type.
///
/// # Safety
///
/// Exactly like [`transmute`] for references, except that here immediate size
/// and alignment validity is automatically checked, and the lifetime of the
/// reference cannot be accidently changed.
///
/// [`transmute`]: core::mem::transmute
#[inline]
#[track_caller]
pub const unsafe fn transmute_ref<T, U>(value: &T) -> &U {
    assert!(size_of::<T>() >= size_of::<U>());
    assert!(align_of::<T>() >= align_of::<U>());

    // SAFETY: The output size was just checked to be no more than the input
    // size, the output alignment was just checked to be no more than the input
    // alignment, the lifetimes match because of the function signature, and the
    // caller must ensure that the bits of the input value are valid for the
    // output type.
    unsafe { transmute::<&T, &U>(value) }
}

/// Transmutes a mutable reference to a mutable reference of a different type.
///
/// # Safety
///
/// Exactly like [`transmute`] for mutable references, except that here size and
/// alignment validity is automatically checked, and the lifetime cannot be
/// accidently changed.
///
/// [`transmute`]: core::mem::transmute
#[inline]
#[track_caller]
pub const unsafe fn transmute_mut<T, U>(value: &mut T) -> &mut U {
    assert!(size_of::<T>() >= size_of::<U>());
    assert!(align_of::<T>() >= align_of::<U>());

    // SAFETY: The output size was just checked to be no more than the input
    // size, the output alignment was just checked to be no more than the input
    // alignment, the lifetimes match because of the function signature, the
    // input is mutably borrowed until the output reference is dropped, and the
    // caller must ensure that the bits of the input value are valid for the
    // output type.
    unsafe { transmute::<&mut T, &mut U>(value) }
}

#[cfg(test)]
mod tests {
    use crate::transmute::{transmute_generic, transmute_mut, transmute_ref};

    #[test]
    fn test_transmute_generic() {
        assert_eq!(unsafe { transmute_generic::<i32, u32>(1984) }, 1984);
    }

    #[test]
    fn test_transmute_ref() {
        assert_eq!(unsafe { transmute_ref::<i32, u32>(&1984) }, &1984);
    }

    #[test]
    fn test_transmute_mut() {
        let mut value = 1984;
        assert_eq!(unsafe { transmute_mut::<i32, u32>(&mut value) }, &mut 1984);
    }
}
