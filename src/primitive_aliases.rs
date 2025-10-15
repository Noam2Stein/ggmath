declare_primitive_aliases!(pub type F => f32);
declare_primitive_aliases!(pub type D => f64);
declare_primitive_aliases!(pub type I8 => i8);
declare_primitive_aliases!(pub type I16 => i16);
declare_primitive_aliases!(pub type I => i32);
declare_primitive_aliases!(pub type I64 => i64);
declare_primitive_aliases!(pub type I128 => i128);
declare_primitive_aliases!(pub type Isize => isize);
declare_primitive_aliases!(pub type U8 => u8);
declare_primitive_aliases!(pub type U16 => u16);
declare_primitive_aliases!(pub type U => u32);
declare_primitive_aliases!(pub type U64 => u64);
declare_primitive_aliases!(pub type U128 => u128);
declare_primitive_aliases!(pub type Usize => usize);
declare_primitive_aliases!(pub type B => bool);

macro_rules! declare_primitive_aliases {
    ($vis:vis type $prefix:ident => $T:ident) => {
        crate::hidden::paste! {
            #[doc = "Type aliases for `" $T "` math types."]
            pub mod $T {
                use crate::declare_vector_aliases;

                declare_vector_aliases!($vis type $prefix => $T);
            }
        }
    };
}

use declare_primitive_aliases;
