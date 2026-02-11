fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(
            &["../proto/v1/fetcher.proto", "../proto/v1/speaker.proto"],
            &["../proto/v1"],
        )?;
    Ok(())
}
