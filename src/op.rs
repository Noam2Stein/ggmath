#[macro_export(local_inner_macros)]
macro_rules! ops {
    { $self:ident $components:tt, [$(($trait:ident, $fn:ident)), * $(,)?] $(,)? } => {
        $(
            op! {
                $self $components, ($trait, $fn)
            }
        )*
    };
}
#[macro_export(local_inner_macros)]
macro_rules! rhs_ops {
    { $self:ident $components:tt, [$(($trait:ident, $fn:ident)), * $(,)?] $(,)? } => {
        $(
            rhs_op! {
                $self $components, ($trait, $fn)
            }
        )*
    };
}
#[macro_export(local_inner_macros)]
macro_rules! assign_ops {
    { $self:ident $components:tt, [$(($trait:ident, $fn:ident)), * $(,)?] $(,)? } => {
        $(
            assign_op! {
                $self $components, ($trait, $fn)
            }
        )*
    };
}
#[macro_export(local_inner_macros)]
macro_rules! op {
    { $self:ident { $($component:ident), * $(,)? }, ($trait:ident, $fn:ident) $(,)? } => {
        impl<T> $trait for $self<T>
        where
            T: Element + $trait<Output: Element>,
        {
            type Output = $self<T::Output>;

            #[inline(always)]
            fn $fn(self) -> <Self as $trait>::Output {
                Self::Output::new($(self.$component.$fn()), *)
            }
        }
    };
}
#[macro_export(local_inner_macros)]
macro_rules! rhs_op {
    { $self:ident { $($component:ident), * $(,)? }, ($trait:ident, $fn:ident) $(,)? } => {
        impl<T, Rhs> $trait<$self<Rhs>> for $self<T>
        where
            T: Element + $trait<Rhs, Output: Element>,
            Rhs: Element,
        {
            type Output = $self<T::Output>;
    
            #[inline(always)]
            fn $fn(self, rhs: $self<Rhs>) -> <Self as $trait<$self<Rhs>>>::Output {
                Self::Output::new($(self.$component.$fn(rhs.$component)), *)
            }
        }
    };
}
#[macro_export(local_inner_macros)]
macro_rules! assign_op {
    { $self:ident { $($component:ident), * $(,)? }, ($trait:ident, $fn:ident) $(,)? } => {
        impl<T, Rhs> $trait<$self<Rhs>> for $self<T>
        where
            T: Element + $trait<Rhs>,
            Rhs: Element,
        {
            #[inline(always)]
            fn $fn(&mut self, rhs: $self<Rhs>) {
                $(
                    self.$component.$fn(rhs.$component);
                )*
            }
        }
    };
}