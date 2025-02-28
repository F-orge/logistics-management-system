fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = tonic_build::configure();

    let config = config.out_dir("./src/generated");

    let config = config.protoc_arg("--experimental_allow_proto3_optional");

    config.compile_protos(
        &[
            "proto/auth.proto",
            "proto/storage.proto",
            "proto/management.proto",
        ],
        &["./proto/"],
    )?;
    Ok(())
}
