fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile_protos(
            &[
                "../proto/common/common.proto",
                "../proto/churner/churner.proto",
                "../proto/disperser/disperser.proto",
                "../proto/node/node.proto",
                "../proto/retriever/retriever.proto",
            ],
            &["../proto"],
        )?;
    Ok(())
}
