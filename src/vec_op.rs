#[macro_export(local_inner_macros)]
macro_rules! vec_op {
    ($trait:ident, $fn:ident, $self:ident, $($component:ident), *) => {
        impl<T: Element + std::ops::$trait<Output: Element>> std::ops::$trait for $self<T> {
            type Output = $self<T::Output>;
            fn $fn(self) -> <Self as std::ops::$trait>::Output {
                Self::Output::new($(self.$component.$fn()), *)
            }
        }
    };
}
#[macro_export(local_inner_macros)]
macro_rules! vec_rhs_op {
    ($trait:ident, $fn:ident, $self:ident, $($component:ident), *) => {
        impl<T: Element + std::ops::$trait<Output: Element>> std::ops::$trait for $self<T> {
            type Output = $self<T::Output>;
            fn $fn(self, rhs: Self) -> <Self as std::ops::$trait>::Output {
                Self::Output::new($(self.$component.$fn(rhs.$component)), *)
            }
        }
    };
}
#[macro_export(local_inner_macros)]
macro_rules! vec_assign_op {
    ($trait:ident, $fn:ident, $self:ident, $($component:ident), *) => {
        impl<T: Element + std::ops::$trait> std::ops::$trait for $self<T> {
            fn $fn(&mut self, rhs: Self) {
                $(
                    self.$component.$fn(rhs.$component);
                )*
            }
        }
    };
}