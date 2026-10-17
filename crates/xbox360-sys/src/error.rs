use core::fmt;

/// A value that no variant of the enum it was converted to has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnknownValue<T> {
    pub value: T,
}

impl<T> fmt::Display for UnknownValue<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value {:?} has no variant", self.value)
    }
}

impl<T> core::error::Error for UnknownValue<T> where T: fmt::Debug {}
