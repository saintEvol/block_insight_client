//! This crate contains all shared UI for the workspace.

mod navbar;
pub use navbar::Navbar;
pub use workspace::workspace::Workspace;

pub mod echo;
pub mod home;
mod help;
pub mod workspace;
pub mod auth;
pub mod modal;

pub use echo::Echo;
pub use help::Help;
