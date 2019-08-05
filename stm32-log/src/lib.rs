#![no_std]

#[cfg(not(feature = "target-selected"))]
compile_error!("This crate requires one of the following device features enabled:
        nucleo-f429zi");

#[cfg(feature = "nucleo-f429zi")]
mod nucleo_f429zi;
#[cfg(feature = "nucleo-f429zi")]
pub use nucleo_f429zi::configure;
#[cfg(feature = "nucleo-f429zi")]
pub(crate) use nucleo_f429zi::write_bytes;

#[cfg(feature = "target-selected")]
mod log;
