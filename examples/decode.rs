extern crate xz;

fn main() {
    let file = std::fs::File::open("sample.xz").expect("Unable to open file");
    let mut decoder = xz::XzDecoder::new(file).expect("Unable to initialize XzDecoder");
    println!("stream header: {:?}", decoder.stream_header());

    let block_header = decoder.read_block_header();
    println!("block_header: {:?}", block_header);
}
