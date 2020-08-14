pub mod wasm;

extern crate async_std;

#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate maplit;
extern crate str_macro;

mod dag;
mod db;
pub mod embed;
pub mod fetch;
mod hash;
mod util;

#[cfg(not(default))]
pub mod kv;

#[cfg(default)]
mod kv;

mod prolly;

#[cfg(feature = "benchmark")]
pub mod benches;
