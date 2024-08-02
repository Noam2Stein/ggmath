#[macro_export(local_inner_macros)]
macro_rules! swizzle {
    { $output_vec:ident<$t:ty>, [$(($fn:ident, $($src:ident => $dst:ident $(; $len:literal)?), * $(,)?)), * $(,)?] } => {
        $(
            #[inline(always)]
            pub fn $fn(self) -> $output_vec<$t> {
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
macro_rules! set_swizzle {
    { $value_vec:ident<$t:ty>, [$(($fn:ident, $($src:ident => $dst:ident $(; $len:literal)?), * $(,)?)), * $(,)?] } => {
        $(
            #[inline(always)]
            pub fn $fn(&mut self, value: $value_vec<$t>) {
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
            pub fn $fn(mut self, value: $value_vec<$t>) -> Self {
                unsafe {
                    copy!($t, $(value.$src => self.$dst $(; $len)?), *)
                }
                self
            }
        )*
    };
}
#[macro_export(local_inner_macros)]
macro_rules! new_swizzle {
    { $self:ident, [$(($tuple:ty, $($src:tt => $dst:ident $(; $len:literal)?), * $(,)?)), * $(,)?] } => {
        $(
            impl<T: Element> From<$tuple> for $self<T> {
                #[inline(always)]
                fn from(value: $tuple) -> Self {
                    let mut output = unsafe { std::mem::MaybeUninit::<Self>::uninit().assume_init() };
                    unsafe {
                        copy!(T, $(value.$src => output.$dst $(; $len)?), *)
                    }
                    output
                }
            }
        )*
    };
}