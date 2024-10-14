// @generated
// Requests and Replys

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisperseBlobRequest {
    /// The data to be dispersed.
    /// The size of data must be <= 2MiB. Every 32 bytes of data chunk is interpreted as an integer in big endian format
    /// where the lower address has more significant bits. The integer must stay in the valid range to be interpreted
    /// as a field element on the bn254 curve. The valid range is 
    /// 0 <= x < 21888242871839275222246405745257275088548364400416034343698204186575808495617
    /// containing slightly less than 254 bits and more than 253 bits. If any one of the 32 bytes chunk is outside the range, 
    /// the whole request is deemed as invalid, and rejected. 
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub blob_header: ::core::option::Option<BlobHeader>,
    /// signature over keccak hash of the blob_header that can be verified by blob_header.account_id
    #[prost(bytes="vec", tag="3")]
    pub authentication_data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisperseBlobReply {
    /// The status of the blob associated with the blob key.
    #[prost(enumeration="BlobStatus", tag="1")]
    pub result: i32,
    #[prost(bytes="vec", tag="2")]
    pub blob_key: ::prost::alloc::vec::Vec<u8>,
}
/// BlobStatusRequest is used to query the status of a blob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobStatusRequest {
    #[prost(bytes="vec", tag="1")]
    pub blob_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobStatusReply {
    /// The status of the blob.
    #[prost(enumeration="BlobStatus", tag="1")]
    pub status: i32,
    /// The signed blob certificate
    #[prost(message, optional, tag="2")]
    pub signed_certificate: ::core::option::Option<SignedCertificate>,
}
/// Utility method used to generate the commitment of blob given its data.
/// This can be used to construct BlobHeader.commitment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobCommitmentRequest {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobCommitmentReply {
    #[prost(message, optional, tag="1")]
    pub blob_commitment: ::core::option::Option<super::super::common::BlobCommitment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobHeader {
    #[prost(uint32, repeated, tag="1")]
    pub quorum_numbers: ::prost::alloc::vec::Vec<u32>,
    /// API version used to disperse the blob
    #[prost(uint32, tag="2")]
    pub version: u32,
    #[prost(uint32, tag="3")]
    pub bin_index: u32,
    #[prost(uint64, tag="4")]
    pub cumulative_payment: u64,
    /// Ethereum Account Address in Hex string "0x..."
    #[prost(string, tag="5")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub commitment: ::core::option::Option<super::super::common::G1Commitment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedCertificate {
    #[prost(message, optional, tag="1")]
    pub blob_certificate: ::core::option::Option<super::super::common::BlobCertificate>,
    #[prost(message, optional, tag="2")]
    pub non_signer_stakes_and_signature: ::core::option::Option<Attestation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attestation {
    #[prost(uint32, repeated, tag="1")]
    pub non_signer_quorum_bitmap_indices: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag="2")]
    pub non_signer_pubkeys: ::prost::alloc::vec::Vec<super::super::common::G1Commitment>,
    #[prost(message, repeated, tag="3")]
    pub quorum_apks: ::prost::alloc::vec::Vec<super::super::common::G1Commitment>,
    #[prost(message, optional, tag="4")]
    pub apk_g2: ::core::option::Option<super::super::common::G2Commitment>,
    #[prost(message, optional, tag="5")]
    pub sigma: ::core::option::Option<super::super::common::G1Commitment>,
    #[prost(uint32, repeated, tag="6")]
    pub quorum_apk_indices: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag="7")]
    pub total_stake_indices: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag="8")]
    pub non_signer_stake_indices: ::prost::alloc::vec::Vec<NonSignerStakeIndicesForQuorum>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NonSignerStakeIndicesForQuorum {
    #[prost(uint32, repeated, tag="1")]
    pub indices: ::prost::alloc::vec::Vec<u32>,
}
// Data Types

/// BlobStatus represents the status of a blob.
/// The status of a blob is updated as the blob is processed by the disperser.
/// The status of a blob can be queried by the client using the GetBlobStatus API.
/// Intermediate states are states that the blob can be in while being processed, and it can be updated to a differet state:
/// - QUEUED
/// - ENCODED
/// Terminal states are states that will not be updated to a different state:
/// - CERTIFIED
/// - FAILED
/// - INSUFFICIENT_SIGNATURES
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlobStatus {
    Unknown = 0,
    /// QUEUED means that the blob has been queued by the disperser for processing
    Queued = 1,
    /// ENCODED means that the blob has been encoded and is ready to be dispersed to DA Nodes
    Encoded = 2,
    /// CERTIFIED means the blob has been dispersed and attested by the DA nodes
    Certified = 3,
    /// FAILED means that the blob has failed permanently (for reasons other than insufficient
    /// signatures, which is a separate state)
    Failed = 4,
    /// INSUFFICIENT_SIGNATURES means that the confirmation threshold for the blob was not met
    /// for at least one quorum.
    InsufficientSignatures = 5,
}
impl BlobStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlobStatus::Unknown => "UNKNOWN",
            BlobStatus::Queued => "QUEUED",
            BlobStatus::Encoded => "ENCODED",
            BlobStatus::Certified => "CERTIFIED",
            BlobStatus::Failed => "FAILED",
            BlobStatus::InsufficientSignatures => "INSUFFICIENT_SIGNATURES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "QUEUED" => Some(Self::Queued),
            "ENCODED" => Some(Self::Encoded),
            "CERTIFIED" => Some(Self::Certified),
            "FAILED" => Some(Self::Failed),
            "INSUFFICIENT_SIGNATURES" => Some(Self::InsufficientSignatures),
            _ => None,
        }
    }
}
include!("disperser.v2.tonic.rs");
// @@protoc_insertion_point(module)