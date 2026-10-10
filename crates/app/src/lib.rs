//! zgbweb — interactive science & maths explainers.
//!
//! Layout of this crate:
//! - [`app`]      the eframe `App`: owns settings, routing, topics, persistence
//! - [`shell`]    page chrome: top bar, right-hand topic menu, footer, consent banner
//! - [`pages`]    static pages: home, about, privacy, license
//! - [`topics`]   the `Topic` trait, the registry, and one module per topic
//! - [`settings`] user settings + storage consent
//! - [`web`]      browser glue (URL hash routing, clearing storage); no-ops natively

mod app;
mod pages;
mod settings;
mod shell;
mod topics;
mod web;

pub use app::ZgbApp;

/// Used as the native window/storage id. Web storage is per-origin anyway.
pub const APP_ID: &str = "zgbweb";
