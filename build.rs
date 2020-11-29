fn main() {
    prost_build::compile_protos(&["storeddata.proto"], &["."]).unwrap();
}
