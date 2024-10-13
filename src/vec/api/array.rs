use gomath_proc_macros::vec_api;

use super::*;

vec_api!(
    Array:

    fn from_array(array: [T; N]) -> Self;
    fn into_array(self) -> [T; N];

    fn as_array(&self) -> &[T; N];
    fn as_array_mut(&mut self) -> &mut [T; N];
);
