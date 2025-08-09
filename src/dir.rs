use super::*;

/// A trait for the `ZERO` constant.
pub trait Zero {
    /// The `0` value of the type.
    const ZERO: Self;
}

/// A trait for the `ONE` constant.
pub trait One {
    /// The `1` value of the type.
    const ONE: Self;
}

/// A trait for the `NEG_ONE` constant.
pub trait NegOne {
    /// The `-1` value of the type.
    const NEG_ONE: Self;
}

impl<const N: usize, T: Scalar + Zero, A: VecAlignment> Zero for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    const ZERO: Self = Self::splat(T::ZERO);
}

impl<const N: usize, T: Scalar + One, A: VecAlignment> One for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    const ONE: Self = Self::splat(T::ONE);
}

impl<const N: usize, T: Scalar + NegOne, A: VecAlignment> NegOne for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

repetitive! {
    @for [prim, prim_is_signed] in [
        ['u8, false],
        ['u16, false],
        ['u32, false],
        ['u64, false],
        ['u128, false],
        ['usize, false],
        ['i8, true],
        ['i16, true],
        ['i32, true],
        ['i64, true],
        ['i128, true],
        ['isize, true],
        ['f32, true],
        ['f64, true],
    ] {
        impl Zero for @prim {
            const ZERO: Self = 0 as @prim;
        }

        impl One for @prim {
            const ONE: Self = 1 as @prim;
        }

        @if prim_is_signed {
            impl NegOne for @prim {
                const NEG_ONE: Self = -1 as @prim;
            }
        }
    }
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
        impl<T: Scalar + One> @PositiveRightExt for T {
            const @RIGHT: Self = Self::ONE;
        }

        #[cfg(feature = @str[right])]
        impl<T: Scalar + NegOne> @NegativeLeftExt for T {
            const @LEFT: Self = Self::NEG_ONE;
        }

        #[cfg(feature = @str[left])]
        impl<T: Scalar + NegOne> @NegativeRightExt for T {
            const @RIGHT: Self = Self::NEG_ONE;
        }

        #[cfg(feature = @str[left])]
        impl<T: Scalar + One> @PositiveLeftExt for T {
            const @LEFT: Self = Self::ONE;
        }

        @for N in 2..=4 {
            @if axis_idx < N {
                #[cfg(feature = @str[right])]
                impl<T: Scalar + Zero + One, A: VecAlignment> @PositiveRightExt for Vector<@N, T, A> {
                    const @RIGHT: Self = Self::ZERO.@['with_ axis](T::@RIGHT);
                }

                #[cfg(feature = @str[right])]
                impl<T: Scalar + Zero + NegOne, A: VecAlignment> @NegativeLeftExt for Vector<@N, T, A> {
                    const @LEFT: Self = Self::ZERO.@['with_ axis](T::@LEFT);
                }

                #[cfg(feature = @str[left])]
                impl<T: Scalar + Zero + NegOne, A: VecAlignment> @NegativeRightExt for Vector<@N, T, A> {
                    const @RIGHT: Self = Self::ZERO.@['with_ axis](T::@RIGHT);
                }

                #[cfg(feature = @str[left])]
                impl<T: Scalar + Zero + One, A: VecAlignment> @PositiveLeftExt for Vector<@N, T, A> {
                    const @LEFT: Self = Self::ZERO.@['with_ axis](T::@LEFT);
                }
            }
        }
    }
}
