//! Configuration types for the Vixen stream server.

use std::net::SocketAddr;

use crate::config::VixenConfig;

/// Root configuration type for [the Vixen stream server](super::Server).
#[derive(Debug, clap::Args, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StreamConfig<M: clap::Args> {
    /// gRPC server configuration.
    #[command(flatten)]
    #[serde(default = "default_grpc")]
    pub grpc: GrpcConfig,

    /// Configuration for the underlying Vixen runtime.
    #[command(flatten)]
    #[serde(flatten)]
    pub runtime: VixenConfig<M>,
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
fn default_grpc() -> GrpcConfig {
    GrpcConfig {
        address: default_addr(),
    }
}

#[inline]
fn default_addr() -> SocketAddr { "[::]:3030".parse().unwrap() }
