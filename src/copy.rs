#[macro_export(local_inner_macros)]
macro_rules! copy {
    ($type:ty, $($src:expr => $dst:expr $(; $count:literal)?), * $(,)?) => {
        {
            $(
                let src = &$src as *const _ $(as *const [$type; $count])?;
                let dst = &mut $dst as *mut $type $(as *mut [$type; $count])?;
                *dst = *src;
            )*
        }
    };
}