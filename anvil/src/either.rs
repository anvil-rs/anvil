use std::path::Path;

use crate::Forge;

/// A struct that can be used to combine two Anvil structs
/// and write them into a file. If the first Anvil struct
/// fails to write, the second one will be written instead.
pub struct Either<L: Forge, R: Forge> {
    left: L,
    right: R,
}

impl<L: Forge, R: Forge> Forge for Either<L, R> {
    type Error = R::Error;
    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        self.left.forge(&into).or_else(|_| self.right.forge(&into))
    }
}

impl<L: Forge, R: Forge> Either<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

pub fn either<L: Forge, R: Forge>(left: L, right: R) -> Either<L, R> {
    Either::new(left, right)
}
