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
#[macro_export(local_inner_macros)]
macro_rules! set_swizzle {
    ($src:ident, $dst:ident, $return_type:ident, $component_type:ident, [$($field:ident -> $field_dst:ident * $len:literal), *]) => {
        {
            $(
                *(&mut $dst.$field_dst as *mut $component_type as *mut [$component_type; $len]) = *(&$src.$field as *const $component_type as *const [$component_type; $len]);
            )*
        }
    };
}
#[macro_export(local_inner_macros)]
macro_rules! with_swizzle {
    ($self:ident, $src:ident, $return_type:ident, $component_type:ident, [$($field:ident -> $field_dst:ident * $len:literal), *]) => {
        {
            $(
                *(&mut $self.$field_dst as *mut $component_type as *mut [$component_type; $len]) = *(&$src.$field as *const $component_type as *const [$component_type; $len]);
            )*

            $self
        }
    };
}