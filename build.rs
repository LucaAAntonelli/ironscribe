fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = tonic_build::Config::new();
    config.type_attribute("booksync.Book", "#[derive(serde::Deserialize, serde::Serialize)]");
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos_with_config(config, &["proto/booksync.proto"], &["proto"])
        .expect("Failed to compile proto files!");
    tonic_build::compile_protos("proto/fileservice.proto")?;
    Ok(())
}
