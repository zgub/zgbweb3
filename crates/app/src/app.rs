use serde::{Deserialize, Serialize};

use crate::pages::{self, Page};
use crate::settings::{Settings, StorageConsent, ThemeChoice};
use crate::shell::{self, Action};
use crate::topics::{self, Nav, Topic};
use crate::web;

/// Storage key for the consent answer. Written once the visitor answers,
/// regardless of the answer (see DECISIONS D-006).
const CONSENT_KEY: &str = "consent";

/// Below this width (in points) the topic menu starts closed and closes after navigation.
const NARROW: f32 = 720.0;

/// Everything app-level that is persisted (with consent) under `eframe::APP_KEY`.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
struct Persisted {
    settings: Settings,
    page: Page,
}

pub struct ZgbApp {
    settings: Settings,
    consent: StorageConsent,
    page: Page,
    topics: Vec<Box<dyn Topic>>,
    /// Route we last wrote to / read from the URL, to detect back/forward.
    last_route: String,
    /// Look settings currently applied to the egui context.
    applied_look: Option<(ThemeChoice, f32)>,
    first_frame: bool,
}

impl ZgbApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        install_fonts(&cc.egui_ctx);
        let mut topics = topics::registry();
        let mut persisted = Persisted::default();
        let mut consent = StorageConsent::Undecided;

        if let Some(storage) = cc.storage {
            consent = eframe::get_value(storage, CONSENT_KEY).unwrap_or_default();
            if consent.may_store() {
                persisted = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
                for t in &mut topics {
                    t.load(storage);
                }
            }
        }

        let mut app = Self {
            settings: persisted.settings,
            consent,
            page: persisted.page,
            topics,
            last_route: String::new(),
            applied_look: None,
            first_frame: true,
        };
        // A link like …/#/energy-ladder wins over the remembered page.
        if let Some(route) = web::initial_route().filter(|r| !r.is_empty()) {
            app.page = app.page_for_route(&route);
        }
        if !app.page_exists(&app.page) {
            app.page = Page::Home;
        }
        app.last_route = app.page.route();
        web::set_route(&app.last_route);
        app
    }

    fn page_exists(&self, page: &Page) -> bool {
        match page {
            Page::Topic(id) => self.topics.iter().any(|t| t.meta().id == id),
            _ => true,
        }
    }

    fn page_for_route(&self, route: &str) -> Page {
        Page::from_route(route, |id| self.topics.iter().any(|t| t.meta().id == id))
    }

    fn go(&mut self, page: Page) {
        self.page = page;
        self.last_route = self.page.route();
        web::set_route(&self.last_route);
    }

    /// Browser back/forward or a pasted link changed the hash.
    fn follow_url(&mut self) {
        if let Some(route) = web::current_route()
            && route != self.last_route
        {
            self.page = self.page_for_route(&route);
            self.last_route = route;
        }
    }

    fn apply_look(&mut self, ctx: &egui::Context) {
        let want = (self.settings.theme, self.settings.zoom);
        if self.applied_look != Some(want) {
            ctx.set_theme(self.settings.theme.to_egui());
            ctx.set_zoom_factor(self.settings.zoom);
            self.applied_look = Some(want);
        }
    }

    fn current_title(&self) -> Option<&'static str> {
        match &self.page {
            Page::Home => None,
            Page::About => Some("About"),
            Page::Privacy => Some("Privacy"),
            Page::License => Some("License"),
            Page::Topic(id) => self
                .topics
                .iter()
                .find(|t| t.meta().id == id)
                .map(|t| t.meta().title),
        }
    }

    fn set_consent(&mut self, new: StorageConsent) {
        if self.consent.may_store() && !new.may_store() {
            // Withdrawn: drop what we stored. The new answer is written on next save.
            web::clear_local_storage();
        }
        self.consent = new;
    }

    fn handle(&mut self, action: Action, narrow: bool) {
        match action {
            Action::Go(page) => {
                self.go(page);
                if narrow {
                    self.settings.menu_open = false;
                }
            }
            Action::ForgetEverything => {
                web::clear_local_storage();
                self.consent = StorageConsent::Undecided;
                self.settings = Settings::default();
                self.topics = topics::registry();
                self.applied_look = None;
            }
        }
    }
}

/// egui's proportional font lacks a few glyphs we use (→, ⁻ in 10⁻⁶);
/// the bundled monospace Hack has them, so use it as the last fallback.
fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        family.push("Hack".to_owned());
    }
    ctx.set_fonts(fonts);
}

impl eframe::App for ZgbApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.apply_look(&ctx);
        self.follow_url();

        let narrow = ui.max_rect().width() < NARROW;
        if self.first_frame {
            if narrow {
                self.settings.menu_open = false;
            }
            self.first_frame = false;
        }

        let mut actions = Vec::new();
        let mut consent = self.consent;
        let title = self.current_title();

        egui::Panel::top("top-bar").show(ui, |ui| {
            shell::top_bar(ui, &mut self.settings, &mut consent, title, &mut actions);
        });
        egui::Panel::bottom("footer").show(ui, |ui| shell::footer(ui, &mut actions));
        egui::Panel::right("topic-menu")
            .default_size(250.0)
            .size_range(180.0..=380.0)
            .show_collapsible(ui, &mut self.settings.menu_open, |ui| {
                shell::topic_menu(ui, &self.topics, &self.page, &mut actions);
            });

        let page = self.page.clone();
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    // Readable line length on wide screens, never wider than what's visible.
                    ui.set_max_width(ui.available_width().min(1100.0));
                    if consent == StorageConsent::Undecided {
                        shell::consent_banner(ui, &mut consent, &mut actions);
                    }
                    match &page {
                        Page::Home => {
                            if let Some(p) = pages::home(ui, &self.topics) {
                                actions.push(Action::Go(p));
                            }
                        }
                        Page::About => pages::about(ui),
                        Page::Privacy => pages::privacy(ui),
                        Page::License => pages::license(ui),
                        Page::Topic(id) => {
                            let mut nav = Nav::default();
                            if let Some(t) = self.topics.iter_mut().find(|t| t.meta().id == id) {
                                t.ui(ui, &mut nav);
                                let meta = t.meta();
                                shell::where_next(ui, meta, &self.topics, &mut actions);
                            }
                            if let Some(go) = nav.go {
                                actions.push(Action::Go(Page::Topic(go.to_string())));
                            }
                        }
                    }
                });
        });

        if consent != self.consent {
            self.set_consent(consent);
        }
        for a in actions {
            self.handle(a, narrow);
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if self.consent != StorageConsent::Undecided {
            eframe::set_value(storage, CONSENT_KEY, &self.consent);
        }
        if self.consent.may_store() {
            let p = Persisted {
                settings: self.settings.clone(),
                page: self.page.clone(),
            };
            eframe::set_value(storage, eframe::APP_KEY, &p);
            for t in &self.topics {
                t.save(storage);
            }
        }
    }

    fn persist_egui_memory(&self) -> bool {
        self.consent.may_store()
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(10)
    }
}
