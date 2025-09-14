use genco::quote;

use crate::{
    constants::{BINARY_OPS, COMPONENTS, LENGTHS, LENGTH_NAMES, UNARY_OPS},
    module::{ModDir, TokensExt},
};

mod dir;
mod primitives;
mod swizzle;

pub fn mod_() -> ModDir {
    quote! {
        //! Vector related types and traits

        use core::{
            fmt::{Debug, Display},
            ops::*,
            slice::SliceIndex,
            mem::{transmute_copy, transmute},
            hash::{Hash, Hasher},
        };

        use crate::{Construct, Usize, return_for_types, IndexOutOfBoundsError};

        mod dir;
        mod primitives;
        mod swizzle;
        pub use dir::*;

        $(let length_list = LENGTHS.iter().map(|&n| n.to_string()).collect::<Vec<_>>().join(", "))

        /// A generic vector type.
        ///
        /// This type is generic over 3 parameters:
        $(format!("/// - `N`: The length of the vector, which currently supports {length_list}."))
        /// - `T`: The type of the vector, which must implement the [`Scalar`] trait.
        /// - `A`: The "alignment" of the vector, which enables or disables SIMD memory alignment.
        ///
        /// This type has very very useful type-aliases:
        /// - `Vec{N}<T>` like `Vec2<f32>` is for SIMD aligned vectors
        /// - `Vec{N}P<T>` like `Vec2P<f32>` is for non-SIMD aligned vectors
        ///
        /// # Length
        ///
        $(format!("/// Currently only the lengths {length_list} are supported in order to specialize their inner vector type."))
        /// In the future if rust gains more type-system features, more lengths will be supported.
        ///
        $(format!("/// Beware that code should never rely on the fact that {length_list} are the only supported lengths."))
        /// Code that branches based on vector length should either properly handle all usize values or use [`VecLenEnum`].
        ///
        /// # Alignment
        ///
        /// The `A` generic parameter controls whether or not the vector is SIMD aligned,
        /// and can be set to either `VecAligned` or `VecPacked`.
        ///
        /// SIMD can improve performance of vector operations,
        /// but it can also increase the size of the vector in memory.
        ///
        /// `Vector<N, T, VecAligned>` vectors are SIMD aligned if it increases performance,
        /// while `Vector<N, T, VecPacked>` vectors are not SIMD aligned and are always stored as `[T; N]`.
        ///
        /// This means that `VecAligned` are for performance and `VecPacked` are for memory efficiency.
        ///
        /// Beware that while `VecPacked` guarentees an exact memory layout of `[T; N]`, `VecAligned` does not guarantee a specific alignment rule/pattern.
        /// For example, `Vector<3, f32, VecAligned`/`Vec3<f32>` isn't guaranteed to be aligned to a 128-bit boundary.
        /// It is up to the implementation of [`Scalar`] to determine `VecAligned` alignment for whatever is most performant.
        ///
        /// # Examples
        /// ```
        /// use ggmath::*;
        ///
        /// // This is a non memory critical scenario so we should use `VecAligned`.
        /// struct PlayerState {
        ///     // Vector<3, f32, VecAligned>
        ///     position: Vec3<f32>,
        ///     // ...
        /// }
        ///
        /// // This is a memory critical scenario so we should use `VecPacked`.
        /// struct GpuVertex {
        ///     // Vector<3, f32, VecPacked>
        ///     position: Vec3P<f32>,
        ///     // Vector<3, f32, VecPacked>
        ///     normal: Vec3P<f32>,
        ///     // Vector<2, f32, VecPacked>
        ///     uv: Vec2P<f32>,
        /// }
        /// ```
        #[repr(transparent)]
        pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>(pub A::InnerVector<N, T>)
        where
            Usize<N>: VecLen;

        $(
            for &n in LENGTHS =>

            $(format!("/// Type alias for [`Vector<{n}, T, VecAligned>`][Vector]."))
            pub type Vec$(n)<T> = Vector<$n, T, VecAligned>;

            $(format!("/// Type alias for [`Vector<{n}, T, VecPacked>`][Vector]."))
            pub type Vec$(n)P<T> = Vector<$n, T, VecPacked>;
        )

        /// Macro that generates vector type aliases for a specific scalar type.
        ///
        /// Syntax: `vector_aliases!(<visibility> <prefix> => <scalar>)`
        ///
        /// # Examples
        ///
        /// ```
        /// use ggmath::*;
        /// 
        /// vector_aliases!(pub F => f32);
        /// ```
        /// Generates:
        /// ```
        $(
            for &n in LENGTHS =>

            type FVec$(n) = ggmath::Vec$(n)<f32>;
            type FVec$(n)P = ggmath::Vec$(n)P<f32>;
        )
        /// ```
        #[macro_export]
        macro_rules! vector_aliases {
            (pub($$($$vis:tt)*) $$prefix:ident => $$t:ty) => {
                $$crate::vector_aliases!(@(pub $$($$vis)*) $$prefix => $$t);
            };
            (pub $$prefix:ident => $$t:ty) => {
                $$crate::vector_aliases!(@(pub) $$prefix => $$t);
            };
            ($$prefix:ident => $$t:ty) => {
                $$crate::vector_aliases!(@() $$prefix => $$t);
            };

            (@($$($$vis:tt)*) $$prefix:ident => $$t:ty) => {
                $$crate::_hidden_::paste! {$(
                    for &n in LENGTHS =>

                    #[doc = $(format!("Type alias to [`Vector<{n}, ")) $$t ", VecAligned>`][Vector]."]
                    $$($$vis)* type [<$$prefix Vec$(n)>] = $$crate::Vec$(n)<$$t>;

                    #[doc = $(format!("Type alias to [`Vector<{n}, ")) $$t ", VecPacked>`][Vector]."]
                    $$($$vis)* type [<$$prefix Vec$(n)P>] = $$crate::Vec$(n)P<$$t>;
                )}
            };
        }

        /// A trait that marks a `Usize<N>` type as a valid vector length.
        /// See [`Vector`] for more information.
        pub trait VecLen {
            /// The inner type contained inside `Vector<N, T, VecAligned>`.
            ///
            /// This redirects to `T::InnerAlignedVec{N}`,
            /// for example `T::InnerAlignedVec2` for `Usize<2>`.
            type InnerAlignedVector<T: Scalar>: Construct;

            /// The length value as an enum.
            const ENUM: VecLenEnum;
        }

        /// An enum with all currently supported vector lengths.
        ///
        /// The enum value of a generic `const N: usize` value can be accessed with [`<Usize<N> as VecLen>::ENUM`][`VecLen::ENUM`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum VecLenEnum {$(
            for (&n, &length_name) in LENGTHS.iter().zip(LENGTH_NAMES.iter()) =>

            $(format!("/// `{n}`"))
            $length_name,
        )*}

        /// A trait that marks a type as a valid scalar type that can be used in a vector.
        /// This trait is implemented for most primitive types, like `f32`, `f64`, `bool`, `usize`, etc.
        ///
        /// # Implementing `Scalar`
        ///
        /// When implementing `Scalar` you need to fill:
        ///
        /// 1.
        /// `InnerAlignedVec2`, `InnerAlignedVec3`, etc
        ///
        /// These are the inner types stored inside `VecAligned` vectors,
        /// for example `Vector<3, f32, VecAligned>` is stored as `f32::InnerAlignedVec3`.
        ///
        /// The reference of these types MUST be transmutable to `&[T; N]`,
        /// if its not then using that vector is undefined behavior.
        /// This means that you cannot do things like expand `Vec3<bool>` into a 128-bit SIMD register with 32-bit lanes.
        ///
        /// 2.
        /// `GARBAGE`, `INNER_ALIGNED_VEC2_GARBAGE`, `INNER_ALIGNED_VEC3_GARBAGE`, etc
        ///
        /// These need to be any valid value of `Self`, `Self::InnerAlignedVec2`, `Self::InnerAlignedVec3`, etc.
        /// This is used to properly initialize aligned vectors.
        ///
        /// # Examples
        ///
        /// ```
        /// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
        /// struct BigInt {
        ///     // private fields
        /// }
        ///
        /// // impl Add, Sub... for BigInt
        ///
        /// // lets say BigInt cannot benefit from SIMD operations, or we just don't want to optimize it yet.
        /// // When not wanting SIMD we can fill `InnerAlignedVec{N}` with `[Self; N]`.
        /// impl Scalar for BigInt {
        ///     {bigint_vector_types}
        /// }
        ///
        /// struct SmallInt(i32);
        ///
        /// // impl Add, Sub... for SmallInt
        ///
        /// // lets say SmallInt can benefit from SIMD operations.
        /// impl Scalar for SmallInt {
        ///     // use i32 vector types for aligned vectors.
        ///     {smallint_vector_types}
        /// }
        /// ```
        pub trait Scalar: Construct {
            $(
                for &n in LENGTHS =>

                $(format!("/// The inner type contained inside `Vector<{n}, Self, VecAligned>` vectors."))
                type InnerAlignedVec$(n): Construct;
            )

            $(
                for &n in LENGTHS =>

                $(format!("/// Constructs an aligned vec{n} from an array."))
                fn vec$(n)_from_array(array: [Self; $n]) -> Vec$(n)<Self>;

                $(format!("/// Converts an aligned vec{n} to an array."))
                fn vec$(n)_as_array(vec: Vec$(n)<Self>) -> [Self; $n];
            )

            $(
                for &n in LENGTHS =>

                $(format!("/// Overridable implementation of `Vector::splat` for aligned vec{n}s."))
                #[inline(always)]
                fn vec$(n)_splat(value: Self) -> Vec$(n)<Self> {
                    Vec$(n)::from_array([value; $n])
                }

                $(format!("/// Overridable implementation of aligned vec{n} getters."))
                #[inline(always)]
                fn vec$(n)_swizzle1<const SRC: usize>(vec: Vec$(n)<Self>) -> Self {{
                    vec.index(SRC)
                }}

                $(
                    for &n2 in LENGTHS =>

                    $(format!("/// Overridable implementation of aligned vec{n} swizzles that return vec{n2}s."))
                    #[inline(always)]
                    fn vec$(n)_swizzle$(n2)<$(for i in 0..n2 join(, ) => const $(COMPONENTS[i])_SRC: usize)>>(vec: Vec$(n)<Self>) -> Vec$(n2)<Self> {{
                        Vec$(n2)::from_array([$(for i in 0..n2 join(, ) => vec.index($(COMPONENTS[i])_SRC))])
                    }}
                )

                $(format!("/// Overridable implementation of aligned vec{n} \"with swizzles\" that replaces scalars."))
                #[inline(always)]
                fn vec$(n)_with_swizzle1<const DST: usize>(vec: Vec$(n)<Self>, value: Self) -> Vec$(n)<Self> {{
                    let mut output = vec;
                    output.set(DST, value);

                    output
                }}

                $(
                    for &n2 in LENGTHS.iter().filter(|&&n2| n2 <= n) =>

                    $(format!("/// Overridable implementation of aligned vec{n} \"with swizzles\" that replaces vec{n2}s."))
                    #[inline(always)]
                    fn vec$(n)_with_swizzle$(n2)<$(for i in 0..n2 join(, ) => const $(COMPONENTS[i])_DST: usize)>>(vec: Vec$(n)<Self>, value: Vec$(n2)<Self>) -> Vec$(n)<Self> {{
                        let mut output = vec;
                        $(
                            for i in 0..n2 =>

                            output.set($(COMPONENTS[i])_DST, value.index(i));
                        )

                        output
                    }}
                )

                $(format!("/// Overridable implementation of `Vector::eq` for aligned vec{n}s."))
                #[inline(always)]
                fn vec$(n)_eq<T2: Scalar>(vec: Vec$(n)<Self>, other: Vec$(n)<T2>) -> bool
                where
                    Self: PartialEq<T2>,
                {{
                    (0..$n).all(|i| vec.index(i) == other.index(i))
                }}
        
                $(format!("/// Overridable implementation of `Vector::ne` for aligned vec{n}s."))
                #[inline(always)]
                fn vec$(n)_ne<T2: Scalar>(vec: Vec$(n)<Self>, other: Vec$(n)<T2>) -> bool
                where
                    Self: PartialEq<T2>,
                {{
                    (0..$n).any(|i| vec.index(i) != other.index(i))
                }}

                $(
                    for &op_camelcase in UNARY_OPS =>

                    $(let op_snakecase = &op_camelcase.to_lowercase())

                    $(format!("/// Overridable implementation of `Vector::{op_snakecase}` for aligned vec{n}s."))
                    #[inline(always)]
                    fn vec$(n)_$(op_snakecase)(vec: Vec$(n)<Self>) -> Vec$(n)<<Self as $op_camelcase>::Output>
                    where
                        Self: $op_camelcase<Output: Scalar>,
                    {{
                        vec.map(|v| v.$op_snakecase())
                    }}
                )

                $(
                    for &op_camelcase in BINARY_OPS =>

                    $(let op_snakecase = &op_camelcase.to_lowercase())

                    $(format!("/// Overridable implementation of `Vector::{op_snakecase}` for aligned vec{n}s."))
                    #[inline(always)]
                    fn vec$(n)_$(op_snakecase)<T2: Scalar>(vec: Vec$(n)<Self>, other: Vec$(n)<T2>) -> Vec$(n)<<Self as $op_camelcase<T2>>::Output>
                    where
                        Self: $op_camelcase<T2, Output: Scalar>,
                    {{
                        Vector::from_fn(|i| vec.index(i).$op_snakecase(other.index(i)))
                    }}
                )
            )
        }

        /// See [`Vector`] for information.
        pub trait VecAlignment: 'static {
            /// The inner type contained inside [`Vector`].
            ///
            /// For `VecAligned` vectors this is `T::InnerAlignedVec{N}`,
            /// for example `T::InnerAlignedVec2` for `Vec2`.
            ///
            /// For `VecPacked` vectors this is `[T; N]`,
            /// for example `[T; 2]` for `Vec2`.
            type InnerVector<const N: usize, T: Scalar>: Construct
            where
                Usize<N>: VecLen;

            /// Whether or not the vector is SIMD aligned.
            const IS_ALIGNED: bool;
        }

        /// See [`Vector`] for information.
        pub struct VecAligned;

        /// See [`Vector`] for information.
        pub struct VecPacked;

        $(
            for (&n, &length_name) in LENGTHS.iter().zip(LENGTH_NAMES.iter()) =>

            impl VecLen for Usize<$n> {
                type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec$(n);

                const ENUM: VecLenEnum = VecLenEnum::$length_name;
            }
        )

        impl VecAlignment for VecAligned {
            type InnerVector<const N: usize, T: Scalar>
                = <Usize<N> as VecLen>::InnerAlignedVector<T>
            where
                Usize<N>: VecLen;

            const IS_ALIGNED: bool = true;
        }

        impl VecAlignment for VecPacked {
            type InnerVector<const N: usize, T: Scalar>
                = [T; N]
            where
                Usize<N>: VecLen;
                
            const IS_ALIGNED: bool = false;
        }

        impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A> {
            /// Returns true if the vector is aligned.
            /// The output is strictly determined by the type of the vector.
            #[inline(always)]
            pub const fn is_aligned(self) -> bool {
                A::IS_ALIGNED
            }

            /// Converts the vector to an aligned vector.
            #[inline(always)]
            pub fn align(self) -> Vector<N, T, VecAligned> {
                self.to_storage()
            }

            /// Converts the vector to a packed vector.
            #[inline(always)]
            pub fn pack(self) -> Vector<N, T, VecPacked> {
                self.to_storage()
            }

            /// Converts the vector to the specified alignment.
            #[inline(always)]
            pub fn to_storage<A2: VecAlignment>(self) -> Vector<N, T, A2> {
                match A::IS_ALIGNED {
                    true => match A2::IS_ALIGNED {
                        true => unsafe { transmute_copy::<Vector<N, T, A>, Vector<N, T, A2>>(&self) },
                        false => Vector::from_array(self.to_array()),
                    },
                    false => match A2::IS_ALIGNED {
                        true => Vector::from_array(self.to_array()),
                        false => unsafe { transmute_copy::<Vector<N, T, A>, Vector<N, T, A2>>(&self) },
                    },
                }
            }

            /// Creates a new vector from an array.
            #[inline(always)]
            pub fn from_array(array: [T; N]) -> Self {
                $(
                    for &n in LENGTHS =>

                    return_for_types! {
                        (array: [T; N] => [T; $n]) -> Vector<$n, T, VecAligned> => Vector<N, T, A> {
                            |array| T::vec$(n)_from_array(array)
                        }
                    }
                )

                return_for_types! {
                    (array: [T; N] => [T; N]) -> Vector<N, T, VecPacked> => Vector<N, T, A> {
                        |array| Vector(array)
                    }
                }

                unreachable!("unusual vector type")
            }

            /// Creates a new vector where each component is the same value.
            #[inline(always)]
            pub fn splat(value: T) -> Self {
                $(
                    for &n in LENGTHS =>

                    return_for_types! {
                        (value: T => T) -> Vector<$n, T, VecAligned> => Vector<N, T, A> {
                            |value| T::vec$(n)_splat(value)
                        }
                    }
                )
                
                Vector::from_array([value; N])
            }

            /// Creates a new vector where each component is evaluated from the given function called with the component index.
            /// The function is called in order.
            #[inline(always)]
            pub fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self {
                Vector::from_array(core::array::from_fn(f))
            }

            /// Returns the number of components in the vector.
            #[inline(always)]
            pub const fn len(self) -> usize {
                N
            }

            /// Converts the vector to an array.
            #[inline(always)]
            pub fn as_array(self) -> [T; N] {
                $(
                    for &n in LENGTHS =>

                    return_for_types! {
                        (self: Vector<N, T, A> => Vector<$n, T, VecAligned>) -> [T; $n] => [T; N] {
                            |(vec,)| T::vec$(n)_as_array(vec)
                        }
                    }
                )

                return_for_types! {
                    (self: Vector<N, T, A> => Vector<N, T, VecPacked>) -> [T; N] => [T; N] {
                        |(vec,)| vec.0
                    }
                }

                unreachable!("unusual vector type")
            }

            /// Returns the component at the given index or panics if the index is out of bounds.
            #[inline(always)]
            pub fn index(self, index: usize) -> T {
                self.as_array().index(index)
            }

            /// Returns the component at the given index or returns None if the index is out of bounds.
            #[inline(always)]
            pub fn get(self, index: usize) -> Option<T> {
                self.as_array().get(index)
            }

            /// Returns the component at the given index with no bounds checking.
            /// 
            /// # Safety
            /// The caller must ensure that the index is in bounds.
            #[inline(always)]
            pub unsafe fn get_unchecked(self, index: usize) -> T {
                unsafe { self.as_array().get_unchecked(index) }
            }

            /// Sets the component at the given index or panics if the index is out of bounds.
            #[inline(always)]
            pub fn set(&mut self, index: usize, value: T) {
                let mut array = self.as_array();
                array[index] = value;
                *self = Self::from_array(array);
            }

            /// Sets the component at the given index or returns an error if the index is out of bounds.
            #[inline(always)]
            pub fn try_set(&mut self, index: usize, value: T) -> Result<(), IndexOutOfBoundsError> {
                if index >= N {
                    return Err(IndexOutOfBoundsError);
                }

                unsafe { self.set_unchecked(index, value) }

                Ok(())
            }

            /// Sets the component at the given index with no bounds checking.
            /// 
            /// # Safety
            /// The caller must ensure that the index is in bounds.
            #[inline(always)]
            pub unsafe fn set_unchecked(&mut self, index: usize, value: T) {
                let mut array = self.as_array();
                unsafe { *array.get_unchecked_mut(index) = value };
                *self = Self::from_array(array);
            }

            /// Maps each component of the vector to a new value using the given function.
            #[inline(always)]
            pub fn map<T2: Scalar, F: Fn(T) -> T2>(self, f: F) -> Vector<N, T2, A>
            where
                Usize<N>: VecLen,
            {
                Vector::from_array(self.as_array().map(f))
            }

            /// Folds the vector into a single value by repeatedly applying the given function to an accumulator on the vector's elements,
            /// using the given initial value.
            #[inline(always)]
            pub fn fold(self, init: T, mut f: impl FnMut(T, T) -> T) -> T {
                let mut output = init;

                for i in 0..N {
                    output = f(output, self.index(i));
                }

                output
            }

            /// Reduces the vector into a single value by repeatedly applying the given function to an accumulator on the vector's elements,
            /// using the first component as the initial value.
            #[inline(always)]
            pub fn reduce(self, mut f: impl FnMut(T, T) -> T) -> T {
                let mut output = self.index(0);

                for i in 1..N {
                    output = f(output, self.index(i));
                }

                output
            }

            /// Returns true if all components of the vector satisfy the given predicate.
            /// If a component does not satisfy the predicate,
            /// the function returns false immediately without evaluating the remaining components.
            #[inline(always)]
            pub fn all(self, f: impl FnMut(T) -> bool) -> bool {
                self.into_iter().all(f)
            }

            /// Returns true if any component of the vector satisfies the given predicate.
            /// If a component satisfies the predicate,
            /// the function returns true immediately without evaluating the remaining components.
            #[inline(always)]
            pub fn any(self, f: impl FnMut(T) -> bool) -> bool {
                self.into_iter().any(f)
            }

            /// Returns the number of components that satisfy the given predicate.
            #[inline(always)]
            pub fn count(self, mut f: impl FnMut(T) -> bool) -> usize {
                self.into_iter().filter(|x| f(*x)).count()
            }
        }

        impl<const N: usize, T: Scalar> Vector<N, T, VecPacked> {
            /// Converts an array reference to a vector reference.
            #[inline(always)]
            pub const fn from_array_ref(array: &[T; N]) -> &Self {
                unsafe { transmute::<&[T; N], &Vector<N, T, VecPacked>>(array) }
            }

            /// Converts a mutable array reference to a mutable vector reference.
            #[inline(always)]
            pub const fn from_mut_array(array: &mut [T; N]) -> &mut Self {
                unsafe { transmute::<&mut [T; N], &mut Vector<N, T, VecPacked>>(array) }
            }

            /// Converts a vector reference to an array reference.
            #[inline(always)]
            pub const fn as_array_ref(&self) -> &[T; N] {
                &self.0
            }

            /// Converts a mutable vector reference to a mutable array reference.
            #[inline(always)]
            pub const fn as_mut_array(&mut self) -> &mut [T; N] {
                &mut self.0
            }

            /// Returns a pointer to the first element of the vector.
            #[inline(always)]
            pub const fn as_ptr(&self) -> *const T {
                self.0.as_ptr()
            }

            /// Returns a mutable pointer to the first element of the vector.
            #[inline(always)]
            pub const fn as_mut_ptr(&mut self) -> *mut T {
                self.0.as_mut_ptr()
            }
        }

        impl<const N: usize, T: Scalar, A: VecAlignment> Clone for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<const N: usize, T: Scalar, A: VecAlignment> Copy for Vector<N, T, A> where Usize<N>: VecLen {}

        impl<const N: usize, T: Scalar, A: VecAlignment> IntoIterator for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            type Item = T;
            type IntoIter = <[T; N] as IntoIterator>::IntoIter;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                self.as_array().into_iter()
            }
        }

        impl<const N: usize, T: Scalar, I: SliceIndex<[T]>> Index<I> for Vector<N, T, VecPacked>
        where
            Usize<N>: VecLen,
        {
            type Output = I::Output;

            #[inline(always)]
            fn index(&self, index: I) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<const N: usize, T: Scalar, I: SliceIndex<[T]>> IndexMut<I> for Vector<N, T, VecPacked>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn index_mut(&mut self, index: I) -> &mut Self::Output {
                &mut self.0[index]
            }
        }

        impl<const N: usize, T: Scalar + PartialEq<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
            PartialEq<Vector<N, T2, A2>> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            #[inline(always)]
            fn eq(&self, other: &Vector<N, T2, A2>) -> bool {{
                $(
                    for &n in LENGTHS =>
                    
                    return_for_types! {{
                        (
                            self: Vector<N, T, A> => Vector<$n, T, VecAligned>,
                            other: Vector<N, T2, A2> => Vector<$n, T2, VecAligned>,
                        ) -> bool => bool {{
                            |(vec, other)| T::vec$(n)_eq(vec, other)
                        }}
                    }}
                )

                (0..N).all(|i| self.index(i) == other.index(i))
            }}

            #[inline(always)]
            fn ne(&self, other: &Vector<N, T2, A2>) -> bool {{
                $(
                    for &n in LENGTHS =>

                    return_for_types! {{
                        (
                            self: Vector<N, T, A> => Vector<$n, T, VecAligned>,
                            other: Vector<N, T2, A2> => Vector<$n, T2, VecAligned>,
                        ) -> bool => bool {{
                            |(vec, other)| T::vec$(n)_ne(vec, other)
                        }}
                    }}
                )

                (0..N).any(|i| self.index(i) != other.index(i))
            }}
        }}

        impl<const N: usize, T: Scalar + Eq, A: VecAlignment> Eq for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{}}

        impl<const N: usize, T: Scalar + Hash, A: VecAlignment> Hash for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            #[inline(always)]
            fn hash<H: Hasher>(&self, state: &mut H) {{
                self.as_array().hash(state)
            }}
        }}

        impl<const N: usize, T: Scalar + Default, A: VecAlignment> Default for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            #[inline(always)]
            fn default() -> Self {{
                Self::splat(T::default())
            }}
        }}

        impl<const N: usize, T: Scalar + Debug, A: VecAlignment> Debug for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                write!(f, "(")?;

                for i in 0..N {{
                    if i != 0 {{
                        write!(f, ", ")?;
                    }}

                    write!(f, "{{:?}}", self.index(i))?;
                }}

                write!(f, ")")?;

                Ok(())
            }}
        }}

        impl<const N: usize, T: Scalar + Display, A: VecAlignment> Display for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                write!(f, "(")?;

                for i in 0..N {{
                    if i != 0 {{
                        write!(f, ", ")?;
                    }}

                    write!(f, "{{}}", self.index(i))?;
                }}

                write!(f, ")")?;

                Ok(())
            }}
        }}

        $(
            for &op_camelcase in UNARY_OPS =>

            $(let op_snakecase = &op_camelcase.to_lowercase())

            impl<const N: usize, T: Scalar + $op_camelcase<Output: Scalar>, A: VecAlignment> $op_camelcase for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self) -> Self::Output {{
                    $(
                        for &n in LENGTHS =>

                        return_for_types! {{
                            (self: Vector<N, T, A> => Vector<$n, T, VecAligned>) -> Vector<$n, T::Output, VecAligned> => Vector<N, T::Output, A> {{
                                |(vec,)| T::vec$(n)_$(op_snakecase)(vec)
                            }}
                        }}
                    )

                    self.map(|v| v.$op_snakecase())
                }}
            }}

            impl<const N: usize, T: Scalar + $op_camelcase<Output: Scalar>, A: VecAlignment> $op_camelcase for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self) -> Self::Output {{
                    (*self).$op_snakecase()
                }}
            }}
        )

        $(
            for &op_camelcase in BINARY_OPS =>

            $(let op_snakecase = &op_camelcase.to_lowercase())

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                $op_camelcase<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: Vector<N, T2, A2>) -> Self::Output {{
                    $(
                        for &n in LENGTHS =>

                        return_for_types! {{
                            (
                                self: Vector<N, T, A> => Vector<$n, T, VecAligned>,
                                rhs: Vector<N, T2, A2> => Vector<$n, T2, VecAligned>,
                            ) -> Vector<$n, T::Output, VecAligned> => Vector<N, T::Output, A> {{
                                |(vec, rhs)| T::vec$(n)_$(op_snakecase)(vec, rhs)
                            }}
                        }}
                    )

                    Vector::from_fn(|i| self.index(i).$op_snakecase(rhs.index(i)))
                }}
            }}

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                $op_camelcase<Vector<N, T2, A2>> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: Vector<N, T2, A2>) -> Self::Output {{
                    (*self).$op_snakecase(rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                $op_camelcase<&Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: &Vector<N, T2, A2>) -> Self::Output {{
                    self.$op_snakecase(*rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                $op_camelcase<&Vector<N, T2, A2>> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: &Vector<N, T2, A2>) -> Self::Output {{
                    (*self).$op_snakecase(*rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + $op_camelcase<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                $op_camelcase Assign<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn $(op_snakecase)_assign(&mut self, rhs: Vector<N, T2, A2>) {{
                    *self = (*self).{op_snakecase}(rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + $op_camelcase<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                $op_camelcase Assign<&Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn $(op_snakecase)_assign(&mut self, rhs: &Vector<N, T2, A2>) {{
                    *self = (*self).{op_snakecase}(*rhs)
                }}
            }}
        )
    }
    .to_mod_dir("vector")
    .with_submod_dir(primitives::mod_())
    .with_submod_file(swizzle::mod_())
    .with_submod_file(dir::mod_())
}
