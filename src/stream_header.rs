/// Stream Header
///
/// # Specification
/// v1.0.4 2.1.1
#[derive(Debug)]
pub struct StreamHeader {
    pub stream_flags: StreamFlags,
}

/// Stream Flags
///
/// # Specification
/// v1.0.4 2.1.1.2
#[derive(Debug)]
pub struct StreamFlags {
    pub check: Check,
}

#[derive(Debug)]
pub enum Check {
    None,
    CRC32,
    CRC64,
    SHA256,
}

impl StreamFlags {
    pub fn new(flags: u8) -> Result<Self, super::Error> {
        let check = flags & 0x0f;
        let reserved = (flags & 0xf0) >> 4;
        if reserved != 0 {
            return Err(super::Error::UnsupportedHeaderFlags);
        }
        Ok(Self { check: check_type_of(check)? })
    }
}

fn check_type_of(flags: u8) -> Result<Check, super::Error> {
    match flags {
        0x00 => Ok(Check::None),
        0x01 => Ok(Check::CRC32),
        0x04 => Ok(Check::CRC64),
        0x0a => Ok(Check::SHA256),
        _ => Err(super::Error::UnsupportedHeaderFlags),
    }
}
