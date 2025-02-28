use tonic_build::Builder;

fn add_sqlx_from_row(config: Builder) -> Builder {
    config.message_attribute(".", "#[derive(sqlx::FromRow)]")
}

fn replace_uuid_to_string(mut config: Builder) -> Builder {
    config = config.field_attribute(
        "FileMetadata.id",
        r#"#[sqlx(try_from = "sqlx::types::Uuid")]"#,
    );
    config = config.field_attribute(
        "FileMetadata.owner_id",
        r#"#[sqlx(try_from = "sqlx::types::Uuid")]"#,
    );
    config.field_attribute("FileMetadata.size", r#"#[sqlx(try_from = "i32")]"#)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = tonic_build::configure();

    let config = config.out_dir("./src/generated");

    let config = config.protoc_arg("--experimental_allow_proto3_optional");

    let config = add_sqlx_from_row(config);

    let config = replace_uuid_to_string(config);

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
