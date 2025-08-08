use super::*;

/// A trait for scalar types that have a constant `0` value.
pub trait ScalarZero: Scalar {
    /// The `0` value of the type.
    const ZERO: Self;
}

/// A trait for scalar types that have a constant `1` value.
pub trait ScalarOne: Scalar {
    /// The `1` value of the type.
    const ONE: Self;
}

/// A trait for scalar types that have a constant `-1` value.
pub trait ScalarNegOne: Scalar {
    /// The `-1` value of the type.
    const NEG_ONE: Self;
}

impl<const N: usize, T: ScalarZero, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<const N: usize, T: ScalarOne, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<const N: usize, T: ScalarNegOne, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

repetitive! {
    @for axis_idx in 0..3 {
        @let axis = ['x, 'y, 'z][axis_idx];
        @let to_the = ["to the ", "", ""][axis_idx];

        @let right = ['right, 'up, 'forward][axis_idx];
        @let Right = ['Right, 'Up, 'Forward][axis_idx];
        @let RIGHT = ['RIGHT, 'UP, 'FORWARD][axis_idx];
        @let PositiveRightExt = @['Positive Right 'Ext];
        @let NegativeRightExt = @['Negative Right 'Ext];

        @let left = ['left, 'down, 'backward][axis_idx];
        @let Left = ['Left, 'Down, 'Backward][axis_idx];
        @let LEFT = ['LEFT, 'DOWN, 'BACKWARD][axis_idx];
        @let PositiveLeftExt = @['Positive Left 'Ext];
        @let NegativeLeftExt = @['Negative Left 'Ext];

        #[doc = @str["A trait for a `" RIGHT "` constant where `" RIGHT "` is the positive direction."]]
        #[cfg(feature = @str[right])]
        pub trait @PositiveRightExt {
            #[doc = @str["A value that points " to_the right " (`" axis " = 1`)."]]
            const @RIGHT: Self;
        }

        #[doc = @str["A trait for a `" LEFT "` constant where `" RIGHT "` is the positive direction."]]
        #[cfg(feature = @str[right])]
        pub trait @NegativeLeftExt {
            #[doc = @str["A value that points " to_the left " (`" axis " = -1`)."]]
            const @LEFT: Self;
        }

        #[doc = @str["A trait for a `" RIGHT "` constant where `" LEFT "` is the positive direction."]]
        #[cfg(feature = @str[left])]
        pub trait @NegativeRightExt {
            #[doc = @str["A value that points " to_the right " (`" axis " = -1`)."]]
            const @RIGHT: Self;
        }

        #[doc = @str["A trait for a `" LEFT "` constant where `" LEFT "` is the positive direction."]]
        #[cfg(feature = @str[left])]
        pub trait @PositiveLeftExt {
            #[doc = @str["A value that points " to_the left " (`" axis " = 1`)."]]
            const @LEFT: Self;
        }

        #[cfg(feature = @str[right])]
        impl<T: ScalarOne> @PositiveRightExt for T {
            const @RIGHT: Self = Self::ONE;
        }

        #[cfg(feature = @str[right])]
        impl<T: ScalarNegOne> @NegativeLeftExt for T {
            const @LEFT: Self = Self::NEG_ONE;
        }

        #[cfg(feature = @str[left])]
        impl<T: ScalarNegOne> @NegativeRightExt for T {
            const @RIGHT: Self = Self::NEG_ONE;
        }

        #[cfg(feature = @str[left])]
        impl<T: ScalarOne> @PositiveLeftExt for T {
            const @LEFT: Self = Self::ONE;
        }

        @for N in 2..=4 {
            @if axis_idx < N {
                #[cfg(feature = @str[right])]
                impl<T: ScalarZero + ScalarOne, A: VecAlignment> @PositiveRightExt for Vector<@N, T, A> {
                    const @RIGHT: Self = Self::ZERO.@['with_ axis](T::@RIGHT);
                }
            }
        }
    }
}
