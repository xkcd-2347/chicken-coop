//! Pages in the console

use yew_nested_router::Target;

mod index;
mod package;
mod vulnerability;

pub use index::*;

#[derive(Clone, Debug, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[target(index)]
    Index,
    Package,
    Vulnerability,
}
