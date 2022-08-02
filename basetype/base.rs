#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Identity {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub age: i32,
    #[prost(message, optional, tag="4")]
    pub job: ::core::option::Option<Job>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
