#[macro_export]
macro_rules! quat {
    ($($tt:tt)*) => {
        $crate::Quat::from_vec($crate::vec4!($($tt)*))
    };
}

#[macro_export]
macro_rules! quatp {
    ($($tt:tt)*) => {
        $crate::QuatP::from_vec($crate::vec4p!($($tt)*))
    };
}

#[macro_export]
macro_rules! quatg {
    ($($tt:tt)*) => {
        $crate::Quaternion::from_vec($crate::vec4g!($($tt)*))
    };
}
