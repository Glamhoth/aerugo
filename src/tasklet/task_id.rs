//! TODO

use core::fmt;

extern crate hash32;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TaskId(pub u32);

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl hash32::Hash for TaskId {
    fn hash<H>(&self, h: &mut H)
    where
        H: hash32::Hasher,
    {
        self.0.hash(h);
    }
}
