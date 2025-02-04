fn main() {
    tonic_build::configure()
        .out_dir("crate-proto/src")
        .compile_well_known_types(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(
            &[
                "crate-proto/proto/health_check.proto",
                "crate-proto/proto/employee_management.proto",
                "crate-proto/proto/auth.proto",
                "crate-proto/proto/file_management.proto",
                "crate-proto/proto/storage.proto",
            ],
            &["crate-proto/proto/"],
        )
        .unwrap();
}
