//! Page chrome in the old zgbweb style (D-018): a slim top bar, a fixed 200 pt
//! menu on the right, a one-line footer, and the storage-consent banner.
//!
//! These functions never mutate app state beyond the settings they are handed;
//! navigation and other effects come back as [`Action`]s.

use content::{PageMeta, Section};
use egui::{Align, Layout, RichText};

use crate::i18n::{I18n, Lang};
use crate::kit;
use crate::pages::Page;
use crate::settings::{Settings, StorageConsent, ThemeChoice, ZOOM_RANGE};
use crate::theme::Palette;

pub enum Action {
    Go(Page),
    ForgetEverything,
}

/// Width of the right-hand menu, as in zgbweb2.
pub const MENU_WIDTH: f32 = 200.0;

pub struct Bar<'a> {
    pub tr: &'a I18n,
    pub settings: &'a mut Settings,
    pub consent: &'a mut StorageConsent,
    /// Show the menu toggle (narrow screens only; wide screens always show the menu).
    pub narrow: bool,
    pub actions: &'a mut Vec<Action>,
}

pub fn top_bar(ui: &mut egui::Ui, bar: Bar) {
    let Bar {
        tr,
        settings,
        consent,
        narrow,
        actions,
    } = bar;
    egui::MenuBar::new().ui(ui, |ui| {
        if ui
            .link(RichText::new(tr.t("site-name")).strong())
            .on_hover_text(tr.t("nav-home"))
            .clicked()
        {
            actions.push(Action::Go(Page::Landing));
        }
        if narrow {
            ui.toggle_value(&mut settings.menu_open, tr.t("nav-menu"))
                .on_hover_text(tr.t("nav-menu-hover"));
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            // right-to-left: added in reverse visual order
            ui.menu_button("⚙", |ui| {
                settings_menu(ui, tr, settings, consent, actions)
            })
            .response
            .on_hover_text(tr.t("nav-settings"));
            ui.separator();
            for lang in Lang::ALL.iter().rev() {
                let current = tr.lang() == *lang;
                if ui
                    .selectable_label(current, lang.short())
                    .on_hover_text(lang.native_name())
                    .clicked()
                {
                    settings.language = Some(*lang);
                }
            }
            ui.separator();
            for (choice, glyph) in [
                (ThemeChoice::Dark, "🌙"),
                (ThemeChoice::Light, "☀"),
                (ThemeChoice::System, "💻"),
            ]
            .into_iter()
            .rev()
            {
                ui.selectable_value(&mut settings.theme, choice, glyph)
                    .on_hover_text(tr.t(choice.key()));
            }
        });
    });
}

fn settings_menu(
    ui: &mut egui::Ui,
    tr: &I18n,
    settings: &mut Settings,
    consent: &mut StorageConsent,
    actions: &mut Vec<Action>,
) {
    ui.set_min_width(240.0);
    ui.horizontal(|ui| {
        ui.label(tr.t("settings-size"));
        ui.add(
            egui::Slider::new(&mut settings.zoom, ZOOM_RANGE)
                .step_by(0.05)
                .show_value(false),
        );
        if ui.small_button(tr.t("settings-size-reset")).clicked() {
            settings.zoom = 1.0;
        }
    });
    ui.checkbox(&mut settings.calm_motion, tr.t("settings-calm"));

    ui.separator();
    ui.strong(tr.t("settings-language"));
    ui.radio_value(&mut settings.language, None, tr.t("lang-auto"));
    for lang in Lang::ALL {
        ui.radio_value(&mut settings.language, Some(lang), lang.native_name());
    }

    ui.separator();
    ui.strong(tr.t("settings-remember-title"));
    let mut remember = consent.may_store();
    if ui
        .checkbox(&mut remember, tr.t("settings-remember"))
        .changed()
    {
        *consent = if remember {
            StorageConsent::Granted
        } else {
            StorageConsent::Denied
        };
    }
    if ui
        .button(tr.t("settings-forget"))
        .on_hover_text(tr.t("settings-forget-hover"))
        .clicked()
    {
        actions.push(Action::ForgetEverything);
        ui.close();
    }
}

/// The right-hand menu: one heading per section, one row per page.
pub fn menu(
    ui: &mut egui::Ui,
    tr: &I18n,
    pal: &Palette,
    current: &Page,
    actions: &mut Vec<Action>,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
            ui.heading(format!("🖱 {}", tr.t("menu-title")));
            ui.separator();
            for s in Section::ALL {
                let on_index = matches!(current, Page::Section(c) if *c == s)
                    || (*current == Page::Landing && s == content::LANDING);
                let label = RichText::new(format!("{} {}", s.glyph(), tr.t(s.key()))).heading();
                if ui
                    .selectable_label(on_index, label)
                    .on_hover_text(tr.attr(s.key(), "intro"))
                    .clicked()
                {
                    actions.push(Action::Go(Page::Section(s)));
                }
                ui.separator();
                for m in content::pages_in(s) {
                    menu_row(ui, tr, pal, m, current, actions);
                }
                ui.add_space(8.0);
            }
        });
    });
}

fn menu_row(
    ui: &mut egui::Ui,
    tr: &I18n,
    pal: &Palette,
    m: &PageMeta,
    current: &Page,
    actions: &mut Vec<Action>,
) {
    let selected = matches!(current, Page::Item(id) if id == m.id);
    // Planned pages are dimmed instead of labelled, so titles fit in 200 pt;
    // drafts get a small marker. Section pages spell out every status.
    let planned = m.status == content::Status::Planned;
    let mut title = RichText::new(format!("{} {}", m.glyph, tr.page_title(m.id)));
    if planned && !selected {
        title = title.color(pal.weak);
    }
    let status = match m.status {
        content::Status::Draft => m
            .status
            .key()
            .map(|k| RichText::new(tr.t(k)).small().color(pal.weak))
            .unwrap_or_default(),
        _ => RichText::default(),
    };
    let r = ui.selectable_label(selected, (title, egui::Atom::grow(), status));
    let hover = match (tr.page_blurb(m.id), m.status.key()) {
        (Some(b), _) => Some(b),
        (None, Some(k)) => Some(tr.t(k)),
        (None, None) => None,
    };
    let r = match hover {
        Some(h) => r.on_hover_text(h),
        None => r,
    };
    if r.clicked() {
        actions.push(Action::Go(Page::Item(m.id.to_string())));
    }
}

/// "Where next?" links at the end of a page, from its cross-links (D-029).
pub fn where_next(
    ui: &mut egui::Ui,
    tr: &I18n,
    pal: &Palette,
    meta: &PageMeta,
    actions: &mut Vec<Action>,
) {
    if meta.links.is_empty() {
        return;
    }
    ui.add_space(14.0);
    ui.separator();
    ui.strong(tr.t("where-next"));
    for l in meta.links {
        let Some(target) = content::page(l.to) else {
            continue;
        };
        ui.horizontal_wrapped(|ui| {
            let hover = format!(
                "{} {} · {} {}",
                target.section.glyph(),
                tr.t(target.section.key()),
                target.glyph,
                tr.page_title(target.id)
            );
            if ui
                .link(format!("{} {} →", target.glyph, tr.t(l.hook)))
                .on_hover_text(hover)
                .clicked()
            {
                actions.push(Action::Go(Page::Item(target.id.to_string())));
            }
            if let Some(k) = target.status.key() {
                ui.label(RichText::new(tr.t(k)).small().color(pal.weak));
            }
        });
    }
}

/// One line at the bottom: name, version, credits, fixed pages.
pub fn footer(ui: &mut egui::Ui, tr: &I18n, actions: &mut Vec<Action>) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        ui.label(format!(
            "{} v{}",
            tr.t("site-name"),
            env!("CARGO_PKG_VERSION")
        ));
        ui.label("·");
        ui.label(tr.t("footer-powered"));
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(tr.t("footer-and"));
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/main/crates/eframe",
        );
        for (key, page) in [
            ("footer-privacy", Page::Privacy),
            ("footer-license", Page::License),
            ("footer-about", Page::About),
        ] {
            ui.label("·");
            if ui.link(tr.t(key)).clicked() {
                actions.push(Action::Go(page));
            }
        }
        egui::warn_if_debug_build(ui);
    });
}

/// Shown at the top of the content area until the visitor answers.
pub fn consent_banner(
    ui: &mut egui::Ui,
    tr: &I18n,
    pal: &Palette,
    consent: &mut StorageConsent,
    actions: &mut Vec<Action>,
) {
    kit::card(pal).show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label("💾");
            ui.label(tr.t("consent-text"));
        });
        ui.horizontal(|ui| {
            if ui.button(tr.t("consent-yes")).clicked() {
                *consent = StorageConsent::Granted;
            }
            if ui.button(tr.t("consent-no")).clicked() {
                *consent = StorageConsent::Denied;
            }
            if ui.link(tr.t("footer-privacy")).clicked() {
                actions.push(Action::Go(Page::Privacy));
            }
        });
    });
    ui.add_space(10.0);
}
