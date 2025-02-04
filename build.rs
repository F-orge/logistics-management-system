fn main() {
    tonic_build::configure()
        .out_dir("./crate-proto/src")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(
            &[
                "crate-proto/proto/auth.proto",
                "crate-proto/proto/storage.proto",
            ],
            &["./crate-proto/proto/"],
        )
        .unwrap();
}
