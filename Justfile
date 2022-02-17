exrinfo:
    RUST_LOG=trace RUSTFLAGS="-Zsanitizer=address" cargo run --target "aarch64-apple-darwin" --example exrinfo -p openexr -- openexr/openexr-images/TestImages/GammaChart.exr