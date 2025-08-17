//! Configuration types for the Vixen stream server.

use std::net::SocketAddr;

use clap::Args;
use serde::Deserialize;
use yellowstone_vixen::config::VixenConfig;

#[derive(Debug, Args)]
pub struct StreamConfig<M, S>
where
    M: Args,
    S: Args,
{
    #[command(flatten)]
    pub grpc: GrpcConfig,

    #[command(flatten)]
    pub runtime: VixenConfig<M, S>,
}

#[derive(Deserialize)]
struct StreamConfigInner<M: Args, S: Args> {
    grpc: GrpcConfig,
    runtime: VixenConfig<M, S>,
}

impl<'de, M, S> Deserialize<'de> for StreamConfig<M, S>
where
    M: Args + Deserialize<'de>,
    S: Args + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let StreamConfigInner { grpc, runtime } =
            StreamConfigInner::<M, S>::deserialize(deserializer)?;
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

#[inline]
fn default_addr() -> SocketAddr { "[::]:3030".parse().unwrap() }
