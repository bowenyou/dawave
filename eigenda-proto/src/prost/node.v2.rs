// @generated
// Requests and replies

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreChunksRequest {
    /// list of blob certificates to process
    #[prost(message, repeated, tag="1")]
    pub blob_certificates: ::prost::alloc::vec::Vec<super::super::common::BlobCertificate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreChunksReply {
    #[prost(message, repeated, tag="1")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChunksRequest {
    #[prost(bytes="vec", tag="1")]
    pub blob_key: ::prost::alloc::vec::Vec<u8>,
    /// Which quorum of the blob to retrieve for (note: a blob can have multiple
    /// quorums and the chunks for different quorums at a Node can be different).
    /// The ID must be in range \[0, 254\].
    #[prost(uint32, tag="2")]
    pub quorum_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChunksReply {
    /// All chunks the Node is storing for the requested blob per RetrieveChunksRequest.
    #[prost(bytes="vec", repeated, tag="1")]
    pub chunks: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlobCertificateRequest {
    #[prost(bytes="vec", tag="1")]
    pub blob_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlobCertificateReply {
    #[prost(message, optional, tag="1")]
    pub blob_certificate: ::core::option::Option<super::super::common::BlobCertificate>,
}
/// Node info request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfoRequest {
}
/// Node info reply
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfoReply {
    #[prost(string, tag="1")]
    pub semver: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub arch: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub os: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub num_cpu: u32,
    #[prost(uint64, tag="5")]
    pub mem_bytes: u64,
}
include!("node.v2.tonic.rs");
// @@protoc_insertion_point(module)