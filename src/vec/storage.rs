use super::*;

#[allow(private_bounds)]
pub trait VecStorage: Seal + inner::VecStorageInnerVecs + VecStorageApi {}

pub struct VecPacked;
impl Seal for VecPacked {}
impl VecStorage for VecPacked {}

pub struct VecAligned;
impl Seal for VecAligned {}
impl VecStorage for VecAligned {}

trait Seal {}
