//! [![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
//! [![docs](https://docs.rs/roctogen/badge.svg)](https://docs.rs/roctogen/)
//!
//! # Roctogen: a rust client library for the GitHub v3 API  
//!
//! This client API is generated from the [upstream OpenAPI
//! specification](https://github.com/github/rest-api-description/). The library currently supports
//! webassembly and synchronous requests with the [Isahc HTTP client](https://github.com/sagebind/isahc). 
//!
//! # Install
//!
//! Add the following to your `Cargo.toml` file
//!
//! ```nocompile
//! [dependencies]
//! roctogen = "0.1.0"
//! ```
//!
//! # API
//! ## Documentation
//!
//! [API docs](https://docs.rs/roctogen/latest).
//!
//! # Usage
//!
//! A quick example of this library:
//!
//! ```no_run
//! use roctogen::api::{self, repos};
//! use roctogen::auth::Auth;
//!
//! let auth = Auth::None;
//! let per_page = api::PerPage::new(10);
//! 
//! let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
//! params = params.author("fussybeaver").page(2);
//!
//! repos::new(&auth).list_commits("fussybeaver", "bollard", Some(params));
//! ```
//!
//! ## Async
//!
//! All the `async` methods are suffixed with `_async` (currently only supported on the `wasm`
//! target).
//!
//! ## Webassembly
//!
//! To compile for webassembly, you can use [`wasm-pack`](https://github.com/rustwasm/wasm-pack) or compile with the
//! `wasm32-unknown-unknown` target:
//!
//! ```nocompile
//! $ wasm-pack build
//! ```
//!
//! ```nocompile
//! $ cargo build --target wasm32-unknown-unknown
//! ```
//!
//! If you are building a [cloudflare worker](https://workers.cloudflare.com/), you would use the
//! `wrangler` wrapper:
//!
//! ```nocompile
//! $ wrangler preview --watch
//! ```
//!
//! ## Isahc
//!
//! Building on non-`wasm` targets generally requires adopting a feature for the desired
//! client adapter. This library only supports [`isahc`](https://github.com/sagebind/isahc) at the
//! moment, but other adapters are planned, and contributions are welcome.
//!
//! Compiling for the [`isahc`](https://github.com/sagebind/isahc) client required the `isahc` feature:
//!
//! ```nocompile
//! $ cargo build --features isahc
//! ```
//!
//! # GitHub preview features
//!
//! GitHub supports a phased rollout of non-stable endpoints behind header flags. These are
//! supported in this library through cargo feature flags. 
//!
//! ```nocompile
//! $ cargo build --features squirrel-girl
//! ```
//!
//! # Generate the API
//!
//! The majority of code is generated through the [Swagger OpenAPI
//! generator](https://github.com/swagger-api/swagger-codegen) (version 3).  Building requires the
//! [`mvn`](https://maven.apache.org/install.html) Java build tool, pegged at Java version 8 (so
//! you'll need an appropriate JDK).
//!
//! ```nocompile
//! $ mvn -D org.slf4j.simpleLogger.defaultLogLevel=info clean compiler:compile generate-resources
//! ```
//!
//! # Tests 
//!
//! Beware, tests are currently still doing real HTTP requests to the GitHub API.
//!
//! Run the wasm tests:
//!
//! ```nocompile
//! $ wasm-pack test --firefox --headless 
//! ```
//!
//! Run the sync tests:
//!
//! ```nocompile
//! $ cargo test --features isahc,mercy,squirrel-girl,inertia,starfox --target x86_64-unknown-linux-gnu -- --nocapture
//! ```
#![allow(
    missing_docs,
    unused_imports,
)]

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

pub mod adapters;
pub mod endpoints;
pub mod models;

pub mod auth {
    pub enum Auth {
        Basic { user: String, pass: String },
        OAuth { token: String },
        None,
    }
}

pub use endpoints as api;
