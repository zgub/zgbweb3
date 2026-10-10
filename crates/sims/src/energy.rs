//! Energy ladder: everyday and not-so-everyday energies, and units to count them in.
//!
//! Every example carries its assumptions in `note`, so a curious reader (or a
//! teacher) can check the arithmetic. Numbers are order-of-magnitude honest, not
//! lab-grade.

use serde::{Deserialize, Serialize};

/// 1 electronvolt in joules (exact since the 2019 SI redefinition).
pub const ELECTRONVOLT_J: f64 = 1.602_176_634e-19;
/// Boltzmann constant, J/K (exact).
pub const BOLTZMANN_J_PER_K: f64 = 1.380_649e-23;
/// Planck constant, J·s (exact).
pub const PLANCK_J_S: f64 = 6.626_070_15e-34;
/// Speed of light, m/s (exact).
pub const LIGHT_M_PER_S: f64 = 299_792_458.0;
/// Standard gravity, m/s².
pub const G0: f64 = 9.806_65;

/// One sugar cube: ~4 g of sucrose at ~16.5 kJ/g.
pub const SUGAR_CUBE_J: f64 = 66_000.0;
/// A flying mosquito: ~2 mg at ~0.4 m/s (≈ 1.5 km/h). Comes out at ≈ 1 TeV.
pub const MOSQUITO_J: f64 = 0.5 * 2.0e-6 * 0.4 * 0.4;

/// Kinetic energy ½·m·v², SI units.
pub fn kinetic(mass_kg: f64, speed_m_s: f64) -> f64 {
    0.5 * mass_kg * speed_m_s * speed_m_s
}

/// Potential energy of lifting `mass_kg` by `height_m` near Earth's surface.
pub fn lift(mass_kg: f64, height_m: f64) -> f64 {
    mass_kg * G0 * height_m
}

/// Energy of one photon of the given wavelength.
pub fn photon(wavelength_m: f64) -> f64 {
    PLANCK_J_S * LIGHT_M_PER_S / wavelength_m
}

pub fn kmh(speed_km_h: f64) -> f64 {
    speed_km_h / 3.6
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EnergyUnit {
    #[default]
    Joule,
    Electronvolt,
    Kilocalorie,
    KilowattHour,
    SugarCube,
    Mosquito,
}

impl EnergyUnit {
    pub const ALL: [EnergyUnit; 6] = [
        EnergyUnit::Joule,
        EnergyUnit::Electronvolt,
        EnergyUnit::Kilocalorie,
        EnergyUnit::KilowattHour,
        EnergyUnit::SugarCube,
        EnergyUnit::Mosquito,
    ];

    /// How many joules one of this unit is.
    pub fn joules(self) -> f64 {
        match self {
            EnergyUnit::Joule => 1.0,
            EnergyUnit::Electronvolt => ELECTRONVOLT_J,
            EnergyUnit::Kilocalorie => 4184.0,
            EnergyUnit::KilowattHour => 3.6e6,
            EnergyUnit::SugarCube => SUGAR_CUBE_J,
            EnergyUnit::Mosquito => MOSQUITO_J,
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            EnergyUnit::Joule => "J",
            EnergyUnit::Electronvolt => "eV",
            EnergyUnit::Kilocalorie => "kcal",
            EnergyUnit::KilowattHour => "kWh",
            EnergyUnit::SugarCube => "sugar cubes",
            EnergyUnit::Mosquito => "mosquitoes",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            EnergyUnit::Joule => "joules",
            EnergyUnit::Electronvolt => "electronvolts",
            EnergyUnit::Kilocalorie => "kilocalories (food calories)",
            EnergyUnit::KilowattHour => "kilowatt-hours (electricity bill)",
            EnergyUnit::SugarCube => "sugar cubes (eaten)",
            EnergyUnit::Mosquito => "flying mosquitoes",
        }
    }

    /// Whether SI prefixes make sense for this unit (kJ yes, "k sugar cubes" no).
    pub fn takes_prefix(self) -> bool {
        matches!(self, EnergyUnit::Joule | EnergyUnit::Electronvolt)
    }

    pub fn convert(self, joules: f64) -> f64 {
        joules / self.joules()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnergyExample {
    pub id: &'static str,
    pub name: &'static str,
    pub joules: f64,
    /// The assumptions behind the number.
    pub note: &'static str,
}

/// The built-in ladder, sorted from smallest to largest.
pub fn examples() -> Vec<EnergyExample> {
    let mut v = vec![
        EnergyExample {
            id: "thermal",
            name: "Jiggle of one air molecule (kT, 25 °C)",
            joules: BOLTZMANN_J_PER_K * 298.15,
            note: "Boltzmann constant × room temperature: the typical thermal energy per degree of freedom scale.",
        },
        EnergyExample {
            id: "photon",
            name: "One photon of green light",
            joules: photon(530e-9),
            note: "Wavelength 530 nm, E = h·c/λ ≈ 2.3 eV.",
        },
        EnergyExample {
            id: "mosquito",
            name: "A flying mosquito",
            joules: MOSQUITO_J,
            note: "2 mg at 0.4 m/s (about 1.5 km/h). Roughly one tera-electronvolt.",
        },
        EnergyExample {
            id: "lhc",
            name: "One proton in the LHC",
            joules: 6.8e12 * ELECTRONVOLT_J,
            note: "6.8 TeV per proton (LHC Run 3 beam energy). A few mosquitoes' worth — in one particle.",
        },
        EnergyExample {
            id: "apple",
            name: "Lifting an apple by one metre",
            joules: lift(0.102, 1.0),
            note: "A 102 g apple raised 1 m: about one joule. Handy picture for '1 J'.",
        },
        EnergyExample {
            id: "tennis",
            name: "A fast tennis serve",
            joules: kinetic(0.058, kmh(200.0)),
            note: "58 g ball at 200 km/h.",
        },
        EnergyExample {
            id: "aa",
            name: "An AA battery",
            joules: 1.2 * 2.5 * 3600.0,
            note: "Alkaline AA: ~2.5 Ah at ~1.2 V average ≈ 3 Wh.",
        },
        EnergyExample {
            id: "sugar",
            name: "One sugar cube (eaten)",
            joules: SUGAR_CUBE_J,
            note: "4 g sucrose × ~16.5 kJ/g.",
        },
        EnergyExample {
            id: "car50",
            name: "Family car at 50 km/h",
            joules: kinetic(1500.0, kmh(50.0)),
            note: "1500 kg at town speed.",
        },
        EnergyExample {
            id: "car130",
            name: "Family car at 130 km/h",
            joules: kinetic(1500.0, kmh(130.0)),
            note: "Same car on the motorway: 2.6× the speed, 6.8× the energy (speed is squared).",
        },
        EnergyExample {
            id: "food_day",
            name: "A grown-up's food for one day",
            joules: 2000.0 * 4184.0,
            note: "2000 kcal.",
        },
        EnergyExample {
            id: "lightning",
            name: "A lightning bolt",
            joules: 1.0e9,
            note: "Very rough: a typical bolt is quoted around one gigajoule; real bolts vary a lot.",
        },
        EnergyExample {
            id: "b747_fuel",
            name: "Full fuel tanks of a Boeing 747-400",
            joules: 216_840.0 * 0.80 * 43.0e6,
            note: "216 840 L of jet fuel × 0.80 kg/L × 43 MJ/kg (chemical energy).",
        },
        EnergyExample {
            id: "iss",
            name: "The ISS flying in orbit (kinetic)",
            joules: kinetic(420_000.0, 7_660.0),
            note: "420 t at 7.66 km/s. More than a full jumbo jet's fuel.",
        },
    ];
    v.sort_by(|a, b| a.joules.total_cmp(&b.joules));
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mosquito_is_about_a_tev() {
        let tev = MOSQUITO_J / (1e12 * ELECTRONVOLT_J);
        assert!((0.5..2.0).contains(&tev), "mosquito = {tev} TeV");
    }

    #[test]
    fn iss_beats_jumbo_fuel() {
        let ex = examples();
        let get = |id| ex.iter().find(|e| e.id == id).unwrap().joules;
        let ratio = get("iss") / get("b747_fuel");
        assert!((1.4..2.0).contains(&ratio), "ratio = {ratio}");
    }

    #[test]
    fn units_round_trip() {
        for u in EnergyUnit::ALL {
            let j = 12_345.0;
            assert!((u.convert(j) * u.joules() - j).abs() < 1e-6);
        }
    }

    #[test]
    fn ids_unique_and_sorted() {
        let ex = examples();
        for w in ex.windows(2) {
            assert!(w[0].joules <= w[1].joules);
        }
        let mut ids: Vec<_> = ex.iter().map(|e| e.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), ex.len());
    }
}
