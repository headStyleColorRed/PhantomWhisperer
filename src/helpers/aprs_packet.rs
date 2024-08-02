use serde::{Deserialize, Serialize};

// AprsPacket struct based on the AX.25 protocol, it consists of the following fields:
// - Destination address
// - Source address
// - Digipeaters: a list of digipeaters that will repeat the packet
// - Information field

#[derive(Deserialize, Serialize)]
pub struct AprsPacket {
    pub source: String,
    pub destination: String,
    pub information: String,
}

impl AprsPacket {
    pub fn new(source: &str, destination: &str, information: &str) -> Self {
        AprsPacket {
            source: source.to_string(),
            destination: destination.to_string(),
            information: information.to_string(),
        }
    }
}
