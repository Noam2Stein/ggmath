use crate::{
    self as ggmath,
    element::{default_impl::*, *},
};

type Ty = bool;

impl Element for Ty {}
impl ElementDefaultImpl for Ty {}
