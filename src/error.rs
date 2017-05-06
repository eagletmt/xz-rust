extern crate std;

#[derive(Debug)]
pub enum Error {
    InvalidHeaderMagic,
    UnsupportedHeaderVersion,
    UnsupportedHeaderFlags,
    CorruptedStreamHeader,
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
