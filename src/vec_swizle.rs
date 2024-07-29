#[macro_export(local_inner_macros)]
macro_rules! vec_swizzle {
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
macro_rules! vec_set_swizzle {
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
macro_rules! vec_with_swizzle {
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