#![doc(
    html_logo_url = "https://raw.githubusercontent.com/Xuanwo/backon/main/.github/assets/logo.jpeg"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! BackON &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![](https://img.shields.io/discord/1111711408875393035?logo=discord&label=discord)](https://discord.gg/8ARnvtJePD)
//!
//! [Build Status]: https://img.shields.io/github/actions/workflow/status/Xuanwo/backon/ci.yml?branch=main
//! [actions]: https://github.com/Xuanwo/backon/actions?query=branch%3Amain
//! [Latest Version]: https://img.shields.io/crates/v/backon.svg
//! [crates.io]: https://crates.io/crates/backon
//!
//! <img src="https://raw.githubusercontent.com/Xuanwo/backon/main/.github/assets/logo.jpeg" alt="BackON" width="38.2%"/>
//!
//! Make **retry** like a built-in feature provided by Rust.
//!
//! - **Simple**: Just like a built-in feature: `your_fn.retry(ExponentialBuilder::default()).await`.
//! - **Flexible**: Supports both blocking and async functions.
//! - **Powerful**: Allows control over retry behavior such as [`when`](https://docs.rs/backon/latest/backon/struct.Retry.html#method.when) and [`notify`](https://docs.rs/backon/latest/backon/struct.Retry.html#method.notify).
//! - **Customizable**: Supports custom retry strategies like [exponential](https://docs.rs/backon/latest/backon/struct.ExponentialBuilder.html), [constant](https://docs.rs/backon/latest/backon/struct.ConstantBuilder.html), etc.
//!
//! # Backoff
//!
//! Any types that implements `Iterator<Item = Duration>` can be used as backoff.
//!
//! backon also provides backoff implementations with reasonable defaults:
//!
//! - [`ConstantBackoff`]: backoff with constant delay and limited times.
//! - [`ExponentialBackoff`]: backoff with exponential delay, also provides jitter supports.
//! - [`FibonacciBackoff`]: backoff with fibonacci delay, also provides jitter supports.
//!
//! # Retry
//!
//! For more examples, please visit [`docs::examples`].
//!
//! ## Retry an async function
//!
//! ```rust
//! use anyhow::Result;
//! use backon::ExponentialBuilder;
//! use backon::Retryable;
//! use std::time::Duration;
//!
//! async fn fetch() -> Result<String> {
//!     Ok("hello, world!".to_string())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let content = fetch
//!         // Retry with exponential backoff
//!         .retry(ExponentialBuilder::default())
//!         // When to retry
//!         .when(|e| e.to_string() == "EOF")
//!         // Notify when retrying
//!         .notify(|err: &anyhow::Error, dur: Duration| {
//!             println!("retrying {:?} after {:?}", err, dur);
//!         })
//!         .await?;
//!     println!("fetch succeeded: {}", content);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Retry a blocking function
//!
//! ```rust
//! use anyhow::Result;
//! use backon::BlockingRetryable;
//! use backon::ExponentialBuilder;
//! use std::time::Duration;
//!
//! fn fetch() -> Result<String> {
//!     Ok("hello, world!".to_string())
//! }
//!
//! fn main() -> Result<()> {
//!     let content = fetch
//!         // Retry with exponential backoff
//!         .retry(ExponentialBuilder::default())
//!         // When to retry
//!         .when(|e| e.to_string() == "EOF")
//!         // Notify when retrying
//!         .notify(|err: &anyhow::Error, dur: Duration| {
//!             println!("retrying {:?} after {:?}", err, dur);
//!         })
//!         .call()?;
//!     println!("fetch succeeded: {}", content);
//!
//!     Ok(())
//! }
//! ```

#![deny(missing_docs)]
#![deny(unused_qualifications)]

mod backoff;
pub use backoff::*;

mod retry;
pub use retry::Retry;
pub use retry::Retryable;

mod retry_with_context;
pub use retry_with_context::RetryWithContext;
pub use retry_with_context::RetryableWithContext;

mod sleep;
pub use sleep::DefaultSleeper;
#[cfg(all(target_arch = "wasm32", feature = "gloo-timers-sleep"))]
pub use sleep::GlooTimersSleep;
pub use sleep::Sleeper;
#[cfg(all(not(target_arch = "wasm32"), feature = "tokio-sleep"))]
pub use sleep::TokioSleeper;

#[cfg(not(target_arch = "wasm32"))]
mod blocking_retry;
#[cfg(not(target_arch = "wasm32"))]
pub use blocking_retry::BlockingRetry;
#[cfg(not(target_arch = "wasm32"))]
pub use blocking_retry::BlockingRetryable;

#[cfg(not(target_arch = "wasm32"))]
mod blocking_retry_with_context;
#[cfg(not(target_arch = "wasm32"))]
pub use blocking_retry_with_context::BlockingRetryWithContext;
#[cfg(not(target_arch = "wasm32"))]
pub use blocking_retry_with_context::BlockingRetryableWithContext;

#[cfg(docsrs)]
pub mod docs;
