use serde::{Deserialize, Serialize};

// AprsPacket struct based on the AX.25 protocol, it consists of the following fields:
// - Source address
// - Destination address
// - Digipeaters: a list of digipeaters that will repeat the packet
// - Information field

#[derive(Deserialize, Serialize)]
pub struct AprsPacket {
    pub source: String,
    pub destination: String,
    pub digipeaters: Vec<String>,
    pub information: String,
}

impl AprsPacket {
    // add digipeaters optional value
    pub fn new(
        source: &str,
        destination: &str,
        digipeaters: &Vec<String>,
        information: &str,
    ) -> Self {
        AprsPacket {
            source: source.to_string(),
            destination: destination.to_string(),
            digipeaters: digipeaters.clone(),
            information: information.to_string()
        }
    }
}
