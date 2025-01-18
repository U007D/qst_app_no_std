// Configure a stable `Never` type which will automatically become `!` if/whenever
// `#[feature(never_type)]` stabilizes.

#[cfg(feature = "never_type")]
pub type Never = !;

#[cfg(not(feature = "never_type"))]
pub enum Never {}
