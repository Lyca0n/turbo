use error::Error;

pub mod application;
pub mod cmd;
pub mod common;
pub mod config;
pub mod util;
pub mod error;
pub mod renderer;

pub const DEFAULT_NAME: &str = "my-project";
pub const VERSION: &str = "V0.0.1";

pub type Result<T> = core::result::Result<T, Error>;