// @generated
/// Module is the config object of the benchmark module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    #[prost(message, optional, tag = "1")]
    pub genesis_params: ::core::option::Option<GeneratorParams>,
}
impl ::prost::Name for Module {
    const NAME: &'static str = "Module";
    const PACKAGE: &'static str = "cosmos.benchmark.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.benchmark.module.v1.{}", Self::NAME)
    }
}
/// GenesisParams defines the genesis parameters for the benchmark module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneratorParams {
    /// seed is the seed for the random number generator.
    #[prost(uint64, tag = "1")]
    pub seed: u64,
    /// bucket_count is the number of store keys to uniformly distribute genesis_count keys across.
    #[prost(uint64, tag = "2")]
    pub bucket_count: u64,
    /// key_mean is the mean size (in normal distribution) of keys in each bucket.
    #[prost(uint64, tag = "3")]
    pub key_mean: u64,
    /// key_std_dev is the standard deviation of key sizes in each bucket.
    #[prost(uint64, tag = "4")]
    pub key_std_dev: u64,
    /// value_mean is the mean size (in normal distribution) of values in each bucket.
    #[prost(uint64, tag = "6")]
    pub value_mean: u64,
    /// value_std_dev is the standard deviation of value sizes in each bucket.
    #[prost(uint64, tag = "7")]
    pub value_std_dev: u64,
    /// genesis_count is the number of keys to insert in the store, distributed across all buckets.
    #[prost(uint64, tag = "8")]
    pub genesis_count: u64,
    /// insert_weight is the weight of insert operations.
    #[prost(float, tag = "9")]
    pub insert_weight: f32,
    /// update_weight is the weight of update operations.
    #[prost(float, tag = "10")]
    pub update_weight: f32,
    /// get_weight is the weight of get operations.
    #[prost(float, tag = "12")]
    pub get_weight: f32,
    /// delete_weight is the weight of delete operations.
    #[prost(float, tag = "11")]
    pub delete_weight: f32,
}
impl ::prost::Name for GeneratorParams {
    const NAME: &'static str = "GeneratorParams";
    const PACKAGE: &'static str = "cosmos.benchmark.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.benchmark.module.v1.{}", Self::NAME)
    }
}
include!("cosmos.benchmark.module.v1.serde.rs");
// @@protoc_insertion_point(module)
