use crate::{testable_scalar::*, *};

pub fn test_scalar<T: TestableScalar>() {
    vector::test_scalar::<T>()
}
