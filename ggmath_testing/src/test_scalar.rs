use std::mem::MaybeUninit;

use ggmath::{scalar::Scalar, vector::*};

use crate::{vector_test_assert, TestableScalar, TestingError};

pub fn test_scalar<T: TestableScalar>() -> Result<(), TestingError> {
    test_scalar_n_a::<2, T, VecAligned>()?;
    test_scalar_n_a::<2, T, VecPacked>()?;
    test_scalar_n_a::<3, T, VecAligned>()?;
    test_scalar_n_a::<3, T, VecPacked>()?;
    test_scalar_n_a::<4, T, VecAligned>()?;
    test_scalar_n_a::<4, T, VecPacked>()?;

    Ok(())
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
        vector_test_assert!(get: vector.get(i), vector.as_array().get(i).map(|some| *some), vector, i);

        for i_1 in 0..=4 {
            vector_test_assert!(get_1_1: vector.get_1_1([i, i_1]), get_n::<N, 2, _, _>(vector, [i, i_1]), vector, i, i_1);

            for i_2 in 0..=4 {
                vector_test_assert!(get_1_1_1: vector.get_1_1_1([i, i_1, i_2]), get_n::<N, 3, _, _>(vector, [i, i_1, i_2]), vector, i, i_1, i_2);

                for i_3 in 0..=4 {
                    vector_test_assert!(get_1_1_1_1: vector.get_1_1_1_1([i, i_1, i_2, i_3]), get_n::<N, 4, _, _>(vector, [i, i_1, i_2, i_3]), vector, i, i_1, i_2, i_3);
                }
            }
        }
    }

    for i in 0..N {
        vector_test_assert!(get_unchecked: unsafe { vector.get_unchecked(i) }, vector[i], vector, i);

        for i_1 in 0..N {
            vector_test_assert!(get_1_1_unchecked: unsafe { vector.get_1_1_unchecked([i, i_1]) }, vector.get_1_1([i, i_1]).unwrap(), vector, i, i_1);

            for i_2 in 0..N {
                vector_test_assert!(get_1_1_1_unchecked: unsafe { vector.get_1_1_1_unchecked([i, i_1, i_2]) }, vector.get_1_1_1([i, i_1, i_2]).unwrap(), vector, i, i_1, i_2);

                for i_3 in 0..N {
                    vector_test_assert!(get_1_1_1_1_unchecked: unsafe { vector.get_1_1_1_1_unchecked([i, i_1, i_2, i_3]) }, vector.get_1_1_1_1([i, i_1, i_2, i_3]).unwrap(), vector, i, i_1, i_2, i_3);
                }
            }
        }
    }

    Ok(())
}
