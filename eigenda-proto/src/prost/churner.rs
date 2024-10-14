// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChurnRequest {
    /// The Ethereum address (in hex like "0x123abcdef...") of the operator.
    #[prost(string, tag="1")]
    pub operator_address: ::prost::alloc::string::String,
    /// The operator making the churn request.
    #[prost(bytes="vec", tag="2")]
    pub operator_to_register_pubkey_g1: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub operator_to_register_pubkey_g2: ::prost::alloc::vec::Vec<u8>,
    /// The operator's BLS signature signed on the keccak256 hash of
    /// concat("ChurnRequest", operator address, g1, g2, salt).
    #[prost(bytes="vec", tag="4")]
    pub operator_request_signature: ::prost::alloc::vec::Vec<u8>,
    /// The salt used as part of the message to sign on for operator_request_signature.
    #[prost(bytes="vec", tag="5")]
    pub salt: ::prost::alloc::vec::Vec<u8>,
    /// The quorums to register for.
    /// Note:
    ///    - If any of the quorum here has already been registered, this entire request
    ///      will fail to proceed.
    ///    - If any of the quorum fails to register, this entire request will fail.
    ///    - Regardless of whether the specified quorums are full or not, the Churner
    ///      will return parameters for all quorums specified here. The smart contract will
    ///      determine whether it needs to churn out existing operators based on whether
    ///      the quorums have available space.
    /// The IDs must be in range \[0, 254\].
    #[prost(uint32, repeated, tag="6")]
    pub quorum_ids: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChurnReply {
    /// The signature signed by the Churner.
    #[prost(message, optional, tag="1")]
    pub signature_with_salt_and_expiry: ::core::option::Option<SignatureWithSaltAndExpiry>,
    /// A list of existing operators that get churned out.
    /// This list will contain all quorums specified in the ChurnRequest even if some quorums
    /// may not have any churned out operators. If a quorum has available space, OperatorToChurn
    /// object will contain the quorum ID and empty operator and pubkey. The smart contract should
    /// only churn out the operators for quorums that are full.
    ///
    /// For example, if the ChurnRequest specifies quorums 0 and 1 where quorum 0 is full
    /// and quorum 1 has available space, the ChurnReply will contain two OperatorToChurn objects
    /// with the respective quorums. OperatorToChurn for quorum 0 will contain the operator to churn
    /// out and OperatorToChurn for quorum 1 will contain empty operator (zero address) and pubkey.
    /// The smart contract should only churn out the operators for quorum 0 because quorum 1
    /// has available space without having any operators churned.
    /// Note: it's possible an operator gets churned out just for one or more quorums
    /// (rather than entirely churned out for all quorums).
    #[prost(message, repeated, tag="2")]
    pub operators_to_churn: ::prost::alloc::vec::Vec<OperatorToChurn>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureWithSaltAndExpiry {
    /// Churner's signature on the Operator's attributes.
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// Salt is the keccak256 hash of
    /// concat("churn", time.Now(), operatorToChurn's OperatorID, Churner's ECDSA private key)
    #[prost(bytes="vec", tag="2")]
    pub salt: ::prost::alloc::vec::Vec<u8>,
    /// When this churn decision will expire.
    #[prost(int64, tag="3")]
    pub expiry: i64,
}
/// This describes an operator to churn out for a quorum.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatorToChurn {
    /// The ID of the quorum of the operator to churn out.
    #[prost(uint32, tag="1")]
    pub quorum_id: u32,
    /// The address of the operator.
    #[prost(bytes="vec", tag="2")]
    pub operator: ::prost::alloc::vec::Vec<u8>,
    /// BLS pubkey (G1 point) of the operator.
    #[prost(bytes="vec", tag="3")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
}
include!("churner.tonic.rs");
// @@protoc_insertion_point(module)