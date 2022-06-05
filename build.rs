fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["proto/address.proto", "proto/authenticate.proto", "proto/user_details_client.proto"],
            &["proto"],
        )?;

    tonic_build::configure()
        .compile(
            &["proto/google/protobuf/timestamp.proto"],
            &["proto/google/protobuf"],
        )?;
    Ok(())
}