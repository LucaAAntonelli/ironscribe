fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/filesync.proto")?;
    tonic_build::compile_protos("proto/newfilesync.proto")?;
    Ok(())
}
