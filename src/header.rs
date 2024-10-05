use std::convert::TryInto;

#[derive(Debug)]
pub struct NetworkProtocolHeader {
    pub version: u8,
    pub message_id: u16,
    pub payload_size: u32,
    pub reserved: [u8; 2],
    pub extra_header_size: u32,
    pub extra_header: Vec<u8>,
}

impl Default for NetworkProtocolHeader {
    fn default() -> Self {
        Self {
            version: 0,
            message_id: 0,
            payload_size: 0,
            reserved: [0; 2],
            extra_header_size: 0,
            extra_header: vec![],
        }
    }
}

impl NetworkProtocolHeader {
    // #[allow(dead_code)]
    pub fn pack(&self) -> Vec<u8> {
        let mut packed_header = vec![0u8; 13 + self.extra_header.len()];
        packed_header[0] = self.version;
        packed_header[1..3].copy_from_slice(&self.message_id.to_be_bytes());
        packed_header[3..7].copy_from_slice(&self.payload_size.to_be_bytes());
        packed_header[7..9].copy_from_slice(&self.reserved);
        packed_header[9..13].copy_from_slice(&self.extra_header_size.to_be_bytes());
        packed_header[13..].copy_from_slice(&self.extra_header);
        packed_header
    }

    // #[allow(dead_code)]
    pub fn unpack(&mut self, buffer: &[u8]) -> Result<(), &'static str> {
        if buffer.len() < 13 {
            return Err("Buffer too short");
        }
        self.version = buffer[0];
        self.message_id = u16::from_be_bytes(buffer[1..3].try_into().unwrap());
        self.payload_size = u32::from_be_bytes(buffer[3..7].try_into().unwrap());
        self.reserved.copy_from_slice(&buffer[7..9]);
        self.extra_header_size = u32::from_be_bytes(buffer[9..13].try_into().unwrap());
        if buffer.len() < 13 + self.extra_header_size as usize {
            return Err("Buffer too short");
        }
        self.extra_header = buffer[13..13 + self.extra_header_size as usize].to_vec();
        Ok(())
    }
}

