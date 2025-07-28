use super::*;

macro_loop! {
    @let names = ["1st", "2nd", "3rd", "4th"];

    @for N in 2..=4 {
        @let components = [x, y, z, w][0..@N];

        impl<T: Scalar, A: VecAlignment> Vector<@N, T, A> {
            // Get

            @for [x_idx, x] in @components.enumerate() {
                #[doc = @("Returns the `" + @x + "` (" + @names[@x_idx] + ") component of the vector.")]
                #[inline(always)]
                pub fn @x(self) -> T {
                    self.array[@x_idx]
                }
            }

            @for [x_idx, x] in @components.enumerate(), [y_idx, y] in @components.enumerate() {
                #[doc = @(
                    "Returns the `"
                    + @x
                    + "` and `"
                    + @y
                    + "` ("
                    + @names[@x_idx]
                    + " and "
                    + @names[@y_idx]
                    + ") components of the vector."
                )]
                #[inline(always)]
                pub fn @[@x @y](self) -> Vector<2, T, A> {
                    Vector::from_array([self.@x(), self.@y()])
                }
            }

            @for
                [x_idx, x] in @components.enumerate(),
                [y_idx, y] in @components.enumerate(),
                [z_idx, z] in @components.enumerate()
            {
                #[doc = @(
                    "Returns the `"
                    + @x
                    + "` and `"
                    + @y
                    + "` and `"
                    + @z
                    + "` ("
                    + @names[@x_idx]
                    + " and "
                    + @names[@y_idx]
                    + " and "
                    + @names[@z_idx]
                    + ") components of the vector."
                )]
                #[inline(always)]
                pub fn @[@x @y @z](self) -> Vector<3, T, A> {
                    Vector::from_array([self.@x(), self.@y(), self.@z()])
                }
            }

            @for
                [x_idx, x] in @components.enumerate(),
                [y_idx, y] in @components.enumerate(),
                [z_idx, z] in @components.enumerate(),
                [w_idx, w] in @components.enumerate()
            {
                #[doc = @(
                    "Returns the `"
                    + @x
                    + "` and `"
                    + @y
                    + "` and `"
                    + @z
                    + "` and `"
                    + @w
                    + "` ("
                    + @names[@x_idx]
                    + " and "
                    + @names[@y_idx]
                    + " and "
                    + @names[@z_idx]
                    + " and "
                    + @names[@w_idx]
                    + ") components of the vector."
                )]
                #[inline(always)]
                pub fn @[@x @y @z @w](self) -> Vector<4, T, A> {
                    Vector::from_array([self.@x(), self.@y(), self.@z(), self.@w()])
                }

            }

            // Get Ref

            @for [idx, x] in @components.enumerate() {
                #[doc = @("Returns a reference to the `" + @x + "` (" + @names[@idx] + ") component of the vector.")]
                #[inline(always)]
                pub fn @[@x _ref](&self) -> &T {
                    &self.array[@idx]
                }
            }

            @for NOutput in 2..=@N, idx in 0..=@N - @NOutput {
                @let ref_components = @components[@idx..@idx + @NOutput];

                #[doc = "Returns a contiguous reference to a sub-vector of the vector."]
                #[inline(always)]
                pub fn @[@ref_components _ref](&self) -> &Vector<@NOutput, T, VecPacked> {
                    unsafe {
                        std::mem::transmute::<
                            &T,
                            &Vector<@NOutput, T, VecPacked>
                        >(&self.array[@idx])
                    }
                }
            }

            // Get Mut

            @for
                len_0 in 1..=@N,

                idx_0 in 0..=@N - @len_0
            {
                @let x = @components[@idx_0..@idx_0 + @len_0];

                @if @len_0 > 1 { #[doc = @("Returns a mutable reference to a sub-vector of the vector.")] }
                @if @len_0 == 1 { #[doc = @("Returns a mutable reference to the component of the vector.")] }
                #[inline(always)]
                pub fn @[@x _mut](&mut self)
                @if @len_0 > 1 { -> &mut Vector<@len_0, T, VecPacked> }
                @if @len_0 == 1 { -> &mut T }
                {
                    unsafe {
                        std::mem::transmute(&mut self.array[@idx_0])
                    }
                }
            }

            @for
                len_0 in 1..=@N,
                len_1 in 1..=@N,

                idx_0 in 0..=@N - @len_0,
                idx_1 in 0..=@N - @len_1
            {
                @if
                    (@idx_1 >= @idx_0 + @len_0)
                {
                    @let x = @components[@idx_0..@idx_0 + @len_0];
                    @let y = @components[@idx_1..@idx_1 + @len_1];

                    /// Splits the vector into 2 mutable references.
                    #[inline(always)]
                    pub fn @[@x _ @y _mut](&mut self) -> (
                        @if @len_0 > 1 { &mut Vector<@len_0, T, VecPacked> }
                        @if @len_0 == 1 { &mut T },

                        @if @len_1 > 1 { &mut Vector<@len_1, T, VecPacked> }
                        @if @len_1 == 1 { &mut T },
                    ) {
                        unsafe {
                            (
                                std::mem::transmute(&mut self.array[@idx_0]),
                                std::mem::transmute(&mut self.array[@idx_1])
                            )
                        }
                    }
                }
            }

            @for
                len_0 in 1..=@N,
                len_1 in 1..=@N,
                len_2 in 1..=@N,

                idx_0 in 0..=@N - @len_0,
                idx_1 in 0..=@N - @len_1,
                idx_2 in 0..=@N - @len_2
            {
                @if
                    (@idx_1 >= @idx_0 + @len_0)
                    && (@idx_2 >= @idx_1 + @len_1)
                {
                    @let x = @components[@idx_0..@idx_0 + @len_0];
                    @let y = @components[@idx_1..@idx_1 + @len_1];
                    @let z = @components[@idx_2..@idx_2 + @len_2];

                    /// Splits the vector into 3 mutable references.
                    #[inline(always)]
                    pub fn @[@x _ @y _ @z _mut](&mut self) -> (
                        @if @len_0 > 1 { &mut Vector<@len_0, T, VecPacked> }
                        @if @len_0 == 1 { &mut T },

                        @if @len_1 > 1 { &mut Vector<@len_1, T, VecPacked> }
                        @if @len_1 == 1 { &mut T },

                        @if @len_2 > 1 { &mut Vector<@len_2, T, VecPacked> }
                        @if @len_2 == 1 { &mut T },
                    ) {
                        unsafe {
                            (
                                std::mem::transmute(&mut self.array[@idx_0]),
                                std::mem::transmute(&mut self.array[@idx_1]),
                                std::mem::transmute(&mut self.array[@idx_2]),
                            )
                        }
                    }
                }
            }

            @for
                len_0 in 1..=@N,
                len_1 in 1..=@N,
                len_2 in 1..=@N,
                len_3 in 1..=@N,

                idx_0 in 0..=@N - @len_0,
                idx_1 in 0..=@N - @len_1,
                idx_2 in 0..=@N - @len_2,
                idx_3 in 0..=@N - @len_3
            {
                @if
                    (@idx_1 >= @idx_0 + @len_0)
                    && (@idx_2 >= @idx_1 + @len_1)
                    && (@idx_3 >= @idx_2 + @len_2)
                {
                    @let x = @components[@idx_0..@idx_0 + @len_0];
                    @let y = @components[@idx_1..@idx_1 + @len_1];
                    @let z = @components[@idx_2..@idx_2 + @len_2];
                    @let w = @components[@idx_3..@idx_3 + @len_3];

                    /// Splits the vector into 4 mutable references.
                    #[inline(always)]
                    pub fn @[@x _ @y _ @z _ @w _mut](&mut self) -> (
                        @if @len_0 > 1 { &mut Vector<@len_0, T, VecPacked> }
                        @if @len_0 == 1 { &mut T },

                        @if @len_1 > 1 { &mut Vector<@len_1, T, VecPacked> }
                        @if @len_1 == 1 { &mut T },

                        @if @len_2 > 1 { &mut Vector<@len_2, T, VecPacked> }
                        @if @len_2 == 1 { &mut T },

                        @if @len_3 > 1 { &mut Vector<@len_3, T, VecPacked> }
                        @if @len_3 == 1 { &mut T },
                    ) {
                        unsafe {
                            (
                                std::mem::transmute(&mut self.array[@idx_0]),
                                std::mem::transmute(&mut self.array[@idx_1]),
                                std::mem::transmute(&mut self.array[@idx_2]),
                                std::mem::transmute(&mut self.array[@idx_3]),
                            )
                        }
                    }
                }
            }

            // Set

            @for [idx, x] in @components.enumerate() {
                #[doc = @("Sets the `" + @x + "` (" + @names[@idx] + ") component of the vector.")]
                #[inline(always)]
                pub fn @[set_ @x](&mut self, value: T) {
                    self.array[@idx] = value;
                }
            }

            @for [x_idx, x] in @components.enumerate(), [y_idx, y] in @components.enumerate() {
                @if @x != @y {
                    #[doc = @(
                        "Sets the `"
                        + @x
                        + "` and `"
                        + @y
                        + "` ("
                        + @names[@x_idx]
                        + " and "
                        + @names[@y_idx]
                        + ") components of the vector."
                    )]
                    #[inline(always)]
                    pub fn @[set_ @x @y](&mut self, value: Vector<2, T, A>) {
                        self.@[set_ @x](value.x());
                        self.@[set_ @y](value.y());
                    }
                }
            }

            @for
                [x_idx, x] in @components.enumerate(),
                [y_idx, y] in @components.enumerate(),
                [z_idx, z] in @components.enumerate()
            {
                @if @x != @y && @x != @z && @y != @z {
                    #[doc = @(
                        "Sets the `"
                        + @x
                        + "` and `"
                        + @y
                        + "` and `"
                        + @z
                        + "` ("
                        + @names[@x_idx]
                        + " and "
                        + @names[@y_idx]
                        + " and "
                        + @names[@z_idx]
                        + ") components of the vector."
                    )]
                    #[inline(always)]
                    pub fn @[set_ @x @y @z](&mut self, value: Vector<3, T, A>) {
                        self.@[set_ @x](value.x());
                        self.@[set_ @y](value.y());
                        self.@[set_ @z](value.z());
                    }
                }
            }

            @for
                [x_idx, x] in @components.enumerate(),
                [y_idx, y] in @components.enumerate(),
                [z_idx, z] in @components.enumerate(),
                [w_idx, w] in @components.enumerate()
            {
                @if @x != @y && @x != @z && @x != @w && @y != @z && @y != @w && @z != @w {
                    #[doc = @(
                        "Sets the `"
                        + @x
                        + "` and `"
                        + @y
                        + "` and `"
                        + @z
                        + "` and `"
                        + @w
                        + "` ("
                        + @names[@x_idx]
                        + " and "
                        + @names[@y_idx]
                        + " and "
                        + @names[@z_idx]
                        + " and "
                        + @names[@w_idx]
                        + ") components of the vector."
                    )]
                    #[inline(always)]
                    pub fn @[set_ @x @y @z @w](&mut self, value: Vector<4, T, A>) {
                        self.@[set_ @x](value.x());
                        self.@[set_ @y](value.y());
                        self.@[set_ @z](value.z());
                        self.@[set_ @w](value.w());
                    }
                }
            }

            // With

            @for [x_idx, x] in @components.enumerate() {
                #[doc = @("Returns a new vector with the `" + @x + "` (" + @names[@x_idx] + ") component set to the given value.")]
                #[inline(always)]
                pub fn @[with_ @x](mut self, value: T) -> Self {
                    self.array[@x_idx] = value;

                    self
                }
            }

            @for [x_idx, x] in @components.enumerate(), [y_idx, y] in @components.enumerate() {
                @if @x != @y {
                    #[doc = @(
                        "Returns a new vector with the `"
                        + @x
                        + "` and `"
                        + @y
                        + "` ("
                        + @names[@x_idx]
                        + " and "
                        + @names[@y_idx]
                        + ") components set to the given value."
                    )]
                    #[inline(always)]
                    pub fn @[with_ @x @y](mut self, value: Vector<2, T, A>) -> Self {
                        self.@[set_ @x](value.x());
                        self.@[set_ @y](value.y());

                        self
                    }
                }
            }

            @for
                [x_idx, x] in @components.enumerate(),
                [y_idx, y] in @components.enumerate(),
                [z_idx, z] in @components.enumerate()
            {
                @if @x != @y && @x != @z && @y != @z {
                    #[doc = @(
                        "Returns a new vector with the `"
                        + @x
                        + "` and `"
                        + @y
                        + "` and `"
                        + @z
                        + "` ("
                        + @names[@x_idx]
                        + " and "
                        + @names[@y_idx]
                        + " and "
                        + @names[@z_idx]
                        + ") components set to the given value."
                    )]
                    #[inline(always)]
                    pub fn @[with_ @x @y @z](mut self, value: Vector<3, T, A>) -> Self {
                        self.@[set_ @x](value.x());
                        self.@[set_ @y](value.y());
                        self.@[set_ @z](value.z());

                        self
                    }
                }
            }

            @for
                [x_idx, x] in @components.enumerate(),
                [y_idx, y] in @components.enumerate(),
                [z_idx, z] in @components.enumerate(),
                [w_idx, w] in @components.enumerate()
            {
                @if @x != @y && @x != @z && @x != @w && @y != @z && @y != @w && @z != @w {
                    #[doc = @(
                        "Returns a new vector with the `"
                        + @x
                        + "` and `"
                        + @y
                        + "` and `"
                        + @z
                        + "` and `"
                        + @w
                        + "` ("
                        + @names[@x_idx]
                        + " and "
                        + @names[@y_idx]
                        + " and "
                        + @names[@z_idx]
                        + " and "
                        + @names[@w_idx]
                        + ") components set to the given value."
                    )]
                    #[inline(always)]
                    pub fn @[with_ @x @y @z @w](mut self, value: Vector<4, T, A>) -> Self {
                        self.@[set_ @x](value.x());
                        self.@[set_ @y](value.y());
                        self.@[set_ @z](value.z());
                        self.@[set_ @w](value.w());

                        self
                    }
                }
            }
        }
    }
}
