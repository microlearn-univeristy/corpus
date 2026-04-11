use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HostProfile {
    Athlete,
    Exercising,
    Sedentary,
    Hypertensive,
    Infected,
    Diabetic,
    Leukemic,
    Stressed,
}

impl HostProfile {
    pub fn label(&self) -> &'static str {
        match self {
            HostProfile::Athlete      => "Athlete",
            HostProfile::Exercising   => "Exercising",
            HostProfile::Sedentary    => "Sedentary",
            HostProfile::Hypertensive => "Hypertensive",
            HostProfile::Infected     => "Active Infection",
            HostProfile::Diabetic     => "Type 2 Diabetic",
            HostProfile::Leukemic     => "Acute Leukemia",
            HostProfile::Stressed     => "Acute Stress Response",
        }
    }

    #[allow(dead_code)]
    pub fn description(&self) -> &'static str {
        match self {
            HostProfile::Athlete =>
                "High cardiovascular efficiency. Strong pulse, clean vessels, \
                 elevated O2 in pulmonary circulation. Low ambient immune activity.",
            HostProfile::Exercising =>
                "Mid-intensity aerobic exercise. Heart rate at 155 bpm and climbing. \
                 Skeletal muscle is consuming oxygen and glucose at three times the resting rate. \
                 Adrenal medulla active. Blood flow redistributed away from gut and toward muscle.",
            HostProfile::Sedentary =>
                "Baseline human physiology. Moderate blood pressure, \
                 standard immune activity, typical nutrient concentrations.",
            HostProfile::Hypertensive =>
                "Elevated systemic pressure. Turbulent flow in major arteries. \
                 Arterial walls thickened; navigation in narrow vessels is harder.",
            HostProfile::Infected =>
                "Active viral or bacterial infection. Immune threat elevated \
                 system-wide. Elevated temperature. Neutrophils on patrol everywhere.",
            HostProfile::Diabetic =>
                "Excess circulating glucose — a windfall for harvesting. \
                 But chronic inflammation and impaired vessel walls \
                 create elevated background threat.",
            HostProfile::Leukemic =>
                "Acute lymphoblastic leukemia. Leukemic blast cells crowd the bone marrow, \
                 suppressing normal hematopoiesis. Elevated blast count in the bloodstream. \
                 Anemia, thrombocytopenia, and immune suppression are active. \
                 CAR-T cells have been co-injected and are dispersing.",
            HostProfile::Stressed =>
                "Acute psychosocial stress. HPA axis and sympathoadrenal system both active. \
                 Cortisol and epinephrine elevated. Blood glucose high. Heart rate and blood \
                 pressure up. Gut perfusion reduced; immune surveillance suppressed.",
        }
    }

    /// Threat level modifier: added to each node's base threat (clamped to Extreme).
    pub fn threat_modifier(&self) -> i32 {
        match self {
            HostProfile::Athlete      => -1,
            HostProfile::Exercising   => -1,
            HostProfile::Sedentary    =>  0,
            HostProfile::Hypertensive =>  0,
            HostProfile::Infected     =>  1,
            HostProfile::Diabetic     =>  1,
            HostProfile::Leukemic     =>  1,
            HostProfile::Stressed     =>  0,
        }
    }

    /// Glucose harvest multiplier.
    pub fn glucose_multiplier(&self) -> f64 {
        match self {
            HostProfile::Diabetic    => 2.0,
            HostProfile::Stressed    => 1.8,
            HostProfile::Athlete     => 0.8,
            HostProfile::Exercising  => 0.5,  // muscle consuming it fast — scarce
            HostProfile::Leukemic    => 1.2,
            _                        => 1.0,
        }
    }

    /// Oxygen harvest multiplier.
    pub fn oxygen_multiplier(&self) -> f64 {
        match self {
            HostProfile::Athlete    => 1.4,
            HostProfile::Exercising => 1.8,  // lungs working hard — high pulmonary O2
            HostProfile::Leukemic   => 0.75,
            HostProfile::Stressed   => 0.9,
            _                       => 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    pub name:    String,
    pub profile: HostProfile,
    pub heart_rate: u32,        // bpm
    pub systolic:   u32,        // mmHg
    pub diastolic:  u32,        // mmHg
}

impl Host {
    pub fn blood_pressure_display(&self) -> String {
        format!("{}/{} mmHg", self.systolic, self.diastolic)
    }
}

static FIRST_NAMES: &[&str] = &[
    "Marcus", "Elena", "James", "Aiko", "Priya", "Carlos",
    "Fatima", "Noah", "Ingrid", "Omar", "Sofia", "Tariq",
];

static LAST_NAMES: &[&str] = &[
    "Chen", "Vasquez", "Okafor", "Tanaka", "Patel", "Reyes",
    "Andersen", "Hassan", "Johansson", "Mbeki", "Kowalski", "Ferreira",
];

pub fn generate_host() -> Host {
    let mut rng = rand::rng();
    let first = FIRST_NAMES[rng.random_range(0..FIRST_NAMES.len())];
    let last  = LAST_NAMES[rng.random_range(0..LAST_NAMES.len())];
    let name  = format!("{} {}", first, last);

    let profiles = [
        HostProfile::Athlete,
        HostProfile::Sedentary,
        HostProfile::Hypertensive,
        HostProfile::Infected,
        HostProfile::Diabetic,
        HostProfile::Leukemic,
    ];
    let profile = profiles[rng.random_range(0..profiles.len())].clone();

    let (hr, sys, dia) = match &profile {
        HostProfile::Athlete      => (55u32, 110u32, 70u32),
        HostProfile::Exercising   => (155,   168,    88),
        HostProfile::Sedentary    => (72,    120,    80),
        HostProfile::Hypertensive => (78,    158,    98),
        HostProfile::Infected     => (88,    125,    82),
        HostProfile::Diabetic     => (76,    135,    88),
        HostProfile::Leukemic     => (98,    105,    65),
        HostProfile::Stressed     => (94,    148,    94),
    };

    Host { name, profile, heart_rate: hr, systolic: sys, diastolic: dia }
}

pub fn generate_host_with_profile(profile: HostProfile) -> Host {
    let mut rng = rand::rng();
    let first = FIRST_NAMES[rng.random_range(0..FIRST_NAMES.len())];
    let last  = LAST_NAMES[rng.random_range(0..LAST_NAMES.len())];
    let name  = format!("{} {}", first, last);

    let (hr, sys, dia) = match &profile {
        HostProfile::Athlete      => (55u32, 110u32, 70u32),
        HostProfile::Exercising   => (155,   168,    88),
        HostProfile::Sedentary    => (72,    120,    80),
        HostProfile::Hypertensive => (78,    158,    98),
        HostProfile::Infected     => (88,    125,    82),
        HostProfile::Diabetic     => (76,    135,    88),
        HostProfile::Leukemic     => (98,    105,    65),
        HostProfile::Stressed     => (94,    148,    94),
    };

    Host { name, profile, heart_rate: hr, systolic: sys, diastolic: dia }
}
