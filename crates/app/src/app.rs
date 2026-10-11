use egui::Margin;
use serde::{Deserialize, Serialize};

use crate::i18n::{I18n, Lang};
use crate::pages::{self, Page};
use crate::settings::{Settings, StorageConsent, ThemeChoice};
use crate::shell::{self, Action, Bar};
use crate::theme;
use crate::topics::{self, Cx, Topic};
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
    tr: I18n,
    /// Route we last wrote to / read from the URL, to detect back/forward.
    last_route: String,
    /// Look settings currently applied to the egui context.
    applied_look: Option<(ThemeChoice, f32)>,
    first_frame: bool,
}

/// The language to show: the visitor's choice, else the browser's, else English.
fn effective_lang(settings: &Settings) -> Lang {
    settings
        .language
        .or_else(|| web::preferred_language_tag().and_then(|t| Lang::from_tag(&t)))
        .unwrap_or_default()
}

impl ZgbApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::install(&cc.egui_ctx);
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

        let lang = effective_lang(&persisted.settings);
        web::set_document_lang(lang.code());
        let mut app = Self {
            tr: I18n::new(lang),
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
            app.apply_route_param(&route);
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
            self.apply_route_param(&route);
            self.last_route = route;
        }
    }

    fn apply_route_param(&mut self, route: &str) {
        if let (Page::Topic(id), Some(param)) = (&self.page, pages::route_param(route))
            && let Some(t) = self.topics.iter_mut().find(|t| t.meta().id == id)
        {
            t.set_param(param);
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

    fn apply_language(&mut self) {
        let want = effective_lang(&self.settings);
        if want != self.tr.lang() {
            self.tr = I18n::new(want);
            web::set_document_lang(want.code());
        }
    }

    fn current_title(&self) -> Option<String> {
        match &self.page {
            Page::Topic(id) => Some(self.tr.topic_title(id)),
            p => p.title_key().map(|k| self.tr.t(k)),
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

impl eframe::App for ZgbApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.apply_look(&ctx);
        self.apply_language();
        self.follow_url();
        let pal = theme::palette(ui.visuals().dark_mode);

        let narrow = ui.max_rect().width() < NARROW;
        if self.first_frame {
            if narrow {
                self.settings.menu_open = false;
            }
            self.first_frame = false;
        }

        let mut actions = Vec::new();
        let mut consent = self.consent;
        // The breadcrumb title only fits beside the buttons on wider screens.
        let title = (ui.max_rect().width() >= 640.0)
            .then(|| self.current_title())
            .flatten();
        let tr = &self.tr;

        egui::Panel::top("top-bar")
            .frame(shell::chrome_frame(pal, Margin::symmetric(14, 8)))
            .show(ui, |ui| {
                shell::top_bar(
                    ui,
                    Bar {
                        tr,
                        pal,
                        settings: &mut self.settings,
                        consent: &mut consent,
                        title,
                        actions: &mut actions,
                    },
                );
            });
        egui::Panel::bottom("footer")
            .frame(shell::chrome_frame(pal, Margin::symmetric(14, 6)))
            .show(ui, |ui| shell::footer(ui, tr, pal, &mut actions));
        egui::Panel::right("topic-menu")
            .frame(shell::chrome_frame(pal, Margin::symmetric(12, 8)))
            .default_size(270.0)
            .size_range(200.0..=380.0)
            .show_collapsible(ui, &mut self.settings.menu_open, |ui| {
                shell::topic_menu(ui, tr, pal, &self.topics, &self.page, &mut actions);
            });

        let page = self.page.clone();
        let calm = self.settings.calm_motion;
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(pal.bg)
                    .inner_margin(Margin::symmetric(22, 14)),
            )
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        // Readable line length on wide screens, never wider than what's visible.
                        ui.set_max_width(ui.available_width().min(1100.0));
                        if consent == StorageConsent::Undecided {
                            shell::consent_banner(ui, tr, pal, &mut consent, &mut actions);
                        }
                        match &page {
                            Page::Home => {
                                if let Some(p) = pages::home(ui, tr, pal, &self.topics) {
                                    actions.push(Action::Go(p));
                                }
                            }
                            Page::About => pages::about(ui, tr, pal),
                            Page::Privacy => pages::privacy(ui, tr, pal),
                            Page::License => pages::license(ui, tr, pal),
                            Page::Topic(id) => {
                                let mut cx = Cx::new(tr, pal, calm);
                                if let Some(t) = self.topics.iter_mut().find(|t| t.meta().id == id)
                                {
                                    t.ui(ui, &mut cx);
                                    let meta = t.meta();
                                    shell::where_next(
                                        ui,
                                        tr,
                                        pal,
                                        meta,
                                        &self.topics,
                                        &mut actions,
                                    );
                                }
                                if let Some(go) = cx.navigation() {
                                    actions.push(Action::Go(Page::Topic(go.to_string())));
                                }
                            }
                        }
                        ui.add_space(24.0);
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
