//! Compile-time generated maps and sets.
//!
//! The `phf::Map` and `phf::Set` types have roughly comparable performance to
//! a standard hash table, but can be generated as compile-time static values.
//!
//! # Usage
//!
//! If the `macros` Cargo feature is enabled, the `phf_map`, `phf_set`,
//! `phf_ordered_map`, and `phf_ordered_set` macros can be used to construct
//! the PHF type. This method can be used with a stable compiler
//! (minimum supported rust version is 1.46.
//! feature).
//!
//! ```toml
//! [dependencies]
//! phf = { version = "0.10", features = ["macros"] }
//! ```
//!
//! ```
//! use phf::{phf_map, phf_set};
//!
//! static MY_MAP: phf::Map<&'static str, u32> = phf_map! {
//!     "hello" => 1,
//!     "world" => 2,
//! };
//!
//! static MY_SET: phf::Set<&'static str> = phf_set! {
//!     "hello world",
//!     "hola mundo",
//! };
//!
//! fn main() {
//!     assert_eq!(MY_MAP["hello"], 1);
//!     assert!(MY_SET.contains("hello world"));
//! }
//! ```
//!
//! Alternatively, you can use the `phf_codegen` crate to generate PHF datatypes
//! in a build script.
//!
//! ## Note
//!
//! Currently, the macro syntax has some limitations and may not
//! work as you want. See [#183] or [#196] for example.
//!
//! [#183]: https://github.com/rust-phf/rust-phf/issues/183
//! [#196]: https://github.com/rust-phf/rust-phf/issues/196
#![doc(html_root_url = "https://docs.rs/phf/0.10")]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std as core;

#[cfg(feature = "macros")]
/// Macro to create a `static` (compile-time) [`Map`].
///
/// Requires the `macros` feature.
///
/// # Example
///
/// ```
/// use phf::{phf_map, Map};
///
/// static MY_MAP: Map<&'static str, u32> = phf_map! {
///     "hello" => 1,
///     "world" => 2,
/// };
///
/// fn main () {
///     assert_eq!(MY_MAP["hello"], 1);
/// }
/// ```
#[::proc_macro_hack::proc_macro_hack]
pub use phf_macros::phf_map;

#[cfg(feature = "macros")]
/// Macro to create a `static` (compile-time) [`OrderedMap`].
///
/// Requires the `macros` feature. Same usage as [`phf_map`].
#[::proc_macro_hack::proc_macro_hack]
pub use phf_macros::phf_ordered_map;

#[cfg(feature = "macros")]
/// Macro to create a `static` (compile-time) [`Set`].
///
/// Requires the `macros` feature.
///
/// # Example
///
/// ```
/// use phf::{phf_set, Set};
///
/// static MY_SET: Set<&'static str> = phf_set! {
///     "hello world",
///     "hola mundo",
/// };
///
/// fn main () {
///     assert!(MY_SET.contains("hello world"));
/// }
/// ```
#[proc_macro_hack::proc_macro_hack]
pub use phf_macros::phf_set;

#[cfg(feature = "macros")]
/// Macro to create a `static` (compile-time) [`OrderedSet`].
///
/// Requires the `macros` feature. Same usage as [`phf_set`].
#[proc_macro_hack::proc_macro_hack]
pub use phf_macros::phf_ordered_set;

#[doc(inline)]
pub use self::map::Map;
#[doc(inline)]
pub use self::ordered_map::OrderedMap;
#[doc(inline)]
pub use self::ordered_set::OrderedSet;
#[doc(inline)]
pub use self::set::Set;
pub use phf_shared::PhfHash;

pub mod map;
pub mod ordered_map;
pub mod ordered_set;
pub mod set;
