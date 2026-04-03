use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub pilot_name:           String,
    /// ID of the current circuit node.
    pub current_node_id:      String,
    /// All node IDs ever visited (for Atlas).
    pub visited_nodes:        Vec<String>,
    /// Objective node IDs reached for the active campaign.
    pub objectives_reached:   Vec<String>,
    /// Objective node IDs required by the active campaign.
    pub active_objectives:    Vec<String>,
    /// Node ID where the player must return to complete the campaign.
    pub win_node:             String,
    /// Identifier of the active campaign.
    pub campaign_id:          String,
    /// Display name of the active campaign.
    pub campaign_name:        String,
    /// True once all objectives reached and player returned to win_node.
    pub mission_complete:     bool,
    /// Whether the current node has been scanned this visit.
    pub scanned_this_visit:   bool,
    /// Whether resources at current node have been harvested this visit.
    pub harvested_this_visit: bool,
}

impl PlayerState {
    pub fn new(
        pilot_name:     String,
        campaign_id:    &str,
        campaign_name:  &str,
        objectives:     Vec<String>,
        win_node:       String,
    ) -> Self {
        use crate::body::atlas::START_NODE;
        PlayerState {
            pilot_name,
            current_node_id:   START_NODE.to_string(),
            visited_nodes:     vec![START_NODE.to_string()],
            objectives_reached: vec![],
            active_objectives: objectives,
            win_node,
            campaign_id:       campaign_id.to_string(),
            campaign_name:     campaign_name.to_string(),
            mission_complete:  false,
            scanned_this_visit:    false,
            harvested_this_visit:  false,
        }
    }

    pub fn visit_node(&mut self, node_id: &str) {
        if !self.visited_nodes.contains(&node_id.to_string()) {
            self.visited_nodes.push(node_id.to_string());
        }
        self.scanned_this_visit   = false;
        self.harvested_this_visit = false;
    }

    pub fn record_objective(&mut self, node_id: &str) {
        if self.active_objectives.contains(&node_id.to_string())
            && !self.objectives_reached.contains(&node_id.to_string())
        {
            self.objectives_reached.push(node_id.to_string());
        }
    }

    pub fn has_visited(&self, node_id: &str) -> bool {
        self.visited_nodes.contains(&node_id.to_string())
    }

    pub fn objectives_complete(&self) -> bool {
        self.active_objectives.iter().all(|obj| self.objectives_reached.contains(obj))
    }
}
