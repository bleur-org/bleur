/// Apply closure function preserving self instance
pub trait TryTap: Sized {
    fn try_tap<E>(self, f: impl FnOnce(&Self) -> Result<(), E>) -> Result<Self, E>;
}

impl<T> TryTap for T {
    fn try_tap<E>(self, f: impl FnOnce(&Self) -> Result<(), E>) -> Result<Self, E> {
        f(&self)?;
        Ok(self)
    }
}
