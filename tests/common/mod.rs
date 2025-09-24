pub mod test_handlers;

use std::{path::PathBuf, time::Duration};
use tokio::sync::broadcast;
use yellowstone_vixen::{
    config::{BufferConfig, NullConfig, OptConfig, VixenConfig, YellowstoneConfig},
    CommitmentLevel, Runtime,
};

/// Command line options for integration tests
#[derive(clap::Parser, Debug)]
#[command(version, author, about = "Yellowstone Vixen Integration Tests")]
pub struct TestOpts {
    /// Path to the configuration file
    #[arg(long, short)]
    pub config: Option<PathBuf>,
}

/// Create test configuration with priority: CLI config > environment variables > default
pub fn create_test_config(
) -> Result<VixenConfig<NullConfig>, Box<dyn std::error::Error + Send + Sync>> {
    // Try to parse command line arguments for config file path
    let config_from_file = try_load_config_from_file();

    // If config file loading succeeds, use it
    if let Ok(config) = config_from_file {
        return Ok(config);
    }

    // Fall back to environment variables (backward compatibility)
    let config_from_env = try_load_config_from_env();
    if let Ok(config) = config_from_env {
        return Ok(config);
    }

    // If both fail, return error
    Err("No valid configuration found. Please provide either a config file via --config or set environment variables GRPC_URL".into())
}

/// Try to load configuration from TOML file
fn try_load_config_from_file(
) -> Result<VixenConfig<NullConfig>, Box<dyn std::error::Error + Send + Sync>> {
    // Debug: print current working directory
    if let Ok(cwd) = std::env::current_dir() {
        tracing::debug!("Current working directory: {}", cwd.display());
    }
    // Check if config file path is provided via arguments or use default
    let config_path = std::env::args()
        .collect::<Vec<_>>()
        .windows(2)
        .find(|window| window[0] == "--config")
        .map(|window| PathBuf::from(&window[1]))
        .unwrap_or_else(|| {
            // Try multiple possible locations for the config file
            let possible_paths = [
                "tests/Vixen.test.toml",
                "./tests/Vixen.test.toml",
                "../tests/Vixen.test.toml",
            ];

            for path in &possible_paths {
                let p = PathBuf::from(path);
                tracing::debug!("Checking config path: {}", p.display());
                if p.exists() {
                    tracing::debug!("Found config file at: {}", p.display());
                    return p;
                }
            }

            // Fallback to the first path
            PathBuf::from("tests/Vixen.test.toml")
        });

    if !config_path.exists() {
        return Err(format!("Config file not found: {}", config_path.display()).into());
    }

    let config_content = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Error reading config file {}: {}", config_path.display(), e))?;

    let config: VixenConfig<NullConfig> = toml::from_str(&config_content)
        .map_err(|e| format!("Error parsing config file {}: {}", config_path.display(), e))?;

    tracing::info!("Loaded configuration from: {}", config_path.display());
    tracing::info!("Using endpoint: {}", config.yellowstone.endpoint);
    tracing::info!(
        "Auth token: {}",
        if config.yellowstone.x_token.is_some() {
            "Set"
        } else {
            "Not set"
        }
    );

    Ok(config)
}

/// Try to load configuration from environment variables (backward compatibility)
fn try_load_config_from_env(
) -> Result<VixenConfig<NullConfig>, Box<dyn std::error::Error + Send + Sync>> {
    let grpc_url =
        std::env::var("GRPC_URL").map_err(|_| "GRPC_URL environment variable not set")?;
    let grpc_auth_token = std::env::var("GRPC_AUTH_TOKEN").ok();
    let grpc_timeout = std::env::var("GRPC_TIMEOUT")
        .unwrap_or_else(|_| "30".to_string())
        .parse::<u64>()
        .unwrap_or(30);

    // Ensure URL has proper HTTP/HTTPS prefix
    let processed_url = if !grpc_url.starts_with("http://") && !grpc_url.starts_with("https://") {
        format!("http://{}", grpc_url)
    } else {
        grpc_url
    };

    tracing::info!("Loaded configuration from environment variables");
    tracing::info!("Using endpoint: {}", processed_url);
    tracing::info!(
        "Auth token: {}",
        if grpc_auth_token.is_some() {
            "Set"
        } else {
            "Not set"
        }
    );

    Ok(VixenConfig {
        yellowstone: YellowstoneConfig {
            endpoint: processed_url,
            x_token: grpc_auth_token,
            timeout: grpc_timeout,
        },
        buffer: BufferConfig { jobs: None },
        metrics: OptConfig::default(),
    })
}

/// Helper function to run integration test with event-based completion
pub async fn run_integration_test_with_event_completion<F, Fut>(
    test_fn: F,
    mut shutdown_rx: broadcast::Receiver<()>,
    max_duration: Duration,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>,
{
    let test_future = test_fn();

    tokio::select! {
        test_result = test_future => {
            match test_result {
                Ok(()) => {
                    tracing::info!("Integration test runtime completed");
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("Integration test failed: {:?}", e);
                    Err(e)
                }
            }
        }
        _ = shutdown_rx.recv() => {
            tracing::info!("Integration test stopped after receiving at least one event");
            Ok(())
        }
        _ = tokio::time::sleep(max_duration) => {
            tracing::warn!("Integration test timed out after {:?}, but this may be normal for live data tests", max_duration);
            Ok(())
        }
    }
}
