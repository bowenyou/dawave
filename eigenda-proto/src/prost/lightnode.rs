// @generated
/// A request from a DA node to an agent light node to stream the availability status of all chunks
/// assigned to the light node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamChunkAvailabilityRequest {
    #[prost(bytes="vec", tag="1")]
    pub authentication_token: ::prost::alloc::vec::Vec<u8>,
}
/// A reply to a StreamAvailabilityStatus request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamChunkAvailabilityReply {
    /// The hash of a blob header corresponding to a chunk the agent received and verified. From the light node's
    /// perspective, the blob is available if all chunks the light node wants to sample are available.
    #[prost(bytes="vec", tag="1")]
    pub header_hash: ::prost::alloc::vec::Vec<u8>,
}
include!("lightnode.tonic.rs");
// @@protoc_insertion_point(module)