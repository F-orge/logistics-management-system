fn main() {
    tonic_build::configure()
        .out_dir("src/models/_proto")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(
            &[
                "proto/health_check.proto",
                "proto/employee_management.proto",
                "proto/auth.proto",
                "proto/file_management.proto",
                "proto/storage.proto",
            ],
            &["proto/"],
        )
        .unwrap();
}
