#![feature(lazy_cell)]
#![warn(clippy::all, rust_2018_idioms)]

pub mod list_possible_versions;

mod app;
pub use app::App;
