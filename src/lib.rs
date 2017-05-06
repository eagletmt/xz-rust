pub mod constants;
pub mod decoder;
pub use decoder::XzDecoder;
pub mod error;
pub use error::Error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
