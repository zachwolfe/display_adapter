use std::fmt::{self, Display};

pub use display_adapter_attr::display_adapter;

struct DisplayAdapter<F> {
    f: F
}

impl<F> Display for DisplayAdapter<F> where F: Fn(&mut fmt::Formatter) -> fmt::Result {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        (self.f)(w)
    }
}

pub fn display_adapter_impl<F>(f: F) -> impl Display
    where F: Fn(&mut fmt::Formatter) -> fmt::Result
{
    DisplayAdapter { f }
}
