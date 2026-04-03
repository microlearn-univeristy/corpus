use serde::{Deserialize, Serialize};

/// ATP synthesis: 1 glucose + 1 oxygen → ATP_PER_SYNTHESIS ATP.
pub const ATP_PER_SYNTHESIS: u32 = 10;
pub const STARTING_ATP: u32 = 80;
pub const MAX_ATP: u32 = 200;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub atp:     u32,
    pub glucose: u32,
    pub oxygen:  u32,
    /// Tissue/cell samples collected for the Atlas log.
    pub samples: Vec<String>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            atp: STARTING_ATP,
            glucose: 10,
            oxygen: 10,
            samples: vec![],
        }
    }

    /// Synthesize ATP from glucose + oxygen (cellular respiration analogue).
    /// Returns the number of ATP synthesized.
    pub fn synthesize_atp(&mut self) -> u32 {
        let batches = self.glucose.min(self.oxygen).min(
            (MAX_ATP.saturating_sub(self.atp)) / ATP_PER_SYNTHESIS
        );
        if batches == 0 { return 0; }
        self.glucose -= batches;
        self.oxygen  -= batches;
        let gained = batches * ATP_PER_SYNTHESIS;
        self.atp = (self.atp + gained).min(MAX_ATP);
        gained
    }

    pub fn spend_atp(&mut self, amount: u32) -> bool {
        if self.atp < amount { return false; }
        self.atp -= amount;
        true
    }
}
