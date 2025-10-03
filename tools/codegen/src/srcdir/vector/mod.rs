use genco::quote;
use strum::IntoEnumIterator;

use crate::{module::{SrcDir, TokensExt}, iter::{Axis, CmpOp, Length}, util::join_or};

mod constructor;
mod dir;
mod ops;
mod primitives;
mod scalar;
mod swizzle;

pub fn srcmod() -> SrcDir {
    quote! {
        $("//! Vector related types and traits")

        use core::{
            fmt::{Debug, Display},
            ops::*,
            slice::SliceIndex,
            mem::transmute,
            hash::{Hash, Hasher},
        };

        use crate::{Construct, Usize, IndexOutOfBoundsError, specialize, sealed::Sealed};

        mod constructor;
        mod dir;
        mod primitives;
        mod ops;
        mod scalar;
        #[cfg(feature = "swizzle")]
        mod swizzle;
        pub use scalar::*;
        pub use constructor::*;
        pub use dir::*;

        $("/// A generic vector type.")
        $("///")
        $("/// Is generic over 3 parameters:")
        $(format!("/// - `N`: The length of the vector, which currently can be {}", join_or(Length::iter().map(|n| n.to_string()))))
        $("/// - `T`: The scalar type of the vector, which must implement [`Scalar`]")
        $("/// - `S`: The \"simdness\" of the vector, which must be either [`Simd`] or [`NonSimd`]")
        $("///")
        $("/// This type has very very useful type-aliases:")
        $("/// - `Vec{N}<T>` like [`Vec2<T>`] is for [`Simd`] vectors")
        $("/// - `Vec{N}S<T>` like [`Vec2S<T>`] is for [`NonSimd`] vectors (\"s\" stands for \"scalar\")")
        $("///")
        $("/// # Simdness")
        $("///")
        $("/// SIMD improves the performance of vector operations but increases the size of the vector in memory.")
        $("///")
        $("/// The `S` generic parameter controls whether or not the vector is SIMD.")
        $("/// [`Simd`] vectors are SIMD and [`NonSimd`] vectors are not.")
        $("///")
        $("/// Most of the time you should use [`Simd`] vectors (e.g. [`Vec3<T>`]),")
        $("/// and only use [`NonSimd`] vectors in memory-critical scenarios.")
        $("///")
        $("/// The exact vector layout rules are:")
        $("///")
        $("/// - [`NonSimd`] vectors are always stored as `[T; N]`.")
        $("///")
        $("/// - [`Simd`] vectors aren't actually guarenteed to be SIMD, especially if it doesn't have a performance benefit.")
        $("///")
        $("/// # Example")
        $("/// ```")
        $("/// use ggmath::*;")
        $("///")
        $("/// // This is a non memory critical scenario so we should use SIMD vectors.")
        $("/// struct PlayerState {")
        $("///     // Vector<3, f32, Simd>")
        $("///     position: Vec3<f32>,")
        $("/// }")
        $("///")
        $("/// // This is a memory critical scenario so we should use non-SIMD vectors.")
        $("/// struct GpuVertex {")
        $("///     // Vector<3, f32, NonSimd>")
        $("///     position: Vec3S<f32>,")
        $("///     // Vector<3, f32, NonSimd>")
        $("///     normal: Vec3S<f32>,")
        $("///     // Vector<2, f32, NonSimd>")
        $("///     uv: Vec2S<f32>,")
        $("/// }")
        $("///")
        $("/// fn initial_player_state() -> PlayerState {")
        $("///     PlayerState {")
        $("///         position: vec3!(0.0, 1.0, 2.0),")
        $("///     }")
        $("/// }")
        $("///")
        $("/// fn triangle_vertices() -> [GpuVertex; 3] {")
        $("///     [")
        $("///         GpuVertex {")
        $("///             position: vec3s!(-1.0, -1.0, 0.0),")
        $("///             normal: vec3s!(0.0, 0.0, 1.0),")
        $("///             uv: vec2s!(0.0, 0.0),")
        $("///         },")
        $("///         GpuVertex {")
        $("///             position: vec3s!(1.0, -1.0, 0.0),")
        $("///             normal: vec3s!(0.0, 0.0, 1.0),")
        $("///             uv: vec2s!(1.0, 0.0),")
        $("///         },")
        $("///         GpuVertex {")
        $("///             position: vec3s!(0.0, 1.0, 0.0),")
        $("///             normal: vec3s!(0.0, 0.0, 1.0),")
        $("///             uv: vec2s!(0.5, 1.0),")
        $("///         },")
        $("///     ]")
        $("/// }")
        $("/// ```")
        #[repr(transparent)]
        pub struct Vector<const N: usize, T: Scalar, S: Simdness>(pub S::VectorStorage<N, T>)
        where
            Usize<N>: VecLen;

        $(
            for n in Length::iter() =>

            $(format!("/// Type alias for [`Vector<{n}, T, Simd>`][Vector]."))
            pub type Vec$(n)<T> = Vector<$n, T, Simd>;
            $['\n']
        )
        $(
            for n in Length::iter() =>

            $(format!("/// Type alias for [`Vector<{n}, T, NonSimd>`][Vector]."))
            $(r#"/// "S" stands for "scalar"."#)
            pub type Vec$(n)S<T> = Vector<$n, T, NonSimd>;
            $['\n']
        )

        $("/// Generates vector type-aliases for a specific scalar type.")
        $("///")
        $("/// # Example")
        $("///")
        $("/// ```")
        $("/// use ggmath::*;")
        $("///")
        $("/// vector_aliases!(pub type F => f32);")
        $("/// ```")
        $("/// Generates:")
        $("/// ```")
        $("/// use ggmath::*;")
        $("///")
        $(for n in Length::iter() => $(format!("/// pub type FVec{n} = Vec{n}<f32>;"))$['\r'])
        $(for n in Length::iter() => $(format!("/// pub type FVec{n}S = Vec{n}S<f32>;"))$['\r'])
        $("/// ```")
        #[macro_export]
        macro_rules! vector_aliases {
            ($$(#[$$($$attr:tt)*])* pub($$($$vis:tt)*) type $$prefix:ident => $$t:ty) => {
                $$crate::vector_aliases!(@(pub type $$($$vis)*) $$(#[$$($$attr)*])* $$prefix => $$t);
            };
            ($$(#[$$($$attr:tt)*])* pub type $$prefix:ident => $$t:ty) => {
                $$crate::vector_aliases!(@(pub) $$(#[$$($$attr)*])* type $$prefix => $$t);
            };
            ($$(#[$$($$attr:tt)*])* type $$prefix:ident => $$t:ty) => {
                $$crate::vector_aliases!(@() $$(#[$$($$attr)*])* type $$prefix => $$t);
            };

            (@($$($$vis:tt)*) $$(#[$$($$attr:tt)*])* type $$prefix:ident => $$t:ty) => {
                $$crate::hidden::paste! {
                    $(
                        for n in Length::iter() =>

                        #[doc = $(format!("\"Type alias to [`Vector<{n}, \"")) $$t ", Simd>`][Vector]."]
                        $$(#[$$($$attr)*])*
                        $$($$vis)* type [<$$prefix Vec$(n)>] = $$crate::Vec$(n)<$$t>;

                        #[doc = $(format!("\"Type alias to [`Vector<{n}, \"")) $$t ", NonSimd>`][Vector]."]
                        #[doc = r#""S" stands for "scalar"."#]
                        $$(#[$$($$attr)*])*
                        $$($$vis)* type [<$$prefix Vec$(n)S>] = $$crate::Vec$(n)S<$$t>;

                        $['\n']
                    )
                }
            };
        }

        $("/// Marks a `Usize<N>` type as a valid vector length.")
        pub trait VecLen: Sealed {
            $("/// Returns the appropriate type for the given vector length (`T2` for `Usize<2>`, `T3` for `Usize<3>`, etc.).")
            type Match<$(for n in Length::iter() join(, ) => T$(n): Construct)>: Construct;

            $("/// The length value as an enum.")
            const ENUM: VecLenEnum;
        }

        $("/// An enum with all currently supported vector lengths.")
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum VecLenEnum {
            $(
                for n in Length::iter() =>

                $(format!("/// `{n}`"))
                $(n.word_camelcase()),
                $['\r']
            )
        }

        $("/// See [`Vector`] for information.")
        pub trait Simdness: Sealed + 'static {
            $("/// The inner type contained inside [`Vector`].")
            type VectorStorage<const N: usize, T: Scalar>: Construct
            where
                Usize<N>: VecLen;

            $("/// Whether or not the vector is SIMD.")
            $("/// Is `true` for `Simd` and `false` for `NonSimd`.")
            const IS_SIMD: bool;
        }

        $("/// See [`Vector`] for information.")
        pub struct Simd;

        $("/// See [`Vector`] for information.")
        pub struct NonSimd;

        $(
            for n in Length::iter() join($['\n'])=>

            impl VecLen for Usize<$n> {
                type Match<$(for n in Length::iter() join(, ) => T$(n): Construct)> = T$(n);

                const ENUM: VecLenEnum = VecLenEnum::$(n.word_camelcase());
            }
        )

        $(
            for n in Length::iter() join($['\n'])=>

            #[doc(hidden)]
            impl Sealed for Usize<$n> {}
        )

        impl Simdness for Simd {
            type VectorStorage<const N: usize, T: Scalar>
                = T::SimdVectorStorage<N>
            where
                Usize<N>: VecLen;

            const IS_SIMD: bool = true;
        }
        
        impl Simdness for NonSimd {
            type VectorStorage<const N: usize, T: Scalar>
                = [T; N]
            where
                Usize<N>: VecLen;
            
            const IS_SIMD: bool = false;
        }
        
        #[doc(hidden)]
        impl Sealed for Simd {}
        #[doc(hidden)]
        impl Sealed for NonSimd {}

        impl<const N: usize, T: Scalar, S: Simdness> Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            $("/// Returns `true` for `Simd` vectors and `false` for `NonSimd` vectors.")
            #[inline(always)]
            pub const fn is_simd(self) -> bool {
                S::IS_SIMD
            }

            $("/// Converts the vector to a `Simd` vector.")
            #[inline(always)]
            pub fn as_simd(self) -> Vector<N, T, Simd> {
                self.as_storage()
            }

            $("/// Converts the vector to a `NonSimd` vector.")
            #[inline(always)]
            pub fn as_non_simd(self) -> Vector<N, T, NonSimd> {
                self.as_storage()
            }

            $("/// Converts the vector to the specified simdness.")
            #[inline(always)]
            pub fn as_storage<S2: Simdness>(self) -> Vector<N, T, S2> {
                specialize! {
                    (self: Vector<N, T, S>) -> Vector<N, T, S2>:

                    for (Vector<N, T, S>) -> Vector<N, T, S> {
                        |vec| vec
                    }
                    else {
                        Vector::from_array(self.as_array())
                    }
                }
            }

            $("/// Creates a new vector from an array.")
            #[inline(always)]
            pub fn from_array(array: [T; N]) -> Self {
                specialize! {
                    (array: [T; N]) -> Vector<N, T, S>:

                    for ([T; N]) -> Vector<N, T, Simd> {
                        |array| T::vec_from_array(array)
                    } for ([T; N]) -> Vector<N, T, NonSimd> {
                        |array| Vector(array)
                    } else {
                        unreachable!("unusual vector type")
                    }
                }
            }

            $("/// Creates a new vector where each component is the same value.")
            #[inline(always)]
            pub fn splat(value: T) -> Self {
                specialize! {
                    (value: T) -> Vector<N, T, S>:
   
                    for (T) -> Vector<N, T, Simd> {
                        |value| T::vec_splat(value)
                    } else {
                        Vector::from_array([value; N])
                    }
                }
            }

            $("/// Creates a new vector where each component is evaluated from the given function called with the component index.")
            $("/// The function is called in order.")
            #[inline(always)]
            pub fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self {
                Vector::from_array(core::array::from_fn(f))
            }

            $("/// Returns the number of components in the vector.")
            #[inline(always)]
            pub const fn len(self) -> usize {
                N
            }

            $("/// Converts the vector to an array.")
            #[inline(always)]
            pub fn as_array(self) -> [T; N] {
                specialize! {
                    (self: Vector<N, T, S>) -> [T; N]:

                    for (Vector<N, T, Simd>) -> [T; N] {
                        |vec| T::vec_as_array(vec)
                    } for (Vector<N, T, NonSimd>) -> [T; N] {
                        |vec| vec.0
                    } else {
                        unreachable!("unusual vector type")
                    }
                }
            }

            $("/// Returns the component at the given index or panics if the index is out of bounds.")
            #[inline(always)]
            pub fn index(self, index: usize) -> T {
                if index >= N {
                    panic!("index out of bounds: the len is {N} but the index is {index}");
                }

                unsafe { self.get_unchecked(index) }
            }

            $("/// Returns the component at the given index or returns None if the index is out of bounds.")
            #[inline(always)]
            pub fn get(self, index: usize) -> Option<T> {
                if index >= N {
                    None
                } else {
                    Some(unsafe { self.get_unchecked(index) })
                }
            }

            $("/// Returns the component at the given index with no bounds checking.")
            $("///")
            $("/// # Safety")
            $("/// The caller must ensure that the index is in bounds.")
            #[inline(always)]
            pub unsafe fn get_unchecked(self, index: usize) -> T {
                specialize! {
                    (self: Vector<N, T, S>, index: usize) -> T:

                    for (Vector<N, T, Simd>, usize) -> T {
                        |vec, index| unsafe { T::vec_get_unchecked(vec, index) }
                    }
                    else {
                        unsafe { *self.as_array().get_unchecked(index) }
                    }
                }
            }

            $("/// Sets the component at the given index or panics if the index is out of bounds.")
            #[inline(always)]
            pub fn set(&mut self, index: usize, value: T) {
                if index >= N {
                    panic!("index out of bounds: the len is {N} but the index is {index}");
                }

                unsafe { self.set_unchecked(index, value) }
            }

            $("/// Sets the component at the given index or returns an error if the index is out of bounds.")
            #[inline(always)]
            pub fn try_set(&mut self, index: usize, value: T) -> Result<(), IndexOutOfBoundsError> {
                if index >= N {
                    Err(IndexOutOfBoundsError)
                } else {
                    unsafe { self.set_unchecked(index, value) }

                    Ok(())
                }
            }

            $("/// Sets the component at the given index with no bounds checking.")
            $("///")
            $("/// # Safety")
            $("/// The caller must ensure that the index is in bounds.")
            #[inline(always)]
            pub unsafe fn set_unchecked(&mut self, index: usize, value: T) {
                *self = specialize! {
                    ((*self): Vector<N, T, S>, index: usize, value: T) -> Vector<N, T, S>:
                    
                    for (Vector<N, T, Simd>, usize, T) -> Vector<N, T, Simd> {
                        |vec, index, value| unsafe { T::vec_with_unchecked(vec, index, value) }
                    } else {
                        let mut array = self.as_array();
                        unsafe {
                            *array.get_unchecked_mut(index) = value;
                        }

                        Vector::from_array(array)
                    }
                };
            }

            $(
                for n2 in Length::iter() join($['\n']) =>

                $(format!("/// Returns a vec{n2} where:"))
                $(
                    for i in 0..n2.as_usize() =>

                    $(format!("/// - The `{}` ({}) component is `self[{}_SRC]`", Axis(i).lowercase_str(), Axis(i).ordinal_str(), Axis(i).uppercase_str()))
                    $['\r']
                )
                $("///")
                $("/// Out of bounds indices are compile time errors.")
                #[inline(always)]
                pub fn shuffle_$(n2)<$(for i in 0..n2.as_usize() join(, ) => const $(Axis(i).uppercase_str())_SRC: usize)>(self) -> Vector<$n2, T, S> {
                    specialize! {
                        (self: Vector<N, T, S>) -> Vector<$n2, T, S>:

                        for (Vector<N, T, Simd>) -> Vector<$n2, T, Simd> {
                            |vec| T::vec_shuffle_$(n2)::<N, $(for i in 0..n2.as_usize() join(, ) => $(Axis(i).uppercase_str())_SRC)>(vec)
                        }
                        else {
                            Vector::<$n2, T, S>::from_array([$(for i in 0..n2.as_usize() join(, ) => self.index($(Axis(i).uppercase_str())_SRC))])
                        }
                    }
                }
            )

            $("/// Maps each component of the vector to a new value using the given function.")
            #[inline(always)]
            pub fn map<T2: Scalar, F: Fn(T) -> T2>(self, f: F) -> Vector<N, T2, S>
            where
                Usize<N>: VecLen,
            {
                Vector::from_array(self.as_array().map(f))
            }

            $("/// Returns an iterator over the components of the vector.")
            #[inline(always)]
            pub fn iter(self) -> <[T; N] as IntoIterator>::IntoIter {
                self.into_iter()
            }

            $("/// Folds the vector into a single value by repeatedly applying the given function to an accumulator on the vector's elements,")
            $("/// using the given initial value.")
            #[inline(always)]
            pub fn fold(self, init: T, mut f: impl FnMut(T, T) -> T) -> T {
                let mut output = init;

                for component in self {
                    output = f(output, component);
                }

                output
            }

            $("/// Reduces the vector into a single value by repeatedly applying the given function to an accumulator on the vector's elements,")
            $("/// using the first component as the initial value.")
            #[inline(always)]
            pub fn reduce(self, mut f: impl FnMut(T, T) -> T) -> T {
                let mut output = self.index(0);

                for component in self.iter().skip(1) {
                    output = f(output, component);
                }

                output
            }

            $("/// Returns true if all components of the vector satisfy the given predicate.")
            $("/// If a component does not satisfy the predicate,")
            $("/// the function returns false immediately without evaluating the remaining components.")
            #[inline(always)]
            pub fn all(self, f: impl FnMut(T) -> bool) -> bool {
                self.iter().all(f)
            }

            $("/// Returns true if any component of the vector satisfies the given predicate.")
            $("/// If a component satisfies the predicate,")
            $("/// the function returns true immediately without evaluating the remaining components.")
            #[inline(always)]
            pub fn any(self, f: impl FnMut(T) -> bool) -> bool {
                self.iter().any(f)
            }

            $("/// Returns the number of components that satisfy the given predicate.")
            #[inline(always)]
            pub fn count(self, mut f: impl FnMut(T) -> bool) -> usize {
                self.iter().filter(|x| f(*x)).count()
            }

            $(
                for cmp_op in CmpOp::iter() join($['\n']) =>

                $(format!("/// Returns a vector of booleans where each component is `true` if the corresponding component of `self` is {} the corresponding component of `other`.", cmp_op.doc_str()))
                #[inline(always)]
                pub fn $(cmp_op.lowercase_str())_mask<T2: Scalar>(self, other: Vector<N, T2, S>) -> Vector<N, bool, S>
                where
                    T: $(cmp_op.trait_str())<T2>,
                {
                    specialize! {
                        (self: Vector<N, T, S>, other: Vector<N, T2, S>) -> Vector<N, bool, S>:
    
                        for (Vector<N, T, Simd>, Vector<N, T2, Simd>) -> Vector<N, bool, Simd> {
                            |vec, other| T::vec_$(cmp_op.lowercase_str())_mask(vec, other)
                        } else {
                            Vector::from_fn(|i| self.index(i) $(cmp_op.punct_str()) other.index(i))
                        }
                    }
                }
            )

            $("/// Returns the sum of the components of the vector.")
            #[inline(always)]
            pub fn sum(self) -> T
            where
                T: Add<Output = T>,
            {
                specialize! {
                    (self: Vector<N, T, S>) -> T:

                    for (Vector<N, T, Simd>) -> T {
                        |vec| T::vec_sum(vec)
                    } else {
                        self.reduce(|a, b| a + b)
                    }
                }
            }

            $("/// Returns the product of the components of the vector.")
            #[inline(always)]
            pub fn product(self) -> T
            where
                T: Mul<Output = T>,
            {
                specialize! {
                    (self: Vector<N, T, S>) -> T:

                    for (Vector<N, T, Simd>) -> T {
                        |vec| T::vec_product(vec)
                    } else {
                        self.reduce(|a, b| a * b)
                    }
                }
            }

            $("/// Returns the square of the magnitude of the vector.")
            #[inline(always)]
            pub fn mag_sq(self) -> T
            where
                T: Add<Output = T> + Mul<Output = T>,
            {
                (self * self).sum()
            }

            $("/// Returns the dot product of `self` and `other`.")
            #[inline(always)]
            pub fn dot(self, other: Self) -> T
            where
                T: Add<Output = T> + Mul<Output = T>,
            {
                (self * other).sum()
            }
        }

        $(
            for n in Length::iter() join($['\n']) =>

            impl<T: Scalar, S: Simdness> Vector<$n, T, S> {
                $(
                    if n == Length::Two =>

                    $(format!("/// Returns `self` rotated 90 degrees counter-clockwise."))
                    #[inline(always)]
                    pub fn perp(self) -> Self
                    where
                        T: Neg<Output = T>,
                    {
                        vec2g!(-self.y(), self.x())
                    }

                    $("/// Returns the perpendicular dot product of `self` and `other`.")
                    #[inline(always)]
                    pub fn perp_dot(self, other: Self) -> T
                    where
                        T: Mul<Output = T> + Sub<Output = T>,
                    {
                        self.x() * other.y() - self.y() * other.x()
                    }
                )

                $(
                    if n == Length::Three =>

                    $(format!("/// Returns the cross product of `self` and `other`."))
                    #[inline(always)]
                    pub fn cross(self, other: Self) -> Self
                    where
                        T: Mul<Output = T> + Sub<Output = T>,
                    {
                        self.yzx() * other.zxy() - self.zxy() * other.yzx()
                    }
                )

                $(
                    for i in 0..n.as_usize() join($['\n']) =>

                    $(format!("/// Returns the `{}` ({}) component of `self`.", Axis(i).lowercase_str(), Axis(i).ordinal_str()))
                    #[inline(always)]
                    pub fn $(Axis(i).lowercase_str())(self) -> T {
                        self.index($i)
                    }
                )

                $(
                    for i in 0..n.as_usize() join($['\n']) =>

                    $(format!("/// Returns `self` but with the `{}` ({}) component set to `value`.", Axis(i).lowercase_str(), Axis(i).ordinal_str()))
                    #[inline(always)]
                    pub fn with_$(Axis(i).lowercase_str())(self, value: T) -> Self {
                        let mut output = self;
                        output.set($i, value);
                        output
                    }
                )

                $(
                    for i in 0..n.as_usize() join($['\n']) =>

                    $(format!("/// Sets the `{}` ({}) component of `self` to `value`.", Axis(i).lowercase_str(), Axis(i).ordinal_str()))
                    #[inline(always)]
                    pub fn set_$(Axis(i).lowercase_str())(&mut self, value: T) {
                        *self = self.with_$(Axis(i).lowercase_str())(value);
                    }
                )

                $(
                    for n2 in Length::iter().filter(|&n2| n2 <= n) join($['\n']) =>
    
                    $(format!("/// Returns `self` but with:"))
                    $(
                        for i in 0..n2.as_usize() =>

                        $(format!("/// - `self[{}_DST]` set to the `{}` ({}) component of `value`", Axis(i).uppercase_str(), Axis(i).lowercase_str(), Axis(i).ordinal_str()))
                        $['\r']
                    )
                    $("///")
                    $("/// Out of bounds indices are compile time errors.")
                    #[inline(always)]
                    pub fn with_shuffle_$(n2)<$(for i in 0..n2.as_usize() join(, ) => const $(Axis(i).uppercase_str())_DST: usize)>(self, value: Vector<$n2, T, S>) -> Self {
                        specialize! {
                            (self: Vector<$n, T, S>, value: Vector<$n2, T, _>) -> Vector<$n, T, S>:
    
                            for (Vector<$n, T, Simd>, Vector<$n2, T, Simd>) -> Vector<$n, T, Simd> {
                                |vec, value| T::vec_with_shuffle_$(n2)::<$n, $(for i in 0..n2.as_usize() join(, ) => $(Axis(i).uppercase_str())_DST)>(vec, value)
                            } else {
                                let mut output = self;
                                $(
                                    for i in 0..n2.as_usize() =>
    
                                    output.set($(Axis(i).uppercase_str())_DST, value.$(Axis(i).lowercase_str())());
                                    $['\r']
                                )
    
                                output
                            }
                        }
                    }
                )
            }
        )

        impl<const N: usize, T: Scalar> Vector<N, T, NonSimd>
        where
            Usize<N>: VecLen,
        {
            $("/// Converts an array reference to a vector reference.")
            #[inline(always)]
            pub const fn from_array_ref(array: &[T; N]) -> &Self {
                unsafe { transmute::<&[T; N], &Vector<N, T, NonSimd>>(array) }
            }

            $("/// Converts a mutable array reference to a mutable vector reference.")
            #[inline(always)]
            pub const fn from_mut_array(array: &mut [T; N]) -> &mut Self {
                unsafe { transmute::<&mut [T; N], &mut Vector<N, T, NonSimd>>(array) }
            }

            $("/// Converts a vector reference to an array reference.")
            #[inline(always)]
            pub const fn as_array_ref(&self) -> &[T; N] {
                &self.0
            }

            $("/// Converts a mutable vector reference to a mutable array reference.")
            #[inline(always)]
            pub const fn as_mut_array(&mut self) -> &mut [T; N] {
                &mut self.0
            }

            $("/// Returns a pointer to the first element of the vector.")
            #[inline(always)]
            pub const fn as_ptr(&self) -> *const T {
                self.0.as_ptr()
            }

            $("/// Returns a mutable pointer to the first element of the vector.")
            #[inline(always)]
            pub const fn as_mut_ptr(&mut self) -> *mut T {
                self.0.as_mut_ptr()
            }

            $("/// Returns an iterator over the references to the components of the vector.")
            #[inline(always)]
            pub fn iter_ref(&self) -> <&[T; N] as IntoIterator>::IntoIter {
                self.into_iter()
            }

            $("/// Returns an iterator over the mutable references to the components of the vector.")
            #[inline(always)]
            pub fn iter_mut(&mut self) -> <&mut [T; N] as IntoIterator>::IntoIter {
                self.into_iter()
            }
        }

        $(
            for n in Length::iter() join($['\n']) =>

            impl<T: Scalar> Vector<$n, T, NonSimd> {
                $(
                    for i in 0..n.as_usize() join($['\n']) =>

                    $(format!("/// Returns a reference to the `{}` ({}) component of `self`.", Axis(i).lowercase_str(), Axis(i).ordinal_str()))
                    #[inline(always)]
                    pub const fn $(Axis(i).lowercase_str())_ref(&self) -> &T {
                        &self.0[$i]
                    }
                )
                
                $(
                    for i in 0..n.as_usize() join($['\n']) =>

                    $(format!("/// Returns a mutable reference to the `{}` ({}) component of `self`.", Axis(i).lowercase_str(), Axis(i).ordinal_str()))
                    #[inline(always)]
                    pub const fn $(Axis(i).lowercase_str())_mut(&mut self) -> &mut T {
                        &mut self.0[$i]
                    }
                )
            }
        )

        impl<const N: usize, T: Scalar, S: Simdness> Clone for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<const N: usize, T: Scalar, S: Simdness> Copy for Vector<N, T, S> where Usize<N>: VecLen {}

        impl<const N: usize, T: Scalar, S: Simdness> IntoIterator for Vector<N, T, S>
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

        impl<'a, const N: usize, T: Scalar> IntoIterator for &'a Vector<N, T, NonSimd>
        where
            Usize<N>: VecLen,
        {
            type Item = &'a T;
            type IntoIter = <&'a [T; N] as IntoIterator>::IntoIter;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                self.as_array_ref().into_iter()
            }
        }

        impl<'a, const N: usize, T: Scalar> IntoIterator for &'a mut Vector<N, T, NonSimd>
        where
            Usize<N>: VecLen,
        {
            type Item = &'a mut T;
            type IntoIter = <&'a mut [T; N] as IntoIterator>::IntoIter;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                self.as_mut_array().into_iter()
            }
        }

        impl<const N: usize, T: Scalar, I: SliceIndex<[T]>> Index<I> for Vector<N, T, NonSimd>
        where
            Usize<N>: VecLen,
        {
            type Output = I::Output;

            #[inline(always)]
            fn index(&self, index: I) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<const N: usize, T: Scalar, I: SliceIndex<[T]>> IndexMut<I> for Vector<N, T, NonSimd>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn index_mut(&mut self, index: I) -> &mut Self::Output {
                &mut self.0[index]
            }
        }

        impl<const N: usize, T: Scalar + PartialEq<T2>, S: Simdness, T2: Scalar>
            PartialEq<Vector<N, T2, S>> for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn eq(&self, other: &Vector<N, T2, S>) -> bool {
                specialize! {
                    ((*self): Vector<N, T, S>, (*other): Vector<N, T2, S>) -> bool:

                    for (Vector<N, T, Simd>, Vector<N, T2, Simd>) -> bool {
                        |vec, other| T::vec_eq(vec, other)
                    } else {
                        self.iter().zip(*other).all(|(a, b)| a == b)
                    }
                }
            }

            #[inline(always)]
            fn ne(&self, other: &Vector<N, T2, S>) -> bool {
                specialize! {
                    ((*self): Vector<N, T, S>, (*other): Vector<N, T2, S>) -> bool:

                    for (Vector<N, T, Simd>, Vector<N, T2, Simd>) -> bool {
                        |vec, other| T::vec_ne(vec, other)
                    } else {
                        self.iter().zip(*other).any(|(a, b)| a != b)
                    }
                }
            }
        }

        impl<const N: usize, T: Scalar + Eq, S: Simdness> Eq for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {}

        impl<const N: usize, T: Scalar + Hash, S: Simdness> Hash for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.as_array().hash(state)
            }
        }

        impl<const N: usize, T: Scalar + Default, S: Simdness> Default for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn default() -> Self {
                Self::splat(T::default())
            }
        }

        impl<const N: usize, T: Scalar + Debug, S: Simdness> Debug for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "(")?;

                for i in 0..N {
                    if i != 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{:?}", self.index(i))?;
                }

                write!(f, ")")?;

                Ok(())
            }
        }

        impl<const N: usize, T: Scalar + Display, S: Simdness> Display for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "(")?;

                for i in 0..N {
                    if i != 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", self.index(i))?;
                }

                write!(f, ")")?;

                Ok(())
            }
        }
    }
    .to_srcdir("vector")
    .with_submod_dir(primitives::srcmod())
    .with_submod_file(constructor::srcmod())
    .with_submod_file(swizzle::srcmod())
    .with_submod_file(scalar::srcmod())
    .with_submod_file(ops::srcmod())
    .with_submod_file(dir::srcmod())
}
