pub mod block;
pub use block::Block;
pub use block::BlockHeader;
pub use block::BlockFlags;
pub use block::FilterFlags;
pub mod constants;
pub mod decoder;
pub use decoder::XzDecoder;
pub mod error;
pub use error::Error;
pub mod stream_header;
pub use stream_header::StreamHeader;
pub use stream_header::StreamFlags;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
