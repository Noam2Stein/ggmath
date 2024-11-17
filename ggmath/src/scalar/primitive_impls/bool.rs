use super::*;

inner_vecs!(bool(1));

impl Scalar for bool {}

impl ScalarDefault for bool {}
impl ScalarPartialEq<bool> for bool {}
impl ScalarPartialOrd for bool {}

impl ScalarNot for bool {}
impl ScalarBitAnd<bool> for bool {}
impl ScalarBitOr<bool> for bool {}
impl ScalarBitXor<bool> for bool {}
