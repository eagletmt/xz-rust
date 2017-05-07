#[derive(Debug)]
pub struct Block {
    pub block_header: BlockHeader,
}

#[derive(Debug)]
pub struct BlockHeader {
    pub block_header_size: u16,
    pub block_flags: BlockFlags,
}

#[derive(Debug)]
pub struct BlockFlags {
    pub number_of_filters: u8,
    pub compressed_size_field_is_present: bool,
    pub uncompressed_size_field_is_present: bool,
}

impl BlockFlags {
    pub fn new(flags: u8) -> Result<Self, super::Error> {
        let number_of_filters = (flags & 0b00000011) + 1;
        let reserved = (flags & 0b00111100) >> 2;
        let compressed_size_field_is_present = ((flags & 0b01000000) >> 6) != 0;
        let uncompressed_size_field_is_present = ((flags & 0b10000000) >> 7) != 0;
        if reserved != 0 {
            Err(super::Error::UnsupportedBlockFlags)
        } else {
            Ok(Self {
                   number_of_filters,
                   compressed_size_field_is_present,
                   uncompressed_size_field_is_present,
               })
        }
    }
}
