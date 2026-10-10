//! Read-only integration with the native rldyour-sysinfo owner.

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
mod adapter;
pub mod view;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
pub use adapter::SystemInfo;
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
pub use nddev_device_sync_adapter_io::{CancellationToken, IoError, LocalIo, NativeIo};

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    #[error(transparent)]
    Io(#[from] IoError),
    #[error("sysinfo protocol version is unsupported")]
    UnsupportedVersion,
    #[error("sysinfo sample is invalid")]
    InvalidSample,
    #[error("native sysinfo provider is not supported on this platform")]
    UnsupportedPlatform,
}

/// Application port: callers depend on a typed sample, not a native socket.
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
pub trait ReadSystemInfo {
    fn sample(
        &self,
        cancellation: CancellationToken,
    ) -> impl std::future::Future<Output = Result<view::SystemSnapshot, Error>> + Send;
}

/// Canonical compiled-in runtime manifest for this integration.
pub const MANIFEST: &str = include_str!("../contracts/module.json");
