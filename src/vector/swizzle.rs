use super::*;

repetitive! {
    @let ordinals = ["1st", "2nd", "3rd", "4th"];

    // Get
    @for N in 2..=4 {
        @let components = ['x, 'y, 'z, 'w][0..N];

        impl<T: Scalar, A: VecAlignment> Vector<@N, T, A> {
            // Get

            @for x_idx in 0..N {
                @let x = components[x_idx];
                @let c = @str["`" x "`"];

                @let x_ordinal = ordinals[x_idx];
                @let c_ordinals = @str[x_ordinal];

                @let fn_name = x;

                #[doc = @str["Returns the " c " (" c_ordinals ") component of the vector."]]
                #[inline(always)]
                pub const fn @fn_name(self) -> T {
                    self.array[@x_idx]
                }
            }

            @for x_idx in 0..N, y_idx in 0..N {
                @let x = components[x_idx];
                @let y = components[y_idx];
                @let c = @str["`" x "` and `" y "`"];

                @let x_ordinal = ordinals[x_idx];
                @let y_ordinal = ordinals[y_idx];
                @let c_ordinals = @str[x_ordinal " and " y_ordinal];

                @let fn_name = @[x y];

                #[doc = @str["Returns the " c " (" c_ordinals ") components of the vector."]]
                #[inline(always)]
                pub fn @fn_name(self) -> Vector<2, T, A> {
                    T::to_vec2_swizzle::<@N, A, @x_idx, @y_idx>(self)
                }
            }

            @for x_idx in 0..N, y_idx in 0..N, z_idx in 0..N {
                @let x = components[x_idx];
                @let y = components[y_idx];
                @let z = components[z_idx];
                @let c = @str["`" x "`, `" y "` and `" z "`"];

                @let x_ordinal = ordinals[x_idx];
                @let y_ordinal = ordinals[y_idx];
                @let z_ordinal = ordinals[z_idx];
                @let c_ordinals = @str[x_ordinal ", " y_ordinal " and " z_ordinal];

                @let fn_name = @[x y z];

                #[doc = @str["Returns the " c " (" c_ordinals ") components of the vector."]]
                #[inline(always)]
                pub fn @fn_name(self) -> Vector<3, T, A> {
                    T::to_vec3_swizzle::<@N, A, @x_idx, @y_idx, @z_idx>(self)
                }
            }

            @for x_idx in 0..N, y_idx in 0..N, z_idx in 0..N, w_idx in 0..N {
                @let x = components[x_idx];
                @let y = components[y_idx];
                @let z = components[z_idx];
                @let w = components[w_idx];
                @let c = @str["`" x "`, `" y "`, `" z "` and `" w "`"];

                @let x_ordinal = ordinals[x_idx];
                @let y_ordinal = ordinals[y_idx];
                @let z_ordinal = ordinals[z_idx];
                @let w_ordinal = ordinals[w_idx];
                @let c_ordinals = @str[x_ordinal ", " y_ordinal ", " z_ordinal " and " w_ordinal];

                @let fn_name = @[x y z w];

                #[doc = @str["Returns the " c " (" c_ordinals ") components of the vector."]]
                #[inline(always)]
                pub fn @fn_name(self) -> Vector<4, T, A> {
                    T::to_vec4_swizzle::<@N, A, @x_idx, @y_idx, @z_idx, @w_idx>(self)
                }
            }

            // Get Ref

            @for [x_idx, x] in components.enumerate() {
                @let c_name = @str[x];
                @let c_ordinal = @str[ordinals[x_idx]];

                #[doc = @str["Returns a reference to the `" c_name "` (" c_ordinal ") component of the vector."]]
                #[inline(always)]
                pub const fn @[x '_ref](&self) -> &T {
                    &self.array[@x_idx]
                }
            }

            @for NOutput in 2..=N, x_idx in 0..=N - NOutput {
                @let ref_components = components[x_idx..x_idx + NOutput];

                #[doc = @str["Returns a reference to the " ref_components.concat_string() " part of the vector."]]
                #[inline(always)]
                pub const fn @[ref_components.concat_string() '_ref](&self) -> &Vector<@NOutput, T, VecPacked> {
                    unsafe {
                        std::mem::transmute::<
                            &T,
                            &Vector<@NOutput, T, VecPacked>
                        >(&self.array[@x_idx])
                    }
                }
            }

            // Get Mut

            @for
                len_0 in 1..=N,

                idx_0 in 0..=N - len_0,
            {
                @let x = components[idx_0..idx_0 + len_0];
                @let x_type = if len_0 > 1 {
                    @{ &mut Vector<@len_0, T, VecPacked> }
                } else {
                    @{ &mut T }
                };

                @let fn_name = @[x.concat_ident() '_mut];

                @let doc = if len_0 > 1 {
                    "Returns a mutable reference to a sub-vector of the vector."
                } else {
                    "Returns a mutable reference to the component of the vector."
                };

                #[doc = @doc]
                #[inline(always)]
                pub const fn @fn_name(&mut self) -> @x_type {
                    unsafe {
                        std::mem::transmute(&mut self.array[@idx_0])
                    }
                }
            }

            @for
                len_0 in 1..=N,
                len_1 in 1..=N,

                idx_0 in 0..=N - len_0,
                idx_1 in 0..=N - len_1,
            {
                @if
                    (idx_1 >= idx_0 + len_0)
                {
                    @let x = components[idx_0..idx_0 + len_0];
                    @let y = components[idx_1..idx_1 + len_1];

                    @let x_type = if len_0 > 1 {
                        @{ &mut Vector<@len_0, T, VecPacked> }
                    } else {
                        @{ &mut T }
                    };
                    @let y_type = if len_1 > 1 {
                        @{ &mut Vector<@len_1, T, VecPacked> }
                    } else {
                        @{ &mut T }
                    };

                    @let fn_name = @[x.concat_ident() '_ y.concat_ident() '_mut];

                    /// Splits the vector into 2 mutable references.
                    #[inline(always)]
                    pub const fn @fn_name(&mut self) -> (@x_type, @y_type) {
                        unsafe {
                            (
                                std::mem::transmute(&mut self.array[@idx_0]),
                                std::mem::transmute(&mut self.array[@idx_1]),
                            )
                        }
                    }
                }
            }

            @for
                len_0 in 1..=N,
                len_1 in 1..=N,
                len_2 in 1..=N,

                idx_0 in 0..=N - len_0,
                idx_1 in 0..=N - len_1,
                idx_2 in 0..=N - len_2,
            {
                @if
                    (idx_1 >= idx_0 + len_0)
                    && (idx_2 >= idx_1 + len_1)
                {
                    @let x = components[idx_0..idx_0 + len_0];
                    @let y = components[idx_1..idx_1 + len_1];
                    @let z = components[idx_2..idx_2 + len_2];

                    @let x_type = if len_0 > 1 {
                        @{ &mut Vector<@len_0, T, VecPacked> }
                    } else {
                        @{ &mut T }
                    };
                    @let y_type = if len_1 > 1 {
                        @{ &mut Vector<@len_1, T, VecPacked> }
                    } else {
                        @{ &mut T }
                    };
                    @let z_type = if len_2 > 1 {
                        @{ &mut Vector<@len_2, T, VecPacked> }
                    } else {
                        @{ &mut T }
                    };

                    @let fn_name = @[x.concat_ident() '_ y.concat_ident() '_ z.concat_ident() '_mut];

                    /// Splits the vector into 3 mutable references.
                    #[inline(always)]
                    pub const fn @fn_name(&mut self) -> (@x_type, @y_type, @z_type) {
                        unsafe {(
                            std::mem::transmute(&mut self.array[@idx_0]),
                            std::mem::transmute(&mut self.array[@idx_1]),
                            std::mem::transmute(&mut self.array[@idx_2]),
                        )}
                    }
                }
            }

            @for
                len_0 in 1..=N,
                len_1 in 1..=N,
                len_2 in 1..=N,
                len_3 in 1..=N,

                idx_0 in 0..=N - len_0,
                idx_1 in 0..=N - len_1,
                idx_2 in 0..=N - len_2,
                idx_3 in 0..=N - len_3,
            {
                @if
                    (idx_1 >= idx_0 + len_0)
                    && (idx_2 >= idx_1 + len_1)
                    && (idx_3 >= idx_2 + len_2)
                {
                    @let x = components[idx_0..idx_0 + len_0];
                    @let y = components[idx_1..idx_1 + len_1];
                    @let z = components[idx_2..idx_2 + len_2];
                    @let w = components[idx_3..idx_3 + len_3];

                    @let x_type = if len_0 > 1 {
                        @{ &mut Vector<@len_0, T, VecPacked> }
                    } else {
                        @{ &mut T }
                    };
                    @let y_type = if len_1 > 1 {
                        @{ &mut Vector<@len_1, T, VecPacked> }
                    } else {
                        @{ &mut T }
                    };
                    @let z_type = if len_2 > 1 {
                        @{ &mut Vector<@len_2, T, VecPacked> }
                    } else {
                        @{ &mut T }
                    };
                    @let w_type = if len_3 > 1 {
                        @{ &mut Vector<@len_3, T, VecPacked> }
                    } else {
                        @{ &mut T }
                    };

                    @let fn_name = @[x.concat_ident() '_ y.concat_ident() '_ z.concat_ident() '_ w.concat_ident() '_mut];

                    /// Splits the vector into 4 mutable references.
                    #[inline(always)]
                    pub const fn @fn_name(&mut self) -> (@x_type, @y_type, @z_type, @w_type) {
                        unsafe {(
                            std::mem::transmute(&mut self.array[@idx_0]),
                            std::mem::transmute(&mut self.array[@idx_1]),
                            std::mem::transmute(&mut self.array[@idx_2]),
                            std::mem::transmute(&mut self.array[@idx_3]),
                        )}
                    }
                }
            }

            // Set

            @for [x_idx, x] in components.enumerate() {
                @let c_name = @str[x];
                @let c_ordinal = @str[ordinals[x_idx]];

                #[doc = @str["Sets the `" c_name "` (" c_ordinal ") component of the vector."]]
                #[inline(always)]
                pub const fn @['set_ x](&mut self, value: T) {
                    self.array[@x_idx] = value;
                }
            }

            @for
                [x_idx, x] in components.enumerate(),
                [y_idx, y] in components.enumerate(),
            {
                @if x != y {
                    @let c_names = @str[x " and " y];
                    @let c_ordinals = @str[ordinals[x_idx] " and " ordinals[y_idx]];

                    #[doc = @str["Sets the `" c_names "` (" c_ordinals ") components of the vector."]]
                    #[inline(always)]
                    pub const fn @['set_ x y](&mut self, value: Vector<2, T, impl VecAlignment>) {
                        self.@['set_ x](value.x());
                        self.@['set_ y](value.y());
                    }
                }
            }

            @for
                [x_idx, x] in components.enumerate(),
                [y_idx, y] in components.enumerate(),
                [z_idx, z] in components.enumerate()
            {
                @if x != y && x != z && y != z {
                    @let c_names = @str[x ", " y " and " z];
                    @let c_ordinals = @str[ordinals[x_idx] ", " ordinals[y_idx] " and " ordinals[z_idx]];

                    #[doc = @str["Sets the `" c_names "` (" c_ordinals ") components of the vector."]]
                    #[inline(always)]
                    pub const fn @['set_ x y z](&mut self, value: Vector<3, T, impl VecAlignment>) {
                        self.@['set_ x](value.x());
                        self.@['set_ y](value.y());
                        self.@['set_ z](value.z());
                    }
                }
            }

            @for
                [x_idx, x] in components.enumerate(),
                [y_idx, y] in components.enumerate(),
                [z_idx, z] in components.enumerate(),
                [w_idx, w] in components.enumerate()
            {
                @if x != y && x != z && x != w && y != z && y != w && z != w {
                    @let c_names = @str[x ", " y ", " z " and " w];
                    @let c_ordinals = @str[ordinals[x_idx] ", " ordinals[y_idx] ", " ordinals[z_idx] " and " ordinals[w_idx]];

                    #[doc = @str["Sets the `" c_names "` (" c_ordinals ") components of the vector."]]
                    #[inline(always)]
                    pub const fn @['set_ x y z w](&mut self, value: Vector<4, T, impl VecAlignment>) {
                        self.@['set_ x](value.x());
                        self.@['set_ y](value.y());
                        self.@['set_ z](value.z());
                        self.@['set_ w](value.w());
                    }
                }
            }

            // With

            @for [x_idx, x] in components.enumerate() {
                @let c_name = @str[x];
                @let c_ordinal = @str[ordinals[x_idx]];

                #[doc = @str["Returns a new vector with the `" c_name "` (" c_ordinal ") component set to the given value."]]
                #[inline(always)]
                pub const fn @['with_ x](mut self, value: T) -> Self {
                    self.array[@x_idx] = value;

                    self
                }
            }

            @for [x_idx, x] in components.enumerate(), [y_idx, y] in components.enumerate() {
                @if x != y {
                    @let c_names = @str[x " and " y];
                    @let c_ordinals = @str[ordinals[x_idx] " and " ordinals[y_idx]];

                    #[doc = @str["Returns a new vector with the `" c_names "` (" c_ordinals ") components set to the given value."]]
                    #[inline(always)]
                    pub const fn @['with_ x y](mut self, value: Vector<2, T, impl VecAlignment>) -> Self {
                        self.@['set_ x](value.x());
                        self.@['set_ y](value.y());

                        self
                    }
                }
            }

            @for
                [x_idx, x] in components.enumerate(),
                [y_idx, y] in components.enumerate(),
                [z_idx, z] in components.enumerate()
            {
                @if x != y && x != z && y != z {
                    @let c_names = @str[x ", " y " and " z];
                    @let c_ordinals = @str[ordinals[x_idx] ", " ordinals[y_idx] " and " ordinals[z_idx]];

                    #[doc = @str["Returns a new vector with the `" c_names "` (" c_ordinals ") components set to the given value."]]
                    #[inline(always)]
                    pub const fn @['with_ x y z](mut self, value: Vector<3, T, impl VecAlignment>) -> Self {
                        self.@['set_ x](value.x());
                        self.@['set_ y](value.y());
                        self.@['set_ z](value.z());

                        self
                    }
                }
            }

            @for
                [x_idx, x] in components.enumerate(),
                [y_idx, y] in components.enumerate(),
                [z_idx, z] in components.enumerate(),
                [w_idx, w] in components.enumerate()
            {
                @if x != y && x != z && x != w && y != z && y != w && z != w {
                    @let c_names = @str[x ", " y ", " z " and " w];
                    @let c_ordinals = @str[ordinals[x_idx] ", " ordinals[y_idx] ", " ordinals[z_idx] " and " ordinals[w_idx]];

                    #[doc = @str["Returns a new vector with the `" c_names "` (" c_ordinals ") components set to the given value."]]
                    #[inline(always)]
                    pub const fn @['with_ x y z w](mut self, value: Vector<4, T, impl VecAlignment>) -> Self {
                        self.@['set_ x](value.x());
                        self.@['set_ y](value.y());
                        self.@['set_ z](value.z());
                        self.@['set_ w](value.w());

                        self
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swizzle() {
        assert_eq!(vec2!(1, 2).xxy(), vec3!(1, 1, 2));
        assert_eq!(vec3!(1, 2, 3).xyzx(), vec4!(1, 2, 3, 1));
        assert_eq!(vec4!(1, 2, 3, 4).xyzw(), vec4!(1, 2, 3, 4));
        assert_eq!(vec2!(1, 2).y(), 2);

        assert_eq!(vec2p!(false, true).xyx(), vec3p!(false, true, false));
        assert_eq!(vec2p!(false, true).y(), true);
    }

    #[test]
    fn test_swizzle_ref() {
        assert_eq!(vec2!(1, 2).x_ref(), &1);
        assert_eq!(vec3p!(1, 2, 3).yz_ref(), &vec2p!(2, 3));
    }

    #[test]
    fn test_swizzle_mut() {
        assert_eq!(vec2!(1, 2).x_mut(), &mut 1);
        assert_eq!(vec3p!(1, 2, 3).yz_mut(), &mut vec2p!(2, 3));
        assert_eq!(vec4p!(1, 2, 3, 4).xy_z_mut(), (&mut vec2p!(1, 2), &mut 3));
    }

    #[test]
    fn test_swizzle_set() {
        let mut vec = vec2!(1, 2);
        vec.set_x(3);
        assert_eq!(vec, vec2!(3, 2));

        let mut vec = vec3p!(1, 2, 3);
        vec.set_zx(vec2!(4, 5));
        assert_eq!(vec, vec3p!(5, 2, 4));

        let mut vec = vec4p!(1, 2, 3, 4);
        vec.set_xzyw(vec4p!(5, 6, 7, 8));
        assert_eq!(vec, vec4p!(5, 7, 6, 8));
    }

    #[test]
    fn test_swizzle_with() {
        assert_eq!(vec2!(1, 2).with_x(3), vec2!(3, 2));
        assert_eq!(vec3p!(1, 2, 3).with_y(4), vec3p!(1, 4, 3));
        assert_eq!(vec4!(1, 2, 3, 4).with_zx(vec2!(5, 6)), vec4p!(6, 2, 5, 4));
    }
}
