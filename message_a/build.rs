use std::path::Path;

fn main() {
    tonic_build::configure().out_dir(Path::new("")).
    extern_path(".basetype", "::basetype").
    compile(&["protobuf/message_a.proto"],&["protobuf",".."]).unwrap()
}