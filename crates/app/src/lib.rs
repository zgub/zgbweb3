//! zgbweb3 — the zgub.net site: projects, playground, workbench, explore, history.
//!
//! Layout of this crate:
//! - [`app`]      the eframe `App`: owns settings, routing, topics, persistence
//! - [`shell`]    page chrome: top bar, right-hand menu, footer, consent banner
//! - [`pages`]    routing, section indexes, planned pages, about/privacy/license
//! - [`topics`]   the `Topic` trait and one module per page that has code
//! - [`i18n`]     Fluent catalogs (en, sk), language choice, number localisation
//! - [`theme`]    egui's stock look, fonts, colours for drawing sims
//! - [`kit`]      small shared widgets (pills, cards, callouts)
//! - [`settings`] user settings + storage consent
//! - [`web`]      browser glue (URL hash routing, language, storage); no-ops natively
//!
//! The site map (sections, pages, cross-links) and the texts live in the
//! `content` crate.

mod app;
mod draw;
// Toolkit for pages with content; idle while every page is a placeholder.
#[allow(dead_code)]
mod i18n;
// Toolkit for pages with content; idle while every page is a placeholder.
#[allow(dead_code)]
mod kit;
mod pages;
mod settings;
mod shell;
// Toolkit for pages with content; idle while every page is a placeholder.
#[allow(dead_code)]
mod theme;
// Toolkit for pages with content; idle while every page is a placeholder.
#[allow(dead_code)]
mod topics;
mod web;

pub use app::ZgbApp;

/// Used as the native window/storage id. Web storage is per-origin anyway.
pub const APP_ID: &str = "zgbweb3";
