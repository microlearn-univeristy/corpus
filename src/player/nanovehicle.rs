use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NanoVehicle {
    pub name:           String,
    /// Hull integrity: 1.0 = pristine, 0.0 = destroyed.
    pub hull_integrity: f64,
    /// Reduces random threat damage by this percentage.
    pub shielding:      f64,
}

impl NanoVehicle {
    pub fn new(pilot_name: &str) -> Self {
        NanoVehicle {
            name: format!("{}'s Vessel", pilot_name),
            hull_integrity: 1.0,
            shielding: 0.0,
        }
    }

    pub fn apply_damage(&mut self, pct: f64) {
        self.hull_integrity = (self.hull_integrity - pct / 100.0).max(0.0);
    }

    pub fn is_destroyed(&self) -> bool {
        self.hull_integrity <= 0.0
    }

    pub fn hull_display(&self) -> String {
        let pct = (self.hull_integrity * 100.0).round() as u32;
        format!("{}%", pct)
    }
}
