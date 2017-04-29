/// TODO: Replace with `std::convert::TryFrom` once stabilized
pub trait TryFrom<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Err;

    /// Performs the conversion.
    fn try_from(T) -> Result<Self, Self::Err>;
}

/// TODO: Replace with `std::convert::TryInto` once stabilized
pub trait TryInto<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Err;

    /// Performs the conversion.
    fn try_into(self) -> Result<T, Self::Err>;
}

impl<T, U> TryInto<U> for T
    where U: TryFrom<T>
{
    type Err = U::Err;

    fn try_into(self) -> Result<U, U::Err> {
        U::try_from(self)
    }
}
