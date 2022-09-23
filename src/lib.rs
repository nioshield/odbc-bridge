#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate log;

pub use odbc_api;

pub mod debug;
pub mod executor;
pub mod extension;

pub use debug::print_all_tables;

pub trait Convert<T>: Sized {
    fn convert(self) -> T;
}

pub trait TryConvert<T>: Sized {
    type Error;
    fn try_convert(self) -> Result<T, Self::Error>;
}
