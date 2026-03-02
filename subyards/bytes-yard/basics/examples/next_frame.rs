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

pub struct Connection {
    buffer: Vec<u8>,
}

impl Connection {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Simulates receiving a chunk of bytes from a TCP socket
    pub fn receive_bytes(&mut self, chunk: &[u8]) {
        self.buffer.extend_from_slice(chunk);
    }

    /// Attempts to extract the next complete frame from the internal buffer.
    pub fn next_frame(&mut self) -> Option<Vec<u8>> {
        // We use a block scope {} to artificially limit the lifetime of the borrow.
        let (payload_owned, total_bytes_to_remove) = {
            // 1. Borrow `self.buffer` immutably
            if let Ok((_, payload)) = parse_frame(&self.buffer) {
                // 2. We got a reference. Let's copy the payload bytes into a brand new heap allocation immediately.
                // .to_vec() is the idiomatic way to copy a slice into an owned Vec.
                let payload_owned = payload.to_vec();

                // 3. Calculate how many bytes we need to remove.
                let total_bytes_to_remove = 4 + payload.len();

                (payload_owned, total_bytes_to_remove)
            } else {
                return None;
            }
        }; // The immutable borrow of `self.buffer` officially DIES right here!

        // Now we are free to borrow `self.buffer` mutably!
        // We drain and immediately drop the consumed bytes. No `.collect()` necessary.
        self.buffer.drain(..total_bytes_to_remove);

        // Return our single, perfectly sized allocation.
        Some(payload_owned)

        // if let Ok((_, payload)) = parse_frame(&self.buffer) {
        //     let p: Vec<u8> = self.buffer.drain(..4 + payload.len()).collect();
        //     return Some(Vec::from(&p[4..]));
        // }
        // None
    }
}

fn main() {
    println!("main");
    let mut conn = Connection::new();

    // 1. Send an incomplete message (just the header and 1 byte of payload)
    // Header says 5 bytes total payload, we only send "h"
    conn.receive_bytes(b"\x00\x00\x00\x05h");
    assert_eq!(conn.next_frame(), None);

    // 2. Send the rest of the payload ("ello") PLUS the start of the next message
    // Next message header says 4 bytes payload, we send "wor"
    conn.receive_bytes(b"ello\x00\x00\x00\x04wor");

    // We should now be able to read the first complete frame
    assert_eq!(conn.next_frame(), Some(b"hello".to_vec()));

    // We still don't have enough for the second frame
    assert_eq!(conn.next_frame(), None);

    // 3. Send the final byte of the second frame
    conn.receive_bytes(b"d");
    assert_eq!(conn.next_frame(), Some(b"word".to_vec())); // Note: "wor" + "ld" = "world", wait, 4 bytes is "worl", let's assume it was "word" in 4 bytes. Ah, "wor" + "d" is 4 bytes.

    // Buffer should be empty now
    assert_eq!(conn.next_frame(), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_state() {
        let mut conn = Connection::new();
        conn.receive_bytes(b"\x00\x00\x00\x05h");
        assert_eq!(conn.next_frame(), None);
        conn.receive_bytes(b"ello\x00\x00\x00\x05wor");
        assert_eq!(conn.next_frame(), Some(b"hello".to_vec()));
        assert_eq!(conn.next_frame(), None);
        conn.receive_bytes(b"ld");
        assert_eq!(conn.next_frame(), Some(b"world".to_vec()));
        assert_eq!(conn.next_frame(), None);
    }
}
