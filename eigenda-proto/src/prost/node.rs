// @generated
// Requests and replies

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreChunksRequest {
    /// Which batch this request is for.
    #[prost(message, optional, tag="1")]
    pub batch_header: ::core::option::Option<BatchHeader>,
    /// The chunks for each blob in the batch to be stored in an EigenDA Node.
    #[prost(message, repeated, tag="2")]
    pub blobs: ::prost::alloc::vec::Vec<Blob>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreChunksReply {
    /// The operator's BLS signature signed on the batch header hash.
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreBlobsRequest {
    /// Blobs to store
    #[prost(message, repeated, tag="1")]
    pub blobs: ::prost::alloc::vec::Vec<Blob>,
    /// The reference block number whose state is used to encode the blobs
    #[prost(uint32, tag="2")]
    pub reference_block_number: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreBlobsReply {
    /// The operator's BLS sgnature signed on the blob header hashes.
    /// The ordering of the signatures must match the ordering of the blobs sent
    /// in the request, with empty signatures in the places for discarded blobs.
    #[prost(message, repeated, tag="1")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestBatchRequest {
    /// header of the batch
    #[prost(message, optional, tag="1")]
    pub batch_header: ::core::option::Option<BatchHeader>,
    /// the header hashes of all blobs in the batch
    #[prost(bytes="vec", repeated, tag="2")]
    pub blob_header_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestBatchReply {
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveChunksRequest {
    /// The hash of the ReducedBatchHeader defined onchain, see:
    /// <https://github.com/Layr-Labs/eigenda/blob/master/contracts/src/interfaces/IEigenDAServiceManager.sol#L43>
    /// This identifies which batch to retrieve for.
    #[prost(bytes="vec", tag="1")]
    pub batch_header_hash: ::prost::alloc::vec::Vec<u8>,
    /// Which blob in the batch to retrieve for (note: a batch is logically an ordered
    /// list of blobs).
    #[prost(uint32, tag="2")]
    pub blob_index: u32,
    /// Which quorum of the blob to retrieve for (note: a blob can have multiple
    /// quorums and the chunks for different quorums at a Node can be different).
    /// The ID must be in range \[0, 254\].
    #[prost(uint32, tag="3")]
    pub quorum_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveChunksReply {
    /// All chunks the Node is storing for the requested blob per RetrieveChunksRequest.
    #[prost(bytes="vec", repeated, tag="1")]
    pub chunks: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// How the above chunks are encoded.
    #[prost(enumeration="ChunkEncodingFormat", tag="2")]
    pub chunk_encoding_format: i32,
}
/// See RetrieveChunksRequest for documentation of each parameter of GetBlobHeaderRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlobHeaderRequest {
    #[prost(bytes="vec", tag="1")]
    pub batch_header_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub blob_index: u32,
    #[prost(uint32, tag="3")]
    pub quorum_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlobHeaderReply {
    /// The header of the blob requested per GetBlobHeaderRequest.
    #[prost(message, optional, tag="1")]
    pub blob_header: ::core::option::Option<BlobHeader>,
    /// Merkle proof that returned blob header belongs to the batch and is
    /// the batch's MerkleProof.index-th blob.
    /// This can be checked against the batch root on chain.
    #[prost(message, optional, tag="2")]
    pub proof: ::core::option::Option<MerkleProof>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerkleProof {
    /// The proof itself.
    #[prost(bytes="vec", repeated, tag="1")]
    pub hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Which index (the leaf of the Merkle tree) this proof is for.
    #[prost(uint32, tag="2")]
    pub index: u32,
}
// Types

/// In EigenDA, the original blob to disperse is encoded as a polynomial via taking
/// taking different point evaluations (i.e. erasure coding). These points are split
/// into disjoint subsets which are assigned to different operator nodes in the EigenDA
/// network.
/// The data in this message is a subset of these points that are assigned to a
/// single operator node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blob {
    /// Which (original) blob this is for.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<BlobHeader>,
    /// Each bundle contains all chunks for a single quorum of the blob.
    /// The number of bundles must be equal to the total number of quorums associated
    /// with the blob, and the ordering must be the same as BlobHeader.quorum_headers.
    /// Note: an operator may be in some but not all of the quorums; in that case the
    /// bundle corresponding to that quorum will be empty.
    #[prost(message, repeated, tag="2")]
    pub bundles: ::prost::alloc::vec::Vec<Bundle>,
}
/// A Bundle is the collection of chunks associated with a single blob, for a single
/// operator and a single quorum.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bundle {
    /// Each chunk corresponds to a collection of points on the polynomial.
    /// Each chunk has same number of points.
    #[prost(bytes="vec", repeated, tag="1")]
    pub chunks: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// All chunks of the bundle encoded in a byte array.
    #[prost(bytes="vec", tag="2")]
    pub bundle: ::prost::alloc::vec::Vec<u8>,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobHeader {
    /// The KZG commitment to the polynomial representing the blob.
    #[prost(message, optional, tag="1")]
    pub commitment: ::core::option::Option<super::common::G1Commitment>,
    /// The KZG commitment to the polynomial representing the blob on G2, it is used
    /// for proving the degree of the polynomial
    #[prost(message, optional, tag="2")]
    pub length_commitment: ::core::option::Option<G2Commitment>,
    /// The low degree proof. It's the KZG commitment to the polynomial shifted to
    /// the largest SRS degree.
    #[prost(message, optional, tag="3")]
    pub length_proof: ::core::option::Option<G2Commitment>,
    /// The length of the original blob in number of symbols (in the field where
    /// the polynomial is defined).
    #[prost(uint32, tag="4")]
    pub length: u32,
    /// The params of the quorums that this blob participates in.
    #[prost(message, repeated, tag="5")]
    pub quorum_headers: ::prost::alloc::vec::Vec<BlobQuorumInfo>,
    /// The ID of the user who is dispersing this blob to EigenDA.
    #[prost(string, tag="6")]
    pub account_id: ::prost::alloc::string::String,
    /// The reference block number whose state is used to encode the blob
    #[prost(uint32, tag="7")]
    pub reference_block_number: u32,
}
/// See BlobQuorumParam as defined in
/// api/proto/disperser/disperser.proto
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobQuorumInfo {
    #[prost(uint32, tag="1")]
    pub quorum_id: u32,
    #[prost(uint32, tag="2")]
    pub adversary_threshold: u32,
    #[prost(uint32, tag="3")]
    pub confirmation_threshold: u32,
    #[prost(uint32, tag="4")]
    pub chunk_length: u32,
    #[prost(uint32, tag="5")]
    pub ratelimit: u32,
}
/// BatchHeader (see core/data.go#BatchHeader)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchHeader {
    /// The root of the merkle tree with hashes of blob headers as leaves.
    #[prost(bytes="vec", tag="1")]
    pub batch_root: ::prost::alloc::vec::Vec<u8>,
    /// The Ethereum block number at which the batch is dispersed.
    #[prost(uint32, tag="3")]
    pub reference_block_number: u32,
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
/// This describes how the chunks returned in RetrieveChunksReply are encoded.
/// Used to facilitate the decoding of chunks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChunkEncodingFormat {
    Unknown = 0,
    Gnark = 1,
    Gob = 2,
}
impl ChunkEncodingFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChunkEncodingFormat::Unknown => "UNKNOWN",
            ChunkEncodingFormat::Gnark => "GNARK",
            ChunkEncodingFormat::Gob => "GOB",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "GNARK" => Some(Self::Gnark),
            "GOB" => Some(Self::Gob),
            _ => None,
        }
    }
}
/// Request a specific chunk
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChunkRequest {
    /// The hash of the blob's header.
    #[prost(bytes="vec", tag="1")]
    pub header_hash: ::prost::alloc::vec::Vec<u8>,
    /// The index of the chunk within the blob.
    #[prost(uint32, tag="2")]
    pub chunk_index: u32,
}
/// Reply to GetChunkRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChunkReply {
    /// The chunk requested.
    #[prost(message, optional, tag="1")]
    pub chunk: ::core::option::Option<super::common::ChunkData>,
}
include!("node.tonic.rs");
// @@protoc_insertion_point(module)