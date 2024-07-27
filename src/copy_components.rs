#[macro_export(local_inner_macros)]
macro_rules! copy_components {
    ($component_type:ident, [$($($src:ident).+ -> $($dst:ident).+ * $component_count:literal), *]) => {
        {
            $(
                let src_ptr = &$($src).+ as *const $component_type as *const [$component_type; $component_count];
                let dst_ptr = &mut $($dst).+ as *mut $component_type as *mut [$component_type; $component_count];
                *dst_ptr = *src_ptr;
            )*
        }
    };
}
#[macro_export(local_inner_macros)]
macro_rules! copy_components_init {
    ($type:ident, $component_type:ident, [$($($src:ident).+ -> $($dst:ident).+ * $component_count:literal), *]) => {
        {
            let mut result = std::mem::MaybeUninit::<$type<$component_type>>::uninit().assume_init();
            
            copy_components!($component_type, [$($($src).+ -> result.$($dst).+ * $component_count), *]);

            result
        }
    };
}