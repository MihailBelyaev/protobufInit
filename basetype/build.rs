use std::path::Path;

fn main() {
    tonic_build::configure()
        .out_dir(Path::new(""))
        .compile(&["protobuf/base.proto"], &["protobuf"])
        .unwrap()
}
