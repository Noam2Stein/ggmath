use super::*;

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    pub fn min(self) -> Vector<N, T, A> {
        match self.resolve() {
            ResolvedRectangle::Centered(rect) => rect.inner.center - rect.inner.extents,
            ResolvedRectangle::Cornered(rect) => rect.inner.min,
            ResolvedRectangle::MinMaxed(rect) => rect.inner.min,
        }
    }

    pub fn max(self) -> Vector<N, T, A> {
        match self.resolve() {
            ResolvedRectangle::Centered(rect) => rect.inner.center + rect.inner.extents,
            ResolvedRectangle::Cornered(rect) => rect.inner.min + rect.inner.size,
            ResolvedRectangle::MinMaxed(rect) => rect.inner.max,
        }
    }

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

    pub fn size(self) -> Vector<N, T, A> {
        match self.resolve() {
            ResolvedRectangle::Centered(rect) => T::rect_mul_vector_by_two(rect.inner.extents),
            ResolvedRectangle::Cornered(rect) => rect.inner.size,
            ResolvedRectangle::MinMaxed(rect) => rect.inner.max - rect.inner.min,
        }
    }

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

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
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
        match self.resolve() {
            ResolvedRectangle::Cornered(_) => self.with_min(self.min() + value),
            ResolvedRectangle::Centered(_) => self.with_center(self.center() + value),
            ResolvedRectangle::MinMaxed(_) => self.with_min(self.min() + value),
        }
    }
    pub fn move_(&mut self, value: Vector<N, T, impl VecAlignment>) {
        *self = self.moved(value);
    }
}

macro_rules! impl_component_fns {
    ($c:ident for $type:ty) => {
        paste::paste! {
            impl<T: RectScalar, A: VecAlignment, R: RectRepr> $type {
                /// Moves the rectangle so that the minimum corner is at the given position,
                /// keeping the size unchanged.
                pub fn [<set_min_ $c>](&mut self, value: T) {
                    self.set_min(self.min().[<with_ $c>](value));
                }

                /// Resizes the rectangle so that the minimum corner is at the given position,
                /// keeping the maximum corner unchanged.
                pub fn [<set_min_ $c _resize>](&mut self, value: T) {
                    self.set_min_resize(self.min().[<with_ $c>](value));
                }

                /// Resizes the rectangle so that the minimum corner is at the given position,
                /// keeping the center unchanged.
                pub fn [<set_min_ $c _centered>](&mut self, value: T) {
                    self.set_min_centered(self.min().[<with_ $c>](value));
                }
            }

            impl<T: RectScalar, A: VecAlignment, R: RectRepr> $type {
                /// Moves the rectangle so that the maximum corner is at the given position,
                /// keeping the size unchanged.
                pub fn [<set_max_ $c>](&mut self, value: T) {
                    self.set_max(self.max().[<with_ $c>](value));
                }

                /// Resizes the rectangle so that the maximum corner is at the given position,
                /// keeping the minimum corner unchanged.
                pub fn [<set_max_ $c _resize>](&mut self, value: T) {
                    self.set_max_resize(self.max().[<with_ $c>](value));
                }

                /// Resizes the rectangle so that the maximum corner is at the given position,
                /// keeping the center unchanged.
                pub fn [<set_max_ $c _centered>](&mut self, value: T) {
                    self.set_max_centered(self.max().[<with_ $c>](value));
                }
            }

            impl<T: RectScalar, A: VecAlignment, R: RectRepr> $type {
                /// Moves the rectangle so that the center is at the given position,
                /// keeping the size unchanged.
                pub fn [<set_center_ $c>](&mut self, value: T) {
                    self.set_center(self.center().[<with_ $c>](value));
                }
            }

            impl<T: RectScalar, A: VecAlignment, R: RectRepr> $type {
                /// Resizes the rectangle to the given size, keeping the center unchanged.
                pub fn [<set_size_ $c _centered>](&mut self, value: T) {
                    self.set_size_centered(self.size().[<with_ $c>](value));
                }

                /// Resizes the rectangle to the given size, keeping the maximum corner unchanged.
                pub fn [<set_size_ $c _minimized>](&mut self, value: T) {
                    self.set_size_minimized(self.size().[<with_ $c>](value));
                }

                /// Resizes the rectangle to the given size, keeping the minimum corner unchanged.
                pub fn [<set_size_ $c _maximized>](&mut self, value: T) {
                    self.set_size_maximized(self.size().[<with_ $c>](value));
                }
            }

            impl<T: RectScalar, A: VecAlignment, R: RectRepr> $type {
                /// Resizes the rectangle to the given extents, keeping the center unchanged.
                pub fn [<set_extents_ $c _centered>](&mut self, value: T) {
                    self.set_extents_centered(self.extents().[<with_ $c>](value));
                }

                /// Resizes the rectangle to the given extents, keeping the maximum corner unchanged.
                pub fn [<set_extents_ $c _minimized>](&mut self, value: T) {
                    self.set_extents_minimized(self.extents().[<with_ $c>](value));
                }

                /// Resizes the rectangle to the given extents, keeping the minimum corner unchanged.
                pub fn [<set_extents_ $c _maximized>](&mut self, value: T) {
                    self.set_extents_maximized(self.extents().[<with_ $c>](value));
                }
            }

            impl<T: RectScalar, A: VecAlignment, R: RectRepr> $type {
                pub fn [<moved_ $c>](self, value: T) -> Self {
                    match self.resolve() {
                        ResolvedRectangle::Cornered(_) => {
                            self.with_min(self.min().with_x(self.min().x() + value))
                        }
                        ResolvedRectangle::Centered(_) => {
                            self.with_center(self.center().with_x(self.center().x() + value))
                        }
                        ResolvedRectangle::MinMaxed(_) => {
                            self.with_min(self.min().with_x(self.min().x() + value))
                        }
                    }
                }

                pub fn [<move_ $c>](&mut self, value: T) {
                    *self = self.[<moved_ $c>](value);
                }
            }
        }
    };
}

impl_component_fns!(x for Rectangle<2, T, A, R>);
impl_component_fns!(y for Rectangle<2, T, A, R>);

impl_component_fns!(x for Rectangle<3, T, A, R>);
impl_component_fns!(y for Rectangle<3, T, A, R>);
impl_component_fns!(z for Rectangle<3, T, A, R>);

impl_component_fns!(x for Rectangle<4, T, A, R>);
impl_component_fns!(y for Rectangle<4, T, A, R>);
impl_component_fns!(z for Rectangle<4, T, A, R>);
impl_component_fns!(w for Rectangle<4, T, A, R>);

impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<2, T, A, R> {
    #[inline(always)]
    pub fn width(&self) -> T {
        self.size().x()
    }

    #[inline(always)]
    pub fn height(&self) -> T {
        self.size().y()
    }
}

impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<3, T, A, R> {
    #[inline(always)]
    pub fn width(&self) -> T {
        self.size().x()
    }

    #[inline(always)]
    pub fn height(&self) -> T {
        self.size().y()
    }

    #[inline(always)]
    pub fn depth(&self) -> T {
        self.size().z()
    }
}

impl<T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<4, T, A, R> {
    #[inline(always)]
    pub fn width(&self) -> T {
        self.size().x()
    }

    #[inline(always)]
    pub fn height(&self) -> T {
        self.size().y()
    }

    #[inline(always)]
    pub fn depth(&self) -> T {
        self.size().z()
    }
}
