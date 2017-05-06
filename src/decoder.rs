extern crate crc;
extern crate std;

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
