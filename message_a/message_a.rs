#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageA {
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub job: ::core::option::Option<super::base::Job>,
    #[prost(message, optional, tag="3")]
    pub manager: ::core::option::Option<super::base::Identity>,
}
