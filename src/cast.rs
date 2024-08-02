#[macro_export(local_inner_macros)]
macro_rules! cast {
    ($a:ty, $b:ty $(, $($generic:tt)*)?) => {
        impl$(<$($generic)*>)? From<$a> for $b {
            fn from(value: $a) -> Self {
                unsafe {
                    *(&value as *const _ as *const $b)
                }
            }
        }
        impl<'a$(, $($generic)*)?> From<&'a $a> for &'a $b {
            fn from(value: &'a $a) -> Self {
                unsafe {
                    &*(value as *const _ as *const $b)
                }
            }
        }
        impl<'a$(, $($generic)*)?> From<&'a mut $a> for &'a mut $b {
            fn from(value: &'a mut $a) -> Self {
                unsafe {
                    &mut *(value as *mut _ as *mut $b)
                }
            }
        }
        impl$(<$($generic)*>)? From<$b> for $a {
            fn from(value: $b) -> Self {
                unsafe {
                    *(&value as *const _ as *const $a)
                }
            }
        }
        impl<'a$(, $($generic)*)?> From<&'a $b> for &'a $a {
            fn from(value: &'a $b) -> Self {
                unsafe {
                    &*(value as *const _ as *const $a)
                }
            }
        }
        impl<'a$(, $($generic)*)?> From<&'a mut $b> for &'a mut $a {
            fn from(value: &'a mut $b) -> Self {
                unsafe {
                    &mut *(value as *mut _ as *mut $a)
                }
            }
        }
    };
}