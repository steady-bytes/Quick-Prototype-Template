#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessControlPolicy {
    /// object key attribute to lookup in the object.
    /// Thinking something like this {"uuid": "06f822e7-3265-4722-beec-d50d8e9140e0"}
    /// I wonder if you can put a unique constrain on a column like that
    #[prost(message, repeated, tag="1")]
    pub lookup_object_key: ::prost::alloc::vec::Vec<LookupObjectKey>,
    /// data_type name. This normally correlates with table name, and or model type name.
    /// I like to think of it as either an aggregate, or an entity. It's worth noteing
    /// if access is restricted to an aggregate, all of it's encompasing entities will
    /// inhert the same restriction restriction
    #[prost(string, tag="2")]
    pub data_type: ::prost::alloc::string::String,
    /// string mapping of the different operations that can be performed on a resource
    #[prost(enumeration="Operation", tag="3")]
    pub operation: i32,
    /// subject of the action being evaluated
    #[prost(message, optional, tag="4")]
    pub subject: ::core::option::Option<Subject>,
    /// outcom is set so a policy can be either a deny filter, or allowed filter.
    /// A example policy might be allow all users with driver role to read this specific
    /// data.
    #[prost(enumeration="Outcome", tag="5")]
    pub outcome: i32,
}
/// ObjectKey is a key value mapping that can be used to lookup an aggregate/entity 
/// example {"uuid": "b7e3597a-88af-4f20-a9b5-0d49f2c8376e"} or (where ?KEY = ?VALUE )
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupObjectKey {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
/// Details of the client/user making the request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subject {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Operations are actions that can be taken on a resource
/// they are currently data specific b/c this system will be used mainly for
/// data related access controls. In the future this could be expanded
/// to contain other resources
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Operation {
    DataOperationUnspecified = 0,
    /// data specific operations 
    Insert = 1,
    Update = 2,
    Read = 3,
    Delete = 4,
    /// experimental field, writing 
    ChangePermission = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Outcome {
    Unspecifield = 0,
    Allowed = 1,
    Denied = 2,
}
/// Is the client allowed to perform the specific operation on the 
/// aggregate/entity that was found with the LookupObjectKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluatePolicyRequest {
    #[prost(message, repeated, tag="1")]
    pub lookup_object_key: ::prost::alloc::vec::Vec<LookupObjectKey>,
    #[prost(string, tag="2")]
    pub data_type: ::prost::alloc::string::String,
    #[prost(enumeration="Operation", tag="3")]
    pub operation: i32,
    #[prost(string, tag="4")]
    pub role: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluatePolicyResponse {
    #[prost(enumeration="Outcome", tag="1")]
    pub outcome: i32,
}
/// Generated client implementations.
pub mod policy_evaluator_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Client integration point
    #[derive(Debug, Clone)]
    pub struct PolicyEvaluatorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PolicyEvaluatorClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PolicyEvaluatorClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PolicyEvaluatorClient<InterceptedService<T, F>>
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
            PolicyEvaluatorClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// integration point for the service clients to call.
        /// policy decition point
        pub async fn evaluate_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::EvaluatePolicyRequest>,
        ) -> Result<tonic::Response<super::EvaluatePolicyResponse>, tonic::Status> {
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
                "/draft.access_controls.v1.PolicyEvaluator/EvaluatePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod policy_evaluator_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with PolicyEvaluatorServer.
    #[async_trait]
    pub trait PolicyEvaluator: Send + Sync + 'static {
        /// integration point for the service clients to call.
        /// policy decition point
        async fn evaluate_policy(
            &self,
            request: tonic::Request<super::EvaluatePolicyRequest>,
        ) -> Result<tonic::Response<super::EvaluatePolicyResponse>, tonic::Status>;
    }
    /// Client integration point
    #[derive(Debug)]
    pub struct PolicyEvaluatorServer<T: PolicyEvaluator> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PolicyEvaluator> PolicyEvaluatorServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PolicyEvaluatorServer<T>
    where
        T: PolicyEvaluator,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/draft.access_controls.v1.PolicyEvaluator/EvaluatePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct EvaluatePolicySvc<T: PolicyEvaluator>(pub Arc<T>);
                    impl<
                        T: PolicyEvaluator,
                    > tonic::server::UnaryService<super::EvaluatePolicyRequest>
                    for EvaluatePolicySvc<T> {
                        type Response = super::EvaluatePolicyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EvaluatePolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).evaluate_policy(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EvaluatePolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
    impl<T: PolicyEvaluator> Clone for PolicyEvaluatorServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PolicyEvaluator> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PolicyEvaluator> tonic::transport::NamedService
    for PolicyEvaluatorServer<T> {
        const NAME: &'static str = "draft.access_controls.v1.PolicyEvaluator";
    }
}
