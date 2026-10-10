//! Number formatting for humans: SI prefixes and scientific notation.

/// One SI prefix, e.g. kilo = 10³.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prefix {
    pub exponent: i32,
    pub symbol: &'static str,
    pub name: &'static str,
}

/// All SI prefixes, quecto (10⁻³⁰) to quetta (10³⁰), ascending.
pub const SI_PREFIXES: [Prefix; 25] = [
    Prefix {
        exponent: -30,
        symbol: "q",
        name: "quecto",
    },
    Prefix {
        exponent: -27,
        symbol: "r",
        name: "ronto",
    },
    Prefix {
        exponent: -24,
        symbol: "y",
        name: "yocto",
    },
    Prefix {
        exponent: -21,
        symbol: "z",
        name: "zepto",
    },
    Prefix {
        exponent: -18,
        symbol: "a",
        name: "atto",
    },
    Prefix {
        exponent: -15,
        symbol: "f",
        name: "femto",
    },
    Prefix {
        exponent: -12,
        symbol: "p",
        name: "pico",
    },
    Prefix {
        exponent: -9,
        symbol: "n",
        name: "nano",
    },
    Prefix {
        exponent: -6,
        symbol: "µ",
        name: "micro",
    },
    Prefix {
        exponent: -3,
        symbol: "m",
        name: "milli",
    },
    Prefix {
        exponent: -2,
        symbol: "c",
        name: "centi",
    },
    Prefix {
        exponent: -1,
        symbol: "d",
        name: "deci",
    },
    Prefix {
        exponent: 0,
        symbol: "",
        name: "",
    },
    Prefix {
        exponent: 1,
        symbol: "da",
        name: "deca",
    },
    Prefix {
        exponent: 2,
        symbol: "h",
        name: "hecto",
    },
    Prefix {
        exponent: 3,
        symbol: "k",
        name: "kilo",
    },
    Prefix {
        exponent: 6,
        symbol: "M",
        name: "mega",
    },
    Prefix {
        exponent: 9,
        symbol: "G",
        name: "giga",
    },
    Prefix {
        exponent: 12,
        symbol: "T",
        name: "tera",
    },
    Prefix {
        exponent: 15,
        symbol: "P",
        name: "peta",
    },
    Prefix {
        exponent: 18,
        symbol: "E",
        name: "exa",
    },
    Prefix {
        exponent: 21,
        symbol: "Z",
        name: "zetta",
    },
    Prefix {
        exponent: 24,
        symbol: "Y",
        name: "yotta",
    },
    Prefix {
        exponent: 27,
        symbol: "R",
        name: "ronna",
    },
    Prefix {
        exponent: 30,
        symbol: "Q",
        name: "quetta",
    },
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
}
