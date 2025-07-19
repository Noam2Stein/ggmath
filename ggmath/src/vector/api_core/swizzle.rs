use super::*;

macro_loop! {
    @for N in 2..=4 {
        @let components = [x, y, z, w][0..@N];

        impl<T: Scalar, A: VecAlignment> Vector<@N, T, A> {
            // Get

            @for [idx, x] in @components.enumerate() {
                #[inline(always)]
                pub fn @x(self) -> T {
                    self.array[@idx]
                }
            }

            @for x in @components, y in @components {
                #[inline(always)]
                pub fn @[@x @y](self) -> Vector<2, T, A> {
                    Vector::from_array([self.@x(), self.@y()])
                }
            }

            @for x in @components, y in @components, z in @components {
                #[inline(always)]
                pub fn @[@x @y @z](self) -> Vector<3, T, A> {
                    Vector::from_array([self.@x(), self.@y(), self.@z()])
                }
            }

            @for x in @components, y in @components, z in @components, w in @components {
                #[inline(always)]
                pub fn @[@x @y @z @w](self) -> Vector<4, T, A> {
                    Vector::from_array([self.@x(), self.@y(), self.@z(), self.@w()])
                }
            }

            // Get Ref

            @for [idx, x] in @components.enumerate() {
                #[inline(always)]
                pub fn @[@x _ref](&self) -> &T {
                    &self.array[@idx]
                }
            }

            @for NOutput in 2..=@N, idx in 0..=@N - @NOutput {
                @let ref_components = @components[@idx..@idx + @NOutput];

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
                #[inline(always)]
                pub fn @[set_ @x](&mut self, value: T) {
                    self.array[@idx] = value;
                }
            }

            @for x in @components, y in @components {
                @if @x != @y {
                    #[inline(always)]
                    pub fn @[set_ @x @y](&mut self, value: Vector<2, T, A>) {
                        self.@[set_ @x](value.x());
                        self.@[set_ @y](value.y());
                    }
                }
            }

            @for x in @components, y in @components, z in @components {
                @if @x != @y && @x != @z && @y != @z {
                    #[inline(always)]
                    pub fn @[set_ @x @y @z](&mut self, value: Vector<3, T, A>) {
                        self.@[set_ @x](value.x());
                        self.@[set_ @y](value.y());
                        self.@[set_ @z](value.z());
                    }
                }
            }

            @for x in @components, y in @components, z in @components, w in @components {
                @if @x != @y && @x != @z && @x != @w && @y != @z && @y != @w && @z != @w {
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

            @for [idx, x] in @components.enumerate() {
                #[inline(always)]
                pub fn @[with_ @x](mut self, value: T) -> Self {
                    self.array[@idx] = value;

                    self
                }
            }

            @for x in @components, y in @components {
                @if @x != @y {
                    #[inline(always)]
                    pub fn @[with_ @x @y](mut self, value: Vector<2, T, A>) -> Self {
                        self.@[set_ @x](value.x());
                        self.@[set_ @y](value.y());

                        self
                    }
                }
            }

            @for x in @components, y in @components, z in @components {
                @if @x != @y && @x != @z && @y != @z {
                    #[inline(always)]
                    pub fn @[with_ @x @y @z](mut self, value: Vector<3, T, A>) -> Self {
                        self.@[set_ @x](value.x());
                        self.@[set_ @y](value.y());
                        self.@[set_ @z](value.z());

                        self
                    }
                }
            }

            @for x in @components, y in @components, z in @components, w in @components {
                @if @x != @y && @x != @z && @x != @w && @y != @z && @y != @w && @z != @w {
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
