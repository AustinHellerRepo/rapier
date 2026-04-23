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

// Profiling macros — #[macro_export] places them at the crate root
//
// The profiling_span! macro supports two forms:
//   1. profiling_span!("name")          — simple span with no context
//   2. profiling_span!("name", type_name = name) — span with a dynamic value
//
// When profiling is disabled, both expand to nothing (zero cost).
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
