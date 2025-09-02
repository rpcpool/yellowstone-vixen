//! Configuration types for the Vixen stream server.

use std::net::SocketAddr;

use clap::Args;
use serde::Deserialize;
use yellowstone_vixen::config::VixenConfig;

#[derive(Debug, Args)]
pub struct StreamConfig<S>
where S: Args
{
    #[command(flatten)]
    pub grpc: GrpcConfig,

    #[command(flatten)]
    pub runtime: VixenConfig<S>,
}

#[derive(Deserialize)]
struct StreamConfigInner<S: Args> {
    #[serde(default)]
    grpc: GrpcConfig,
    #[serde(flatten)]
    runtime: VixenConfig<S>,
}

impl<'de, S> Deserialize<'de> for StreamConfig<S>
where S: Args + Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let StreamConfigInner { grpc, runtime } =
            StreamConfigInner::<S>::deserialize(deserializer)?;
        Ok(Self { grpc, runtime })
    }
}

/// gRPC server configuration.
#[derive(Debug, Clone, Copy, clap::Args, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GrpcConfig {
    /// The address to bind the gRPC server to.
    #[arg(long = "grpc-address", env = "GRPC_ADDRESS", default_value_t = default_addr())]
    #[serde(default = "default_addr")]
    pub address: SocketAddr,
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            address: default_addr(),
        }
    }
}

#[inline]
fn default_addr() -> SocketAddr { "[::]:3030".parse().unwrap() }
