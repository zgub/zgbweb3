// Hide the console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Native: a desktop window. Handy for fast iteration and debugging.
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init();
    // ZGB_SIZE=1280x1400 for screenshots; default 1200×800.
    let forced: Option<[f32; 2]> = std::env::var("ZGB_SIZE").ok().and_then(|s| {
        let (w, h) = s.split_once('x')?;
        Some([w.parse().ok()?, h.parse().ok()?])
    });
    let size = forced.unwrap_or([1200.0, 800.0]);
    let options = eframe::NativeOptions {
        // Stop saving the size when one is forced. (eframe still *restores* a saved
        // size: delete ~/.local/share/zgbweb3/app.ron for exact-size screenshots.)
        persist_window: forced.is_none(),
        viewport: egui::ViewportBuilder::default()
            .with_title("zgb")
            .with_inner_size(size)
            .with_min_inner_size([360.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        zgbweb3::APP_ID,
        options,
        Box::new(|cc| Ok(Box::new(zgbweb3::ZgbApp::new(cc)))),
    )
}

/// Web: attach to the <canvas id="zgb_canvas"> in index.html.
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    eframe::WebLogger::init(log::LevelFilter::Info).ok();
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let canvas = document
            .get_element_by_id("zgb_canvas")
            .expect("index.html must contain <canvas id=\"zgb_canvas\">")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("zgb_canvas is not a canvas");

        let result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(zgbweb3::ZgbApp::new(cc)))),
            )
            .await;

        if let Some(loading) = document.get_element_by_id("loading") {
            match result {
                Ok(()) => loading.remove(),
                Err(e) => {
                    loading.set_inner_html(
                        "<p>Sorry, the page could not start. Your browser may lack WebGPU/WebGL2 support.</p>",
                    );
                    panic!("failed to start eframe: {e:?}");
                }
            }
        }
    });
}
