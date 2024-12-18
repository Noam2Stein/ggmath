#[allow(unused_imports)]
use std::{
    fmt::Debug,
    mem::forget,
    panic::{catch_unwind, set_hook, take_hook},
};
use std::{
    fmt::{self, Formatter},
    mem::MaybeUninit,
    panic::UnwindSafe,
};

use ggmath::{scalar::Scalar, vector::*};

use crate::{TestableScalar, TestingError};

pub fn test_scalar<T: TestableScalar>() -> Result<(), TestingError> {
    set_hook(Box::new(|_| {}));

    test_scalar_n_a::<2, T, VecAligned>()?;
    test_scalar_n_a::<2, T, VecPacked>()?;
    test_scalar_n_a::<3, T, VecAligned>()?;
    test_scalar_n_a::<3, T, VecPacked>()?;
    test_scalar_n_a::<4, T, VecAligned>()?;
    test_scalar_n_a::<4, T, VecPacked>()?;

    let _ = take_hook();

    Ok(())
}

enum MaybePanic<T: UnwindSafe> {
    Value(T),
    Panic,
}
impl<T: UnwindSafe> MaybePanic<T> {
    fn from_fn(f: impl FnOnce() -> T + UnwindSafe) -> Self {
        match catch_unwind(f) {
            Ok(ok) => Self::Value(ok),
            Err(_) => Self::Panic,
        }
    }
}
impl<T: UnwindSafe + Debug> Debug for MaybePanic<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{value:?}"),
            Self::Panic => write!(f, "panic"),
        }
    }
}
impl<T: UnwindSafe + PartialEq<Rhs>, Rhs: UnwindSafe> PartialEq<MaybePanic<Rhs>> for MaybePanic<T> {
    fn eq(&self, other: &MaybePanic<Rhs>) -> bool {
        match self {
            Self::Value(value) => match other {
                MaybePanic::Value(other) => value == other,
                MaybePanic::Panic => false,
            },
            Self::Panic => match other {
                MaybePanic::Value(_) => false,
                MaybePanic::Panic => true,
            },
        }
    }
}

fn assert_lhs_fn<
    const N: usize,
    T: TestableScalar,
    A: VecAlignment,
    V: PartialEq<E> + Debug,
    E: Debug,
>(
    vector_fn: &'static str,
    value: V,
    expected_value: E,
    inputs: &[String],
) -> Result<(), TestingError>
where
    ScalarCount<N>: VecLen,
{
    if value == expected_value {
        Ok(())
    } else {
        Err(TestingError::vector::<N, T, A>(
            vector_fn,
            &format!(
                "{}\nexpected `{expected_value:?}`\nfound `{value:?}`\n",
                inputs.join("")
            ),
        ))
    }
}
macro_rules! assert_lhs {
    ($vector_fn:ident: $value:expr, $expected_value:expr $(, $input:ident)* $(,)?) => {
        assert_lhs_fn::<N, T, A, _, _>(
            stringify!($vector_fn),
            MaybePanic::from_fn(|| $value),
            MaybePanic::from_fn(|| $expected_value),
            &[$(format!(" * {} = {:?} \n", stringify!($input), $input)), *],
        )?;
    };
}

fn get_n<const N: usize, const N_OUTPUT: usize, T: Scalar, A: VecAlignment>(
    vector: Vector<N, T, A>,
    indicies: [usize; N_OUTPUT],
) -> Option<Vector<N_OUTPUT, T, A>>
where
    ScalarCount<N>: VecLen,
    ScalarCount<N_OUTPUT>: VecLen,
{
    let mut output = unsafe { MaybeUninit::<[T; N_OUTPUT]>::uninit().assume_init() };

    for index in 0..N_OUTPUT {
        match vector.get(indicies[index]) {
            Some(item) => output[index] = item,
            None => return None,
        }
    }

    Some(Vector::from_array(output))
}

fn test_scalar_n_a<const N: usize, T: TestableScalar, A: VecAlignment>() -> Result<(), TestingError>
where
    ScalarCount<N>: VecLen,
{
    test_vector_get::<N, T, A>()?;

    Ok(())
}

fn test_vector_get<const N: usize, T: TestableScalar, A: VecAlignment>() -> Result<(), TestingError>
where
    ScalarCount<N>: VecLen,
{
    let vector = Vector::<N, T, A>::from_array(T::n_values(0));

    for i in 0..=4 {
        assert_lhs!(get: vector.get(i), vector.as_array().get(i).map(|some| *some), vector, i);

        for i_1 in 0..=4 {
            assert_lhs!(get_1_1: vector.get_1_1([i, i_1]), get_n::<N, 2, _, _>(vector, [i, i_1]), vector, i, i_1);

            for i_2 in 0..=4 {
                assert_lhs!(get_1_1_1: vector.get_1_1_1([i, i_1, i_2]), get_n::<N, 3, _, _>(vector, [i, i_1, i_2]), vector, i, i_1, i_2);

                for i_3 in 0..=4 {
                    assert_lhs!(get_1_1_1_1: vector.get_1_1_1_1([i, i_1, i_2, i_3]), get_n::<N, 4, _, _>(vector, [i, i_1, i_2, i_3]), vector, i, i_1, i_2, i_3);
                }
            }
        }
    }

    for i in 0..N {
        assert_lhs!(get_unchecked: unsafe { vector.get_unchecked(i) }, vector[i], vector, i);

        for i_1 in 0..N {
            assert_lhs!(get_1_1_unchecked: unsafe { vector.get_1_1_unchecked([i, i_1]) }, vector.get_1_1([i, i_1]).unwrap(), vector, i, i_1);

            for i_2 in 0..N {
                assert_lhs!(get_1_1_1_unchecked: unsafe { vector.get_1_1_1_unchecked([i, i_1, i_2]) }, vector.get_1_1_1([i, i_1, i_2]).unwrap(), vector, i, i_1, i_2);

                for i_3 in 0..N {
                    assert_lhs!(get_1_1_1_1_unchecked: unsafe { vector.get_1_1_1_1_unchecked([i, i_1, i_2, i_3]) }, vector.get_1_1_1_1([i, i_1, i_2, i_3]).unwrap(), vector, i, i_1, i_2, i_3);
                }
            }
        }
    }

    Ok(())
}
