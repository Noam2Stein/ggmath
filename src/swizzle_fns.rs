#[macro_export(local_inner_macros)]
macro_rules! swizzle_fns {
    ($dst_type:ty, $element_type:ident, [$(($fn:ident, [$($src:ident -> $dst:ident $(* $count:literal)?), * $(,)?])), * $(,)?]) => {
        $(
            #[inline(always)] pub fn $fn(self) -> $dst_type {
                unsafe {
                    copy_elements_init!($dst_type, $element_type, [$(self.$src -> $dst $(* $count)?), *])
                }
            }
        )*
    };
}
#[macro_export(local_inner_macros)]
macro_rules! set_swizzle_fns {
    ($value_type:ty, $element_type:ident, [$(($fn:ident, [$($src:ident -> $dst:ident $(* $count:literal)?), * $(,)?])), * $(,)?]) => {
        $(
            #[inline(always)] pub fn $fn(&mut self, value: $value_type) {
                unsafe {
                    copy_elements!($element_type, [$(value.$src -> self.$dst $(* $count)?), *])
                }
            }
        )*
    };
}
#[macro_export(local_inner_macros)]
macro_rules! with_swizzle_fns {
    ($value_type:ty, $element_type:ident, [$(($fn:ident, [$($src:ident -> $dst:ident $(* $count:literal)?), * $(,)?])), * $(,)?]) => {
        $(
            #[inline(always)] pub fn $fn(mut self, value: $value_type) -> Self {
                unsafe {
                    copy_elements!($element_type, [$(value.$src -> self.$dst $(* $count)?), *])
                }

                self
            }
        )*
    };
}