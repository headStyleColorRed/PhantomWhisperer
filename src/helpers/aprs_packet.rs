use serde::{Deserialize, Serialize};

// AprsPacket struct based on the AX.25 protocol, it consists of the following fields:
// - Destination address
// - Source address
// - Digipeaters: a list of digipeaters that will repeat the packet
// - Information field

#[derive(Deserialize, Serialize)]
pub struct AprsPacket {
    pub destination: String,
    pub source: String,
    pub information: String,
}

impl AprsPacket {
    pub fn new(destination: &str, source: &str, information: &str) -> Self {
        AprsPacket {
            destination: destination.to_string(),
            source: source.to_string(),
            information: information.to_string(),
        }
    }
}
