declare_dir!(for N in [2, 3, 4] { "right", right, RightExt, NegLeftExt, RIGHT, LEFT, X, NEG_X });
declare_dir!(for N in [2, 3, 4] { "left", left, LeftExt, NegRightExt, LEFT, RIGHT, X, NEG_X });
declare_dir!(for N in [2, 3, 4] { "up", up, UpExt, NegDownExt, UP, DOWN, Y, NEG_Y });
declare_dir!(for N in [2, 3, 4] { "down", down, DownExt, NegUpExt, DOWN, UP, Y, NEG_Y });
declare_dir!(for N in [3, 4] { "forwards", forwards, ForwardsExt, NegBackwardsExt, FORWARDS, BACKWARDS, Z, NEG_Z });
declare_dir!(for N in [3, 4] { "backwards", backwards, BackwardsExt, NegForwardsExt, BACKWARDS, FORWARDS, Z, NEG_Z });

macro_rules! declare_dir {
    (for N in [$($N:literal),* $(,)?] { $dir_str:literal, $dir:ident, $DirExt:ident, $NegDir2Ext:ident, $DIR:ident, $DIR2:ident, $AXIS:ident, $NEG_AXIS:ident }) => {
        #[cfg(feature = $dir_str)]
        #[doc = concat!(
            "`",
            stringify!($DIR),
            "` and `",
            stringify!($DIR2),
            "` constants where `",
            stringify!($DIR),
            "` is positive and `",
            stringify!($DIR2),
            "` is negative."
        )]
        pub mod $dir {
            use crate::{Vector, ScalarNegOne, ScalarOne, ScalarZero};

            #[doc = concat!(
                "`",
                stringify!($DIR),
                "` constant where `",
                stringify!($DIR),
                "` is positive and `",
                stringify!($DIR2),
                "` is negative."
            )]
            pub trait $DirExt {
                #[doc = concat!(
                    "`",
                    stringify!($DIR),
                    "` constant where `",
                    stringify!($DIR),
                    "` is positive and `",
                    stringify!($DIR2),
                    "` is negative."
                )]
                const $DIR: Self;
            }

            #[doc = concat!(
                "`",
                stringify!($DIR2),
                "` constant where `",
                stringify!($DIR),
                "` is positive and `",
                stringify!($DIR2),
                "` is negative."
            )]
            pub trait $NegDir2Ext {
                #[doc = concat!(
                    "`",
                    stringify!($DIR2),
                    "` constant where `",
                    stringify!($DIR),
                    "` is positive and `",
                    stringify!($DIR2),
                    "` is negative."
                )]
                const $DIR2: Self;
            }

            impl<T: ScalarOne> $DirExt for T {
                const $DIR: Self = Self::ONE;
            }

            impl<T: ScalarNegOne> $NegDir2Ext for T {
                const $DIR2: Self = Self::NEG_ONE;
            }

            $(
                impl<T: ScalarZero + ScalarOne> $DirExt for Vector<$N, T> {
                    const $DIR: Self = Self::$AXIS;
                }

                impl<T: ScalarZero + ScalarNegOne> $NegDir2Ext for Vector<$N, T> {
                    const $DIR2: Self = Self::$NEG_AXIS;
                }
            )*
        }
    };
}

use declare_dir;
