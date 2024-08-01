
use crate::helpers::encoder;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_message() {
        // Test case 1: Short message (single packet)
        let source = "N0CALL";
        let destination = "APRS";
        let short_message = "Hello, World!";
        let packets = encoder::encode_message(source, destination, short_message);

        assert_eq!(packets.len(), 1, "Short message should produce a single packet");

        // Verify packet structure
        let packet = &packets[0];
        assert!(packet.len() > 16, "Packet should be long enough to contain addresses, control field, and protocol ID");
        assert_eq!(packet[14], 0x03, "Control field should be 0x03");
        assert_eq!(packet[15], 0xF0, "Protocol ID should be 0xF0");
        assert!(packet[16..].starts_with(short_message.as_bytes()), "Information field should contain the message");

        // Test case 2: Long message (multiple packets)
        let long_message = "A".repeat(300); // Message longer than MAX_PAYLOAD_SIZE
        let packets = encoder::encode_message(source, destination, &long_message);

        assert!(packets.len() > 1, "Long message should produce multiple packets");

        // Check first packet
        let first_packet = &packets[0];
        assert!(first_packet[16..].starts_with(b"{1:"), "First packet should start with sequence info");

        // Check last packet
        let last_packet = packets.last().unwrap();
        assert!(last_packet[16..].starts_with(format!("{{{}:", packets.len()).as_bytes()),
                "Last packet should have correct sequence number");
        assert!(last_packet[16..].windows(format!(":{}}}", packets.len()).len())
            .any(|window| window == format!(":{}}}", packets.len()).as_bytes()),
            "Last packet should have correct total packet count");

        // Verify total content
        let total_content: String = packets.iter()
            .map(|p| {
                let content = &p[16..p.len()-2]; // Skip header and CRC
                if content.starts_with(b"{") {
                    // Extract actual message content from sequenced packets
                    let end_of_seq = content.iter().position(|&r| r == b'}').unwrap_or(0) + 1;
                    String::from_utf8_lossy(&content[end_of_seq..]).into_owned()
                } else {
                    String::from_utf8_lossy(content).into_owned()
                }
            })
            .collect();
        assert_eq!(total_content, long_message, "Concatenated packet content should match original message");
    }
}
