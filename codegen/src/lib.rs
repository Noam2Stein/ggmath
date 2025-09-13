use indoc::formatdoc;

mod constants;
mod r#gen;

mod float_ext;
mod primitive_aliases;
mod vector;

pub fn codegen() {
    r#gen::ModDir::new(
        "lib",
        formatdoc! {r#"
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

            /// An auto trait for types that can be sent anywhere anytime.
            ///
            /// This trait is required for all `ggmath` types,
            /// like scalars, vectors, matrices, etc.
            pub trait Construct: Sized + Send + Sync + 'static + Copy + UnwindSafe + RefUnwindSafe {{}}

            /// A simple marker type that is generic over a `usize` constant.
            ///
            /// This is used to implement traits for specific `usize` values.
            /// As is used in vectors with the [`VecLen`][vector::VecLen] trait.
            pub struct Usize<const N: usize>;

            /// An error type for when an index is out of bounds.
            pub struct IndexOutOfBoundsError;
            
            impl<T: Sized + Send + Sync + 'static + Copy + UnwindSafe + RefUnwindSafe> Construct for T {{}}

            impl core::error::Error for IndexOutOfBoundsError {{}}

            impl core::fmt::Display for IndexOutOfBoundsError {{
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                    write!(f, "Index out of bounds")
                }}
            }}

            #[doc(hidden)]
            pub mod _hidden_ {{
                #[cfg(feature = "aliases")]
                pub use paste;
            }}
            
            #[inline(always)]
            pub(crate) fn typecheck<T1: Construct, T2: Construct>(value: T1) -> Option<T2> {{
                use core::any::TypeId;
                            
                if TypeId::of::<T1>() == TypeId::of::<T2>() {{
                    let ptr = &value as *const T1 as *const T2;
                    Some(unsafe {{ ptr.read() }})
                }} else {{
                    None
                }}
            }}

            #[macro_export(local_inner_macros)]
            macro_rules! return_for_types {{
                (
                    ($($arg:ident: $arg_type_1:ty => $arg_type_2:ty),* $(,)?) {{
                        $closure:expr
                    }}
                ) => {{
                    $crate::return_for_types! {{
                        ($($arg: $arg_type_1 => $arg_type_2),*) -> () => () {{
                            $closure
                        }}
                    }}
                }};

                (
                    ($($arg:ident: $arg_type_1:ty => $arg_type_2:ty),* $(,)?) -> $ret_type_1:ty => $ret_type_2:ty {{
                        $closure:expr
                    }}
                ) => {{
                    if $(let Some($arg) = $crate::typecheck::<$arg_type_1, $arg_type_2>($arg))&&* {{
                        if core::any::TypeId::of::<$ret_type_1>() == core::any::TypeId::of::<$ret_type_2>() {{
                            let closure: fn($($arg_type_2),*) -> $ret_type_1 = $closure;

                            return $crate::typecheck::<$ret_type_1, $ret_type_2>(closure($($arg),*)).unwrap();
                        }}
                    }}
                }};
            }}
        "#},
        vec![float_ext::module()],
        vec![vector::module()],
        vec![],
    )
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

#[expect(unused)]
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
