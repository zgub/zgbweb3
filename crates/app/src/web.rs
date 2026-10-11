//! Browser glue. Every function has a native twin so callers stay cfg-free.
//!
//! Routing uses the URL hash (`#/energy-ladder`), so links to a topic can be
//! shared and the browser back button works, without any server-side routing.

#[cfg(target_arch = "wasm32")]
mod imp {
    fn window() -> Option<web_sys::Window> {
        web_sys::window()
    }

    /// Current route without the leading `#/`.
    pub fn current_route() -> Option<String> {
        let hash = window()?.location().hash().ok()?;
        Some(
            hash.trim_start_matches('#')
                .trim_start_matches('/')
                .to_owned(),
        )
    }

    pub fn initial_route() -> Option<String> {
        current_route()
    }

    pub fn set_route(route: &str) {
        if let Some(w) = window() {
            // set_hash adds a history entry, which is what makes "back" work.
            let _ = w.location().set_hash(&format!("/{route}"));
        }
    }

    /// The browser's preferred language tag, e.g. "sk-SK".
    pub fn preferred_language_tag() -> Option<String> {
        window()?.navigator().language()
    }

    /// Keep `<html lang>` in sync for screen readers and browser translation.
    pub fn set_document_lang(code: &str) {
        if let Some(el) = window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
        {
            let _ = el.set_attribute("lang", code);
        }
    }

    /// Remove everything this site stored in the browser (localStorage is per-origin).
    pub fn clear_local_storage() {
        if let Some(Ok(Some(storage))) = window().map(|w| w.local_storage()) {
            let _ = storage.clear();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    pub fn current_route() -> Option<String> {
        None
    }

    /// Native dev convenience: `cargo run -- energy-ladder` opens that page.
    pub fn initial_route() -> Option<String> {
        std::env::args().nth(1)
    }

    pub fn set_route(_route: &str) {}

    /// Native: `ZGB_LANG`, then the usual POSIX locale variables.
    pub fn preferred_language_tag() -> Option<String> {
        ["ZGB_LANG", "LC_ALL", "LC_MESSAGES", "LANG"]
            .iter()
            .find_map(|v| std::env::var(v).ok().filter(|s| !s.is_empty()))
    }

    pub fn set_document_lang(_code: &str) {}

    pub fn clear_local_storage() {
        log::info!(
            "clear_local_storage: native build keeps state in eframe's storage dir; delete it by hand"
        );
    }
}

pub use imp::*;
