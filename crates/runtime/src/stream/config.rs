use std::net::SocketAddr;

use crate::config::VixenConfig;

#[derive(Debug, clap::Args, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StreamConfig<M: clap::Args> {
    #[command(flatten)]
    #[serde(default = "default_grpc")]
    pub grpc: GrpcConfig,

    #[command(flatten)]
    #[serde(flatten)]
    pub runtime: VixenConfig<M>,
}

#[derive(Debug, Clone, Copy, clap::Args, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GrpcConfig {
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
