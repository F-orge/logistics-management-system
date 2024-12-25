fn main() {
    tonic_build::configure()
        .out_dir("src/models")
        .compile_protos(&["proto/health_check.proto","proto/employee_management.proto"], &["proto/"])
        .unwrap();
}
