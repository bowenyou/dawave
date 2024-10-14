// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobRequest {
    /// The hash of the ReducedBatchHeader defined onchain, see:
    /// <https://github.com/Layr-Labs/eigenda/blob/master/contracts/src/interfaces/IEigenDAServiceManager.sol#L43>
    /// This identifies the batch that this blob belongs to.
    #[prost(bytes="vec", tag="1")]
    pub batch_header_hash: ::prost::alloc::vec::Vec<u8>,
    /// Which blob in the batch this is requesting for (note: a batch is logically an
    /// ordered list of blobs).
    #[prost(uint32, tag="2")]
    pub blob_index: u32,
    /// The Ethereum block number at which the batch for this blob was constructed.
    #[prost(uint32, tag="3")]
    pub reference_block_number: u32,
    /// Which quorum of the blob this is requesting for (note a blob can participate in
    /// multiple quorums).
    #[prost(uint32, tag="4")]
    pub quorum_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobReply {
    /// The blob retrieved and reconstructed from the EigenDA Nodes per BlobRequest.
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
include!("retriever.tonic.rs");
// @@protoc_insertion_point(module)