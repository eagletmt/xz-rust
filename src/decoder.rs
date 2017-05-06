extern crate std;

pub struct XzDecoder<R> {
    inner: std::io::BufReader<R>,
}

impl<R> XzDecoder<R>
    where R: std::io::Read
{
    pub fn new(reader: R) -> Result<Self, super::Error> {
        let mut reader = std::io::BufReader::new(reader);
        read_header_magic(&mut reader)?;
        Ok(Self { inner: reader })
    }
}

fn read_header_magic<R>(reader: &mut R) -> Result<(), super::Error>
    where R: std::io::Read
{
    let mut buf = [0; 6];
    reader.read_exact(&mut buf)?;
    if buf != super::constants::HEADER_MAGIC {
        Err(super::Error::InvalidHeaderMagic)
    } else {
        Ok(())
    }
}
