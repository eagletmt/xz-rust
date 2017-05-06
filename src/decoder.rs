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

fn read_stream_header<R>(reader: &mut R) -> Result<super::StreamHeader, super::Error>
    where R: std::io::Read
{
    let mut header_magic = [0; 6];
    reader.read_exact(&mut header_magic)?;
    if header_magic != super::constants::HEADER_MAGIC {
        return Err(super::Error::InvalidHeaderMagic);
    }

    let mut stream_version = [0; 1];
    reader.read_exact(&mut stream_version)?;
    if stream_version[0] != 0 {
        return Err(super::Error::UnsupportedHeaderVersion);
    }

    let mut stream_flags = [0; 1];
    reader.read_exact(&mut stream_flags)?;
    let stream_flags = super::StreamFlags::new(stream_flags[0])?;
    Ok(super::StreamHeader { stream_flags: stream_flags })
}
