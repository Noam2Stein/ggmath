#[macro_export(local_inner_macros)]
macro_rules! copy_elements {
    ($element_type:ident, [$($($src:ident).+ -> $($dst:ident).+ $(* $count:literal)?), * $(,)?]) => {
        {
            $(
                let src_ptr = &$($src).+ as *const $element_type $(as *const [$element_type; $count])?;
                let dst_ptr = &mut $($dst).+ as *mut $element_type $(as *mut [$element_type; $count])?;
                *dst_ptr = *src_ptr;
            )*
        }
    };
}
#[macro_export(local_inner_macros)]
macro_rules! copy_elements_init {
    ($type:ty, $element_type:ident, [$($($src:ident).+ -> $($dst:ident).+ $(* $count:literal)?), * $(,)?]) => {
        {
            let mut result = std::mem::MaybeUninit::<$type>::uninit().assume_init();
            
            copy_elements!($element_type, [$($($src).+ -> result.$($dst).+ $(* $count)?), *]);

            result
        }
    };
}