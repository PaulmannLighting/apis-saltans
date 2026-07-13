use zb_core::Direction;

/// Trait for commands that have a single direction when sent.
pub trait Directed {
    /// The direction to use when sending the command.
    const DIRECTION: Direction;
}

impl<T> Directed for Box<T>
where
    T: Directed,
{
    const DIRECTION: Direction = T::DIRECTION;
}
