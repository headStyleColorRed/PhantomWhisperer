use crc16::*;
use crate::helpers::constants::*;

// AprsPacket struct based on the AX.25 protocol, it consists of the following fields:
// - Destination address
// - Source address
// - Digipeaters: a list of digipeaters that will repeat the packet
// - Information field
pub struct AprsPacket {
    destination: String,
    source: String,
    information: String,
}

impl AprsPacket {
    pub fn new(destination: &str, source: &str, information: &str) -> Self {
        AprsPacket {
            destination: destination.to_string(),
            source: source.to_string(),
            information: information.to_string(),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut packet = Vec::new();

        // Add addresses
        packet.extend(encode_address(&self.destination, false));
        packet.extend(encode_address(&self.source, true));

        // Control field and Protocol ID
        packet.push(0x03); // Control field: UI-frame
        packet.push(0xf0); // Protocol ID: no layer 3

        // Information field
        packet.extend(self.information.as_bytes());

        // Calculate and append CRC
        let crc = State::<CCITT_FALSE>::calculate(&packet);
        packet.extend(&crc.to_le_bytes());

        packet
    }
}

fn encode_address(address: &str, last: bool) -> Vec<u8> {
    let mut encoded = Vec::new();
    let parts: Vec<&str> = address.split('-').collect();
    let callsign = parts[0].to_uppercase();
    let ssid = parts.get(1).unwrap_or(&"0").parse::<u8>().unwrap_or(0);

    for (i, byte) in callsign.as_bytes().iter().enumerate() {
        if i < 6 {
            encoded.push(byte << 1);
        }
    }

    while encoded.len() < 6 {
        encoded.push(b' ' << 1);
    }

    let mut ssid_byte = 0b01100000 | (ssid << 1);
    if last {
        ssid_byte |= 1;
    }
    encoded.push(ssid_byte);

    encoded
}

pub fn encode_message(source: &str, destination: &str, message: &str) -> Vec<Vec<u8>> {
    let mut packets = Vec::new();
    let chunks = message.as_bytes().chunks(MAX_PAYLOAD_SIZE);
    let total_chunks = chunks.len();

    for (i, chunk) in chunks.enumerate() {
        let info = if total_chunks > 1 {
            format!("{{{}:{}}}{}", i + 1, total_chunks, String::from_utf8_lossy(chunk))
        } else {
            String::from_utf8_lossy(chunk).into_owned()
        };

        let packet = AprsPacket::new(destination, source, &info);
        packets.push(packet.encode());
    }

    packets
}
