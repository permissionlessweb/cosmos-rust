// @generated
/// ContinuousFund defines the fields of continuous fund proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContinuousFund {
    /// Recipient is the address string of the account receiving funds.
    #[prost(string, tag = "1")]
    pub recipient: ::prost::alloc::string::String,
    /// Percentage is the percentage of funds to be allocated from Community pool.
    #[prost(string, tag = "2")]
    pub percentage: ::prost::alloc::string::String,
    /// Optional, if expiry is set, removes the state object when expired.
    #[prost(message, optional, tag = "3")]
    pub expiry: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for ContinuousFund {
    const NAME: &'static str = "ContinuousFund";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// Params defines the parameters for the protocolpool module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// EnabledDistributionDenoms lists the denoms that are allowed to be distributed.
    /// This is to avoid spending time distributing undesired tokens to continuous funds and budgets.
    #[prost(string, repeated, tag = "1")]
    pub enabled_distribution_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// DistributionFrequency is the frequency (in terms of blocks) that funds are distributed out from the
    /// x/protocolpool module.
    #[prost(uint64, tag = "2")]
    pub distribution_frequency: u64,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// GenesisState defines the protocolpool module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// ContinuousFunds defines the continuous funds at genesis.
    #[prost(message, repeated, tag = "1")]
    pub continuous_funds: ::prost::alloc::vec::Vec<ContinuousFund>,
    /// Params defines the parameters of this module, currently only contains the
    /// denoms that will be used for continuous fund distributions.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// QueryCommunityPoolRequest is the request type for the Query/CommunityPool RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCommunityPoolRequest {}
impl ::prost::Name for QueryCommunityPoolRequest {
    const NAME: &'static str = "QueryCommunityPoolRequest";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// QueryCommunityPoolResponse is the response type for the Query/CommunityPool
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCommunityPoolResponse {
    /// pool defines community pool's coins.
    #[prost(message, repeated, tag = "1")]
    pub pool: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
impl ::prost::Name for QueryCommunityPoolResponse {
    const NAME: &'static str = "QueryCommunityPoolResponse";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// QueryContinuousFundRequest is the request type for the Query/ContinuousFund
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContinuousFundRequest {
    /// recipient is the recipient address to query unclaimed budget amount for.
    #[prost(string, tag = "1")]
    pub recipient: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryContinuousFundRequest {
    const NAME: &'static str = "QueryContinuousFundRequest";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// QueryUnclaimedBudgetResponse is the response type for the Query/ContinuousFund
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContinuousFundResponse {
    /// ContinuousFunds is the given continuous fund returned in the query.
    #[prost(message, optional, tag = "1")]
    pub continuous_fund: ::core::option::Option<ContinuousFund>,
}
impl ::prost::Name for QueryContinuousFundResponse {
    const NAME: &'static str = "QueryContinuousFundResponse";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// QueryContinuousFundRequest is the request type for the Query/ContinuousFunds
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContinuousFundsRequest {}
impl ::prost::Name for QueryContinuousFundsRequest {
    const NAME: &'static str = "QueryContinuousFundsRequest";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// QueryUnclaimedBudgetResponse is the response type for the Query/ContinuousFunds
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContinuousFundsResponse {
    /// ContinuousFunds defines all continuous funds in state.
    #[prost(message, repeated, tag = "1")]
    pub continuous_funds: ::prost::alloc::vec::Vec<ContinuousFund>,
}
impl ::prost::Name for QueryContinuousFundsResponse {
    const NAME: &'static str = "QueryContinuousFundsResponse";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// QueryParamsRequest is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryParamsResponse {
    const NAME: &'static str = "QueryParamsResponse";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// MsgFundCommunityPool allows an account to directly
/// fund the community pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundCommunityPool {
    #[prost(string, tag = "1")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgFundCommunityPool {
    const NAME: &'static str = "MsgFundCommunityPool";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// MsgFundCommunityPoolResponse defines the Msg/FundCommunityPool response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundCommunityPoolResponse {}
impl ::prost::Name for MsgFundCommunityPoolResponse {
    const NAME: &'static str = "MsgFundCommunityPoolResponse";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// MsgCommunityPoolSpend defines a message for sending tokens from the community
/// pool to another account. This message is typically executed via a governance
/// proposal with the governance module being the executing authority.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCommunityPoolSpend {
    /// Authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgCommunityPoolSpend {
    const NAME: &'static str = "MsgCommunityPoolSpend";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// MsgCommunityPoolSpendResponse defines the response to executing a
/// MsgCommunityPoolSpend message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCommunityPoolSpendResponse {}
impl ::prost::Name for MsgCommunityPoolSpendResponse {
    const NAME: &'static str = "MsgCommunityPoolSpendResponse";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// MsgCreateContinuousFund defines a message for adding continuous funds.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateContinuousFund {
    /// Authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Recipient address of the account receiving funds.
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    /// Percentage is the percentage of funds to be allocated from Community pool.
    #[prost(string, tag = "3")]
    pub percentage: ::prost::alloc::string::String,
    /// Optional, if expiry is set, removes the state object when expired.
    #[prost(message, optional, tag = "4")]
    pub expiry: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for MsgCreateContinuousFund {
    const NAME: &'static str = "MsgCreateContinuousFund";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// MsgCreateContinuousFundResponse defines the response to executing a
/// MsgCreateContinuousFund message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateContinuousFundResponse {}
impl ::prost::Name for MsgCreateContinuousFundResponse {
    const NAME: &'static str = "MsgCreateContinuousFundResponse";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// MsgCancelContinuousFund defines a message to cancel continuous funds for a specific recipient.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelContinuousFund {
    /// Authority is the account address of authority.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Recipient is the account address string of the recipient whose funds are to be cancelled.
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCancelContinuousFund {
    const NAME: &'static str = "MsgCancelContinuousFund";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// MsgCancelContinuousFundResponse defines the response to executing a
/// MsgCancelContinuousFund message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelContinuousFundResponse {
    /// CanceledTime is the canceled time.
    #[prost(message, optional, tag = "1")]
    pub canceled_time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// CanceledHeight defines the canceled block height.
    #[prost(uint64, tag = "2")]
    pub canceled_height: u64,
    /// Recipient is the account address string of the recipient whose funds are cancelled.
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCancelContinuousFundResponse {
    const NAME: &'static str = "MsgCancelContinuousFundResponse";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/protocolpool parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "cosmos.protocolpool.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.protocolpool.v1.{}", Self::NAME)
    }
}
include!("cosmos.protocolpool.v1.serde.rs");
include!("cosmos.protocolpool.v1.tonic.rs");
// @@protoc_insertion_point(module)
