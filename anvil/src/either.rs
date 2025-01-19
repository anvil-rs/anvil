use std::path::Path;

use crate::Anvil;

/// A struct that can be used to combine two Anvil structs
/// and write them into a file. If the first Anvil struct
/// fails to write, the second one will be written instead.
pub struct Either<L, R>
where
    L: Anvil,
    R: Anvil,
{
    left: L,
    right: R,
}

impl<L, R> Anvil for Either<L, R>
where
    L: Anvil,
    R: Anvil,
{
    type Error = R::Error;
    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        self.left.forge(&into).or_else(|_| self.right.forge(&into))
    }
}

impl<L, R> Either<L, R>
where
    L: Anvil,
    R: Anvil,
{
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

pub fn either<L: Anvil, R: Anvil>(left: L, right: R) -> Either<L, R> {
    Either::new(left, right)
}
