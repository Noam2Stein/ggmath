use genco::quote;

use crate::module::TokensExt;

mod constants;
mod module;

mod float_ext;
mod primitive_aliases;
mod vector;

fn main() {
    quote! {
        #![deny(missing_docs)]
        #![doc = include_str!("../README.md")]

        use core::panic::{RefUnwindSafe, UnwindSafe};

        mod float_ext;
        pub use float_ext::*;

        #[cfg(feature = "vector")]
        pub mod vector;
        #[cfg(feature = "vector")]
        pub use vector::*;
        
        #[cfg(feature = "primitive_aliases")]
        mod primitive_aliases;
        #[cfg(feature = "primitive_aliases")]
        pub use primitive_aliases::*;

        $("/// The base trait for all `ggmath` types.")
        $("/// This is an auto trait that requires basic traits,")
        $("/// and most importantly `Copy + 'static`.")
        $("///")
        $("/// This does mean that `ggmath` does not support references or heap-based types as scalars.")
        pub trait Construct: Sized + Send + Sync + 'static + Copy + UnwindSafe + RefUnwindSafe {}

        $("/// Marker type that is generic over a `usize` constant,")
        $("/// which is used to implement traits for specific `usize` values.")
        pub struct Usize<const N: usize>;

        $("/// An error type for when an index is out of bounds.")
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct IndexOutOfBoundsError;

        $("/// A macro for type specialization.")
        $("/// Takes a generic signature and a set of cases,")
        $("/// and picks the appropriate case based on the types of the arguments.")
        $("///")
        $("/// # Example")
        $("///")
        $("/// ```")
        $("/// use ggmath::{specialize, Construct};")
        $("///")
        $("/// fn mul_by_int_size<T: Construct>(value: T) -> T {")
        $("///     specialize! {")
        $("///         (value: T) -> T:")
        $("///")
        $("///         for (i32) -> i32 { |value| value * 32 }")
        $("///         for (i64) -> i64 { |value| value * 64 }")
        $("///         else {")
        $("///             // If the type is not an integer, return the original value")
        $("///             value")
        $("///         }")
        $("///     }")
        $("/// }")
        $("/// ```")
        #[macro_export]
        macro_rules! specialize {
            (
                ($$($$arg:tt: $$arg_type:ty),* $$(,)?):

                for ($$($$first_case_arg_type:ty),* $$(,)?) { $$first_case_closure:expr }

                $$(for ($$($$case_arg_type:ty),* $$(,)?) { $$case_closure:expr })*

                $$(else { $$($$else_tt:tt)* })?
            ) => {
                $$crate::specialize! {
                    ($$($$arg: $$arg_type),*) -> ():

                    for ($$($$first_case_arg_type),*) -> () { $$first_case_closure }

                    $$(for ($$($$case_arg_type),*) -> () { $$case_closure })*

                    $$(else { $$($$else_tt:tt)* })?
                }
            };

            (
                ($$($$arg:tt: $$arg_type:ty),* $$(,)?) -> $$ret_type:ty:

                for ($$($$first_case_arg_type:ty),* $$(,)?) -> $$first_case_ret_type:ty { $$first_case_closure:expr }

                $$(for ($$($$case_arg_type:ty),* $$(,)?) -> $$case_ret_type:ty { $$case_closure:expr })*

                $$(else { $$($$else_tt:tt)* })?
            ) => {
                $$crate::specialize! {
                    @with_tuple

                    (($$($$arg,)*): ($$($$arg_type,)*)) -> $$ret_type:

                    for ($$($$first_case_arg_type),*) -> $$first_case_ret_type { $$first_case_closure }

                    $$(for ($$($$case_arg_type),*) -> $$case_ret_type { $$case_closure })*

                    $$(else { $$($$else_tt)* })?
                }
            };

            (
                @with_tuple

                ($$args:tt: $$args_type:ty) -> $$ret_type:ty:

                for ($$($$first_case_arg_type:ty),* $$(,)?) -> $$first_case_ret_type:ty { $$first_case_closure:expr }

                $$(for ($$($$case_arg_type:ty),* $$(,)?) -> $$case_ret_type:ty { $$case_closure:expr })*

                $$(else { $$($$else_tt:tt)* })?
            ) => {
                if let Some(tuple) = $$crate::typecheck::<$$args_type, ($$($$first_case_arg_type,)*)>($$args)
                    && core::any::TypeId::of::<$$ret_type>() == core::any::TypeId::of::<$$first_case_ret_type>()
                {
                    let closure: fn($$($$first_case_arg_type),*) -> $$first_case_ret_type = $$first_case_closure;
                    let result: $$first_case_ret_type = $$crate::specialize!(@expand_tuple_into_closure closure(tuple); $$($$first_case_arg_type),*);

                    $$crate::typecheck::<$$first_case_ret_type, $$ret_type>(result).unwrap()
                }
                $$(else if let Some(tuple) = $$crate::typecheck::<$$args_type, ($$($$case_arg_type,)*)>($$args)
                    && core::any::TypeId::of::<$$ret_type>() == core::any::TypeId::of::<$$case_ret_type>()
                {
                    let closure: fn($$($$case_arg_type),*) -> $$case_ret_type = $$case_closure;
                    let result: $$case_ret_type = $$crate::specialize!(@expand_tuple_into_closure closure(tuple); $$($$case_arg_type),*);

                    $$crate::typecheck::<$$case_ret_type, $$ret_type>(result).unwrap()
                })*
                $$(else {
                    $$($$else_tt)*
                })?
            };

            $(
                for n in 0..8 =>

                (@expand_tuple_into_closure $$closure:ident($$tuple:ident); $(for i in 0..n join(, ) => $$_type$(i):ty)) => {
                    $$closure($(for i in 0..n join(, )=> $$tuple.$i))
                };$['\n']
            )
        }
        
        impl<T: Sized + Send + Sync + 'static + Copy + UnwindSafe + RefUnwindSafe> Construct for T {}

        impl core::error::Error for IndexOutOfBoundsError {}

        impl core::fmt::Display for IndexOutOfBoundsError {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "index out of bounds")
            }
        }

        #[doc(hidden)]
        pub mod _hidden_ {
            pub use paste::paste;
        }
        
        #[doc(hidden)]
        #[inline(always)]
        pub fn typecheck<T1: Construct, T2: Construct>(value: T1) -> Option<T2> {
            use core::any::TypeId;
                        
            if TypeId::of::<T1>() == TypeId::of::<T2>() {
                let ptr = &value as *const T1 as *const T2;
                Some(unsafe { ptr.read() })
            } else {
                None
            }
        }

        use sealed::Sealed;

        mod sealed {
            pub trait Sealed {}
        }
    }
    .to_src_dir("")
    .with_submod_dir(vector::src_mod())
    .with_submod_dir(primitive_aliases::src_mod())
    .with_submod_file(float_ext::src_mod())
    .write_as_root();

    quote! {
        mod vector;
    }
    .to_test_dir("")
    .with_submod_dir(vector::test_mod())
    .write_as_root();
}

fn join_and(iter: impl Iterator<Item = String>) -> String {
    let mut vec = iter.collect::<Vec<_>>();
    let last = vec.pop();

    if let Some(last) = last {
        if vec.is_empty() {
            last
        } else {
            format!("{} and {last}", vec.join(", "))
        }
    } else {
        String::new()
    }
}

fn join_or(iter: impl Iterator<Item = String>) -> String {
    let mut vec = iter.collect::<Vec<_>>();
    let last = vec.pop();

    if let Some(last) = last {
        if vec.is_empty() {
            last
        } else {
            format!("{} or {last}", vec.join(", "))
        }
    } else {
        String::new()
    }
}
