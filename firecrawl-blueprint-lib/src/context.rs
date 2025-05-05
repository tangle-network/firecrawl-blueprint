use bollard::Docker;
use bollard::container::{
    Config as ContainerConfig, CreateContainerOptions, StartContainerOptions,
};
use ipfs_api::IpfsClient;
use std::env;
use url::Url;

/// Application configuration parameters loaded from environment.
#[derive(Clone)] // Add Clone derive
pub struct BlueprintEnvironment {
    /// HTTP port for the Firecrawl service container.
    pub firecrawl_http_port: u16,
    /// API endpoint URL for local IPFS node.
    pub ipfs_api_url: String,
}

impl BlueprintEnvironment {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        let firecrawl_http_port = env::var("FIRECRAWL_HTTP_PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .expect("Invalid FIRECRAWL_HTTP_PORT");
        let ipfs_api_url =
            env::var("IPFS_API_URL").unwrap_or_else(|_| "http://127.0.0.1:5001".into());
        BlueprintEnvironment {
            firecrawl_http_port,
            ipfs_api_url,
        }
    }
}

/// Extended execution context for Firecrawl and IPFS integration.
#[derive(Clone)] // Add Clone derive as Context must be Clone for Router
pub struct Context {
    /// Application environment configuration.
    pub env: BlueprintEnvironment, // Add env field
    /// Docker client for managing containers.
    pub docker: Docker,
    /// Reference to the Firecrawl service Docker image.
    pub firecrawl_image: String,
    /// IPFS HTTP API client.
    pub ipfs_client: IpfsClient,
}

impl Context {
    /// Initialize Docker client, IPFS client, and store environment.
    pub fn new(env: BlueprintEnvironment) -> Self {
        // Take env by value
        // Initialize Docker client
        let docker =
            Docker::connect_with_local_defaults().expect("Failed to connect to Docker daemon");

        // Define Firecrawl image reference
        let firecrawl_image = "firecrawl-service:latest".to_string();

        // Create and start Firecrawl container
        let container_config = ContainerConfig {
            image: Some(firecrawl_image.clone()),
            env: Some(vec![format!("PORT={}", env.firecrawl_http_port)]),
            ..Default::default()
        };
        let create_opts = CreateContainerOptions {
            name: "firecrawl_service",
        };
        let container = docker
            .create_container(Some(create_opts), container_config)
            .expect("Failed to create Firecrawl container");
        docker
            .start_container(&container.id, None::<StartContainerOptions<String>>())
            .expect("Failed to start Firecrawl container");

        // Initialize IPFS client
        let url = Url::parse(&env.ipfs_api_url).expect("Invalid IPFS_API_URL");
        let ipfs_client = IpfsClient::new(url.host_str().unwrap(), url.port().unwrap_or(5001));

        Context {
            env, // Store env
            docker,
            firecrawl_image,
            ipfs_client,
        }
    }
}
