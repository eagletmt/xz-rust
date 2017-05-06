extern crate xz;

fn main() {
    let file = std::fs::File::open("sample.xz").expect("Unable to open file");
    let decoder = xz::XzDecoder::new(file).expect("Unable to initialize XzDecoder");
}
