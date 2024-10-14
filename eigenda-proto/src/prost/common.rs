// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct G1Commitment {
    /// The X coordinate of the KZG commitment. This is the raw byte representation of the field element.
    #[prost(bytes="vec", tag="1")]
    pub x: ::prost::alloc::vec::Vec<u8>,
    /// The Y coordinate of the KZG commitment. This is the raw byte representation of the field element.
    #[prost(bytes="vec", tag="2")]
    pub y: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct G2Commitment {
    /// The A0 element of the X coordinate of G2 point.
    #[prost(bytes="vec", tag="1")]
    pub x_a0: ::prost::alloc::vec::Vec<u8>,
    /// The A1 element of the X coordinate of G2 point.
    #[prost(bytes="vec", tag="2")]
    pub x_a1: ::prost::alloc::vec::Vec<u8>,
    /// The A0 element of the Y coordinate of G2 point.
    #[prost(bytes="vec", tag="3")]
    pub y_a0: ::prost::alloc::vec::Vec<u8>,
    /// The A1 element of the Y coordinate of G2 point.
    #[prost(bytes="vec", tag="4")]
    pub y_a1: ::prost::alloc::vec::Vec<u8>,
}
/// BlobCommitment represents commitment of a specific blob, containing its
/// KZG commitment, degree proof, the actual degree, and data length in number of symbols.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobCommitment {
    #[prost(message, optional, tag="1")]
    pub commitment: ::core::option::Option<G1Commitment>,
    #[prost(message, optional, tag="2")]
    pub length_commitment: ::core::option::Option<G2Commitment>,
    #[prost(message, optional, tag="3")]
    pub length_proof: ::core::option::Option<G2Commitment>,
    #[prost(uint32, tag="4")]
    pub data_length: u32,
}
/// BlobCertificate is what gets attested by the network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobCertificate {
    #[prost(uint32, tag="1")]
    pub version: u32,
    #[prost(bytes="vec", tag="2")]
    pub blob_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub blob_commitment: ::core::option::Option<BlobCommitment>,
    #[prost(uint32, repeated, tag="4")]
    pub quorum_numbers: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, tag="5")]
    pub reference_block_number: u32,
}
// ///////////////////////////////////////////////////////////////////////////////////
// Experimental: the following definitions are experimental and subject to change. //
// ///////////////////////////////////////////////////////////////////////////////////

/// A chunk of a blob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkData {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)
