declare_dir!(for N = 2, 3, 4: right, Right, RIGHT <=> left, Left, LEFT => x, X);
declare_dir!(for N = 2, 3, 4: left, Left, LEFT <=> right, Right, RIGHT => x, X);
declare_dir!(for N = 2, 3, 4: up, Up, UP <=> down, Down, DOWN => y, Y);
declare_dir!(for N = 2, 3, 4: down, Down, DOWN <=> up, Up, UP => y, Y);
declare_dir!(for N = 3, 4: forwards, Forwards, FORWARDS <=> backwards, Backwards, BACKWARDS => z, Z);
declare_dir!(for N = 3, 4: backwards, Backwards, BACKWARDS <=> forwards, Forwards, FORWARDS => z, Z);

macro_rules! declare_dir {
    (for N = $($N:literal),*: $dir:ident, $Dir:ident, $DIR:ident <=> $neg_dir:ident, $NegDir:ident, $NEG_DIR:ident => $axis:ident, $AXIS:ident) => {
        $crate::hidden::paste! {
            #[cfg(feature = "" $dir "")]
            #[doc = "`" $DIR "` and `" $NEG_DIR "` constants where `" $DIR "` is positive and `" $NEG_DIR "` is negative."]
            pub mod $dir {
                use crate::{ScalarOne, ScalarNegOne, Vector, ScalarZero, Simdness};

                #[doc = "`" $DIR "` constant where `" $DIR "` is positive and `" $NEG_DIR "` is negative."]
                pub trait [<Positive $Dir Ext>] {
                    #[doc = "`" $DIR "` constant where `" $DIR "` is positive and `" $NEG_DIR "` is negative."]
                    const $DIR: Self;
                }

                #[doc = "`" $NEG_DIR "` constant where `" $DIR "` is positive and `" $NEG_DIR "` is negative."]
                pub trait [<Negative $NegDir Ext>] {
                    #[doc = "`" $NEG_DIR "` constant where `" $DIR "` is positive and `" $NEG_DIR "` is negative."]
                    const $NEG_DIR: Self;
                }

                impl<T: ScalarOne> [<Positive $Dir Ext>] for T {
                    const $DIR: Self = Self::ONE;
                }

                impl<T: ScalarNegOne> [<Negative $NegDir Ext>] for T {
                    const $NEG_DIR: Self = Self::NEG_ONE;
                }

                $(
                    impl<T: ScalarZero + ScalarOne, S: Simdness> [<Positive $Dir Ext>] for Vector<$N, T, S> {
                        const $DIR: Self = Self::$AXIS;
                    }

                    impl<T: ScalarZero + ScalarNegOne, S: Simdness> [<Negative $NegDir Ext>] for Vector<$N, T, S> {
                        const $NEG_DIR: Self = Self::[<NEG_ $AXIS>];
                    }
                )*
            }
        }
    };
}

use declare_dir;
