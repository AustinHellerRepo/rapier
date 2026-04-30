/// CPU profiling support for bevy_rapier.
///
/// When the `profiling` feature is enabled, this module re-exports
/// `info_span` from `tracing` so that the `profiling_span!` macro
/// can use `tracing::info_span`.
///
/// When the feature is **disabled** (the default), this module is a
/// no-op — `profiling_span!` expands to `{{}}` and has zero runtime cost.
///
/// # Enabling
///
/// Build with `--features profiling`:
///
/// ```text
/// cargo check --features profiling
/// ```

#[cfg(feature = "profiling")]
pub use tracing::info_span;

/// No-op fallback for when profiling is disabled.
#[cfg(not(feature = "profiling"))]
pub fn info_span(_name: &str) -> NoopSpan {
    NoopSpan
}

/// A no-op span used when profiling is disabled.
#[cfg(not(feature = "profiling"))]
pub struct NoopSpan;

#[cfg(not(feature = "profiling"))]
impl NoopSpan {
    #[allow(unused_variables)]
    pub fn entered(self) -> NoopEntered {
        NoopEntered
    }
}

/// A no-op guard used when profiling is disabled.
#[cfg(not(feature = "profiling"))]
pub struct NoopEntered;

#[cfg(not(feature = "profiling"))]
impl Drop for NoopEntered {
    fn drop(&mut self) {}
}

// Profiling macro — #[macro_export] places it at the crate root.
// Always defined; the profiling_span! macro supports two forms:
//   1. profiling_span!("name")          — simple span with no context
//   2. profiling_span!("name", type_name = name) — span with a dynamic value
#[cfg(feature = "profiling")]
#[macro_export]
macro_rules! profiling_span {
    ($name:expr) => {
        let _span = $crate::profiling::info_span!($name).entered();
    };
    ($name:expr, $( $key:ident = $value:expr ),* $(,)?) => {
        let _span = $crate::profiling::info_span!($name, $( $key = $value ),*).entered();
    };
}

#[cfg(not(feature = "profiling"))]
#[macro_export]
macro_rules! profiling_span {
    ($name:expr) => {{}};
    ($name:expr, $( $key:ident = $value:expr ),* $(,)?) => {{}};
}
