//! Powers of ten, built from puzzle cubes.
//!
//! The model: every puzzle cube (a standard 3×3 twisty cube, 5.7 cm) sits in the
//! middle of its own invisible 10 cm box. That leaves 4.3 cm of air between
//! neighbours, and makes the arithmetic round: 10 boxes in a row are exactly 1 m,
//! 1000 boxes make a 1 m cube, and the pattern repeats at every level.

/// Edge of a standard 3×3 puzzle cube (the classic size is 57 mm; speed cubes ~55–56 mm).
pub const CUBE_EDGE_M: f64 = 0.057;
/// Each cube sits in its own box of this size.
pub const PITCH_M: f64 = 0.10;
/// Air between two neighbouring cubes.
pub const GAP_M: f64 = PITCH_M - CUBE_EDGE_M;
/// A typical 3×3 puzzle cube weighs 70–100 g; we use 80 g.
pub const CUBE_MASS_KG: f64 = 0.080;
/// A ~10-year-old, for scale.
pub const KID_HEIGHT_M: f64 = 1.40;

/// How 10^e puzzle cubes (e ≥ 0) are arranged: blocks of 10^(3·level) cubes,
/// laid out as one block, a row of 10, or a 10×10 slab.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arrangement {
    pub exponent: u32,
    /// Each block holds 10^(3·level) cubes. Level 0 = a single cube.
    pub level: u32,
    /// Blocks along x, y (z is always 1).
    pub blocks: (u32, u32),
    /// Side of one block's box in metres (0.1 m × 10^level).
    pub block_box_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Cube,
    Row,
    Slab,
}

impl Arrangement {
    pub fn new(exponent: u32) -> Self {
        let level = exponent / 3;
        let blocks = match exponent % 3 {
            0 => (1, 1),
            1 => (10, 1),
            _ => (10, 10),
        };
        Self {
            exponent,
            level,
            blocks,
            block_box_m: PITCH_M * 10f64.powi(level as i32),
        }
    }

    pub fn shape(&self) -> Shape {
        match self.blocks {
            (1, 1) => Shape::Cube,
            (_, 1) => Shape::Row,
            _ => Shape::Slab,
        }
    }

    pub fn cube_count(&self) -> f64 {
        10f64.powi(self.exponent as i32)
    }

    /// Size of the whole arrangement's boxes (x, y, z) in metres.
    pub fn extent_m(&self) -> (f64, f64, f64) {
        (
            self.blocks.0 as f64 * self.block_box_m,
            self.blocks.1 as f64 * self.block_box_m,
            self.block_box_m,
        )
    }

    pub fn mass_kg(&self) -> f64 {
        self.cube_count() * CUBE_MASS_KG
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_cubes_make_a_metre() {
        let a = Arrangement::new(1);
        assert_eq!(a.shape(), Shape::Row);
        assert!((a.extent_m().0 - 1.0).abs() < 1e-12);
        assert!((GAP_M - 0.043).abs() < 1e-12);
    }

    #[test]
    fn thousand_cubes_make_a_metre_cube() {
        let a = Arrangement::new(3);
        assert_eq!(a.shape(), Shape::Cube);
        let (x, y, z) = a.extent_m();
        assert!((x - 1.0).abs() < 1e-12 && (y - 1.0).abs() < 1e-12 && (z - 1.0).abs() < 1e-12);
        assert!((a.mass_kg() - 80.0).abs() < 1e-9);
    }

    #[test]
    fn a_quintillion_reaches_space() {
        let a = Arrangement::new(18);
        assert!((a.extent_m().2 - 100_000.0).abs() < 1e-6);
    }

    #[test]
    fn counts_match_layout() {
        for e in 0..=18 {
            let a = Arrangement::new(e);
            let per_block = 10f64.powi(3 * a.level as i32);
            let n = per_block * a.blocks.0 as f64 * a.blocks.1 as f64;
            assert!((n / a.cube_count() - 1.0).abs() < 1e-12, "e = {e}");
        }
    }
}
