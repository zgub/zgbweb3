//! Shared drawing pieces for canvases (docs/DESIGN.md, section 5), so the same
//! picture type looks the same on every page.

use egui::{Align2, Color32, FontId, Painter, Pos2, Rect, Shape, Stroke, StrokeKind, pos2, vec2};

use crate::theme::{Palette, lerp};

/// Text inside a drawing.
pub fn text(p: &Painter, pos: Pos2, anchor: Align2, s: impl ToString, size: f32, c: Color32) {
    p.text(pos, anchor, s.to_string(), FontId::proportional(size), c);
}

/// Text that wraps inside `width`, top-left at `pos`. Returns its height.
pub fn wrapped(p: &Painter, pos: Pos2, s: String, size: f32, c: Color32, width: f32) -> f32 {
    let g = p.layout(s, FontId::proportional(size), c, width.max(60.0));
    let h = g.size().y;
    p.galley(pos, g, c);
    h
}

/// The canvas background.
pub fn canvas(p: &Painter, pal: &Palette, r: Rect) {
    p.rect_filled(r, 3.0, pal.deep);
}

/// Dashed line: a model, a prediction, "what the rule says" (DESIGN.md "Line styles").
pub fn dashed(p: &Painter, a: Pos2, b: Pos2, width: f32, c: Color32) {
    p.extend(Shape::dashed_line(&[a, b], Stroke::new(width, c), 6.0, 4.0));
}

/// The empty colour of a unit grid cell.
pub fn empty_cell(pal: &Palette) -> Color32 {
    lerp(pal.deep, pal.series[1], 0.14)
}

/// A grid of `cols × rows` unit squares fitted into `r` (top-left aligned).
/// `fill(i)` gives the colour of cell `i` (row by row). Returns the grid's rect.
pub fn unit_grid(
    p: &Painter,
    r: Rect,
    cols: usize,
    rows: usize,
    gap: f32,
    fill: impl Fn(usize) -> Color32,
) -> Rect {
    let side = (r.width() / cols as f32).min(r.height() / rows as f32);
    let origin = r.min;
    for i in 0..cols * rows {
        let (row, col) = (i / cols, i % cols);
        let sq = Rect::from_min_size(
            origin + vec2(col as f32 * side, row as f32 * side),
            vec2(side, side),
        )
        .shrink(gap);
        p.rect_filled(sq, (side * 0.08).min(1.5), fill(i));
    }
    Rect::from_min_size(origin, vec2(side * cols as f32, side * rows as f32))
}

/// Outline one cell of a grid made by [`unit_grid`] (the "one piece" highlight).
pub fn outline_cell(p: &Painter, pal: &Palette, grid: Rect, cols: usize, i: usize) {
    let side = grid.width() / cols as f32;
    let (row, col) = (i / cols, i % cols);
    let r = Rect::from_min_size(
        grid.min + vec2(col as f32 * side, row as f32 * side),
        vec2(side, side),
    );
    p.rect_stroke(r, 2.0, Stroke::new(2.2, pal.strong), StrokeKind::Outside);
}

/// Bracket under or over a span, with a label (DESIGN.md "pieces bar").
pub fn bracket(p: &Painter, x0: f32, x1: f32, y: f32, up: bool, c: Color32, label: String) {
    let d = if up { -6.0 } else { 6.0 };
    let s = Stroke::new(1.4, c);
    p.line_segment([pos2(x0, y), pos2(x0, y + d)], s);
    p.line_segment([pos2(x1, y), pos2(x1, y + d)], s);
    p.line_segment([pos2(x0, y + d), pos2(x1, y + d)], s);
    let (ty, anchor) = if up {
        (y + d - 3.0, Align2::CENTER_BOTTOM)
    } else {
        (y + d + 3.0, Align2::CENTER_TOP)
    };
    text(p, pos2((x0 + x1) / 2.0, ty), anchor, label, 14.0, c);
}
