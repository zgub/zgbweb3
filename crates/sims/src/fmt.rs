//! Number formatting for humans: SI prefixes, scientific notation, lengths, masses.
//!
//! Output always uses `.` as the decimal point; the app localises it
//! (`I18n::num`). Prefix *names* (kilo, mili, …) live in the language catalogs.

/// One SI prefix, e.g. kilo = 10³.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prefix {
    pub exponent: i32,
    pub symbol: &'static str,
}

const fn p(exponent: i32, symbol: &'static str) -> Prefix {
    Prefix { exponent, symbol }
}

/// All SI prefixes, quecto (10⁻³⁰) to quetta (10³⁰), ascending. Exponent 0 = no prefix.
pub const SI_PREFIXES: [Prefix; 25] = [
    p(-30, "q"),
    p(-27, "r"),
    p(-24, "y"),
    p(-21, "z"),
    p(-18, "a"),
    p(-15, "f"),
    p(-12, "p"),
    p(-9, "n"),
    p(-6, "µ"),
    p(-3, "m"),
    p(-2, "c"),
    p(-1, "d"),
    p(0, ""),
    p(1, "da"),
    p(2, "h"),
    p(3, "k"),
    p(6, "M"),
    p(9, "G"),
    p(12, "T"),
    p(15, "P"),
    p(18, "E"),
    p(21, "Z"),
    p(24, "Y"),
    p(27, "R"),
    p(30, "Q"),
];

/// Prefix for an exact power of ten, if SI has one.
pub fn prefix_for_exponent(exponent: i32) -> Option<Prefix> {
    SI_PREFIXES.iter().copied().find(|p| p.exponent == exponent)
}

/// Split a value into mantissa and an engineering prefix (multiples of 10³ only),
/// so that 1 ≤ |mantissa| < 1000 whenever the value is in prefix range.
pub fn engineering(value: f64) -> (f64, Prefix) {
    let unit = prefix_for_exponent(0).expect("unit prefix exists");
    if value == 0.0 || !value.is_finite() {
        return (value, unit);
    }
    let exp3 = ((value.abs().log10() / 3.0).floor() as i32 * 3).clamp(-30, 30);
    let prefix = prefix_for_exponent(exp3).unwrap_or(unit);
    (value / 10f64.powi(exp3), prefix)
}

/// "12.3 kJ" style, with `sig` significant digits.
pub fn si(value: f64, unit: &str, sig: usize) -> String {
    let (m, p) = engineering(value);
    format!("{} {}{}", sig_digits(m, sig), p.symbol, unit)
}

/// Lengths in the unit a person would use: "1 cm", "46.4 cm", "10 km", "2.15 µm".
pub fn length(meters: f64, sig: usize) -> String {
    const UNITS: [(f64, &str); 6] = [
        (1e3, "km"),
        (1.0, "m"),
        (1e-2, "cm"),
        (1e-3, "mm"),
        (1e-6, "µm"),
        (1e-9, "nm"),
    ];
    let unit = UNITS
        .iter()
        .find(|(size, _)| meters.abs() >= size * 0.9995)
        .unwrap_or(&UNITS[UNITS.len() - 1]);
    format!("{} {}", sig_digits(meters / unit.0, sig), unit.1)
}

/// Masses the way people say them: "80 g", "8 kg", "80 t", "80 000 t", "8 × 10⁷ t".
pub fn mass(kg: f64, sig: usize) -> String {
    if kg < 1.0 {
        format!("{} g", sig_digits(kg * 1000.0, sig))
    } else if kg < 1000.0 {
        format!("{} kg", sig_digits(kg, sig))
    } else {
        let t = kg / 1000.0;
        if t < 1e6 {
            format!("{} t", group_thousands(&sig_digits(t, sig)))
        } else {
            format!("{} t", sci(t, sig))
        }
    }
}

/// "1000000" → "1 000 000", using a thin space (U+2009) between groups.
/// Only the integer part is grouped.
pub fn group_thousands(s: &str) -> String {
    let (int, rest) = match s.find('.') {
        Some(i) => s.split_at(i),
        None => (s, ""),
    };
    let (sign, digits) = int.strip_prefix('-').map_or(("", int), |d| ("-", d));
    if digits.len() <= 4 {
        // "1000" stays compact, like most style guides.
        return s.to_string();
    }
    let mut out = String::from(sign);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push('\u{2009}');
        }
        out.push(c);
    }
    out + rest
}

/// "1.23 × 10¹³" style. Values in [0.01, 10000) are written plainly.
pub fn sci(value: f64, sig: usize) -> String {
    if value == 0.0 || !value.is_finite() {
        return format!("{value}");
    }
    let exp = value.abs().log10().floor() as i32;
    if (-2..4).contains(&exp) {
        return sig_digits(value, sig);
    }
    let mantissa = value / 10f64.powi(exp);
    format!("{} × 10{}", sig_digits(mantissa, sig), superscript(exp))
}

/// A number that picks the right plural form for the text [`sci`] shows:
/// "1 cube", "3.4 cubes", "2 × 10⁻⁵ of a cube" (a fraction), "1.2 × 10⁷ cubes" (whole).
pub fn sci_count(value: f64, sig: usize) -> f64 {
    if value == 0.0 || !value.is_finite() {
        return value;
    }
    let exp = value.abs().log10().floor() as i32;
    if (-2..4).contains(&exp) {
        sig_digits(value, sig).parse().unwrap_or(value)
    } else if exp < 0 {
        0.5 // tiny: grammatically a fraction
    } else {
        value.round()
    }
}

/// Round to `sig` significant digits and drop trailing zeros.
pub fn sig_digits(value: f64, sig: usize) -> String {
    if value == 0.0 || !value.is_finite() {
        return format!("{value}");
    }
    let sig = sig.max(1) as i32;
    let exp = value.abs().log10().floor() as i32;
    let decimals = (sig - 1 - exp).max(0) as usize;
    let s = format!("{value:.decimals$}");
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

/// 13 → "¹³", -21 → "⁻²¹".
pub fn superscript(n: i32) -> String {
    n.to_string()
        .chars()
        .map(|c| match c {
            '-' => '⁻',
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            other => other,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes_are_sorted_and_unique() {
        for w in SI_PREFIXES.windows(2) {
            assert!(w[0].exponent < w[1].exponent);
        }
    }

    #[test]
    fn engineering_picks_kilo_and_milli() {
        let (m, p) = engineering(12_345.0);
        assert_eq!(p.symbol, "k");
        assert!((m - 12.345).abs() < 1e-9);
        let (m, p) = engineering(0.004_2);
        assert_eq!(p.symbol, "m");
        assert!((m - 4.2).abs() < 1e-9);
    }

    #[test]
    fn formatting() {
        assert_eq!(si(66_000.0, "J", 2), "66 kJ");
        assert_eq!(sci(1.232e13, 3), "1.23 × 10¹³");
        assert_eq!(sci(4.1e-21, 2), "4.1 × 10⁻²¹");
        assert_eq!(sci(87.7, 3), "87.7");
        assert_eq!(superscript(-30), "⁻³⁰");
        assert_eq!(length(0.01, 3), "1 cm");
        assert_eq!(length(0.0215443, 3), "2.15 cm");
        assert_eq!(length(0.1, 3), "10 cm");
        assert_eq!(length(10_000.0, 3), "10 km");
        assert_eq!(length(1e-6, 3), "1 µm");
    }

    #[test]
    fn masses() {
        assert_eq!(mass(0.08, 2), "80 g");
        assert_eq!(mass(80.0, 2), "80 kg");
        assert_eq!(mass(80_000.0, 2), "80 t");
        assert_eq!(mass(8.0e7, 2), "80\u{2009}000 t");
        assert_eq!(mass(8.0e10, 2), "8 × 10⁷ t");
    }

    #[test]
    fn grouping() {
        assert_eq!(group_thousands("1000"), "1000");
        assert_eq!(group_thousands("1000000"), "1\u{2009}000\u{2009}000");
        assert_eq!(group_thousands("12345.6"), "12\u{2009}345.6");
    }

    #[test]
    fn counts_for_plurals() {
        assert_eq!(sci_count(1.0000001, 3), 1.0);
        assert_eq!(sci_count(3.4, 2), 3.4);
        assert_eq!(sci_count(1.23e13, 3), 1.23e13);
        assert_eq!(sci_count(6.24e-26, 3), 0.5);
    }
}
