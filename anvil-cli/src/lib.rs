pub mod filters;

// TODO: Derive proc macro similar to askama for templating the file name based on static
// information.
pub trait Anvil {
    fn path(&self) -> String;
}
