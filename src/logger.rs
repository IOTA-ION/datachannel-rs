#[cfg(feature = "log")]
pub use log::debug;
#[cfg(feature = "log")]
pub use log::error;
#[cfg(feature = "log")]
pub use log::info;
#[cfg(feature = "log")]
pub use log::trace;
#[cfg(feature = "log")]
pub use log::warn;
#[cfg(feature = "tracing")]
pub use tracing::debug;
#[cfg(feature = "tracing")]
pub use tracing::error;
#[cfg(feature = "tracing")]
pub use tracing::info;
#[cfg(feature = "tracing")]
pub use tracing::trace;
#[cfg(feature = "tracing")]
pub use tracing::warn;