#[macro_export(local_inner_macros)]
macro_rules! swizzle {
    ($src:ident, $return_type:ident, $component_type:ident, [$($field:ident -> $field_dst:ident * $len:literal), *]) => {
        {
            let mut result = std::mem::MaybeUninit::<$return_type<$component_type>>::uninit().assume_init();
            $(
                *(&mut result.$field_dst as *mut $component_type as *mut [$component_type; $len]) = *(&$src.$field as *const $component_type as *const [$component_type; $len]);
            )*

            result
        }
    };
}