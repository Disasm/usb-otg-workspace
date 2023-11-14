#![no_std]

#[cfg(not(feature = "target-selected"))]
compile_error!(
    "This crate requires one of the following device features enabled:
        nucleo-f429zi
        nucleo-f446ze"
);

#[cfg(any(feature = "nucleo-f429zi", feature = "nucleo-f446ze"))]
mod nucleo_f429zi;
#[cfg(any(feature = "nucleo-f429zi", feature = "nucleo-f446ze"))]
pub use nucleo_f429zi::configure;
#[cfg(any(feature = "nucleo-f429zi", feature = "nucleo-f446ze"))]
pub(crate) use nucleo_f429zi::write_bytes;

#[cfg(feature = "target-selected")]
mod log;
