//! Canonical JSON serialization for observations (single path for CLI + HTTP).

use crate::Observation;

pub fn observation_to_vec(obs: &Observation) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(obs)
}

pub fn observation_to_string(obs: &Observation) -> Result<String, serde_json::Error> {
    serde_json::to_string(obs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Intent, Simulation, SimulationConfig};

    #[test]
    fn wire_bytes_match_direct_serde_value() {
        let mut s = Simulation::with_config(SimulationConfig::new("w", 9));
        s.apply_intent(Intent {
            kind: "k".to_string(),
        });
        s.step();
        let obs = s.snapshot_observation();
        let from_wire = serde_json::from_slice::<serde_json::Value>(&observation_to_vec(&obs).unwrap())
            .unwrap();
        let from_direct = serde_json::to_value(&obs).unwrap();
        assert_eq!(from_wire, from_direct);
    }
}
