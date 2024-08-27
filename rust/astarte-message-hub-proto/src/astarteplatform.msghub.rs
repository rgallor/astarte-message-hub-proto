/// An array of doubles for transmission with protobuf.
/// To be used nested inside an `AstarteDataTypeIndividual`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AstarteDoubleArray {
    #[prost(double, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<f64>,
}
/// An array of int32 for transmission with protobuf.
/// To be used nested inside an `AstarteDataTypeIndividual`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AstarteIntegerArray {
    #[prost(int32, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<i32>,
}
/// An array of booleans for transmission with protobuf.
/// To be used nested inside an `AstarteDataTypeIndividual`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AstarteBooleanArray {
    #[prost(bool, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<bool>,
}
/// An array of int64 for transmission with protobuf.
/// To be used nested inside an `AstarteDataTypeIndividual`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AstarteLongIntegerArray {
    #[prost(int64, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<i64>,
}
/// An array of strings for transmission with protobuf.
/// To be used nested inside an `AstarteDataTypeIndividual`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AstarteStringArray {
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// An array of bytes for transmission with protobuf.
/// To be used nested inside an `AstarteDataTypeIndividual`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AstarteBinaryBlobArray {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// An array of timestamps for transmission with protobuf.
/// To be used nested inside an `AstarteDataTypeIndividual`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AstarteDateTimeArray {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::pbjson_types::Timestamp>,
}
/// An aggregated object data type for transmission with protobuf.
/// To be used nested inside an `AstarteDataType`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AstarteDataTypeObject {
    #[prost(map = "string, message", tag = "1")]
    pub object_data: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        AstarteDataTypeIndividual,
    >,
}
/// An individual data type for transmission with protobuf.
/// To be used nested inside an `AstarteDataType`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AstarteDataTypeIndividual {
    #[prost(
        oneof = "astarte_data_type_individual::IndividualData",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14"
    )]
    pub individual_data: ::core::option::Option<
        astarte_data_type_individual::IndividualData,
    >,
}
/// Nested message and enum types in `AstarteDataTypeIndividual`.
pub mod astarte_data_type_individual {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum IndividualData {
        #[prost(double, tag = "1")]
        AstarteDouble(f64),
        #[prost(int32, tag = "2")]
        AstarteInteger(i32),
        #[prost(bool, tag = "3")]
        AstarteBoolean(bool),
        #[prost(int64, tag = "4")]
        AstarteLongInteger(i64),
        #[prost(string, tag = "5")]
        AstarteString(::prost::alloc::string::String),
        #[prost(bytes, tag = "6")]
        AstarteBinaryBlob(::prost::alloc::vec::Vec<u8>),
        #[prost(message, tag = "7")]
        AstarteDateTime(::pbjson_types::Timestamp),
        #[prost(message, tag = "8")]
        AstarteDoubleArray(super::AstarteDoubleArray),
        #[prost(message, tag = "9")]
        AstarteIntegerArray(super::AstarteIntegerArray),
        #[prost(message, tag = "10")]
        AstarteBooleanArray(super::AstarteBooleanArray),
        #[prost(message, tag = "11")]
        AstarteLongIntegerArray(super::AstarteLongIntegerArray),
        #[prost(message, tag = "12")]
        AstarteStringArray(super::AstarteStringArray),
        #[prost(message, tag = "13")]
        AstarteBinaryBlobArray(super::AstarteBinaryBlobArray),
        #[prost(message, tag = "14")]
        AstarteDateTimeArray(super::AstarteDateTimeArray),
    }
}
/// A generic data type to be used nested in an `AstarteMessage`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AstarteDataType {
    /// Content for an `AstarteDataType`.
    #[prost(oneof = "astarte_data_type::Data", tags = "1, 2")]
    pub data: ::core::option::Option<astarte_data_type::Data>,
}
/// Nested message and enum types in `AstarteDataType`.
pub mod astarte_data_type {
    /// Content for an `AstarteDataType`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// An individual data type.
        #[prost(message, tag = "1")]
        AstarteIndividual(super::AstarteDataTypeIndividual),
        /// An aggregated data type.
        #[prost(message, tag = "2")]
        AstarteObject(super::AstarteDataTypeObject),
    }
}
/// MessageHub error type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageHubError {
    /// Human-readable string describing the error.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// List representing a backtrace of an Astarte Message Hub error.
    #[prost(string, repeated, tag = "2")]
    pub source: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MessageHubEvent is a type of message for returning and propagating errors.
/// It is an enum with the variants, AstarteMessage(message), representing success and
/// containing an astarte message value, and MessageHubError(E) representing error and
/// containing an error value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageHubEvent {
    #[prost(oneof = "message_hub_event::Event", tags = "1, 2")]
    pub event: ::core::option::Option<message_hub_event::Event>,
}
/// Nested message and enum types in `MessageHubEvent`.
pub mod message_hub_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// A message that contains data sent from Astarte.
        #[prost(message, tag = "1")]
        Message(super::AstarteMessage),
        /// A message that contains a specific Astarte Message Hub error.
        #[prost(message, tag = "2")]
        Error(super::MessageHubError),
    }
}
/// Astarte message to be used when sending data to Astarte.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AstarteMessage {
    /// Name of the interface to send data on.
    #[prost(string, tag = "1")]
    pub interface_name: ::prost::alloc::string::String,
    /// Endpoint to send the data on.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// Explicit timestamp for the message transmission.
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Content of the message.
    #[prost(oneof = "astarte_message::Payload", tags = "3, 4")]
    pub payload: ::core::option::Option<astarte_message::Payload>,
}
/// Nested message and enum types in `AstarteMessage`.
pub mod astarte_message {
    /// Content of the message.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Effective payload.
        #[prost(message, tag = "3")]
        AstarteData(super::AstarteDataType),
        /// Null payload.
        #[prost(message, tag = "4")]
        AstarteUnset(super::AstarteUnset),
    }
}
/// Null payload for an `AstarteMessage`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AstarteUnset {}
/// This message defines a node to be attached to the Astarte message hub.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    /// Array of string representing all .json interface files of the node.
    #[prost(string, repeated, tag = "2")]
    pub interfaces_json: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// This message defines a list of json interfaces to be added/removed to the Astarte message hub.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfacesJson {
    /// An array of json interfaces.
    #[prost(string, repeated, tag = "1")]
    pub interfaces_json: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// This message defines a list of interfaces' names to be removed from the Astarte message hub.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfacesName {
    /// An array of interfaces' names
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod message_hub_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MessageHubClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MessageHubClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MessageHubClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MessageHubClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MessageHubClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// This function should be used to attach a node to an instance of the Astarte message hub.
        /// Returns a data stream from the Astarte message hub.
        pub async fn attach(
            &mut self,
            request: impl tonic::IntoRequest<super::Node>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MessageHubEvent>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/astarteplatform.msghub.MessageHub/Attach",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("astarteplatform.msghub.MessageHub", "Attach"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// This function should be used to send an `AstarteMessage` to Astarte.
        pub async fn send(
            &mut self,
            request: impl tonic::IntoRequest<super::AstarteMessage>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/astarteplatform.msghub.MessageHub/Send",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("astarteplatform.msghub.MessageHub", "Send"));
            self.inner.unary(req, path, codec).await
        }
        /// This function should be used to detach a node from an instance of the Astarte message hub.
        pub async fn detach(
            &mut self,
            request: impl tonic::IntoRequest<::pbjson_types::Empty>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/astarteplatform.msghub.MessageHub/Detach",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("astarteplatform.msghub.MessageHub", "Detach"));
            self.inner.unary(req, path, codec).await
        }
        /// This function should be used to add one or more interfaces to an instance of the Astarte message hub.
        pub async fn add_interfaces(
            &mut self,
            request: impl tonic::IntoRequest<super::InterfacesJson>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/astarteplatform.msghub.MessageHub/AddInterfaces",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("astarteplatform.msghub.MessageHub", "AddInterfaces"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// This function should be used to remove one or more interfaces from an instance of the Astarte message hub.
        pub async fn remove_interfaces(
            &mut self,
            request: impl tonic::IntoRequest<super::InterfacesName>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/astarteplatform.msghub.MessageHub/RemoveInterfaces",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "astarteplatform.msghub.MessageHub",
                        "RemoveInterfaces",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod message_hub_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MessageHubServer.
    #[async_trait]
    pub trait MessageHub: Send + Sync + 'static {
        /// Server streaming response type for the Attach method.
        type AttachStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::MessageHubEvent, tonic::Status>,
            >
            + Send
            + 'static;
        /// This function should be used to attach a node to an instance of the Astarte message hub.
        /// Returns a data stream from the Astarte message hub.
        async fn attach(
            &self,
            request: tonic::Request<super::Node>,
        ) -> std::result::Result<tonic::Response<Self::AttachStream>, tonic::Status>;
        /// This function should be used to send an `AstarteMessage` to Astarte.
        async fn send(
            &self,
            request: tonic::Request<super::AstarteMessage>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>;
        /// This function should be used to detach a node from an instance of the Astarte message hub.
        async fn detach(
            &self,
            request: tonic::Request<::pbjson_types::Empty>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>;
        /// This function should be used to add one or more interfaces to an instance of the Astarte message hub.
        async fn add_interfaces(
            &self,
            request: tonic::Request<super::InterfacesJson>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>;
        /// This function should be used to remove one or more interfaces from an instance of the Astarte message hub.
        async fn remove_interfaces(
            &self,
            request: tonic::Request<super::InterfacesName>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MessageHubServer<T: MessageHub> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MessageHub> MessageHubServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MessageHubServer<T>
    where
        T: MessageHub,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/astarteplatform.msghub.MessageHub/Attach" => {
                    #[allow(non_camel_case_types)]
                    struct AttachSvc<T: MessageHub>(pub Arc<T>);
                    impl<
                        T: MessageHub,
                    > tonic::server::ServerStreamingService<super::Node>
                    for AttachSvc<T> {
                        type Response = super::MessageHubEvent;
                        type ResponseStream = T::AttachStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Node>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageHub>::attach(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AttachSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/astarteplatform.msghub.MessageHub/Send" => {
                    #[allow(non_camel_case_types)]
                    struct SendSvc<T: MessageHub>(pub Arc<T>);
                    impl<
                        T: MessageHub,
                    > tonic::server::UnaryService<super::AstarteMessage> for SendSvc<T> {
                        type Response = ::pbjson_types::Empty;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AstarteMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageHub>::send(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/astarteplatform.msghub.MessageHub/Detach" => {
                    #[allow(non_camel_case_types)]
                    struct DetachSvc<T: MessageHub>(pub Arc<T>);
                    impl<
                        T: MessageHub,
                    > tonic::server::UnaryService<::pbjson_types::Empty>
                    for DetachSvc<T> {
                        type Response = ::pbjson_types::Empty;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<::pbjson_types::Empty>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageHub>::detach(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DetachSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/astarteplatform.msghub.MessageHub/AddInterfaces" => {
                    #[allow(non_camel_case_types)]
                    struct AddInterfacesSvc<T: MessageHub>(pub Arc<T>);
                    impl<
                        T: MessageHub,
                    > tonic::server::UnaryService<super::InterfacesJson>
                    for AddInterfacesSvc<T> {
                        type Response = ::pbjson_types::Empty;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InterfacesJson>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageHub>::add_interfaces(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddInterfacesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/astarteplatform.msghub.MessageHub/RemoveInterfaces" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveInterfacesSvc<T: MessageHub>(pub Arc<T>);
                    impl<
                        T: MessageHub,
                    > tonic::server::UnaryService<super::InterfacesName>
                    for RemoveInterfacesSvc<T> {
                        type Response = ::pbjson_types::Empty;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InterfacesName>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageHub>::remove_interfaces(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveInterfacesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: MessageHub> Clone for MessageHubServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: MessageHub> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MessageHub> tonic::server::NamedService for MessageHubServer<T> {
        const NAME: &'static str = "astarteplatform.msghub.MessageHub";
    }
}
/// Configuration message to be used to send configuration to the Astarte message hub.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigMessage {
    #[prost(string, tag = "1")]
    pub realm: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub device_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub credentials_secret: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub pairing_url: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub pairing_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "6")]
    pub grpc_socket_port: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "7")]
    pub grpc_socket_host: ::core::option::Option<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod message_hub_config_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MessageHubConfigClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MessageHubConfigClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MessageHubConfigClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MessageHubConfigClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MessageHubConfigClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Set the configuration for the Astarte message hub.
        pub async fn set_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigMessage>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/astarteplatform.msghub.MessageHubConfig/SetConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "astarteplatform.msghub.MessageHubConfig",
                        "SetConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod message_hub_config_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MessageHubConfigServer.
    #[async_trait]
    pub trait MessageHubConfig: Send + Sync + 'static {
        /// Set the configuration for the Astarte message hub.
        async fn set_config(
            &self,
            request: tonic::Request<super::ConfigMessage>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MessageHubConfigServer<T: MessageHubConfig> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MessageHubConfig> MessageHubConfigServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MessageHubConfigServer<T>
    where
        T: MessageHubConfig,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/astarteplatform.msghub.MessageHubConfig/SetConfig" => {
                    #[allow(non_camel_case_types)]
                    struct SetConfigSvc<T: MessageHubConfig>(pub Arc<T>);
                    impl<
                        T: MessageHubConfig,
                    > tonic::server::UnaryService<super::ConfigMessage>
                    for SetConfigSvc<T> {
                        type Response = ::pbjson_types::Empty;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfigMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageHubConfig>::set_config(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: MessageHubConfig> Clone for MessageHubConfigServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: MessageHubConfig> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MessageHubConfig> tonic::server::NamedService for MessageHubConfigServer<T> {
        const NAME: &'static str = "astarteplatform.msghub.MessageHubConfig";
    }
}
