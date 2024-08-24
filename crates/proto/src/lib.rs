pub extern crate prost;
#[cfg(feature = "stream")]
pub extern crate prost_types;
#[cfg(feature = "stream")]
pub extern crate tonic;
#[cfg(feature = "stream")]
pub extern crate tonic_reflection;

#[cfg(feature = "stream")]
pub mod stream {
    tonic::include_proto!("stream");

    pub const DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("stream_descriptor");
}
