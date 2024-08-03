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
        impl$(<$($generic)*>)? From<$b> for $a {
            fn from(value: $b) -> Self {
                unsafe {
                    *(&value as *const _ as *const $a)
                }
            }
        }
        impl$(<$($generic)*>)? AsRef<$a> for $b {
            fn as_ref(&self) -> &$a {
                unsafe {
                    &*(self as *const _ as *const $a)
                }
            }
        }
        impl$(<$($generic)*>)? AsRef<$b> for $a {
            fn as_ref(&self) -> &$b {
                unsafe {
                    &*(self as *const _ as *const $b)
                }
            }
        }
        impl$(<$($generic)*>)? AsMut<$a> for $b {
            fn as_mut(&mut self) -> &mut $a {
                unsafe {
                    &mut *(self as *mut _ as *mut $a)
                }
            }
        }
        impl$(<$($generic)*>)? AsMut<$b> for $a {
            fn as_mut(&mut self) -> &mut $b {
                unsafe {
                    &mut *(self as *mut _ as *mut $b)
                }
            }
        }
    };
}