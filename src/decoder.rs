extern crate crc;
extern crate std;

use std::io::Read;

pub struct XzDecoder<R> {
    inner: std::io::BufReader<R>,
    stream_header: super::StreamHeader,
}

impl<R> XzDecoder<R>
    where R: std::io::Read
{
    pub fn new(reader: R) -> Result<Self, super::Error> {
        let mut reader = std::io::BufReader::new(reader);
        let stream_header = read_stream_header(&mut reader)?;
        Ok(Self {
               inner: reader,
               stream_header: stream_header,
           })
    }

    pub fn read_block_header(&mut self) -> Result<super::BlockHeader, super::Error> {
        // Specification: v1.0.4 3.1.1
        let mut encoded_header_size = [0; 1];
        self.inner.read_exact(&mut encoded_header_size)?;
        if encoded_header_size[0] == 0 {
            panic!("TODO: This is index");
        }
        let block_header_size = ((encoded_header_size[0] as u16) + 1) * 4;
        let mut buf = vec![0; block_header_size as usize - 5]; // 1 byte for encoded_header_size, 4 bytes for crc32
        self.inner.read_exact(&mut buf)?;
        let crc32 = read_u32le(&mut self.inner)?;

        {
            use self::crc::Hasher32;
            let mut digest = crc::crc32::Digest::new(crc::crc32::IEEE);
            digest.write(&encoded_header_size);
            digest.write(&buf);
            if crc32 != digest.sum32() {
                return Err(super::Error::CorruptedBlockHeader);
            }
        }

        let mut cursor = std::io::Cursor::new(buf);
        let mut block_flags = [0; 1];
        cursor.read_exact(&mut block_flags)?;
        let block_flags = super::BlockFlags::new(block_flags[0])?;

        let compressed_size = if block_flags.compressed_size_field_is_present {
            Some(read_multibyte_integer(&mut self.inner)?)
        } else {
            None
        };

        let uncompressed_size = if block_flags.uncompressed_size_field_is_present {
            Some(read_multibyte_integer(&mut self.inner)?)
        } else {
            None
        };

        let mut filters = Vec::new();
        for _ in 0..block_flags.number_of_filters {
            filters.push(read_filter_flags(&mut cursor)?);
        }

        // Specification: v1.0.4 3.1.6
        for padding in cursor.bytes() {
            if padding? != 0 {
                return Err(super::Error::InvalidBlockHeaderPadding);
            }
        }

        Ok(super::BlockHeader {
               block_header_size,
               block_flags,
               compressed_size,
               uncompressed_size,
               filters,
           })
    }
}

impl<R> XzDecoder<R> {
    pub fn stream_header(&self) -> &super::StreamHeader {
        &self.stream_header
    }
}

fn read_stream_header<R>(mut reader: &mut R) -> Result<super::StreamHeader, super::Error>
    where R: std::io::Read
{
    let mut header_magic = [0; 6];
    reader.read_exact(&mut header_magic)?;
    if header_magic != super::constants::HEADER_MAGIC {
        return Err(super::Error::InvalidHeaderMagic);
    }

    let mut stream_flags = [0; 2];
    reader.read_exact(&mut stream_flags)?;
    let crc32 = read_u32le(&mut reader)?;
    // Specification: v1.0.4 2.1.1.3
    if crc32 != crc::crc32::checksum_ieee(&stream_flags) {
        return Err(super::Error::CorruptedStreamHeader);
    }
    if stream_flags[0] != 0 {
        return Err(super::Error::UnsupportedHeaderVersion);
    }
    let stream_flags = super::StreamFlags::new(stream_flags[1])?;

    Ok(super::StreamHeader { stream_flags: stream_flags })
}

fn read_u32le<R>(reader: &mut R) -> Result<u32, std::io::Error>
    where R: std::io::Read
{
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok((buf[0] as u32) | ((buf[1] as u32) << 8) | ((buf[2] as u32) << 16) | ((buf[3] as u32) << 24))
}

// Specification v1.0.4 3.1.5
fn read_filter_flags<R>(mut reader: &mut R) -> Result<super::FilterFlags, super::Error>
    where R: std::io::Read
{
    let filter_id = read_multibyte_integer(&mut reader)?;
    let size_of_properties = read_multibyte_integer(&mut reader)? as usize;
    let mut properties = vec![0; size_of_properties];
    reader.read_exact(&mut properties)?;

    match filter_id {
        0x21 => Ok(super::FilterFlags::Lzma2(super::Lzma2FilterFlags::new(&properties)?)),
        _ => Err(super::Error::UnsupportedFilterId(filter_id)),
    }
}

// Specification v1.0.4 1.2
fn read_multibyte_integer<R>(reader: &mut R) -> Result<u64, std::io::Error>
    where R: std::io::Read
{
    let mut buf = [0; 1];

    reader.read_exact(&mut buf)?;
    let mut num = (buf[0] & 0x7f) as u64;
    let mut i = 0;
    while (buf[0] & 0x80) != 0 {
        reader.read_exact(&mut buf)?;
        i += 1;
        num |= ((buf[0] & 0x7f) as u64) << (i * 7);
    }
    Ok(num)
}
