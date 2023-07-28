// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountMissingBlocksRequest {
    #[prost(string, tag="1")]
    pub start_date: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub end_date: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissingBlocks {
    #[prost(message, repeated, tag="1")]
    pub missing_blocks: ::prost::alloc::vec::Vec<MissingBlockTime>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissingBlockTime {
    #[prost(string, tag="1")]
    pub number: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub date_time: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
