mod vector;

macro_rules! vec2 {
    ($($arg:expr),* $(,)?) => {
        ggmath::Vector::<2, T>::from(($($arg,)*))
    };
}

use vec2;

macro_rules! vec3 {
    ($($arg:expr),* $(,)?) => {
        ggmath::Vector::<3, T>::from(($($arg,)*))
    };
}

use vec3;

macro_rules! vec4 {
    ($($arg:expr),* $(,)?) => {
        ggmath::Vector::<4, T>::from(($($arg,)*))
    };
}

use vec4;
