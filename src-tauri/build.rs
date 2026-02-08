fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("../proto/v1/fetcher.proto")?;
    tauri_build::build();
    Ok(())
}
