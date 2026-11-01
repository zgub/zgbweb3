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
    fill: &impl Fn(usize) -> Color32,
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

/// The hundred squares (DESIGN.md "unit grid"): 10 × 10 in `area`, top-left,
/// each square showing `label` (what one square is worth) when it fits.
/// Text is dark on bright squares and soft on dark ones. Returns the grid's rect.
pub fn hundred_squares(
    p: &Painter,
    pal: &Palette,
    area: Rect,
    fill: impl Fn(usize) -> Color32,
    label: Option<&str>,
) -> Rect {
    let grid = unit_grid(p, area, 10, 10, 1.5, &fill);
    let Some(label) = label.filter(|l| !l.is_empty()) else {
        return grid;
    };
    let side = grid.width() / 10.0;
    // largest size up to 14 pt that fits inside a square; skip if it would be tiny
    let mut size = (side * 0.42).min(14.0);
    let fits = |size: f32| {
        p.layout_no_wrap(label.to_owned(), FontId::proportional(size), pal.text)
            .size()
            .x
            <= side - 5.0
    };
    while size > 8.0 && !fits(size) {
        size -= 0.5;
    }
    if !fits(size) {
        return grid;
    }
    for i in 0..100 {
        let c = fill(i);
        let bright = 0.299 * c.r() as f32 + 0.587 * c.g() as f32 + 0.114 * c.b() as f32 > 110.0;
        let tc = if bright {
            pal.deep
        } else {
            lerp(c, pal.text, 0.55)
        };
        let center = grid.min
            + vec2(
                ((i % 10) as f32 + 0.5) * side,
                ((i / 10) as f32 + 0.5) * side,
            );
        text(p, center, Align2::CENTER_CENTER, label, size, tc);
    }
    grid
}
