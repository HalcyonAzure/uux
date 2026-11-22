#[cfg(target_os = "linux")]
#[path = "platform_linux.rs"]
mod imp;
#[cfg(target_os = "macos")]
#[path = "platform_macos.rs"]
mod imp;
pub use imp::*;
