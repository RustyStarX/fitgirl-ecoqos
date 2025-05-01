#[cfg_attr(feature = "nightly", doc(cfg(feature = "find_thread")))]
#[cfg(feature = "find_thread")]
mod wmi;

#[cfg_attr(feature = "nightly", doc(cfg(feature = "find_thread")))]
#[cfg(feature = "find_thread")]
pub use wmi::{Thread, Threads};

#[cfg_attr(feature = "nightly", doc(cfg(feature = "find_process")))]
#[cfg(feature = "find_process")]
mod process;

#[cfg_attr(feature = "nightly", doc(cfg(feature = "find_process")))]
#[cfg(feature = "find_process")]
pub use process::*;
