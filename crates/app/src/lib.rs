//! zgbweb — interactive science & maths explainers.
//!
//! Layout of this crate:
//! - [`app`]      the eframe `App`: owns settings, routing, topics, persistence
//! - [`shell`]    page chrome: top bar, right-hand topic menu, footer, consent banner
//! - [`pages`]    static pages: home, about, privacy, license
//! - [`topics`]   the `Topic` trait, the registry, and one module per topic
//! - [`i18n`]     Fluent catalogs (en, sk), language choice, number localisation
//! - [`theme`]    palette, fonts, widget styling
//! - [`kit`]      small shared widgets (pills, cards, callouts)
//! - [`settings`] user settings + storage consent
//! - [`web`]      browser glue (URL hash routing, language, storage); no-ops natively

mod app;
mod i18n;
mod kit;
mod pages;
mod settings;
mod shell;
mod theme;
mod topics;
mod web;

pub use app::ZgbApp;

/// Used as the native window/storage id. Web storage is per-origin anyway.
pub const APP_ID: &str = "zgbweb";
