#[macro_export(local_inner_macros)]
macro_rules! swizzle {
    { $output_vec:ident<$t:ty>, [$(($fn:ident, $($src:ident => $dst:ident $(; $len:literal)?), * $(,)?)), * $(,)?] } => {
        $(
            #[inline(always)]
            pub const fn $fn(self) -> $output_vec<$t> {
                let mut output = unsafe { std::mem::MaybeUninit::<$output_vec<$t>>::uninit().assume_init() };
                unsafe {
                    copy!($t, $(self.$src => output.$dst $(; $len)?), *)
                }
                output
            }
        )*
    };
}
#[macro_export(local_inner_macros)]
macro_rules! swizzle_mut {
    { $output_vec:ident<$t:ty>, [$(($fn:ident, $component:ident $(,)?)), * $(,)?] } => {
        $(
            #[inline(always)]
            pub const fn $fn<'a>(&'a mut self) -> &'a mut $output_vec<$t> {
                unsafe {
                    &mut *(&mut self.$component as *mut _ as *mut $output_vec<$t>)
                }
            }
        )*
    };
}
#[macro_export(local_inner_macros)]
macro_rules! set_swizzle {
    { $value_vec:ident<$t:ty>, [$(($fn:ident, $($src:ident => $dst:ident $(; $len:literal)?), * $(,)?)), * $(,)?] } => {
        $(
            #[inline(always)]
            pub const fn $fn(&mut self, value: $value_vec<$t>) {
                unsafe {
                    copy!($t, $(value.$src => self.$dst $(; $len)?), *)
                }
            }
        )*
    };
}
#[macro_export(local_inner_macros)]
macro_rules! with_swizzle {
    { $value_vec:ident<$t:ty>, [$(($fn:ident, $($src:ident => $dst:ident $(; $len:literal)?), * $(,)?)), * $(,)?] } => {
        $(
            #[inline(always)]
            pub const fn $fn(mut self, value: $value_vec<$t>) -> Self {
                unsafe {
                    copy!($t, $(value.$src => self.$dst $(; $len)?), *)
                }
                self
            }
        )*
    };
}