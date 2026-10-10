use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeChoice {
    #[default]
    Dark,
    Light,
    System,
}

impl ThemeChoice {
    pub const ALL: [ThemeChoice; 3] = [ThemeChoice::Dark, ThemeChoice::Light, ThemeChoice::System];

    pub fn label(self) -> &'static str {
        match self {
            ThemeChoice::Dark => "🌙 Dark",
            ThemeChoice::Light => "☀ Light",
            ThemeChoice::System => "💻 Follow system",
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
    /// Whole-UI zoom (egui zoom factor). 1.0 = default.
    pub zoom: f32,
    /// Is the right-hand topic menu open?
    pub menu_open: bool,
    /// Reduce animation (auto-play sims start paused, etc.).
    pub calm_motion: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: ThemeChoice::Dark,
            zoom: 1.0,
            menu_open: true,
            calm_motion: false,
        }
    }
}

pub const ZOOM_RANGE: std::ops::RangeInclusive<f32> = 0.75..=2.0;
