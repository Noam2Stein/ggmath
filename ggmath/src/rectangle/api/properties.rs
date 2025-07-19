use macro_loop::macro_loop;

use super::*;

// Get

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    pub fn min(self) -> Vector<N, T, A> {
        match self.resolve() {
            ResolvedRectangle::Centered(rect) => rect.inner.center - rect.inner.extents,
            ResolvedRectangle::Cornered(rect) => rect.inner.min,
            ResolvedRectangle::MinMaxed(rect) => rect.inner.min,
        }
    }

    #[inline(always)]
    pub fn max(self) -> Vector<N, T, A> {
        match self.resolve() {
            ResolvedRectangle::Centered(rect) => rect.inner.center + rect.inner.extents,
            ResolvedRectangle::Cornered(rect) => rect.inner.min + rect.inner.size,
            ResolvedRectangle::MinMaxed(rect) => rect.inner.max,
        }
    }

    #[inline(always)]
    pub fn center(self) -> Vector<N, T, A> {
        match self.resolve() {
            ResolvedRectangle::Centered(rect) => rect.inner.center,
            ResolvedRectangle::Cornered(rect) => {
                rect.inner.min + T::rect_div_vector_by_two(rect.inner.size)
            }
            ResolvedRectangle::MinMaxed(rect) => {
                T::rect_div_vector_by_two(rect.inner.min + rect.inner.max)
            }
        }
    }

    #[inline(always)]
    pub fn size(self) -> Vector<N, T, A> {
        match self.resolve() {
            ResolvedRectangle::Centered(rect) => T::rect_mul_vector_by_two(rect.inner.extents),
            ResolvedRectangle::Cornered(rect) => rect.inner.size,
            ResolvedRectangle::MinMaxed(rect) => rect.inner.max - rect.inner.min,
        }
    }

    #[inline(always)]
    pub fn extents(self) -> Vector<N, T, A> {
        match self.resolve() {
            ResolvedRectangle::Centered(rect) => rect.inner.extents,
            ResolvedRectangle::Cornered(rect) => T::rect_div_vector_by_two(rect.inner.size),
            ResolvedRectangle::MinMaxed(rect) => {
                T::rect_div_vector_by_two(rect.inner.max - rect.inner.min)
            }
        }
    }
}

// With

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    /// returns `self` but moved so that the minimum corner is at the given position,
    /// keeping the size unchanged.
    #[inline(always)]
    pub fn with_min(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_size(value, self.size())
    }

    /// returns `self` but resized so that the minimum corner is at the given position,
    /// keeping the maximum corner unchanged.
    #[inline(always)]
    pub fn with_min_resize(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_max(value, self.max())
    }

    /// returns `self` but resized so that the minimum corner is at the given position,
    /// keeping the center unchanged.
    #[inline(always)]
    pub fn with_min_centered(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_center(value, self.center())
    }
}

macro_loop! {
    @for N in 2..=4, x in [x, y, z, w][0..@N] {
        impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<@N, T, A, R> {
            /// returns `self` but moved so that the minimum corner is at the given position,
            /// keeping the size unchanged.
            #[inline(always)]
            pub fn @[with_min_ @x](self, value: T) -> Self {
                self.with_min(self.min().@[with_ @x](value))
            }

            /// returns `self` but resized so that the minimum corner is at the given position,
            /// keeping the maximum corner unchanged.
            #[inline(always)]
            pub fn @[with_min_ @x _resize](self, value: T) -> Self {
                self.with_min_resize(self.min().@[with_ @x](value))
            }

            /// returns `self` but resized so that the minimum corner is at the given position,
            /// keeping the center unchanged.
            #[inline(always)]
            pub fn @[with_min_ @x _centered](self, value: T) -> Self {
                self.with_min_centered(self.min().@[with_ @x](value))
            }
        }
    }
}

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    /// returns `self` but moved so that the maximum corner is at the given position,
    /// keeping the size unchanged.
    #[inline(always)]
    pub fn with_max(self, value: Vector<N, T, A>) -> Self {
        Self::from_max_size(value, self.size())
    }

    /// returns `self` but resized so that the maximum corner is at the given position,
    /// keeping the minimum corner unchanged.
    #[inline(always)]
    pub fn with_max_resize(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_max(self.min(), value)
    }

    /// returns `self` but resized so that the maximum corner is at the given position,
    /// keeping the center unchanged.
    #[inline(always)]
    pub fn with_max_centered(self, value: Vector<N, T, A>) -> Self {
        Self::from_center_max(self.center(), value)
    }
}

macro_loop! {
    @for N in 2..=4, x in [x, y, z, w][0..@N] {
        impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<@N, T, A, R> {
            /// returns `self` but moved so that the maximum corner is at the given position,
            /// keeping the size unchanged.
            #[inline(always)]
            pub fn @[with_max_ @x](self, value: T) -> Self {
                self.with_max(self.max().@[with_ @x](value))
            }

            /// returns `self` but resized so that the maximum corner is at the given position,
            /// keeping the minimum corner unchanged.
            #[inline(always)]
            pub fn @[with_max_ @x _resize](self, value: T) -> Self {
                self.with_max_resize(self.max().@[with_ @x](value))
            }

            /// returns `self` but resized so that the maximum corner is at the given position,
            /// keeping the center unchanged.
            #[inline(always)]
            pub fn @[with_max_ @x _centered](self, value: T) -> Self {
                self.with_max_centered(self.max().@[with_ @x](value))
            }
        }
    }
}

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    /// returns `self` but moved so that the center is at the given position,
    /// keeping the size unchanged.
    #[inline(always)]
    pub fn with_center(self, value: Vector<N, T, A>) -> Self {
        Self::from_center_size(value, self.size())
    }

    /// returns `self` but resized so that the center is at the given position,
    /// keeping the maximum corner unchanged.
    #[inline(always)]

    pub fn with_center_minimized(self, value: Vector<N, T, A>) -> Self {
        Self::from_center_max(value, self.max())
    }

    /// returns `self` but resized so that the center is at the given position,
    /// keeping the minimum corner unchanged.
    #[inline(always)]
    pub fn with_center_maximized(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_center(self.min(), value)
    }

    #[inline(always)]
    pub fn moved(self, value: Vector<N, T, impl VecAlignment>) -> Self {
        match self.resolve() {
            ResolvedRectangle::Cornered(_) => self.with_min(self.min() + value),
            ResolvedRectangle::Centered(_) => self.with_center(self.center() + value),
            ResolvedRectangle::MinMaxed(_) => self.with_min(self.min() + value),
        }
    }
}

macro_loop! {
    @for N in 2..=4, x in [x, y, z, w][0..@N] {
        impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<@N, T, A, R> {
            /// returns `self` but moved so that the center is at the given position,
            /// keeping the size unchanged.
            #[inline(always)]
            pub fn @[with_center_ @x](self, value: T) -> Self {
                self.with_center(self.center().@[with_ @x](value))
            }

            /// returns `self` but resized so that the center is at the given position,
            /// keeping the maximum corner unchanged.
            #[inline(always)]
            pub fn @[with_center_ @x _minimized](self, value: T) -> Self {
                self.with_center_minimized(self.center().@[with_ @x](value))
            }

            /// returns `self` but resized so that the center is at the given position,
            /// keeping the minimum corner unchanged.
            #[inline(always)]
            pub fn @[with_center_ @x _maximized](self, value: T) -> Self {
                self.with_center_maximized(self.center().@[with_ @x](value))
            }

            #[inline(always)]
            pub fn @[moved_ @x](self, value: T) -> Self {
                match self.resolve() {
                    ResolvedRectangle::Cornered(_) => self.@[with_min_ @x](self.min().@x() + value),
                    ResolvedRectangle::Centered(_) => self.@[with_center_ @x](self.center().@x() + value),
                    ResolvedRectangle::MinMaxed(_) => self.@[with_min_ @x](self.min().@x() + value),
                }
            }
        }
    }
}

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    /// returns `self` but resized to the given size, keeping the center unchanged.
    #[inline(always)]
    pub fn resized_centered(self, value: Vector<N, T, A>) -> Self {
        Self::from_center_size(self.center(), value)
    }

    /// returns `self` but resized to the given size, keeping the maximum corner unchanged.
    #[inline(always)]
    pub fn resized_minimized(self, value: Vector<N, T, A>) -> Self {
        Self::from_max_size(self.max(), value)
    }

    /// returns `self` but resized to the given size, keeping the minimum corner unchanged.
    #[inline(always)]
    pub fn resized_maximized(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_size(self.min(), value)
    }
}

macro_loop! {
    @for N in 2..=4, x in [x, y, z, w][0..@N] {
        impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<@N, T, A, R> {
            /// returns `self` but resized to the given size, keeping the center unchanged.
            #[inline(always)]
            pub fn @[resized_ @x _centered](self, value: T) -> Self {
                self.resized_centered(self.size().@[with_ @x](value))
            }

            /// returns `self` but resized to the given size, keeping the maximum corner unchanged.
            #[inline(always)]
            pub fn @[resized_ @x _minimized](self, value: T) -> Self {
                self.resized_minimized(self.size().@[with_ @x](value))
            }

            /// returns `self` but resized to the given size, keeping the minimum corner unchanged.
            #[inline(always)]
            pub fn @[resized_ @x _maximized](self, value: T) -> Self {
                self.resized_maximized(self.size().@[with_ @x](value))
            }
        }
    }
}

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    /// returns `self` but resized to the given extents, keeping the center unchanged.
    #[inline(always)]
    pub fn with_extents_centered(self, value: Vector<N, T, A>) -> Self {
        Self::from_center_extents(self.center(), value)
    }

    /// returns `self` but resized to the given extents, keeping the maximum corner unchanged.
    #[inline(always)]
    pub fn with_extents_minimized(self, value: Vector<N, T, A>) -> Self {
        Self::from_max_extents(self.max(), value)
    }

    /// returns `self` but resized to the given extents, keeping the minimum corner unchanged.
    #[inline(always)]
    pub fn with_extents_maximized(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_extents(self.min(), value)
    }
}

macro_loop! {
    @for N in 2..=4, x in [x, y, z, w][0..@N] {
        impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<@N, T, A, R> {
            /// returns `self` but resized to the given extents, keeping the center unchanged.
            #[inline(always)]
            pub fn @[with_extents_ @x _centered](self, value: T) -> Self {
                self.with_extents_centered(self.extents().@[with_ @x](value))
            }

            /// returns `self` but resized to the given extents, keeping the maximum corner unchanged.
            #[inline(always)]
            pub fn @[with_extents_ @x _minimized](self, value: T) -> Self {
                self.with_extents_minimized(self.extents().@[with_ @x](value))
            }

            /// returns `self` but resized to the given extents, keeping the minimum corner unchanged.
            #[inline(always)]
            pub fn @[with_extents_ @x _maximized](self, value: T) -> Self {
                self.with_extents_maximized(self.extents().@[with_ @x](value))
            }
        }
    }
}

// Set

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    /// Moves the rectangle so that the minimum corner is at the given position,
    /// keeping the size unchanged.
    #[inline(always)]
    pub fn set_min(&mut self, value: Vector<N, T, A>) {
        *self = self.with_min(value);
    }

    /// Resizes the rectangle so that the minimum corner is at the given position,
    /// keeping the maximum corner unchanged.
    #[inline(always)]
    pub fn set_min_resize(&mut self, value: Vector<N, T, A>) {
        *self = self.with_min_resize(value);
    }

    /// Resizes the rectangle so that the minimum corner is at the given position,
    /// keeping the center unchanged.
    #[inline(always)]
    pub fn set_min_centered(&mut self, value: Vector<N, T, A>) {
        *self = self.with_min_centered(value);
    }
}

macro_loop! {
    @for N in 2..=4, x in [x, y, z, w][0..@N] {
        impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<@N, T, A, R> {
            /// Moves the rectangle so that the minimum corner is at the given position,
            /// keeping the size unchanged.
            #[inline(always)]
            pub fn @[set_min_ @x](&mut self, value: T) {
                *self = self.@[with_min_ @x](value);
            }

            /// Resizes the rectangle so that the minimum corner is at the given position,
            /// keeping the maximum corner unchanged.
            #[inline(always)]
            pub fn @[set_min_ @x _resize](&mut self, value: T) {
                *self = self.@[with_min_ @x _resize](value);
            }

            /// Resizes the rectangle so that the minimum corner is at the given position,
            /// keeping the center unchanged.
            #[inline(always)]
            pub fn @[set_min_ @x _centered](&mut self, value: T) {
                *self = self.@[with_min_ @x _centered](value);
            }
        }
    }
}

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    /// Moves the rectangle so that the maximum corner is at the given position,
    /// keeping the size unchanged.
    #[inline(always)]
    pub fn set_max(&mut self, value: Vector<N, T, A>) {
        *self = self.with_max(value);
    }

    /// Resizes the rectangle so that the maximum corner is at the given position,
    /// keeping the minimum corner unchanged.
    #[inline(always)]
    pub fn set_max_resize(&mut self, value: Vector<N, T, A>) {
        *self = self.with_max_resize(value);
    }

    /// Resizes the rectangle so that the maximum corner is at the given position,
    /// keeping the center unchanged.
    #[inline(always)]
    pub fn set_max_centered(&mut self, value: Vector<N, T, A>) {
        *self = self.with_max_centered(value);
    }
}

macro_loop! {
    @for N in 2..=4, x in [x, y, z, w][0..@N] {
        impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<@N, T, A, R> {
            /// Moves the rectangle so that the maximum corner is at the given position,
            /// keeping the size unchanged.
            #[inline(always)]
            pub fn @[set_max_ @x](&mut self, value: T) {
                *self = self.@[with_max_ @x](value);
            }

            /// Resizes the rectangle so that the maximum corner is at the given position,
            /// keeping the minimum corner unchanged.
            #[inline(always)]
            pub fn @[set_max_ @x _resize](&mut self, value: T) {
                *self = self.@[with_max_ @x _resize](value);
            }

            /// Resizes the rectangle so that the maximum corner is at the given position,
            /// keeping the center unchanged.
            #[inline(always)]
            pub fn @[set_max_ @x _centered](&mut self, value: T) {
                *self = self.@[with_max_ @x _centered](value);
            }
        }
    }
}

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    /// Moves the rectangle so that the center is at the given position,
    /// keeping the size unchanged.
    #[inline(always)]
    pub fn set_center(&mut self, value: Vector<N, T, A>) {
        *self = self.with_center(value);
    }

    /// Moves the rectangle so that the center is at the given position,
    /// keeping the maximum unchanged.
    #[inline(always)]
    pub fn set_center_minimized(&mut self, value: Vector<N, T, A>) {
        *self = self.with_center_minimized(value);
    }

    /// Moves the rectangle so that the center is at the given position,
    /// keeping the minimum unchanged.
    #[inline(always)]
    pub fn set_center_maximized(&mut self, value: Vector<N, T, A>) {
        *self = self.with_center_maximized(value);
    }

    #[inline(always)]
    pub fn move_(&mut self, value: Vector<N, T, impl VecAlignment>) {
        *self = self.moved(value);
    }
}

macro_loop! {
    @for N in 2..=4, x in [x, y, z, w][0..@N] {
        impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<@N, T, A, R> {
            /// Moves the rectangle so that the center is at the given position,
            /// keeping the size unchanged.
            #[inline(always)]
            pub fn @[set_center_ @x](&mut self, value: T) {
                *self = self.@[with_center_ @x](value);
            }

            /// Moves the rectangle so that the center is at the given position,
            /// keeping the maximum unchanged.
            #[inline(always)]
            pub fn @[set_center_ @x _minimized](&mut self, value: T) {
                *self = self.@[with_center_ @x _minimized](value);
            }

            /// Moves the rectangle so that the center is at the given position,
            /// keeping the minimum unchanged.
            #[inline(always)]
            pub fn @[set_center_ @x _maximized](&mut self, value: T) {
                *self = self.@[with_center_ @x _maximized](value);
            }

            #[inline(always)]
            pub fn @[move_ @x](&mut self, value: T) {
                *self = self.@[moved_ @x](value);
            }
        }
    }
}

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    /// Resizes the rectangle to the given size, keeping the center unchanged.
    #[inline(always)]
    pub fn resize_centered(&mut self, value: Vector<N, T, A>) {
        *self = self.resized_centered(value);
    }

    /// Resizes the rectangle to the given size, keeping the maximum corner unchanged.
    #[inline(always)]
    pub fn resize_minimized(&mut self, value: Vector<N, T, A>) {
        *self = self.resized_minimized(value);
    }

    /// Resizes the rectangle to the given size, keeping the minimum corner unchanged.
    #[inline(always)]
    pub fn resize_maximized(&mut self, value: Vector<N, T, A>) {
        *self = self.resized_maximized(value);
    }
}

macro_loop! {
    @for N in 2..=4, x in [x, y, z, w][0..@N] {
        impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<@N, T, A, R> {
            /// Resizes the rectangle to the given size, keeping the center unchanged.
            #[inline(always)]
            pub fn @[resize_ @x _centered](&mut self, value: T) {
                *self = self.@[resized_ @x _centered](value);
            }

            /// Resizes the rectangle to the given size, keeping the maximum corner unchanged.
            #[inline(always)]
            pub fn @[resize_ @x _minimized](&mut self, value: T) {
                *self = self.@[resized_ @x _minimized](value);
            }

            /// Resizes the rectangle to the given size, keeping the minimum corner unchanged.
            #[inline(always)]
            pub fn @[resize_ @x _maximized](&mut self, value: T) {
                *self = self.@[resized_ @x _maximized](value);
            }
        }
    }
}

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    /// Resizes the rectangle to the given extents, keeping the center unchanged.
    #[inline(always)]
    pub fn set_extents_centered(&mut self, value: Vector<N, T, A>) {
        *self = self.with_extents_centered(value);
    }

    /// Resizes the rectangle to the given extents, keeping the maximum corner unchanged.
    #[inline(always)]
    pub fn set_extents_minimized(&mut self, value: Vector<N, T, A>) {
        *self = self.with_extents_minimized(value);
    }

    /// Resizes the rectangle to the given extents, keeping the minimum corner unchanged.
    #[inline(always)]
    pub fn set_extents_maximized(&mut self, value: Vector<N, T, A>) {
        *self = self.with_extents_maximized(value);
    }
}

macro_loop! {
    @for N in 2..=4, x in [x, y, z, w][0..@N] {
        impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<@N, T, A, R> {
            /// Resizes the rectangle to the given extents, keeping the center unchanged.
            #[inline(always)]
            pub fn @[set_extents_ @x _centered](&mut self, value: T) {
                *self = self.@[with_extents_ @x _centered](value);
            }

            /// Resizes the rectangle to the given extents, keeping the maximum corner unchanged.
            #[inline(always)]
            pub fn @[set_extents_ @x _minimized](&mut self, value: T) {
                *self = self.@[with_extents_ @x _minimized](value);
            }

            /// Resizes the rectangle to the given extents, keeping the minimum corner unchanged.
            #[inline(always)]
            pub fn @[set_extents_ @x _maximized](&mut self, value: T) {
                *self = self.@[with_extents_ @x _maximized](value);
            }
        }
    }
}

// Width Height...

macro_loop! {
    @for N in 2..=4, [x, x_word] in [[x, width], [y, height], [z, depth]][0..@N.min(3)] {
        impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<@N, T, A, R> {
            // Get

            #[inline(always)]
            pub fn @x_word(&self) -> T {
                self.size().@x()
            }

            // With

            #[inline(always)]
            pub fn @[with_ @x_word _centered](self, value: T) -> Self {
                self.@[resized_ @x _centered](value)
            }

            #[inline(always)]
            pub fn @[with_ @x_word _minimized](self, value: T) -> Self {
                self.@[resized_ @x _minimized](value)
            }

            #[inline(always)]
            pub fn @[with_ @x_word _maximized](self, value: T) -> Self {
                self.@[resized_ @x _maximized](value)
            }

            // Set

            #[inline(always)]
            pub fn @[set_ @x_word _centered](&mut self, value: T) {
                self.@[resize_ @x _centered](value)
            }

            #[inline(always)]
            pub fn @[set_ @x_word _minimized](&mut self, value: T)  {
                self.@[resize_ @x _minimized](value)
            }

            #[inline(always)]
            pub fn @[set_ @x_word _maximized](&mut self, value: T)  {
                self.@[resize_ @x _maximized](value)
            }
        }
    }
}
