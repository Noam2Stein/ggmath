use indoc::formatdoc;

use crate::constants::LENGTHS;

pub fn impl_api(scalar_override_fns: &mut Vec<String>) -> String {
    let storage_fns = storage_fns(scalar_override_fns).replace("\n", "\n\t");
    let array_fns = array_fns(scalar_override_fns).replace("\n", "\n\t");
    let packed_array_fns = packed_array_fns(scalar_override_fns).replace("\n", "\n\t");

    formatdoc! {r#"
        impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            {storage_fns}

            {array_fns}
        }}

        impl<const N: usize, T: Scalar> Vector<N, T, VecPacked>
        where
            Usize<N>: VecLen,
        {{
            {packed_array_fns}
        }}
    "#}
}

fn storage_fns(_scalar_fns: &mut Vec<String>) -> String {
    formatdoc! {r#"
        /// Returns true if the vector is aligned.
        /// The output is strictly determined by the type of the vector.
        #[inline(always)]
        pub const fn is_aligned(self) -> bool {{
            A::IS_ALIGNED
        }}

        /// Converts the vector to an aligned vector.
        #[inline(always)]
        pub fn align(self) -> Vector<N, T, VecAligned> {{
            self.to_storage()
        }}

        /// Converts the vector to a packed vector.
        #[inline(always)]
        pub fn pack(self) -> Vector<N, T, VecPacked> {{
            self.to_storage()
        }}

        /// Converts the vector to the specified alignment.
        #[inline(always)]
        pub fn to_storage<A2: VecAlignment>(self) -> Vector<N, T, A2> {{
            match A::IS_ALIGNED {{
                true => match A2::IS_ALIGNED {{
                    true => unsafe {{ transmute_copy::<Vector<N, T, A>, Vector<N, T, A2>>(&self) }},
                    false => Vector::from_array(self.to_array()),
                }},
                false => match A2::IS_ALIGNED {{
                    true => Vector::from_array(self.to_array()),
                    false => unsafe {{ transmute_copy::<Vector<N, T, A>, Vector<N, T, A2>>(&self) }},
                }},
            }}
        }}
    "#}
}

fn array_fns(scalar_fns: &mut Vec<String>) -> String {
    for n in LENGTHS {
        scalar_fns.push(formatdoc! {r#"
            /// Constructs an aligned vec{n} from an array.
            fn vec{n}_from_array(array: [Self; {n}]) -> Vec{n}<Self>;

            /// Converts an aligned vec{n} to an array.
            fn vec{n}_as_array(vec: Vec{n}<Self>) -> [Self; {n}];
        "#});
    }

    for n in LENGTHS {
        scalar_fns.push(formatdoc! {r#"
            /// Overridable implementation of `Vector::splat` for aligned vec{n}s.
            #[inline(always)]
            fn vec{n}_splat(value: Self) -> Vec{n}<Self> {{
                Vec{n}::from_array([value; {n}])
            }}
        "#});
    }

    let from_array_aligned_returns = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                return_for_types! {{
                    (array: [T; N] => [T; {n}]) -> Vector<{n}, T, VecAligned> => Vector<N, T, A> {{
                        |array| T::vec{n}_from_array(array)
                    }}
                }}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\n", "\n\t");

    let splat_aligned_returns = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                return_for_types! {{
                    (value: T => T) -> Vector<{n}, T, VecAligned> => Vector<N, T, A> {{
                        |value| T::vec{n}_splat(value)
                    }}
                }}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\n", "\n\t");

    let as_array_aligned_returns = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                return_for_types! {{
                    (self: Vector<N, T, A> => Vector<{n}, T, VecAligned>) -> [T; {n}] => [T; N] {{
                        |(vec,)| T::vec{n}_as_array(vec)
                    }}
                }}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\n", "\n\t");

    formatdoc! {r#"
        /// Creates a new vector from an array.
        #[inline(always)]
        pub fn from_array(array: [T; N]) -> Self {{
            {from_array_aligned_returns}

            return_for_types! {{
                (array: [T; N] => [T; N]) -> Vector<N, T, VecPacked> => Vector<N, T, A> {{
                    |array| Vector(array)
                }}
            }}

            unreachable!("unusual vector type")
        }}

        /// Creates a new vector where each component is the same value.
        #[inline(always)]
        pub fn splat(value: T) -> Self {{
            {splat_aligned_returns}
            
            Vector::from_array([value; N])
        }}

        /// Creates a new vector where each component is evaluated from the given function called with the component index.
        /// The function is called in order.
        #[inline(always)]
        pub fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self {{
            Vector::from_array(core::array::from_fn(f))
        }}

        /// Returns the number of components in the vector.
        #[inline(always)]
        pub const fn len(self) -> usize {{
            N
        }}

        /// Converts the vector to an array.
        #[inline(always)]
        pub fn as_array(self) -> [T; N] {{
            {as_array_aligned_returns}

            return_for_types! {{
                (self: Vector<N, T, A> => Vector<N, T, VecPacked>) -> [T; N] => [T; N] {{
                    |(vec,)| vec.0
                }}
            }}

            unreachable!("unusual vector type")
        }}

        /// Returns the component at the given index or panics if the index is out of bounds.
        #[inline(always)]
        pub fn index(self, index: usize) -> T {{
            self.as_array().index(index)
        }}

        /// Returns the component at the given index or returns None if the index is out of bounds.
        #[inline(always)]
        pub fn get(self, index: usize) -> Option<T> {{
            self.as_array().get(index)
        }}

        /// Returns the component at the given index with no bounds checking.
        /// 
        /// # Safety
        /// The caller must ensure that the index is in bounds.
        #[inline(always)]
        pub unsafe fn get_unchecked(self, index: usize) -> T {{
            unsafe {{ self.as_array().get_unchecked(index) }}
        }}

        /// Sets the component at the given index or panics if the index is out of bounds.
        #[inline(always)]
        pub fn set(&mut self, index: usize, value: T) {{
            let mut array = self.as_array();
            array[index] = value;
            *self = Self::from_array(array);
        }}

        /// Sets the component at the given index or returns an error if the index is out of bounds.
        #[inline(always)]
        pub fn try_set(&mut self, index: usize, value: T) -> Result<(), IndexOutOfBoundsError> {{
            if index >= N {{
                return Err(IndexOutOfBoundsError);
            }}

            unsafe {{ self.set_unchecked(index, value) }}

            Ok(())
        }}

        /// Sets the component at the given index with no bounds checking.
        /// 
        /// # Safety
        /// The caller must ensure that the index is in bounds.
        #[inline(always)]
        pub unsafe fn set_unchecked(&mut self, index: usize, value: T) {{
            let mut array = self.as_array();
            unsafe {{ *array.get_unchecked_mut(index) = value }};
            *self = Self::from_array(array);
        }}

        /// Maps each component of the vector to a new value using the given function.
        #[inline(always)]
        pub fn map<T2: Scalar, F: Fn(T) -> T2>(self, f: F) -> Vector<N, T2, A>
        where
            Usize<N>: VecLen,
        {{
            Vector::from_array(self.as_array().map(f))
        }}

        /// Folds the vector into a single value by repeatedly applying the given function to an accumulator on the vector's elements,
        /// using the given initial value.
        #[inline(always)]
        pub fn fold(self, init: T, mut f: impl FnMut(T, T) -> T) -> T {{
            let mut output = init;

            for i in 0..N {{
                output = f(output, self.index(i));
            }}

            output
        }}

        /// Reduces the vector into a single value by repeatedly applying the given function to an accumulator on the vector's elements,
        /// using the first component as the initial value.
        #[inline(always)]
        pub fn reduce(self, mut f: impl FnMut(T, T) -> T) -> T {{
            let mut output = self.index(0);

            for i in 1..N {{
                output = f(output, self.index(i));
            }}

            output
        }}

        /// Returns true if all components of the vector satisfy the given predicate.
        /// If a component does not satisfy the predicate,
        /// the function returns false immediately without evaluating the remaining components.
        #[inline(always)]
        pub fn all(self, f: impl FnMut(T) -> bool) -> bool {{
            self.into_iter().all(f)
        }}

        /// Returns true if any component of the vector satisfies the given predicate.
        /// If a component satisfies the predicate,
        /// the function returns true immediately without evaluating the remaining components.
        #[inline(always)]
        pub fn any(self, f: impl FnMut(T) -> bool) -> bool {{
            self.into_iter().any(f)
        }}

        /// Returns the number of components that satisfy the given predicate.
        #[inline(always)]
        pub fn count(self, mut f: impl FnMut(T) -> bool) -> usize {{
            self.into_iter().filter(|x| f(*x)).count()
        }}
    "#}
}

fn packed_array_fns(_scalar_override_fns: &mut Vec<String>) -> String {
    formatdoc! {r#"
        /// Converts an array reference to a vector reference.
        #[inline(always)]
        pub const fn from_array_ref(array: &[T; N]) -> &Self {{
            unsafe {{ transmute::<&[T; N], &Vector<N, T, VecPacked>>(array) }}
        }}

        /// Converts a mutable array reference to a mutable vector reference.
        #[inline(always)]
        pub const fn from_mut_array(array: &mut [T; N]) -> &mut Self {{
            unsafe {{ transmute::<&mut [T; N], &mut Vector<N, T, VecPacked>>(array) }}
        }}

        /// Converts a vector reference to an array reference.
        #[inline(always)]
        pub const fn as_array_ref(&self) -> &[T; N] {{
            &self.0
        }}

        /// Converts a mutable vector reference to a mutable array reference.
        #[inline(always)]
        pub const fn as_mut_array(&mut self) -> &mut [T; N] {{
            &mut self.0
        }}

        /// Returns a pointer to the first element of the vector.
        #[inline(always)]
        pub const fn as_ptr(&self) -> *const T {{
            self.0.as_ptr()
        }}

        /// Returns a mutable pointer to the first element of the vector.
        #[inline(always)]
        pub const fn as_mut_ptr(&mut self) -> *mut T {{
            self.0.as_mut_ptr()
        }}
    "#}
}
