//! Headless screenshot of any page, no window and no GPU needed.
//!
//! Runs the real app for a few frames, tessellates egui's output and fills the
//! triangles on the CPU. Good enough to check layout, colours and texts in both
//! languages; not pixel-identical to the wgpu renderer (no gamma tricks).
//!
//! ```text
//! cargo run --example shot -- rule-of-three/fuel/rulers
//! ZGB_LANG=sk ZGB_SIZE=400x860 ZGB_SHOT=shots/r3-sk.png cargo run --example shot -- rule-of-three
//! ```
//! The route is the part after `#/`. Output: `ZGB_SHOT`, default `shot.png`.
//! `ZGB_HOVER=1150,492` puts the mouse at that point (in points) to show a tooltip.
//! Storage consent is answered "no" so the banner stays out of the picture.

use std::collections::HashMap;

use egui::{Color32, ImageData, TextureId, epaint::Primitive};

/// Physical pixels per point, as on a typical laptop screen with 150 % scaling.
const PPP: f32 = 1.5;

fn main() {
    let (w, h) = std::env::var("ZGB_SIZE")
        .ok()
        .and_then(|s| {
            let (w, h) = s.split_once('x')?;
            Some((w.parse().ok()?, h.parse().ok()?))
        })
        .unwrap_or((1280.0_f32, 900.0_f32));
    let out = std::env::var("ZGB_SHOT").unwrap_or_else(|_| "shot.png".into());

    let ctx = egui::Context::default();
    let cc = eframe::CreationContext::_new_kittest(ctx.clone());
    let mut app = zgbweb3::ZgbApp::new(&cc);
    app.decline_storage_for_screenshots();
    let mut frame = eframe::Frame::_new_kittest();

    let mut textures: HashMap<TextureId, (usize, usize, Vec<Color32>)> = HashMap::new();
    let mut last = None;
    // ZGB_HOVER=x,y (in points) moves the mouse there, to capture tooltips.
    let hover: Option<egui::Pos2> = std::env::var("ZGB_HOVER").ok().and_then(|s| {
        let (x, y) = s.split_once(',')?;
        Some(egui::pos2(x.parse().ok()?, y.parse().ok()?))
    });
    for frame_no in 0..12 {
        let mut raw = egui::RawInput {
            time: Some(frame_no as f64 * 0.5),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(w, h),
            )),
            ..Default::default()
        };
        // move once, then rest: moving resets egui's tooltip delay
        if let Some(pos) = hover.filter(|_| frame_no == 0) {
            raw.events.push(egui::Event::PointerMoved(pos));
        }
        raw.viewports
            .entry(egui::ViewportId::ROOT)
            .or_default()
            .native_pixels_per_point = Some(PPP);
        let mut output = ctx.run_ui(raw, |ui| eframe::App::ui(&mut app, ui, &mut frame));
        for (id, deltas) in &output.textures_delta.set {
            for delta in deltas.iter() {
                let ImageData::Color(img) = &delta.image;
                let (dw, dh) = (img.size[0], img.size[1]);
                match delta.pos {
                    None => {
                        textures.insert(*id, (dw, dh, img.pixels.clone()));
                    }
                    Some([x0, y0]) => {
                        let (tw, _, px) = textures.get_mut(id).expect("texture exists");
                        for y in 0..dh {
                            for x in 0..dw {
                                px[(y0 + y) * *tw + x0 + x] = img.pixels[y * dw + x];
                            }
                        }
                    }
                }
            }
        }
        output.textures_delta.clear();
        last = Some(output);
    }
    let output = last.expect("ran at least one frame");
    let prims = ctx.tessellate(output.shapes, output.pixels_per_point);
    let img = rasterize(&prims, &textures, w, h);
    if let Some(dir) = std::path::Path::new(&out).parent() {
        std::fs::create_dir_all(dir).ok();
    }
    img.save(&out).expect("write png");
    println!("{out}");
}

fn rasterize(
    prims: &[egui::ClippedPrimitive],
    textures: &HashMap<TextureId, (usize, usize, Vec<Color32>)>,
    w: f32,
    h: f32,
) -> image::RgbaImage {
    let (iw, ih) = ((w * PPP) as usize, (h * PPP) as usize);
    let mut fb = vec![[0.0f32; 4]; iw * ih];
    for cp in prims {
        let Primitive::Mesh(mesh) = &cp.primitive else {
            continue;
        };
        let tex = &textures[&mesh.texture_id];
        let clip = cp.clip_rect;
        let cx0 = (clip.min.x * PPP).max(0.0) as i32;
        let cy0 = (clip.min.y * PPP).max(0.0) as i32;
        let cx1 = ((clip.max.x * PPP).ceil() as i32).min(iw as i32);
        let cy1 = ((clip.max.y * PPP).ceil() as i32).min(ih as i32);
        for tri in mesh.indices.chunks(3) {
            let v = [
                &mesh.vertices[tri[0] as usize],
                &mesh.vertices[tri[1] as usize],
                &mesh.vertices[tri[2] as usize],
            ];
            let p: Vec<(f32, f32)> = v.iter().map(|v| (v.pos.x * PPP, v.pos.y * PPP)).collect();
            let area = edge(p[0], p[1], p[2]);
            if area.abs() < 1e-8 {
                continue;
            }
            let fold = |f: fn(f32, f32) -> f32, init: f32, i: usize| {
                p.iter()
                    .map(|q| if i == 0 { q.0 } else { q.1 })
                    .fold(init, f)
            };
            let minx = fold(f32::min, f32::MAX, 0).floor() as i32;
            let maxx = fold(f32::max, f32::MIN, 0).ceil() as i32;
            let miny = fold(f32::min, f32::MAX, 1).floor() as i32;
            let maxy = fold(f32::max, f32::MIN, 1).ceil() as i32;
            for y in miny.max(cy0)..maxy.min(cy1) {
                for x in minx.max(cx0)..maxx.min(cx1) {
                    let pc = (x as f32 + 0.5, y as f32 + 0.5);
                    let w0 = edge(p[1], p[2], pc) / area;
                    let w1 = edge(p[2], p[0], pc) / area;
                    let w2 = edge(p[0], p[1], pc) / area;
                    if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                        continue;
                    }
                    let u = w0 * v[0].uv.x + w1 * v[1].uv.x + w2 * v[2].uv.x;
                    let t = w0 * v[0].uv.y + w1 * v[1].uv.y + w2 * v[2].uv.y;
                    let texel = sample(tex, u, t);
                    let d = &mut fb[y as usize * iw + x as usize];
                    let mut src = [0.0f32; 4];
                    for k in 0..4 {
                        let c = w0 * v[0].color[k] as f32
                            + w1 * v[1].color[k] as f32
                            + w2 * v[2].color[k] as f32;
                        src[k] = c / 255.0 * texel[k];
                    }
                    for k in 0..4 {
                        d[k] = src[k] + d[k] * (1.0 - src[3]);
                    }
                }
            }
        }
    }
    let mut img = image::RgbaImage::new(iw as u32, ih as u32);
    for (i, px) in img.pixels_mut().enumerate() {
        let to8 = |x: f32| (x.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
        *px = image::Rgba([to8(fb[i][0]), to8(fb[i][1]), to8(fb[i][2]), 255]);
    }
    img
}

fn edge(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

/// Bilinear sample of a premultiplied texture, 0..=1 per channel.
fn sample(t: &(usize, usize, Vec<Color32>), u: f32, v: f32) -> [f32; 4] {
    let (tw, th, px) = t;
    let x = (u * *tw as f32 - 0.5).max(0.0);
    let y = (v * *th as f32 - 0.5).max(0.0);
    let (x0, y0) = (x as usize, y as usize);
    let (x1, y1) = ((x0 + 1).min(tw - 1), (y0 + 1).min(th - 1));
    let (fx, fy) = (x - x0 as f32, y - y0 as f32);
    let g = |xx: usize, yy: usize| px[yy * tw + xx];
    let mut out = [0.0; 4];
    for (k, o) in out.iter_mut().enumerate() {
        let a = g(x0, y0)[k] as f32 * (1.0 - fx) + g(x1, y0)[k] as f32 * fx;
        let b = g(x0, y1)[k] as f32 * (1.0 - fx) + g(x1, y1)[k] as f32 * fx;
        *o = (a * (1.0 - fy) + b * fy) / 255.0;
    }
    out
}
