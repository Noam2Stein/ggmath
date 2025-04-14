use super::*;

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    pub fn min(self) -> Vector<N, T, A> {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => rect.inner.center - rect.inner.extents,
            ReprResolvedRectangle::Cornered(rect) => rect.inner.min,
            ReprResolvedRectangle::MinMaxed(rect) => rect.inner.min,
        }
    }
    pub fn max(self) -> Vector<N, T, A> {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => rect.inner.center + rect.inner.extents,
            ReprResolvedRectangle::Cornered(rect) => rect.inner.min + rect.inner.size,
            ReprResolvedRectangle::MinMaxed(rect) => rect.inner.max,
        }
    }
    pub fn center(self) -> Vector<N, T, A> {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => rect.inner.center,
            ReprResolvedRectangle::Cornered(rect) => {
                rect.inner.min + T::rect_div_vector_by_two(rect.inner.size)
            }
            ReprResolvedRectangle::MinMaxed(rect) => {
                T::rect_div_vector_by_two(rect.inner.min + rect.inner.max)
            }
        }
    }

    pub fn size(self) -> Vector<N, T, A> {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => T::rect_mul_vector_by_two(rect.inner.extents),
            ReprResolvedRectangle::Cornered(rect) => rect.inner.size,
            ReprResolvedRectangle::MinMaxed(rect) => rect.inner.max - rect.inner.min,
        }
    }
    pub fn extents(self) -> Vector<N, T, A> {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => rect.inner.extents,
            ReprResolvedRectangle::Cornered(rect) => T::rect_div_vector_by_two(rect.inner.size),
            ReprResolvedRectangle::MinMaxed(rect) => {
                T::rect_div_vector_by_two(rect.inner.max - rect.inner.min)
            }
        }
    }

    /// returns `self` but moved so that the minimum corner is at the given position,
    /// keeping the size unchanged.
    pub fn with_min(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_size(value, self.size())
    }
    /// returns `self` but moved so that the maximum corner is at the given position,
    /// keeping the size unchanged.
    pub fn with_max(self, value: Vector<N, T, A>) -> Self {
        Self::from_max_size(value, self.size())
    }
    /// returns `self` but moved so that the center is at the given position,
    /// keeping the size unchanged.
    pub fn with_center(self, value: Vector<N, T, A>) -> Self {
        Self::from_center_size(value, self.size())
    }
    /// returns `self` but resized so that the minimum corner is at the given position,
    /// keeping the maximum corner unchanged.
    pub fn with_min_resize(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_max(value, self.max())
    }
    /// returns `self` but resized so that the maximum corner is at the given position,
    /// keeping the minimum corner unchanged.
    pub fn with_max_resize(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_max(self.min(), value)
    }
    /// returns `self` but resized so that the minimum corner is at the given position,
    /// keeping the center unchanged.
    pub fn with_min_centered(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_center(value, self.center())
    }
    /// returns `self` but resized so that the maximum corner is at the given position,
    /// keeping the center unchanged.
    pub fn with_max_centered(self, value: Vector<N, T, A>) -> Self {
        Self::from_center_max(self.center(), value)
    }

    /// returns `self` but resized to the given size, keeping the center unchanged.
    pub fn with_size_centered(self, value: Vector<N, T, A>) -> Self {
        Self::from_center_size(self.center(), value)
    }
    /// returns `self` but resized to the given extents, keeping the center unchanged.
    pub fn with_extents_centered(self, value: Vector<N, T, A>) -> Self {
        Self::from_center_extents(self.center(), value)
    }
    /// returns `self` but resized to the given size, keeping the maximum corner unchanged.
    pub fn with_size_minimized(self, value: Vector<N, T, A>) -> Self {
        Self::from_max_size(self.max(), value)
    }
    /// returns `self` but resized to the given extents, keeping the maximum corner unchanged.
    pub fn with_extents_minimized(self, value: Vector<N, T, A>) -> Self {
        Self::from_max_extents(self.max(), value)
    }
    /// returns `self` but resized to the given size, keeping the minimum corner unchanged.
    pub fn with_size_maximized(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_size(self.min(), value)
    }
    /// returns `self` but resized to the given extents, keeping the minimum corner unchanged.
    pub fn with_extents_maximized(self, value: Vector<N, T, A>) -> Self {
        Self::from_min_extents(self.min(), value)
    }

    /// Moves the rectangle so that the minimum corner is at the given position,
    /// keeping the size unchanged.
    pub fn set_min(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_min_size(value, self.size());
    }
    /// Moves the rectangle so that the maximum corner is at the given position,
    /// keeping the size unchanged.
    pub fn set_max(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_max_size(value, self.size());
    }
    /// Moves the rectangle so that the center is at the given position,
    /// keeping the size unchanged.
    pub fn set_center(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_center_size(value, self.size());
    }
    /// Resizes the rectangle so that the minimum corner is at the given position,
    /// keeping the maximum corner unchanged.
    pub fn set_min_resize(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_min_max(value, self.max());
    }
    /// Resizes the rectangle so that the maximum corner is at the given position,
    /// keeping the minimum corner unchanged.
    pub fn set_max_resize(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_min_max(self.min(), value);
    }
    /// Resizes the rectangle so that the minimum corner is at the given position,
    /// keeping the center unchanged.
    pub fn set_min_centered(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_min_center(value, self.center());
    }
    /// Resizes the rectangle so that the maximum corner is at the given position,
    /// keeping the center unchanged.
    pub fn set_max_centered(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_center_max(self.center(), value);
    }

    /// Resizes the rectangle to the given size, keeping the center unchanged.
    pub fn set_size_centered(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_center_size(self.center(), value);
    }
    /// Resizes the rectangle to the given extents, keeping the center unchanged.
    pub fn set_extents_centered(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_center_extents(self.center(), value);
    }
    /// Resizes the rectangle to the given size, keeping the maximum corner unchanged.
    pub fn set_size_minimized(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_max_size(self.max(), value);
    }
    /// Resizes the rectangle to the given extents, keeping the maximum corner unchanged.
    pub fn set_extents_minimized(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_max_extents(self.max(), value);
    }
    /// Resizes the rectangle to the given size, keeping the minimum corner unchanged.
    pub fn set_size_maximized(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_min_size(self.min(), value);
    }
    /// Resizes the rectangle to the given extents, keeping the minimum corner unchanged.
    pub fn set_extents_maximized(&mut self, value: Vector<N, T, A>) {
        *self = Self::from_min_extents(self.min(), value);
    }

    pub fn moved(self, value: Vector<N, T, impl VecAlignment>) -> Self {
        match self.resolve_repr() {
            ReprResolvedRectangle::Cornered(_) => self.with_min(self.min() + value),
            ReprResolvedRectangle::Centered(_) => self.with_center(self.center() + value),
            ReprResolvedRectangle::MinMaxed(_) => self.with_min(self.min() + value),
        }
    }
    pub fn move_(&mut self, value: Vector<N, T, impl VecAlignment>) {
        *self = self.moved(value);
    }
}

macro_rules! x_fns {
    () => {
        /// Moves the rectangle so that the minimum corner is at the given position,
        /// keeping the size unchanged.
        pub fn set_min_x(&mut self, value: T) {
            self.set_min(self.min().with_x(value));
        }
        /// Moves the rectangle so that the maximum corner is at the given position,
        /// keeping the size unchanged.
        pub fn set_max_x(&mut self, value: T) {
            self.set_max(self.max().with_x(value));
        }
        /// Moves the rectangle so that the center is at the given position,
        /// keeping the size unchanged.
        pub fn set_center_x(&mut self, value: T) {
            self.set_center(self.center().with_x(value));
        }
        /// Resizes the rectangle so that the minimum corner is at the given position,
        /// keeping the maximum corner unchanged.
        pub fn set_min_x_resize(&mut self, value: T) {
            self.set_min_resize(self.min().with_x(value));
        }
        /// Resizes the rectangle so that the maximum corner is at the given position,
        /// keeping the minimum corner unchanged.
        pub fn set_max_x_resize(&mut self, value: T) {
            self.set_max_resize(self.max().with_x(value));
        }
        /// Resizes the rectangle so that the minimum corner is at the given position,
        /// keeping the center unchanged.
        pub fn set_min_x_centered(&mut self, value: T) {
            self.set_min_centered(self.min().with_x(value));
        }
        /// Resizes the rectangle so that the maximum corner is at the given position,
        /// keeping the center unchanged.
        pub fn set_max_x_centered(&mut self, value: T) {
            self.set_max_centered(self.max().with_x(value));
        }

        /// Resizes the rectangle to the given size, keeping the center unchanged.
        pub fn set_size_x_centered(&mut self, value: T) {
            self.set_size_centered(self.size().with_x(value));
        }
        /// Resizes the rectangle to the given extents, keeping the center unchanged.
        pub fn set_extents_x_centered(&mut self, value: T) {
            self.set_extents_centered(self.extents().with_x(value));
        }
        /// Resizes the rectangle to the given size, keeping the maximum corner unchanged.
        pub fn set_size_x_minimized(&mut self, value: T) {
            self.set_size_minimized(self.size().with_x(value));
        }
        /// Resizes the rectangle to the given extents, keeping the maximum corner unchanged.
        pub fn set_extents_x_minimized(&mut self, value: T) {
            self.set_extents_minimized(self.extents().with_x(value));
        }
        /// Resizes the rectangle to the given size, keeping the minimum corner unchanged.
        pub fn set_size_x_maximized(&mut self, value: T) {
            self.set_size_maximized(self.size().with_x(value));
        }
        /// Resizes the rectangle to the given extents, keeping the minimum corner unchanged.
        pub fn set_extents_x_maximized(&mut self, value: T) {
            self.set_extents_maximized(self.extents().with_x(value));
        }

        pub fn moved_x(self, value: T) -> Self {
            match self.resolve_repr() {
                ReprResolvedRectangle::Cornered(_) => {
                    self.with_min(self.min().with_x(self.min().x() + value))
                }
                ReprResolvedRectangle::Centered(_) => {
                    self.with_center(self.center().with_x(self.center().x() + value))
                }
                ReprResolvedRectangle::MinMaxed(_) => {
                    self.with_min(self.min().with_x(self.min().x() + value))
                }
            }
        }
        pub fn move_x(&mut self, value: T) {
            *self = self.moved_x(value);
        }
    };
}
macro_rules! y_fns {
    () => {
        /// Moves the rectangle so that the minimum corner is at the given position,
        /// keeping the size unchanged.
        pub fn set_min_y(&mut self, value: T) {
            self.set_min(self.min().with_y(value));
        }
        /// Moves the rectangle so that the maximum corner is at the given position,
        /// keeping the size unchanged.
        pub fn set_max_y(&mut self, value: T) {
            self.set_max(self.max().with_y(value));
        }
        /// Moves the rectangle so that the center is at the given position,
        /// keeping the size unchanged.
        pub fn set_center_y(&mut self, value: T) {
            self.set_center(self.center().with_y(value));
        }
        /// Resizes the rectangle so that the minimum corner is at the given position,
        /// keeping the maximum corner unchanged.
        pub fn set_min_y_resize(&mut self, value: T) {
            self.set_min_resize(self.min().with_y(value));
        }
        /// Resizes the rectangle so that the maximum corner is at the given position,
        /// keeping the minimum corner unchanged.
        pub fn set_max_y_resize(&mut self, value: T) {
            self.set_max_resize(self.max().with_y(value));
        }
        /// Resizes the rectangle so that the minimum corner is at the given position,
        /// keeping the center unchanged.
        pub fn set_min_y_centered(&mut self, value: T) {
            self.set_min_centered(self.min().with_y(value));
        }
        /// Resizes the rectangle so that the maximum corner is at the given position,
        /// keeping the center unchanged.
        pub fn set_max_y_centered(&mut self, value: T) {
            self.set_max_centered(self.max().with_y(value));
        }

        /// Resizes the rectangle to the given size, keeping the center unchanged.
        pub fn set_size_y_centered(&mut self, value: T) {
            self.set_size_centered(self.size().with_y(value));
        }
        /// Resizes the rectangle to the given extents, keeping the center unchanged.
        pub fn set_extents_y_centered(&mut self, value: T) {
            self.set_extents_centered(self.extents().with_y(value));
        }
        /// Resizes the rectangle to the given size, keeping the maximum corner unchanged.
        pub fn set_size_y_minimized(&mut self, value: T) {
            self.set_size_minimized(self.size().with_y(value));
        }
        /// Resizes the rectangle to the given extents, keeping the maximum corner unchanged.
        pub fn set_extents_y_minimized(&mut self, value: T) {
            self.set_extents_minimized(self.extents().with_y(value));
        }
        /// Resizes the rectangle to the given size, keeping the minimum corner unchanged.
        pub fn set_size_y_maximized(&mut self, value: T) {
            self.set_size_maximized(self.size().with_y(value));
        }
        /// Resizes the rectangle to the given extents, keeping the minimum corner unchanged.
        pub fn set_extents_y_maximized(&mut self, value: T) {
            self.set_extents_maximized(self.extents().with_y(value));
        }

        pub fn moved_y(self, value: T) -> Self {
            match self.resolve_repr() {
                ReprResolvedRectangle::Cornered(_) => {
                    self.with_min(self.min().with_y(self.min().y() + value))
                }
                ReprResolvedRectangle::Centered(_) => {
                    self.with_center(self.center().with_y(self.center().y() + value))
                }
                ReprResolvedRectangle::MinMaxed(_) => {
                    self.with_min(self.min().with_y(self.min().y() + value))
                }
            }
        }
        pub fn move_y(&mut self, value: T) {
            *self = self.moved_y(value);
        }
    };
}
macro_rules! z_fns {
    () => {
        /// Moves the rectangle so that the minimum corner is at the given position,
        /// keeping the size unchanged.
        pub fn set_min_z(&mut self, value: T) {
            self.set_min(self.min().with_z(value));
        }
        /// Moves the rectangle so that the maximum corner is at the given position,
        /// keeping the size unchanged.
        pub fn set_max_z(&mut self, value: T) {
            self.set_max(self.max().with_z(value));
        }
        /// Moves the rectangle so that the center is at the given position,
        /// keeping the size unchanged.
        pub fn set_center_z(&mut self, value: T) {
            self.set_center(self.center().with_z(value));
        }
        /// Resizes the rectangle so that the minimum corner is at the given position,
        /// keeping the maximum corner unchanged.
        pub fn set_min_z_resize(&mut self, value: T) {
            self.set_min_resize(self.min().with_z(value));
        }
        /// Resizes the rectangle so that the maximum corner is at the given position,
        /// keeping the minimum corner unchanged.
        pub fn set_max_z_resize(&mut self, value: T) {
            self.set_max_resize(self.max().with_z(value));
        }
        /// Resizes the rectangle so that the minimum corner is at the given position,
        /// keeping the center unchanged.
        pub fn set_min_z_centered(&mut self, value: T) {
            self.set_min_centered(self.min().with_z(value));
        }
        /// Resizes the rectangle so that the maximum corner is at the given position,
        /// keeping the center unchanged.
        pub fn set_max_z_centered(&mut self, value: T) {
            self.set_max_centered(self.max().with_z(value));
        }

        /// Resizes the rectangle to the given size, keeping the center unchanged.
        pub fn set_size_z_centered(&mut self, value: T) {
            self.set_size_centered(self.size().with_z(value));
        }
        /// Resizes the rectangle to the given extents, keeping the center unchanged.
        pub fn set_extents_z_centered(&mut self, value: T) {
            self.set_extents_centered(self.extents().with_z(value));
        }
        /// Resizes the rectangle to the given size, keeping the maximum corner unchanged.
        pub fn set_size_z_minimized(&mut self, value: T) {
            self.set_size_minimized(self.size().with_z(value));
        }
        /// Resizes the rectangle to the given extents, keeping the maximum corner unchanged.
        pub fn set_extents_z_minimized(&mut self, value: T) {
            self.set_extents_minimized(self.extents().with_z(value));
        }
        /// Resizes the rectangle to the given size, keeping the minimum corner unchanged.
        pub fn set_size_z_maximized(&mut self, value: T) {
            self.set_size_maximized(self.size().with_z(value));
        }
        /// Resizes the rectangle to the given extents, keeping the minimum corner unchanged.
        pub fn set_extents_z_maximized(&mut self, value: T) {
            self.set_extents_maximized(self.extents().with_z(value));
        }

        pub fn moved_z(self, value: T) -> Self {
            match self.resolve_repr() {
                ReprResolvedRectangle::Cornered(_) => {
                    self.with_min(self.min().with_z(self.min().z() + value))
                }
                ReprResolvedRectangle::Centered(_) => {
                    self.with_center(self.center().with_z(self.center().z() + value))
                }
                ReprResolvedRectangle::MinMaxed(_) => {
                    self.with_min(self.min().with_z(self.min().z() + value))
                }
            }
        }
        pub fn move_z(&mut self, value: T) {
            *self = self.moved_z(value);
        }
    };
}
macro_rules! w_fns {
    () => {
        /// Moves the rectangle so that the minimum corner is at the given position,
        /// keeping the size unchanged.
        pub fn set_min_w(&mut self, value: T) {
            self.set_min(self.min().with_w(value));
        }
        /// Moves the rectangle so that the maximum corner is at the given position,
        /// keeping the size unchanged.
        pub fn set_max_w(&mut self, value: T) {
            self.set_max(self.max().with_w(value));
        }
        /// Moves the rectangle so that the center is at the given position,
        /// keeping the size unchanged.
        pub fn set_center_w(&mut self, value: T) {
            self.set_center(self.center().with_w(value));
        }
        /// Resizes the rectangle so that the minimum corner is at the given position,
        /// keeping the maximum corner unchanged.
        pub fn set_min_w_resize(&mut self, value: T) {
            self.set_min_resize(self.min().with_w(value));
        }
        /// Resizes the rectangle so that the maximum corner is at the given position,
        /// keeping the minimum corner unchanged.
        pub fn set_max_w_resize(&mut self, value: T) {
            self.set_max_resize(self.max().with_w(value));
        }
        /// Resizes the rectangle so that the minimum corner is at the given position,
        /// keeping the center unchanged.
        pub fn set_min_w_centered(&mut self, value: T) {
            self.set_min_centered(self.min().with_w(value));
        }
        /// Resizes the rectangle so that the maximum corner is at the given position,
        /// keeping the center unchanged.
        pub fn set_max_w_centered(&mut self, value: T) {
            self.set_max_centered(self.max().with_w(value));
        }

        /// Resizes the rectangle to the given size, keeping the center unchanged.
        pub fn set_size_w_centered(&mut self, value: T) {
            self.set_size_centered(self.size().with_w(value));
        }
        /// Resizes the rectangle to the given extents, keeping the center unchanged.
        pub fn set_extents_w_centered(&mut self, value: T) {
            self.set_extents_centered(self.extents().with_w(value));
        }
        /// Resizes the rectangle to the given size, keeping the maximum corner unchanged.
        pub fn set_size_w_minimized(&mut self, value: T) {
            self.set_size_minimized(self.size().with_w(value));
        }
        /// Resizes the rectangle to the given extents, keeping the maximum corner unchanged.
        pub fn set_extents_w_minimized(&mut self, value: T) {
            self.set_extents_minimized(self.extents().with_w(value));
        }
        /// Resizes the rectangle to the given size, keeping the minimum corner unchanged.
        pub fn set_size_w_maximized(&mut self, value: T) {
            self.set_size_maximized(self.size().with_w(value));
        }
        /// Resizes the rectangle to the given extents, keeping the minimum corner unchanged.
        pub fn set_extents_w_maximized(&mut self, value: T) {
            self.set_extents_maximized(self.extents().with_w(value));
        }

        pub fn moved_w(self, value: T) -> Self {
            match self.resolve_repr() {
                ReprResolvedRectangle::Cornered(_) => {
                    self.with_min(self.min().with_w(self.min().w() + value))
                }
                ReprResolvedRectangle::Centered(_) => {
                    self.with_center(self.center().with_w(self.center().w() + value))
                }
                ReprResolvedRectangle::MinMaxed(_) => {
                    self.with_min(self.min().with_w(self.min().w() + value))
                }
            }
        }
        pub fn move_w(&mut self, value: T) {
            *self = self.moved_w(value);
        }
    };
}

impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<2, T, A, R>
where
    ScalarCount<2>: VecLen,
{
    x_fns!();
    y_fns!();
}

impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<3, T, A, R>
where
    ScalarCount<3>: VecLen,
{
    x_fns!();
    y_fns!();
    z_fns!();
}

impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<4, T, A, R>
where
    ScalarCount<4>: VecLen,
{
    x_fns!();
    y_fns!();
    z_fns!();
    w_fns!();
}
