/// Tries to cast the arguments to the provided types, and returns the provided output if the types match.
/// This is used to specialize vector functions based on vector length or vector alignment.
///
/// Syntax:
/// `
/// vector_optimization! {
///     (<arg_name>: <arg_type_1> => <arg_type_2>, ...) -> <ret_type_1> => <ret_type_2> {
///         <closure>
///     }
/// }
/// `
///
/// ## Example
///
/// ```
/// use ggmath::*;
///
/// #[inline(always)]
/// fn vec_add<const N: usize, A: VecAlignment>(
///     vec: Vector<N, f32, A>,
///     other: Vector<N, f32, impl VecAlignment>
/// ) -> Vector<N, f32, A>
/// where
///     Usize<N>: VecLen,
/// {
///     // Use a more performant addition using SIMD if the vector length is 2 and the alignment is VecAligned
///     #[cfg(target_feature = "sse")]
///     vector_optimization! {
///         (
///             vec: Vector<N, f32, A> => Vector<2, f32, VecAligned>,
///             other: Vector<N, f32, _> => Vector<2, f32, VecAligned>
///         ) -> Vector<2, f32, VecAligned> => Vector<N, f32, A> {
///             |vec, other| Vector(vec.0 + other.0)
///         }
///     }
///
///     Vector::from_fn(|i| vec[i] + other[i])
/// }
/// ```
///
/// ## Safety
///
/// This macro is always safe to use.
/// Using an incorrect output type panics and is not undefined behavior.
#[macro_export]
macro_rules! vector_optimization {
    (
        ($($arg:ident: $arg_type_1:ty => $arg_type_2:ty),* $(,)?) {
            $closure:expr
        }
    ) => {
        $crate::vector_optimization! {
            ($($arg: $arg_type_1 => $arg_type_2),*) -> () => () {
                $closure
            }
        }
    };

    (
        ($($arg:ident: $arg_type_1:ty => $arg_type_2:ty),* $(,)?) -> $ret_type_1:ty => $ret_type_2:ty {
            $closure:expr
        }
    ) => {{
        #[inline(always)]
        fn _typecheck<T1: $crate::Construct, T2: $crate::Construct>(value: T1) -> Option<T2> {
            if core::any::TypeId::of::<T1>() == core::any::TypeId::of::<T2>() {
                let ptr = &value as *const T1 as *const T2;
                Some(unsafe { ptr.read() })
            } else {
                None
            }
        }

        if $(let Some($arg) = _typecheck::<$arg_type_1, $arg_type_2>($arg))&&* {
            let closure: fn($($arg_type_2),*) -> $ret_type_1 = $closure;

            return _typecheck::<$ret_type_1, $ret_type_2>(closure($($arg),*)).expect(
                format!(
                    "wrong output type for vector optimization: {} -> {}",
                    core::any::type_name::<$ret_type_1>(),
                    core::any::type_name::<$ret_type_2>()
                )
                .as_str()
            );
        }
    }};
}

#[cfg(test)]
mod tests {
    use core::ops::Add;

    use crate::Construct;

    #[test]
    fn test_vector_optimization() {
        assert_eq!(helper::<f32>(1.0, 2.0), 1.0 - 2.0);
        assert_eq!(helper::<i32>(1, 2), 1 + 2);
    }

    #[test]
    #[should_panic]
    fn test_vector_optimization_panic() {
        helper::<u8>(9, 10);
    }

    fn helper<T: Add<Output: Construct> + Construct>(val: T, rhs: T) -> T::Output {
        vector_optimization! {
            (val: T => f32, rhs: T => f32) -> f32 => T::Output {
                |val, rhs| val - rhs
            }
        }

        // This optimization has a wrong output type, so it should be cancelled
        vector_optimization! {
            (val: T => u8, rhs: T => u8) -> i8 => T::Output {
                |val, rhs| (val * rhs) as i8
            }
        }

        val + rhs
    }
}
