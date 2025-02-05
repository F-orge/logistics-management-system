fn main() {
    tonic_build::configure()
        .out_dir("./src")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&["proto/auth.proto", "proto/storage.proto"], &["./proto/"])
        .unwrap();
}
