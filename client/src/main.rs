//! Minimal REv2 client: asks a remote execution endpoint for its capabilities.

use remote_execution::build::bazel::remote::execution::v2::GetCapabilitiesRequest;
use remote_execution::build::bazel::remote::execution::v2::capabilities_client::CapabilitiesClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let endpoint = args
        .next()
        .unwrap_or_else(|| "grpc://localhost:8980".to_string())
        .replacen("grpc://", "http://", 1);
    let instance_name = args.next().unwrap_or_default();

    let mut client = CapabilitiesClient::connect(endpoint).await?;
    let capabilities = client
        .get_capabilities(GetCapabilitiesRequest { instance_name })
        .await?
        .into_inner();
    println!("{capabilities:#?}");
    Ok(())
}
