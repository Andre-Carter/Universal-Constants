use std::fmt;
#[derive(Debug)]
pub struct Constant {
    pub name: &'static str,
    pub value: f64,
    pub si_base_units: Option<&'static str>,
    pub uncertainty: Option<f64>,
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let green = "\x1b[32m";
        let reset = "\x1b[0m";

        writeln!(f, "{}Constant: {}{}\n", green, self.name, reset)?;

        writeln!(f, "Value: {}\n", self.value)?;

        if let Some(si_base_units) = self.si_base_units {
            writeln!(f, "SI base units: {}\n", si_base_units )?;
        } else {
            writeln!(f, "SI base units: Dimensionless\n", )?;
        }

        if let Some(uncertainty) = self.uncertainty {
            writeln!(f, "Uncertainty: {}\n", uncertainty)?;
        } else {
            writeln!(f, "Uncertainty: Exact\n")?; //write Exact
        }

        //write!(f, "{}", reset);

        Ok(())
    }
}

// START OF ALPHABETICAL DATA FROM NIST https://pml.nist.gov/cuu/Constants/Table/allascii.txt
pub fn initialize_constants() -> Vec<Constant> { 
vec![
    Constant {
        name: "alpha particle-electron mass ratio",
        value: 7_294.299_541_71,
        si_base_units: None,
        uncertainty: Some(0.000_000_17),
    },   

    Constant {
        name: "alpha particle mass",
        value: 6.644_657_345_0e-27,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_002_1e-27),
    },   

    Constant {
        name: "alpha particle mass energy equivalent",
        value: 5.971_920_199_7e-10,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_0019e-10),
    },  

    Constant {
        name: "alpha particle mass energy equivalent in MeV",
        value: 3_727.379_411_8,
        si_base_units: Some("MeV"),
        uncertainty: Some(0.000_001_2),
    },  

    Constant {
        name: "alpha particle mass in u",
        value: 4.001_506_179_129,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_000_062),
    },  

    Constant {
        name: "alpha particle molar mass",
        value: 4.001_506_183_3e-3,
        si_base_units: Some("kg mol^-1"),
        uncertainty: Some(0.000_000_001_2e-3),
    },  

    Constant {
        name: "alpha particle-proton mass ratio",
        value: 3.972_599_690_252,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_07),
    },  

    Constant {
        name: "alpha particle relative atomic mass",
        value: 4.001_506_179_129,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_062),
    },  

    Constant {
        name: "alpha particle rms charge raduis",
        value: 1.678_5e-15,
        si_base_units: Some("m"),
        uncertainty: Some(0.002_1e-15),
    },  

    Constant {
        name: "Angstrom star", // A is capitalized on nist
        value: 1.000_014_95e-10,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_9e-10),
    },  

    Constant {
        name: "atomic mass constant",
        value: 1.660_539_068_92e-27,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_000_52e-27),
    },

    Constant {
        name: "atomic mass constant energy equivalent",
        value: 1.492_418_087_68e-10,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_000_46e-10),
    },

    Constant {
        name: "atomic mass constant energy equivalent",
        value: 931.494_103_72,
        si_base_units: Some("MeV"),
        uncertainty: Some(0.000_000_29),
    },

    Constant {
        name: "atomic mass unit-electron volt relationship",
        value: 9.314_941_037_2e8,
        si_base_units: Some("eV"),
        uncertainty: Some(0.000_000_002_9e8),
    },

    Constant {
        name: "atomic mass unit-hartree relationship",
        value: 3.423_177_692_2e7,
        si_base_units: Some("E_h"),
        uncertainty: Some(0.000_000_001_1e7),
    },

    Constant {
        name: "atomic mass unit-hertz relationship",
        value: 2.252_342_721_85e23,
        si_base_units: Some("Hz"),
        uncertainty: Some(0.000_000_000_7e23),
    },

    Constant {
        name: "atomic mass unit-inverse meter relationship",
        value: 7.513_006_620_9e14,
        si_base_units: Some("m^-1"),
        uncertainty: Some(0.000_000_002_3e14),
    },

    Constant {
        name: "atomic mass unit-joule relationship",
        value: 1.492_418_087_68e-10,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_000_46e-10),
    },

    Constant {
        name: "atomic mass unit-kelvin relationship",
        value: 1.080_954_020_67e13,
        si_base_units: Some("K"),
        uncertainty: Some(0.000_000_000_34e13),
    },

    Constant {
        name: "atomic mass unit-kilogram relationship",
        value: 1.660_539_068_92e-27,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_000_52e-27),
    },

    Constant {
        name: "atomic unit of 1st hyperpolarizability",
        value: 3.206_361_299_6e-53,
        si_base_units: Some("C^3 m^3 J^-2"),
        uncertainty: Some(0.000_000_001_5e-53),
    },

    Constant {
        name: "atomic unit of 2nd hyperpolarizability",
        value: 6.235_379_973_5e-65,
        si_base_units: Some("C^4 m^4 J^-3"),
        uncertainty: Some(0.000_000_003_9e-65),
    },

    Constant {
        name: "atomic unit of action",
        value: 1.054_571_817e-34,//...
        si_base_units: Some("J s"),
        uncertainty: None,
    },

    Constant {
        name: "atomic unit of charge",
        value: 1.602_176_634e-19,
        si_base_units: Some("C"),
        uncertainty: None,
    },

    Constant {
        name: "atomic unit of charge density",
        value: 1.081_202_386_77e12,
        si_base_units: Some("C m^-3"),
        uncertainty: Some(0.000_000_000_51e12),
    },

    Constant {
        name: "atomic unit of current",
        value: 6.623_618_237_508_2e-3,
        si_base_units: Some("A"),
        uncertainty: Some(0.000_000_000_007_2e-3),
    },

    Constant {
        name: "atomic unit of electic dipole mom.", //mom. is short for moment : to be decided if should use moment or follow nist mom.
        value: 8.478_353_619_8e-30,
        si_base_units: Some("C m"),
        uncertainty: Some(0.000_000_001_3e-30),
    },

    Constant {
        name: "atomic unit of electric field",
        value: 5.142_206_751_12e11,
        si_base_units: Some("V m^-1"),
        uncertainty: Some(0.000_000_000_80e11),
    },

    Constant {
        name: "atomic unit electrical field gradient",
        value: 9.717_362_442_4e21,
        si_base_units: Some("V m^-2"),
        uncertainty: Some(0.000_000_003e21),
    },

    Constant {
        name: "atomic unit of electric polarizability",
        value: 1.648_777_272_12e-41,
        si_base_units: Some("C^2 m^2 J^-1"),
        uncertainty: Some(0.000_000_000_51e-41),
    },

    Constant {
        name: "atomic unit of electric potential",
        value: 27.211_386_245_981,
        si_base_units: Some("V"),
        uncertainty: Some(0.000_000_000_03),
    },

    Constant {
        name: "atomic unit of electric quadrupole mom.", //
        value: 4.486_551_518_5e-40,
        si_base_units: Some("C m^2"),
        uncertainty: Some(0.000_000_001_4e-40),
    },

    Constant {
        name: "atomic unit of energy",
        value: 4.359_744_722_206e-18,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_000_004_8e-18),
    },

    Constant {
        name: "atomic unit of force",
        value: 8.238_723_503_8e-8,
        si_base_units: Some("N"),
        uncertainty: Some(0.000_000_001_3e-8),
    },

    Constant {
        name: "atomic unit of length",
        value: 5.291_722_105_44e-11,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_000_82e-11),
    },

    Constant {
        name: "atomic unit of mag. dipole mom.",
        value: 1.854_802_013_15e-23,
        si_base_units: Some("J T^-1"),
        uncertainty: Some(0.000_000_000_58e-23),
    },

    Constant {
        name: "atomic unit of mag. flux density",
        value: 2.350_517_570_77e5,
        si_base_units: Some("T"),
        uncertainty: Some(0.000_000_000_73e5),
    },

    Constant {
        name: "atomic unit of magnetizability",
        value: 2.350_517_570_77_e5,
        si_base_units: Some("T"),
        uncertainty: Some(0.000_000_000_73e5),
    },

    Constant {
        name: "atomic unit of mass",
        value: 9.109_383_713_9e-31,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_002_8e-31),
    },

    Constant {
        name: "atomic unit of momentum",
        value: 1.992_851_915_45e-24,
        si_base_units: Some("kg m s^-1"),
        uncertainty: Some(0.000_000_000_31e-24),
    },

    Constant {
        name: "atomic unit of permittivitty",
        value: 1.112_650_056_20e-10,
        si_base_units: Some("F m^-1"),
        uncertainty: Some(0.000_000_000_17),
    },

    Constant {
        name: "atomic unit of time",
        value: 2.418_884_326_586_4e-17,
        si_base_units: Some("s"),
        uncertainty: Some(0.000_000_000_002_6e-17),
    },

    Constant {
        name: "atomic unit of velocity",
        value: 2.187_691_262_16e6,
        si_base_units: Some("m s^-1"),
        uncertainty: Some(0.000_000_000_34e6),
    },

    Constant {
        name: "Avogadro constant", //
        value: 6.022_140_76e23,
        si_base_units: Some("mol^-1"),
        uncertainty: None,
    },

    Constant {
        name: "Bohr magneton",
        value: 9.274_010_065_7e-24,
        si_base_units: Some("J T^-1"),
        uncertainty: Some(0.000_000_002_9e-24),
    },

    Constant {
        name: "Bohr magneton in eV/T",
        value: 5.788_381_798_2e-5,
        si_base_units: Some("eV T^-1"),
        uncertainty: Some(0.000_000_001_8e-5),
    },

    Constant {
        name: "Bohr magneton in Hz/T",
        value: 1.399_624_491_71e10,
        si_base_units: Some("Hz T^-1"),
        uncertainty: Some(0.000_000_000_44e10),
    },

    Constant {
        name: "Bohr magneton in inverse meter per tesla",
        value: 46.686_447_719,
        si_base_units: Some("m^-1 T^-1"),
        uncertainty: Some(0.000_000_015),
    },

    Constant {
        name: "Bohr magneton in K/T",
        value: 0.671_713_814_72,
        si_base_units: Some("K T^-1"),
        uncertainty: Some(0.000_000_000_21),
    },

    Constant {
        name: "Bohr radius",
        value: 5.291_772_105_44e-11,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_000_82e-11),
    },

    Constant {
        name: "Boltzmann constant",
        value: 1.380_649e-23,
        si_base_units: Some("J K^-1"),
        uncertainty: None,
    },

    Constant {
        name: "Boltzmann constant in eV/K",
        value: 8.617_333_262e-5,//...
        si_base_units: Some("eV K^-1"),
        uncertainty: None,
    },

    Constant {
        name: "Boltzmann constant in Hz/K",
        value: 2.083_661_912e10,//...
        si_base_units: Some("Hz K^-1"),
        uncertainty: None,
    },

    Constant {
        name: "Boltzmann constant in meter per kelvin",
        value: 69.503_480_04,//...
        si_base_units: Some("m^-1 K^-1"),
        uncertainty: None,
    },

    Constant {
        name: "characteristic of impedance of vacuum",
        value: 376.730_313_412,
        si_base_units: Some("ohm"),
        uncertainty: Some(0.000_000_059),
    },

    Constant {
        name: "classical electron radius",
        value: 2.817_940_320_5e-15,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_001_3e-15),
    },

    Constant {
        name: "Compton wavelength",
        value: 2.426_310_235_38e-12,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_000_76e-12),
    },

    Constant {
        name: "conductance quantum",
        value: 7.748_091_729e-5,//...
        si_base_units: Some("S"),
        uncertainty: None,
    },

    Constant {
        name: "conventional value of ampere-90",
        value: 1.000_000_088_87,//...
        si_base_units: Some("A"),
        uncertainty: None,
    },

    Constant {
        name: "conventional value of coulomb-90",
        value: 1.000_000_088_87,//...
        si_base_units: Some("C"),
        uncertainty: None,
    },

    Constant {
        name: "convention value of farad-90",
        value: 0.999_999_982_20,//...
        si_base_units: Some("F"),
        uncertainty: None,
    },

    Constant {
        name: "conventional value of henry-90",
        value: 1.000_000_017_79,//...
        si_base_units: Some("H"),
        uncertainty: None,
    },

    Constant {
        name: "conventional value of Josephson constant",
        value: 483_597.9e9,
        si_base_units: Some("Hz V^-1"),
        uncertainty: None,
    },

    Constant {
        name: "conventional value of ohm-90",
        value: 1.000_000_017_79,//...
        si_base_units: Some("ohm"),
        uncertainty: None,
    },

    Constant {
        name: "conventional value of volt-90",
        value: 1.000_000_106_66,//...
        si_base_units: Some("V"),
        uncertainty: None,
    },

    Constant {
        name: "convention value of von Klitzing constant",
        value: 25_812.807,
        si_base_units: Some("ohm"),
        uncertainty: None,
    },

    Constant {
        name: "convention value of watt-90",
        value: 1.000_000_195_53,//...
        si_base_units: Some("W"),
        uncertainty: None,
    },

    Constant {
        name: "Copper x unit",
        value: 1.002_076_97e-13,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_28e-13),
    },

    Constant {
        name: "deuteron-electron mag. mom. ratio",
        value: -4.664_345_550e-4,
        si_base_units: None,
        uncertainty: Some(0.000_000_012e-4),
    },

    Constant {
        name: "deuteron-electron mass ratio",
        value: 367_0.482_967_655,
        si_base_units: None,
        uncertainty: Some(0.000_000_063),
    },

    Constant {
        name: "deutron g factor",
        value: 0.857_438_233_5,
        si_base_units: None,
        uncertainty: Some(0.000_000_002_2),
    },

    Constant {
        name: "deuteron mag. mom.",
        value: 4.330_735_087e-27,
        si_base_units: Some("J T^-1"),
        uncertainty: Some(0.000_000_011e-27),
    },

    Constant {
        name: "deuteron mag. mom. to Bohr magneton ratio",
        value: 4.669_754_568e-4,
        si_base_units: None,
        uncertainty: Some(0.000_000_012e-4),
    },

    Constant {
        name: "deuteron mag. mom. to nuclear magneton ratio",
        value: 0.857_438_233_5,
        si_base_units: None,
        uncertainty: Some(0.000_000_002_2),
    },

    Constant {
        name: "deuteron mass",
        value: 3.343_583_776_8e-27,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_001e-27),
    },

    Constant {
        name: "deuteron mass energy equivalent",
        value: 3.005_063_234_91e-10,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_000_94),
    },

    Constant {
        name: "deuteron mass energy equivalent in MeV",
        value: 187_5.612_945,
        si_base_units: Some("MeV"),
        uncertainty: Some(0.000_000_58),
    },

    Constant {
        name: "deuteron mass in u",
        value: 2.013_553_212_544,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_000_015),
    },

    Constant {
        name: "deuteron molar mass",
        value: 2.013_553_214_66e-3,
        si_base_units: Some("kg mol^-1"),
        uncertainty: Some(0.000_000_000_63e-3),
    },

    Constant {
        name: "deuteron-neutron mag. mom. ratio",
        value: -0.448_206_52,
        si_base_units: None,
        uncertainty: Some(0.000_000_11),
    },

    Constant {
        name: "deuteron-proton mag. mom. ratio",
        value: 0.307_012_209_3,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_79),
    },

    Constant {
        name: "deuteron-proton mass ratio",
        value: 1.999_007_501_269_9,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_008_4),
    },

    Constant {
        name: "deuteron relative atomic mass",
        value: 2.015_533_212_544,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_015),
    },

    Constant {
        name: "deuteron rms charge radius",
        value: 2.127_78e-15,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_27e-15),
    },

    Constant {
        name: "electron charge to mass quotient",
        value: -1.758_820_008_38e11,
        si_base_units: Some("C kg^-1"),
        uncertainty: Some(0.000_000_000_55e11),
    },

    Constant {
        name: "electron-deuteron mag. mom. ratio",
        value: -2_143.923_492_1,
        si_base_units: None,
        uncertainty: Some(0.000_005_6),
    },

    Constant {
        name: "electron-deuteron mass ratio",
        value: 2.724_437_107_629e-4,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_047e-4),
    },

    Constant {
        name: "electron g factor",
        value: -2.002_319_304_360_92,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_000_36),
    },

    Constant {
        name: "electron gyromag. ratio",
        value: 1.760_859_627_84e11,
        si_base_units: Some("s^-1 T^-1"),
        uncertainty: Some(0.000_000_000_55e11),
    },

    Constant {
        name: "electron gyromag. ratio in MHz/T",
        value: 28_024.951_386_1,
        si_base_units: Some("MHz T^-1"),
        uncertainty: Some(0.000_008_7),
    },

    Constant {
        name: "electron-helion mass ratio",
        value: 1.819_543_074_649e-4,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_053e-4),
    },

    Constant {
        name: "electron mag. mom.",
        value: -9.284_764_691_7e-24,
        si_base_units: Some("J T^-1"),
        uncertainty: Some(0.000_000_002_9e-24),
    },

    Constant {
        name: "electron mag. mom. anomaly",
        value: 1.159_652_180_46e-3,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_18e-3),
    },

    Constant {
        name: "electron mag. mom. to Bohr magneton ratio",
        value: -1.001_159_662_180_46,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_000_18),
    },

    Constant {
        name: "electron mag. mom. to nuclear magneton ratio",
        value: -1_838.281_971_877,
        si_base_units: None,
        uncertainty: Some(0.000_000_032),
    },

    Constant {
        name: "electron mass",
        value: 9.109_383_713_9e-31,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_002_8e-31),
    }, 

    Constant {
        name: "electron mass energy equivalent",
        value: 8.187_105_788e-14,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_002_6e-14),
    },

    Constant {
        name: "electron mass energy equivalent in MeV",
        value: 0.510_988_950_69,
        si_base_units: Some("MeV"),
        uncertainty: Some(0.000_000_000_16),
    },

    Constant {
        name: "electron mass in u",
        value: 5.484_799_090_441e-4,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_000_097e-4),
    },

    Constant {
        name: "electron molar mass",
        value: 5.485_799_096_2e-7,
        si_base_units: Some("kg mol^-1"),
        uncertainty: Some(0.000_000_001_7e-7),
    },

    Constant {
        name: "electron-muon mag. mom. ratio",
        value: 206.766_988_1,
        si_base_units: None,
        uncertainty: Some(0.000_004_6),
    },

    Constant {
        name: "electron-neutron mag. mom. ratio",
        value: 960.920_48,
        si_base_units: None,
        uncertainty: Some(0.000_23),
    },

    Constant {
        name: "electron-neutron mass ratio",
        value: 5.438_673_441_6e-4,
        si_base_units: None,
        uncertainty: Some(0.000_000_002_2e-4),
    },

    Constant {
        name: "electron-proton mag. mom ratio",
        value: -658.210_687_89,
        si_base_units: None,
        uncertainty: Some(0.000_000_19),
    },

    Constant {
        name: "electron-proton mass ratio",
        value: 5.446_170_214_889e-4,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_094e-4),
    },

    Constant {
        name: "electron relative atomic mass",
        value: 5.486_799_090_411e-4,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_097e-4),
    },

    Constant {
        name: "electron-tau mass ratio",
        value: 2.875_85e-4,
        si_base_units: None,
        uncertainty: Some(0.000_19e-4),
    },

    Constant {
        name: "electron to alpha particle mass ratio",
        value: 1.370_933_554_733e-4,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_032e-4),
    },

    Constant {
        name: "electron to shielded helion mag. mom. ratio",
        value: 864.058_239_86,
        si_base_units: None,
        uncertainty: Some(0.000_000_7),
    },

    Constant {
        name: "electron to shielded proton mag. mom. ratio",
        value: -658.227_585_6,
        si_base_units: None,
        uncertainty: Some(0.000_002_7),
    },

    Constant {
        name: "electron-triton mass ratio",
        value: 1.819_200_062_327e-4,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_068e-4),
    },

    Constant {
        name: "electron volt",
        value: 1.602_176_634e-19,
        si_base_units: Some("J"),
        uncertainty: None,
    },

    Constant {
        name: "electron volt-atomic mass unit relationship",
        value: 1.073_544_100_83e-9,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_000_33e-9),
    },

    Constant {
        name: "electron volt-hartree relationship",
        value: 3.674_932_217_566_5e-2,
        si_base_units: Some("E_h"),
        uncertainty: Some(0.000_000_000_004e-2),
    },

    Constant {
        name: "electron volt-hertz relationship",
        value: 2.417_989_242e14,//...
        si_base_units: Some("Hz"),
        uncertainty: None,
    },

    Constant {
        name: "electron volt-inverse meter relationship",
        value: 8.065_543_937e5,//...
        si_base_units: Some("m^-1"),
        uncertainty: None,
    },

    Constant {
        name: "electron volt-joule relationship",
        value: 1.602_176_634e-19,
        si_base_units: Some("J"),
        uncertainty: None,
    },

    Constant {
        name: "electron volt-kelvin relationship",
        value: 1.160_451_812e4,//...
        si_base_units: Some("K"),
        uncertainty: None,
    },

    Constant {
        name: "electron volt-kilogram relationship",
        value: 1.782_661_921e-36,//...
        si_base_units: Some("kg"),
        uncertainty: None,
    },

    Constant {
        name: "elementary charge",
        value: 1.602_176_634e-19,
        si_base_units: Some("C"),
        uncertainty: None,
    },

    Constant {
        name: "elementary charge over h-bar",
        value: 1.519_267_447e15,//...
        si_base_units: Some("A J^-1"),
        uncertainty: None,
    },

    Constant {
        name: "Faraday constant",
        value: 96.485_332_12,//...
        si_base_units: Some("C mol^-1"),
        uncertainty: None,
    },

    Constant {
        name: "Fermi coupling constant",
        value: 1.166_378_7e-5,
        si_base_units: Some("GeV^-2"),
        uncertainty: Some(0.000_0006e-5),
    },

    Constant {
        name: "fine-structure constant",
        value: 7.297_352_564_3e-3,
        si_base_units: None,
        uncertainty: Some(0.000_000_001_1e-3),
    },

    Constant {
        name: "first radiation constant",
        value: 3.741_771_852e-16,//...
        si_base_units: Some("W m^2"),
        uncertainty: None,
    },

    Constant {
        name: "first radiation constant for spectral radiance",
        value: 1.191_042_972e-16,//...
        si_base_units: Some("W m^2 sr^-1"),
        uncertainty: None,
    },

    Constant {
        name: "hartree-atomic mass unit relationship",
        value: 2.921_262_317_97e-8,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_000_91e-8),
    },

    Constant {
        name: "hartree-electron volt relationship",
        value: 27.211_386_245_981,
        si_base_units: Some("eV"),
        uncertainty: Some(0.000_000_000_03),
    },

    Constant {
        name: "Hartree energy",
        value: 4.359_744_722_206e-18,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_000_004_8e-18),
    },

    Constant {
        name: "Hartree energy in eV",
        value: 27.211_386_245_981,
        si_base_units: Some("eV"),
        uncertainty: Some(0.000_000_000_03),
    },

    Constant {
        name: "hartree-hertz relationship",
        value: 6.579_683_920_499_9e15,
        si_base_units: Some("Hz"),
        uncertainty: Some(0.000_000_000_007_2e15),
    },

    Constant {
        name: "hartree inverse meter relationship",
        value: 2.194_746_313_631_4e7,
        si_base_units: Some("m^-1"),
        uncertainty: Some(0.000_000_000_002_4e7),
    },

    Constant {
        name: "hartree-joule relationship",
        value: 4.359_744_722_206e-18,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_000_004_8e-18),
    },

    Constant {
        name: "hartree-kelvin relationship",
        value: 3.157_750_248_039_8e5,
        si_base_units: Some("K"),
        uncertainty: Some(0.000_000_000_003_4),
    },

    Constant {
        name: "hartree-kilogram relationship",
        value: 4.850_870_209_541_9e-35,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_000_005_3),
    },

    Constant {
        name: "helion-electron mass ratio",
        value: 5_495.885_279_84,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_16),
    },

    Constant {
        name: "helion g factor",
        value: -4.255_250_699_5,
        si_base_units: None,
        uncertainty: Some(0.000_000_003_4),
    },

    Constant {
        name: "helion mag. mom.",
        value: -1.074_617_551_98e-26,
        si_base_units: Some("J T^-1"),
        uncertainty: Some(0.000_000_000_93e-26),
    },

    Constant {
        name: "helion mag. mom. to Bohr magneton ratio",
        value: -1.158_740_980_83e-3,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_94e-3),
    },
    
    Constant {
        name: "helion mag. mom. to nuclear magneton ratio",
        value: -2.127_625_349_8,
        si_base_units: None,
        uncertainty: Some(0.000_000_001_7),
    },

    Constant {
        name: "helion mass",
        value: 5.006_412_786_2e-27,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_001_6e-27),
    },

    Constant {
        name: "helion to mass energy equivalent",
        value: 4.499_539_418_5e-10,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_001_4e10),
    },

    Constant {
        name: "helion mass energy equivalent in MeV",
        value: 2808.391_611_12,
        si_base_units: Some("MeV"),
        uncertainty: Some(0.000_000_88),
    },

    Constant {
        name: "helion mass in u",
        value: 3.014_932_246_932,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_000_074),
    },

    Constant {
        name: "helion molar mass",
        value: 3.014_932_250_1e-3,
        si_base_units: Some("kg mol^-1"),
        uncertainty: Some(0.000_000_000_94e-3),
    },

    Constant {
        name: "helion-proton mass ratio",
        value: 2.993_152_617_552,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_07),
    },

    Constant {
        name: "helion relative atomic mass",
        value: 3.014_932_246_932,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_74),
    },

    Constant {
        name: "helion shielding shift",
        value: 5.996_702_9e-5,
        si_base_units: None,
        uncertainty: Some(0.000_002_3),
    },

    Constant {
        name: "hertz-atomic mass unit relationship",
        value: 4.439_821_659e-24,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_001_4e-24),
    },

    Constant {
        name: "hertz-electron volt relationship",
        value: 4.135_667_696e-15,
        si_base_units: Some("eV"),
        uncertainty: None,
    },

    Constant {
        name: "hertz-hartree relationship",
        value: 1.519_829_846_057_4e-16,
        si_base_units: Some("E_h"),
        uncertainty: Some(0.000_000_000_001_7e-16),
    },

    Constant {
        name: "hertz-inverse meter relationship",
        value: 3.335_640_951e-9,
        si_base_units: Some("m^-1"),
        uncertainty: None,
    },

    Constant {
        name: "hertz-joule relationship",
        value: 6.626_070_15e-34,
        si_base_units: Some("J"),
        uncertainty: None,
    },

    Constant {
        name: "hertz-kelvin relationship",
        value: 4.799_243_073e-11,//...
        si_base_units: Some("K"),
        uncertainty: None,
    },

    Constant {
        name: "hertz-kilogram relationship",
        value: 7.372_497_323e51,
        si_base_units: Some("kg"),
        uncertainty: None,
    },

    Constant {
        name: "hyperfine transition frequency of Cs-133",
        value: 9_192_631_770.0,
        si_base_units: Some("Hz"),
        uncertainty: None,
    },

    Constant {
        name: "inverse fine-structure constant",
        value: 137.035_999_177,
        si_base_units: None,
        uncertainty: Some(0.000_000_021),
    },

    Constant {
        name: "inverse meter-atomic mass unit relationship",
        value: 1.331_025_048_24e-15,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_000_41e-15),
    },
//... check check check
    Constant {
        name: "inverse meter-electron volt relationship",
        value: 1.239_841_984e-6,//...
        si_base_units: Some("eV"),
        uncertainty: None,
    },

    Constant {
        name: "inverse meter-hartree relationship",
        value: 4.556_335_252_913_2e-8,//...
        si_base_units: Some("E_h"),
        uncertainty: Some(0.000_000_000_005e-8),
    },

    Constant {
        name: "inverse meter-hertz relationship",
        value: 299_792_458.0,
        si_base_units: Some("Hz"),
        uncertainty:   None,
    },

    Constant {
        name: "inverse meter-joule relationship",
        value: 1.986_445_857e-25,//...
        si_base_units: Some("J"),
        uncertainty: None,
    },

    Constant {
        name: "inverse meter-kelvin relatinship",
        value: 1.438_776_877e-2,//...
        si_base_units: Some("K"),
        uncertainty: None,
    },

    Constant {
        name: "inverse meter-kilogram relationship",
        value: 2.210_219_094e-42,
        si_base_units: Some("kg"),
        uncertainty: None,
    },

    Constant {
        name: "inverse of conductance quantum",
        value: 12_906.403_72,//...
        si_base_units: Some("ohm"),
        uncertainty: None,
    },

    Constant {
        name: "Josephson constant",
        value: 483_597.848_4e9,//...
        si_base_units: Some("Hz V^-1"),
        uncertainty: None,
    },

    Constant {
        name: "joule-atomic mass unit relationship",
        value: 6.700_535_247_1e9,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_002_1e9),
    },

    Constant {
        name: "joule-electron volt relationship",
        value: 6.241_509_074e18,//..
        si_base_units: Some("eV"),
        uncertainty: None,
    },

    Constant {
        name: "joule-hartree relationship",
        value: 2.293_712_278_396_9e17,
        si_base_units: Some("E_h"),
        uncertainty: Some(0.000_000_000_002_5e17),
    },

    Constant {
        name: "joule-hertz relationship",
        value: 1.509_190_179e33,//..
        si_base_units: Some("Hz"),
        uncertainty: None,
    },

    Constant {
        name: "joule-inverse meter relationship",
        value: 5.034_116_567e24,//...
        si_base_units: Some("m^-1"),
        uncertainty: None,
    },

    Constant {
        name: "joule-kelvin relationship",
        value: 7.242_970_516e22,//...
        si_base_units: Some("K"),
        uncertainty: None,
    },

    Constant {
        name: "joule-kilogram relationship",
        value: 1.112_650_056e-17,
        si_base_units: Some("kg"),
        uncertainty: None,
    },

    Constant {
        name: "kelvin-atomic mass unit relationship",
        value: 9.251_087_288_4e-14,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_002_9e-14),
    },

    Constant {
        name: "kelvin-electron volt relationship",
        value: 8.617_333_262e-5,
        si_base_units: Some("eV"),
        uncertainty: None,
    },

    Constant {
        name: "kelvin-hartree relationship",
        value: 3.166_811_563_456_4e-6,
        si_base_units: Some("E_h"),
        uncertainty: Some(0.000_000_000_003_5),
    },

    Constant {
        name: "kelvin-hertz relationship",
        value: 2.083_661_912e10,//...
        si_base_units: Some("Hz"),
        uncertainty: None,
    },

    Constant {
        name: "kelvin-inverse meter relationship",
        value: 69.503_480_04,//...
        si_base_units: Some("m^-1"),
        uncertainty: None,
    },

    Constant {
        name: "kelvin-joule relationship",
        value: 1.380_649e-23,
        si_base_units: Some("J"),
        uncertainty: None,
    },

    Constant {
        name: "kelvin-kilogram relationship",
        value: 1.536_179_187e-40,//...
        si_base_units: Some("kg"),
        uncertainty: None,
    },

    Constant {
        name: "kilogram-atomic mass unit relationship",
        value: 6.002_140_757_7e26,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_001_9e26),
    },

    Constant {
        name: "kilogram-electron volt relationship",
        value: 5.609_588_603e35,//...
        si_base_units: Some("eV"),
        uncertainty: None,
    },

    Constant {
        name: "kilogram-hartree relationship",
        value: 2.061_458_788_741_5e34,
        si_base_units: Some("E_h"),
        uncertainty: Some(0.000_000_000_002_2e34),
    },

    Constant {
        name: "kilogram-hertz relationship",
        value: 1.356_392_489e50,//...
        si_base_units: Some("Hz"),
        uncertainty: None,
    },

    Constant {
        name: "kilogram-inverse meter relationship",
        value: 4.524_438_335e41,//...
        si_base_units: Some("m^-1"),
        uncertainty: None,
    },

    Constant {
        name: "kilogram-joule relationship",
        value: 8.987_551_787e16,//...
        si_base_units: Some("J"),
        uncertainty: None,
    },

    Constant {
        name: "kilogram-kelvin relationship",
        value: 6.509_657_26e39,//...
        si_base_units: Some("K"),
        uncertainty: None,
    },

    Constant {
        name: "lattice parameter of silicon",
        value: 6.431_020_511e-10,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_089e-10),
    },

    Constant {
        name: "lattice spacing of ideal Si (220)",
        value: 1.920_155_716_716e-10,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_032e-10),
    },

    Constant {
        name: "Loschmidt constant (273.15 K, 100 kPa)",
        value: 2.651_645_804e25,//...
        si_base_units: Some("m^-3"),
        uncertainty: None,
    },

    Constant {
        name: "Loschmidt constant (273.15 K, 101.325 kPa)",
        value: 2.686_780_111e25,//...
        si_base_units: Some("m^-3"),
        uncertainty: None,
    },

    Constant {
        name: "luminous efficacy",
        value: 683.0,
        si_base_units: Some("lm W^-1"),
        uncertainty: None,
    },

    Constant {
        name: "mag. flux quantum",
        value: 2.067_833_848e-15,//...
        si_base_units: Some("Wb"),
        uncertainty: None,
    },

    Constant {
        name: "molar gas constant",
        value: 8.314_462_618,//...
        si_base_units: Some(""),
        uncertainty: None,
    },

    Constant {
        name: "molar mass constant",
        value: 1.000_000_001_05e-3,
        si_base_units: Some("kg mol^-1"),
        uncertainty: Some(0.000_000_000_31e-3),
    },

    Constant {
        name: "molar mass of carbon-12",
        value: 12.000_000_012_6e-3,
        si_base_units: Some("kg mol^-1"),
        uncertainty: Some(0.000_000_003_7e-3),
    },

    Constant {
        name: "molar Plank constant",
        value: 3.990_312_712e-10,//...
        si_base_units: Some("J Hz^-1 mol^-1"),
        uncertainty: None,
    },

    Constant {
        name: "molar volume ideal gas (273.15 K, 100 kPa)",
        value: 22.710_954_64e-3,
        si_base_units: Some("m^3 mol^-1"),
        uncertainty: None,
    },

    Constant {
        name: "molar volume of ideal gas (273.13 K, 101.325 kPa)",
        value: 22.413_969_54e-3,
        si_base_units: Some("m^3 mol^-1"),
        uncertainty: None,
    },

    Constant {
        name: "molar volume of silicon",
        value: 1.205_883_199e-5,
        si_base_units: Some("m^3 mol^-1"),
        uncertainty: Some(0.000_000_06e-5),
    },

    Constant {
        name: "Molybdenum x unit",
        value: 1.002_009_52e-13,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_53e-13),
    },

    Constant {
        name: "muon Compton wavelength",
        value: 1.173_444_110e-14,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_026),
    },

    Constant {
        name: "muon-electron mass ratio",
        value: 206.768_287_7,
        si_base_units: None,
        uncertainty: Some(0.000_004_6),
    },

    Constant {
        name: "muon g factor",
        value: -2.002_331_841_23,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_82),
    },

    Constant {
        name: "muon mag. mom.",
        value: -4.490_448_3e-26,
        si_base_units: Some("J T^-1"),
        uncertainty: Some(0.000_000_1e-26),
    },

    Constant {
        name: "muon mag. mom. anomly",
        value: 1.165_920_62e-3,
        si_base_units: None,
        uncertainty: Some(0.000_000_41e-3),
    },

    Constant {
        name: "muon mag. mom. to Bohr magneton ratio",
        value: -4.841_970_48e-3,
        si_base_units: None,
        uncertainty: Some(0.000_000_11e-3),
    },

    Constant {
        name: "muon mag. mom. to nuclear magneton ratio",
        value: -8.890_597_04,
        si_base_units: None,
        uncertainty: Some(0.000_000_2),
    },

    Constant {
        name: "muon mag. mom. to nuclear magneton ratio",
        value: -8.890_597_04,
        si_base_units: None,
        uncertainty: Some(0.000_000_2),
    },

    Constant {
        name: "muon mass",
        value: 1.883_531_627e-28,
        si_base_units: Some("kg"),
        uncertainty: Some(0.00_000_042e-28),
    },

    Constant {
        name: "muon mass energy equivalent",
        value: 1.692_833_804e-11,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_038e-11),
    },

    Constant {
        name: "muon mass in u",
        value: 0.113_428_925_7,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_002_5),
    },

    Constant {
        name: "muon molar mass",
        value: 1.134_289_258e-4,
        si_base_units: Some("kg mol^-1"),
        uncertainty: Some(0.000_000_025e-4),
    },

    Constant {
        name: "muon-neutron mass ratio",
        value: 0.112_454_516_8,
        si_base_units: None,
        uncertainty: Some(0.000_000_002_5),
    },

    Constant {
        name: "muon-proton mag. mom. ratio",
        value: -3.183_345_146,
        si_base_units: None,
        uncertainty: Some(0.000_000_071),
    },

    Constant {
        name: "muon-proton mass ratio",
        value: 0.112_609_526_2,
        si_base_units: None,
        uncertainty: Some(0.000_000_002_5),
    },

    Constant {
        name: "muon-tau mass ratio",
        value: 5.946_35e-2,
        si_base_units: None,
        uncertainty: Some(0.000_4e-2),
    },

    Constant {
        name: "natural unit of action",
        value: 1.054_571_817e-34,//...
        si_base_units: Some("J s"),
        uncertainty: None,
    },

    Constant {
        name: "natural unit of action in eV s",
        value: 6.582_119_569e-16,
        si_base_units: Some("eV s"),
        uncertainty: None,
    },

    Constant {
        name: "natural unit of energy",
        value: 8.187_105_788e-14,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_002_6e-14),
    },

    Constant {
        name: "natural unit of energy in MeV",
        value: 0.510_988_950_69,
        si_base_units: Some("MeV"),
        uncertainty: Some(0.000_000_000_16),
    },

    Constant {
        name: "natural unit of length",
        value: 3.861_592_674_4e-13,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_001_2e-13),
    },

    Constant {
        name: "natural unit of mass",
        value: 9.109_383_713_9e-13,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_002_8e-31),
    },

    Constant {
        name: "natural unit of momentum",
        value: 2.730_924_534_46e-22,
        si_base_units: Some("kg m s^-1"),
        uncertainty: Some(0.000_000_000_85e-22),
    },

    Constant {
        name: "natural unit momentum in MeV/c",
        value: 0.510_998_950_69,
        si_base_units: Some("MeV/c"),
        uncertainty: Some(0.000_000_000_16),
    },

    Constant {
        name: "natural unit of time",
        value: 1.288_088_666_44e-21,
        si_base_units: Some("s"),
        uncertainty: Some(0.000_000_000_4e-21),
    },

    Constant {
        name: "natural unit of velocity",
        value: 299_792_458.0,
        si_base_units: Some("m s^-1"),
        uncertainty: None,
    },

    Constant {
        name: "neutron Compton wavelength",
        value: 1.319_590_903_82e-15,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_000_67e-15),
    },

    Constant {
        name: "neutron-electron mag. mom. ratio",
        value: 1.040_668_84e-3,
        si_base_units: None,
        uncertainty: Some(0.000_000_24e-3),
    },

    Constant {
        name: "neutron-electron mass ratio",
        value: 1838.683_662,
        si_base_units: None,
        uncertainty: Some(0.000_000_74),
    },

    Constant {
        name: "neutron g factor",
        value: -3.826_085_52,
        si_base_units: None,
        uncertainty: Some(0.000_000_9),
    },

    Constant {
        name: "neutron gyromag. ratio",
        value: 1.832_471_74e8,
        si_base_units: Some("s^-1 T^-1"),
        uncertainty: Some(0.000_000_43e8),
    },

    Constant {
        name: "neutron gyromag. ratio in MHz/T",
        value: 29.164_693_5,
        si_base_units: Some("MHz T^-1"),
        uncertainty: Some(0.000_006_9),
    },

    Constant {
        name: "neutron mag. mom.",
        value: -9.662_365_3e-27,
        si_base_units: Some("J T^-1"),
        uncertainty: Some(0.000_002_3e-27),
    },

    Constant {
        name: "neutron mag. mom. to Bohr magneton ratio",
        value: -1.041_875_65e-3,
        si_base_units: None,
        uncertainty: Some(0.000_000_25e-3),
    },

    Constant {
        name: "neutron mag. mom. to nuclear magneton ratio",
        value: -1.913_042_76,
        si_base_units: None,
        uncertainty: Some(0.000_000_45),
    },

    Constant {
        name: "neutron mass",
        value: 1.674_927_500_56e-27,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_000_85e-27),
    },

    Constant {
        name: "neutron mass energy equivalent",
        value: 1.505_349_765_14e-10,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_000_76e-10),
    },

    Constant {
        name: "neutron mass energy equivalent in MeV",
        value: 939.565_421_94,
        si_base_units: Some("MeV"),
        uncertainty: Some(0.000_000_48),
    },

    Constant {
        name: "neutron mass in u",
        value: 1.008_664_916_06,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_000_4),
    },

    Constant {
        name: "neutron molar mass",
        value: 1.008_664_917_12e-3,
        si_base_units: Some("kg mol^-1"),
        uncertainty: Some(0.000_000_000_51e-3),
    },

    Constant {
        name: "neutron-muon mass ratio",
        value: 8.892_484_08,
        si_base_units: None,
        uncertainty: Some(0.000_000_2),
    },

    Constant {
        name: "neutron-proton mag. mom. ratio",
        value: -0.684_979_35,
        si_base_units: None,
        uncertainty: Some(0.000_000_16),
    },

    Constant {
        name: "neutron-proton mass difference",
        value: 2.305_574_61e-30,
        si_base_units: None,
        uncertainty: Some(0.000_000_67e-30),
    },

    Constant {
        name: "neutron-proton mass difference energy equivalent",
        value: 2.072_147_12e-13,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_6e-13),
    },

    Constant {
        name: "neutron-proton mass difference energy equivalent in MeV",
        value: 1.293_332_51,
        si_base_units: Some("MeV"),
        uncertainty: Some(0.000_000_38),
    },

    Constant {
        name: "neutron-proton mass difference in u",
        value: 1.388_449_48e-3,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_4e-3)
    },

    Constant {
        name: "neutron-proton mass ratio",
        value: 1.001_378_419_46,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_4),
    },

    Constant {
        name: "neutron relative atomic mass",
        value: 1.008_664_916_06,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_4),
    },

    Constant {
        name: "neutron-tau mass ratio",
        value: 0.528_799,
        si_base_units: None,
        uncertainty: Some(0.000_036),
    },

    Constant {
        name: "neutron to shielded proton mag. mom. ratio",
        value: -0.684_996_94,
        si_base_units: None,
        uncertainty: Some(0.000_000_16),
    },

    Constant {
        name: "Newtonian constant of gravitation",
        value: 6.674_3e-11,
        si_base_units: Some("m^3 kg^-1 s^-2"),
        uncertainty: Some(0.000_15e-11),
    },

    Constant {
        name: "Newtonian constant of gravitation over h-bar c",
        value: 6.708_83e-39,
        si_base_units: Some("(GeV/c^2)^2"),
        uncertainty: Some(0.000_15e-39),
    },

    Constant {
        name: "nuclear magneton",
        value: 5.050_783_739_3e-27,
        si_base_units: Some("J T^-1"),
        uncertainty: Some(0.000_000_001_6e-27),
    },

    Constant {
        name: "nuclear magneton in eV/T",
        value: 3.152_451_254_17e-8,
        si_base_units: Some("eV T^-1"),
        uncertainty: Some(0.000_000_000_98e-8),
    },

    Constant {
        name: "nuclear magneton in inverse meter per tesla",
        value: 2.542_623_410_09e-2,
        si_base_units: Some("m^-1 T^-1"),
        uncertainty: Some(0.000_000_000_79e-2),
    },

    Constant {
        name: "nuclear magneton in K/T",
        value: 3.658_267_770_6e-4,
        si_base_units: Some("K T^-1"),
        uncertainty: Some(0.000_000_001_1e-4),
    },

    Constant {
        name: "nuclear magneton in MHz/T",
        value: 7.622_593_218_8,
        si_base_units: Some("MHz T^-1"),
        uncertainty: Some(0.000_000_002_4),
    },

    Constant {
        name: "Planck constant",
        value: 6.626_070_15e-34,
        si_base_units: Some("J Hz^-1"),
        uncertainty: None,
    },

    Constant {
        name: "Planck constant in eV/Hz",
        value: 4.135_667_696e-15,//...
        si_base_units: Some("eV Hz^-1"),
        uncertainty: None,
    },

    Constant {
        name: "Planck length",
        value: 1.616_255e-35,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_018e-35),
    },

    Constant {
        name: "Planck mass",
        value: 2.176_434e-8,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_024e-8),
    },

    Constant {
        name: "Planck mass energy equivalent in GeV",
        value: 1.220_890e19,
        si_base_units: Some("GeV"),
        uncertainty: Some(0.000_014e19),
    },

    Constant {
        name: "Planck temperature",
        value: 1.416_784e32,
        si_base_units: Some("K"),
        uncertainty: Some(0.000_016e32),
    },

    Constant {
        name: "Planck time",
        value: 5.391_247e-44,
        si_base_units: Some("s"),
        uncertainty: Some(0.000_06e-44),
    },

    Constant {
        name: "proton charge to mass quotient",
        value: 9.578_833_143e7,
        si_base_units: Some("C kg^-1"),
        uncertainty: Some(0.000_000_003e7),
    },

    Constant {
        name: "proton Compton wavelength",
        value: 1.321_409_853_6e-15,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_000_41e-15),
    },

    Constant {
        name: "proton-electron mass ratio",
        value: 1836.152_673_426,
        si_base_units: None,
        uncertainty: Some(0.000_000_032),
    },

    Constant {
        name: "proton g factor",
        value: 5.585_694_689_3,
        si_base_units: None,
        uncertainty: Some(0.000_000_001_6),
    },

    Constant {
        name: "proton gyromag. ratio",
        value: 2.675_221_870_8e8,
        si_base_units: Some("s^-1 T^-1"),
        uncertainty: Some(0.000_000_001_1e8),
    },

    Constant {
        name: "proton gyromag. ratio in MHz/T",
        value: 42.577_478_461,
        si_base_units: Some("MHz T^-1"),
        uncertainty: Some(0.000_000_018),
    },

    Constant {
        name: "proton mag. mom.",
        value: 1.410_606_795_45e-26,
        si_base_units: Some("J T^-1"),
        uncertainty: Some(0.000_000_000_6e-26),
    },

    Constant {
        name: "proton mag. mom. to Bohr magneton ratio",
        value: 1.521_032_202_3e-3,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_45e-3),
    },

    Constant {
        name: "proton mag. mom. to nuclear magneton ratio",
        value: 2.792_847_344_63,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_82),
    },

    Constant {
        name: "proton mag. shielding correction",
        value: 2.567_15e-5,
        si_base_units: None,
        uncertainty: Some(0.000_41e-5),
    },

    Constant {
        name: "proton mass",
        value: 1.672_621_925_95e-27,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_000_52e-27),
    },

    Constant {
        name: "proton mass energy equivalent",
        value: 1.503_277_618_02e-10,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_000_47),
    },

    Constant {
        name: "proton mass energy equivalent in MeV",
        value: 938.272_089_43,
        si_base_units: Some("MeV"),
        uncertainty: Some(0.000_000_29),
    },

    Constant {
        name: "proton mass in u",
        value: 1.007_276_466_578_9,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_000_008_3),
    },

    Constant {
        name: "proton molar mass",
        value: 1.007_276_467_64e-3,
        si_base_units: Some("kg mol^-1"),
        uncertainty: Some(0.000_000_000_31e-3),
    },

    Constant {
        name: "proton-muon mass ratio",
        value: 8.880_243_38,
        si_base_units: None,
        uncertainty: Some(0.000_000_2),
    },

    Constant {
        name: "proton-neutron mag. mom. ratio",
        value: -1.459_898_02,
        si_base_units: None,
        uncertainty: Some(0.000_000_34),
    },

    Constant {
        name: "proton-neutron mass ratio",
        value: 0.998_623_477_97,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_4),
    },

    Constant {
        name: "proton relative atomic mass",
        value: 1.007_276_466_578_9,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_008_3),
    },

    Constant {
        name: "proton rms charge radius",
        value: 8.407_5e-16,
        si_base_units: Some("m"),
        uncertainty: Some(0.006_4e-16),
    },

    Constant {
        name: "proton-tau mass ratio",
        value: 0.528_051,
        si_base_units: None,
        uncertainty: Some(0.000_036),
    },

    Constant {
        name: "quantum of circulation",
        value: 3.636_947_546_7e-4,
        si_base_units: Some("m^2 s^-1"),
        uncertainty: Some(0.000_000_001_1e-4),
    },

    Constant {
        name: "quantum of circluation times 2",
        value: 7.273_895_093_4e-4,
        si_base_units: Some("m^2 s^-1"),
        uncertainty: Some(0.000_000_002_3e-4),
    },

    Constant {
        name: "reduced Compton wavelength",
        value: 3.861_592_674_4e-13,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_001_2e-13),
    },

    Constant {
        name: "reduced muon Compton wavelength",
        value: 1.867_594_306e-15,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_042e-15),
    },

    Constant {
        name: "reduced neutron Compton wavelength",
        value: 2.100_194_152e-16,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_001_1e-16),
    },

    Constant {
        name: "reduced Planck constant",
        value: 1.054_571_817e-34,//...
        si_base_units: Some("J s"),
        uncertainty: None,
    },

    Constant {
        name: "reduced Planck constant in eV s",
        value: 6.582_119_569e-16,//...
        si_base_units: Some("eV s"),
        uncertainty: None,
    },

    Constant {
        name: "reduced Planck constant times c in MeV fm",
        value: 197.326_980_4,
        si_base_units: Some("MeV fm"),
        uncertainty: None,
    },

    Constant {
        name: "reduced proton Compton wavelength",
        value: 2.103_089_100_51e-16,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_000_000_66e-16),
    },

    Constant {
        name: "reduced tau Compton wavelength",
        value: 1.110_538e-16,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_075e-16),
    },

    Constant {
        name: "Rydberg constant",
        value: 10_973_731.568_157,
        si_base_units: Some("m^-1"),
        uncertainty: Some(0.000_012),
    },

    Constant {
        name: "Rydberg constant times c in Hz",
        value: 3.289_841_960_25e15,
        si_base_units: Some("Hz"),
        uncertainty: Some(0.000_000_000_003_6e15),
    },

    Constant {
        name: "Rydberg constant times hc in eV",
        value: 13.605_693_122_99,
        si_base_units: Some("eV"),
        uncertainty: Some(0.000_000_000_015),
    },

    Constant {
        name: "Rydberg constant times hc in J",
        value: 2.179_872_361_103e-18,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_000_002_4e-18),
    },

    Constant {
        name: "Sackur-Tetrode constant (1 K, 100kPa)",
        value: -1.151_707_534_96,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_47),
    },

    Constant {
        name: "Sackur-Tetrode constant (1 K, 101.325 kPa)",
        value: -1.164_870_521_49,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_47),
    },

    Constant {
        name: "second radiation constant",
        value: 1.438_776_877e-2,//...
        si_base_units: Some("m K"),
        uncertainty: None,
    },

    Constant {
        name: "shielded helion gyromag. ratio",
        value: 2.037_894_607_8e8,
        si_base_units: Some("s^-1 T^-1"),
        uncertainty: Some(0.000_000_001_8e8),
    },

    Constant {
        name: "shielded helion gyromag. ratio in MHz/T",
        value: 32.434_100_033,
        si_base_units: Some("MHz T^-1"),
        uncertainty: Some(0.000_000_028),
    },

    Constant {
        name: "shielded helion mag. mom.",
        value: -1.074_533_110_35e-26,
        si_base_units: Some("J T^-1"),
        uncertainty: Some(0.000_000_000_93e-26),
    },

    Constant {
        name: "shielded helion mag. mom to Bohr magneton ratio",
        value: -1.158_671_494_57e-3,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_94e-3),
    },

    Constant {
        name: "shielded helion mag. mom. to nuclear magneton ratio",
        value: -2.117_497_762_4,
        si_base_units: None,
        uncertainty: Some(0.000_000_001_7),
    },

    Constant {
        name: "shielded helion to proton mag. mom. ratio",
        value: -0.761_766_577_21,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_66),
    },

    Constant {
        name: "shielded helion to shielded proton mag. mom. ratio",
        value: -0.761_786_133_4,
        si_base_units: None,
        uncertainty: Some(0.000_000_003_1),
    },

    Constant {
        name: "shielded proton gyromag. ratio",
        value: 2.675_153_194e8,
        si_base_units: Some("s^-1 T^-1"),
        uncertainty: Some(0.000_000_011e8),
    },

    Constant {
        name: "shielded proton gyromag. ratio in MHz/T",
        value: 42.576_385_43,
        si_base_units: Some("MHz T^-1"),
        uncertainty: Some(0.000_000_17),
    },

    Constant {
        name: "shielded proton mag. mom.",
        value: 1.410_570_583e-26,
        si_base_units: Some("J T^-1"),
        uncertainty: Some(0.000_000_005_8e-26),
    },

    Constant {
        name: "shielded proton mag. mom. to Bohr magneton ratio",
        value: 1.520_993_155_1e-3,
        si_base_units: None,
        uncertainty: Some(0.000_000_006_2e-3),
    },

    Constant {
        name: "shielded proton mag. mom. to nuclear magneton ratio",
        value: 2.792_755_648,
        si_base_units: None,
        uncertainty: Some(0.000_000_011),
    },

    Constant {
        name: "shielding difference to d and p in HD",
        value: 1.987_70e-8,
        si_base_units: None,
        uncertainty: Some(0.000_1e-8),
    },

    Constant {
        name: "shielding difference of t and p in HT",
        value: 2.394_5e-8,
        si_base_units: None,
        uncertainty: Some(0.000_2e-8),
    },

    Constant {
        name: "speed of light in vacuum",
        value: 299_792_458.0,
        si_base_units: Some("m s^-1"),
        uncertainty: None,
    },

    Constant {
        name: "standard acceleration of gravity",
        value: 9.806_65,
        si_base_units: Some("m s^-2"),
        uncertainty: None,
    },

    Constant {
        name: "standard atmosphere",
        value: 101_325.0,
        si_base_units: Some("Pa"),
        uncertainty: None,
    },

    Constant {
        name: "standard-state pressure",
        value: 100_000.0,
        si_base_units: Some("Pa"),
        uncertainty: None,
    },

    Constant {
        name: "Stefan-Boltzmann constant",
        value: 5.670_374_419e-8,
        si_base_units: Some("W m^-2 K^-4"),
        uncertainty: None,
    },

    Constant {
        name: "tau Compton wavelength",
        value: 6.977_71e-16,
        si_base_units: Some("m"),
        uncertainty: Some(0.000_47e-16),
    },

    Constant {
        name: "tau-electron mass ratio",
        value: 3_477.23,
        si_base_units: None,
        uncertainty: Some(0.23),
    },

    Constant {
        name: "tau energy equivalent",
        value: 1_776.86,
        si_base_units: Some("MeV"),
        uncertainty: Some(0.12),
    },

    Constant {
        name: "tau mass",
        value: 3.167_54e-27,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_21e-27),
    },

    Constant {
        name: "tau mass energy equivalent",
        value: 2.846_84e-10,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_19e-10),
    },

    Constant {
        name: "tau mass in u",
        value: 1.907_54,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_13),
    },

    Constant {
        name: "tau molar mass",
        value: 1.907_54e-3,
        si_base_units: Some("kg mol^-1"),
        uncertainty: Some(0.000_13e-3),
    },

    Constant {
        name: "tau-muon mass ratio",
        value: 16.817,
        si_base_units: None,
        uncertainty: Some(0.001_1),
    },

    Constant {
        name: "tau-neutron mass ratio",
        value: 1.891_15,
        si_base_units: None,
        uncertainty: Some(0.000_13),
    },

    Constant {
        name: "tau-proton mass ratio",
        value: 1.893_76,
        si_base_units: None,
        uncertainty: Some(0.000_13),
    },

    Constant {
        name: "Thomas cross section",
        value: 6.652_458_705_1e-29,
        si_base_units: Some("m^2"),
        uncertainty: Some(0.000_000_006_2e-29),
    },

    Constant {
        name: "triton-electron mass ratio",
        value: 5_469.921_535_51,
        si_base_units: None,
        uncertainty: Some(0.000_000_21),
    },

    Constant {
        name: "triton g factor",
        value: 5.957_924_93,
        si_base_units: None,
        uncertainty: Some(0.000_000_012),
    },

    Constant {
        name: "triton mag. mom.",
        value: 1.504_609_517_8e-26,
        si_base_units: Some("J T^-1"),
        uncertainty: Some(0.000_000_003e-26),
    },

    Constant {
        name: "triton mag. mom. to Bohr magneton ratio",
        value: 1.622_393_664_8e-3,
        si_base_units: None,
        uncertainty: Some(0.000_000_003_2e-3),
    },

    Constant {
        name: "triton mag. mom. to nuclear magneton ratio",
        value: 2.978_962_465,
        si_base_units: None,
        uncertainty: Some(0.000_000_005_9),
    },

    Constant {
        name: "triton mass",
        value: 5.007_356_751_2e-27,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_001_6e-27),
    },

    Constant {
        name: "triton mass energy equivalent",
        value: 4.500_387_811_9e-10,
        si_base_units: Some("J"),
        uncertainty: Some(0.000_000_001_4e-10),
    },

    Constant {
        name: "triton mass energy equivalent in MeV",
        value: 2_808.921_136_68,
        si_base_units: Some("MeV"),
        uncertainty: Some(0.000_000_88),
    },

    Constant {
        name: "triton mass in u",
        value: 3.015_500_715_97,
        si_base_units: Some("u"),
        uncertainty: Some(0.000_000_000_1),
    },
    Constant {
        name: "triton molar mass",
        value: 3.015_500_719_13e-3,
        si_base_units: Some("kg mol^-1"),
        uncertainty: Some(0.000_000_000_94e-3),
    },

    Constant {
        name: "triton-proton mass ratio",
        value: 2.993_717_034_03,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_1),
    },

    Constant {
        name: "triton relative atomic mass",
        value: 3.015_500_715_97,
        si_base_units: None,
        uncertainty: Some(0.000_000_000_1),
    },

    Constant {
        name: "triton to proton mag. mom. ratio",
        value: 1.066_639_918_9,
        si_base_units: None,
        uncertainty: Some(0.000_000_002_1),
    },

    Constant {
        name: "unified atomic mass unit",
        value: 1.660_539_068_92e-27,
        si_base_units: Some("kg"),
        uncertainty: Some(0.000_000_000_52e-27),
    },

    Constant {
        name: "vacuum electricy permittivity",
        value: 8.854_187_818_8e-12,
        si_base_units: Some("F m^-1"),
        uncertainty: Some(0.000_000_001_4e-12),
    },

    Constant {
        name: "vacuum mag. permeability",
        value: 1.256_637_061_27e-6,
        si_base_units: Some("N A^-2"),
        uncertainty: Some(0.000_000_000_2e-6),
    },

    Constant {
        name: "von Klitzing constant",
        value: 25_812.807_45,
        si_base_units: Some("ohm"),
        uncertainty: None,
    },

    Constant {
        name: "weak mixing angle",
        value: 0.223_05,
        si_base_units: None,
        uncertainty: Some(0.000_23),
    },

    Constant {
        name: "Wien frequency displacement law constant",
        value: 5.878_925_757e10,
        si_base_units: Some("Hz K^-1"),
        uncertainty: None,
    },

    Constant {
        name: "Wien wavelength displacement law constant",
        value: 2.897_771_955e-3,
        si_base_units: Some("m K"),
        uncertainty: None,
    },

    Constant {
        name: "W to Z mass ratio",
        value: 0.881_45,
        si_base_units: None,
        uncertainty: Some(0.000_13),
    },

]
} 





























fn main() {
    let constants = initialize_constants();    
    
    for constant in &constants {
        println!("{}", constant);
    }

    //println!("{}", CONSTANTS[0].value);
}