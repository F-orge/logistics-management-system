fn main() {
    tonic_build::configure()
        .compile_protos(&["proto/health_check.proto"], &["proto/health_check"])
        .unwrap();
}
