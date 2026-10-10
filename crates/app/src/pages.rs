//! Non-topic pages and routing.

use egui::RichText;
use serde::{Deserialize, Serialize};

use crate::topics::{Group, Topic};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Page {
    #[default]
    Home,
    About,
    Privacy,
    License,
    Topic(String),
}

/// Routes taken by pages; topic ids must not use these (checked in tests).
#[cfg_attr(not(test), allow(dead_code))]
pub const RESERVED_ROUTES: [&str; 4] = ["", "about", "privacy", "license"];

impl Page {
    /// The part after `#/` in the URL.
    pub fn route(&self) -> String {
        match self {
            Page::Home => String::new(),
            Page::About => "about".into(),
            Page::Privacy => "privacy".into(),
            Page::License => "license".into(),
            Page::Topic(id) => id.clone(),
        }
    }

    /// Unknown routes fall back to Home.
    pub fn from_route(route: &str, is_topic: impl Fn(&str) -> bool) -> Page {
        match route.trim_matches('/') {
            "" => Page::Home,
            "about" => Page::About,
            "privacy" => Page::Privacy,
            "license" => Page::License,
            id if is_topic(id) => Page::Topic(id.to_string()),
            _ => Page::Home,
        }
    }
}

pub fn home(ui: &mut egui::Ui, topics: &[Box<dyn Topic>]) -> Option<Page> {
    let mut go = None;
    ui.heading("Big numbers, small numbers, and how the world adds up");
    ui.label(
        "Little experiments you can poke at. Move the sliders, roll the dice, break things. \
         Made for curious kids — grown-ups welcome.",
    );
    ui.add_space(12.0);

    for group in Group::ALL {
        let in_group: Vec<_> = topics.iter().filter(|t| t.meta().group == group).collect();
        if in_group.is_empty() {
            continue;
        }
        ui.label(RichText::new(group.label()).strong().size(16.0));
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            for t in in_group {
                let m = t.meta();
                let card = egui::Frame::group(ui.style())
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        ui.set_width(260.0);
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(format!("{} {}", m.icon, m.title)).strong());
                                ui.label(m.maturity.badge());
                            });
                            ui.label(RichText::new(m.blurb).weak());
                        });
                    });
                let resp =
                    ui.interact(card.response.rect, ui.id().with(m.id), egui::Sense::click());
                if resp
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    go = Some(Page::Topic(m.id.to_string()));
                }
            }
        });
        ui.add_space(12.0);
    }
    go
}

pub fn about(ui: &mut egui::Ui) {
    ui.heading("About");
    ui.label(
        "zgbweb is a small collection of interactive explainers about scale, chance, physics and maths. \
         Everything runs in your browser; nothing you do here is sent anywhere.",
    );
    ui.add_space(6.0);
    ui.label("Built with Rust, egui and WebAssembly.");
    ui.label(RichText::new("Source code link: TBD").weak());
}

pub fn privacy(ui: &mut egui::Ui) {
    ui.heading("Privacy");
    ui.label(
        RichText::new("Draft — to be finalised before the site goes public.")
            .weak()
            .italics(),
    );
    ui.add_space(6.0);
    for (title, body) in [
        (
            "No tracking",
            "No accounts, no analytics, no advertising and no cookies.",
        ),
        (
            "What stays on your device",
            "If you choose \"Yes, remember\", your settings and the state of the experiments are saved \
             in your browser's local storage on this device. They are never sent to us. Your answer \
             to that question is itself remembered so we don't ask every time. \
             Use ⚙ → \"Forget everything\" to delete it all.",
        ),
        (
            "Server logs",
            "The web server that delivers this page may record standard technical data (IP address, \
             time, requested address, browser type) to keep the service running and secure. \
             Retention period: TBD.",
        ),
        (
            "If this changes",
            "If advertising or accounts are ever added, this page will be updated and you will be \
             asked before anything new is stored or shared.",
        ),
        ("Who runs this site", "Operator and contact: TBD."),
    ] {
        ui.add_space(6.0);
        ui.label(RichText::new(title).strong());
        ui.label(body);
    }
}

pub fn license(ui: &mut egui::Ui) {
    ui.heading("License");
    ui.label(
        RichText::new("Draft — final choice pending (see QUESTIONS.md).")
            .weak()
            .italics(),
    );
    ui.add_space(6.0);
    ui.label("Program code: GNU AGPL v3.0 or later.");
    ui.label("Texts and pictures: Creative Commons BY-SA 4.0.");
    ui.add_space(6.0);
    ui.label(RichText::new("Third-party parts").strong());
    ui.label("egui / eframe — MIT or Apache-2.0.");
    ui.label("Fonts bundled with egui: Ubuntu Light (Ubuntu Font Licence), Hack (MIT), Noto Emoji (OFL), Emoji Icon Font (MIT/OFL).");
}
