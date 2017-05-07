#[derive(Debug)]
pub enum FilterFlags {
    Lzma2(Lzma2FilterFlags),
}

#[derive(Debug)]
pub struct Lzma2FilterFlags {
    pub dictionary_size: u32,
}

impl Lzma2FilterFlags {
    /// Decode LZMA2 filter properties
    ///
    /// # Specification
    /// v1.0.4 5.3.1
    pub fn new(properties: &[u8]) -> Result<Self, super::Error> {
        if properties.len() != 1 {
            return Err(super::Error::InvalidFilterFlags(format!("LZMA2 properties must be 1 but got {}",
                                                                properties.len())));
        }
        let bits = properties[0] & 0x3f;
        if bits > 40 {
            return Err(super::Error::InvalidFilterFlags(format!("LZMA2 dictionary size cannot be larger than 4GiB")));
        }
        let dictionary_size = if bits == 40 {
            <u32>::max_value()
        } else {
            let mantissa = 2 | (bits & 1);
            (mantissa as u32) << ((bits >> 1) + 11)
        };
        Ok(Self { dictionary_size })
    }
}
