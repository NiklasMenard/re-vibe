use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use std::error::Error;
use std::time::Duration;

pub async fn create_client() -> Result<Client, Box<dyn Error>> {
    let access_key_id = std::env::var("DIGITAL_OCEAN_ACCESS_KEY")?;
    let secret_access_key = std::env::var("DIGITAL_SECRET_ACCESS_KEY")?;

    // Set up region provider chain with DigitalOcean Spaces region
    let region_provider = RegionProviderChain::first_try(Some(Region::new("fra1")))
        .or_default_provider()
        .or_else(Region::new("fra1"));

    let creds = Credentials::new(
        access_key_id,
        secret_access_key,
        None,
        None,
        "DigitalOceanSpaces",
    );

    // Load shared config with updated function and BehaviorVersion argument
    let shared_config = aws_config::defaults(aws_config::BehaviorVersion::v2024_03_28())
        .region(region_provider)
        .credentials_provider(creds)
        .endpoint_url("https://fra1.digitaloceanspaces.com")
        .load()
        .await;

    // Create S3 client with custom endpoint
    let client = Client::new(&shared_config);

    Ok(client)
}

/// Generate a URL for a presigned GET request.
pub async fn generate_presigned_url(
    client: &Client,
    bucket: &str,
    key: &str,
    expires_in_seconds: u64,
) -> Result<String, Box<dyn Error>> {
    let expires_in = Duration::from_secs(expires_in_seconds);
    let presigning_config = PresigningConfig::builder().expires_in(expires_in).build()?;

    let presigned_request = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .presigned(presigning_config)
        .await?;

    Ok(presigned_request.uri().to_string())
}
