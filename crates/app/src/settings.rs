use serde::{Deserialize, Serialize};

use crate::i18n::Lang;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeChoice {
    Dark,
    Light,
    /// Follow the device (default, D-018).
    #[default]
    System,
}

impl ThemeChoice {
    /// Catalog key for the label.
    pub fn key(self) -> &'static str {
        match self {
            ThemeChoice::Dark => "theme-dark",
            ThemeChoice::Light => "theme-light",
            ThemeChoice::System => "theme-system",
        }
    }

    pub fn to_egui(self) -> egui::ThemePreference {
        match self {
            ThemeChoice::Dark => egui::ThemePreference::Dark,
            ThemeChoice::Light => egui::ThemePreference::Light,
            ThemeChoice::System => egui::ThemePreference::System,
        }
    }
}

/// Whether the visitor allowed us to keep things in their browser's local storage.
///
/// Nothing is written before they choose. The choice itself is stored either way
/// (otherwise "no" would mean asking again on every visit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum StorageConsent {
    #[default]
    Undecided,
    Granted,
    Denied,
}

impl StorageConsent {
    pub fn may_store(self) -> bool {
        self == StorageConsent::Granted
    }
}

/// User-facing settings. Persisted only with consent.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub theme: ThemeChoice,
    /// `None` = follow the browser's language.
    pub language: Option<Lang>,
    /// Whole-UI zoom (egui zoom factor). 1.0 = default.
    pub zoom: f32,
    /// Is the menu open on narrow screens? (Wide screens always show it.)
    pub menu_open: bool,
    /// Reduce animation (auto-play sims start paused, etc.).
    pub calm_motion: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: ThemeChoice::System,
            language: None,
            zoom: 1.0,
            menu_open: false,
            calm_motion: false,
        }
    }
}

pub const ZOOM_RANGE: std::ops::RangeInclusive<f32> = 0.75..=2.0;
