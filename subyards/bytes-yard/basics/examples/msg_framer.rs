#[derive(Debug, PartialEq)]
pub enum FrameError {
    /// Not enough bytes in the buffer to read a complete message
    Incomplete,
}

/// Attempts to parse a single length-prefixed message from the buffer.
///
/// Returns `Ok((remaining_buffer, payload))` if a message was successfully parsed.
/// Returns `Err(FrameError::Incomplete)` if we need to wait for more bytes.
pub fn parse_frame(buffer: &[u8]) -> Result<(&[u8], &[u8]), FrameError> {
    let (head, rest) = buffer.split_at_checked(4).ok_or(FrameError::Incomplete)?;

    let header: [u8; 4] = head.try_into().unwrap();
    let size = u32::from_be_bytes(header);

    let (payload, reminder) = rest
        .split_at_checked(size as usize)
        .ok_or(FrameError::Incomplete)?;

    Ok((reminder, payload))
}

fn main() {
    let buffer = b"\x00\x00\x00\x05hello";
    println!("{buffer:?}");
    let (remaining, payload) = parse_frame(buffer).unwrap();
    println!("{payload:?}");

    assert_eq!(payload, b"hello");
    assert_eq!(remaining, b""); // Nothing left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_message() {
        // 4 byte header indicating a 5 byte payload ("hello")
        let buffer = b"\x00\x00\x00\x05hello";
        let (remaining, payload) = parse_frame(buffer).unwrap();

        assert_eq!(payload, b"hello");
        assert_eq!(remaining, b""); // Nothing left
    }

    #[test]
    fn test_incomplete_header() {
        // Only 3 bytes received so far
        let buffer = b"\x00\x00\x00";
        let result = parse_frame(buffer);

        assert_eq!(result, Err(FrameError::Incomplete));
    }

    #[test]
    fn test_incomplete_payload() {
        // Header says 10 bytes, but we only have 5 bytes of payload
        let buffer = b"\x00\x00\x00\x0Aworld";
        let result = parse_frame(buffer);

        assert_eq!(result, Err(FrameError::Incomplete));
    }

    #[test]
    fn test_multiple_messages() {
        // Two complete messages: "hi" (2 bytes) and "rust" (4 bytes)
        let buffer = b"\x00\x00\x00\x02hi\x00\x00\x00\x04rust";

        // Parse first message
        let (remaining, payload1) = parse_frame(buffer).unwrap();
        assert_eq!(payload1, b"hi");

        // Parse second message from the remaining buffer
        let (remaining, payload2) = parse_frame(remaining).unwrap();
        assert_eq!(payload2, b"rust");
        assert_eq!(remaining, b"");
    }
}
