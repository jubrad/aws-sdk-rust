// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`SendCommandInput`](crate::input::SendCommandInput)
pub mod send_command_input {
    /// A builder for [`SendCommandInput`](crate::input::SendCommandInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) session_token: std::option::Option<std::string::String>,
        pub(crate) start_session: std::option::Option<crate::model::StartSessionRequest>,
        pub(crate) start_transaction: std::option::Option<crate::model::StartTransactionRequest>,
        pub(crate) end_session: std::option::Option<crate::model::EndSessionRequest>,
        pub(crate) commit_transaction: std::option::Option<crate::model::CommitTransactionRequest>,
        pub(crate) abort_transaction: std::option::Option<crate::model::AbortTransactionRequest>,
        pub(crate) execute_statement: std::option::Option<crate::model::ExecuteStatementRequest>,
        pub(crate) fetch_page: std::option::Option<crate::model::FetchPageRequest>,
    }
    impl Builder {
        /// <p>Specifies the session token for the current command. A session token is constant
        /// throughout the life of the session.</p>
        /// <p>To obtain a session token, run the <code>StartSession</code> command. This
        /// <code>SessionToken</code> is required for every subsequent command that is issued during
        /// the current session.</p>
        pub fn session_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.session_token = Some(input.into());
            self
        }
        pub fn set_session_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.session_token = input;
            self
        }
        /// <p>Command to start a new session. A session token is obtained as part of the
        /// response.</p>
        pub fn start_session(mut self, input: crate::model::StartSessionRequest) -> Self {
            self.start_session = Some(input);
            self
        }
        pub fn set_start_session(
            mut self,
            input: std::option::Option<crate::model::StartSessionRequest>,
        ) -> Self {
            self.start_session = input;
            self
        }
        /// <p>Command to start a new transaction.</p>
        pub fn start_transaction(mut self, input: crate::model::StartTransactionRequest) -> Self {
            self.start_transaction = Some(input);
            self
        }
        pub fn set_start_transaction(
            mut self,
            input: std::option::Option<crate::model::StartTransactionRequest>,
        ) -> Self {
            self.start_transaction = input;
            self
        }
        /// <p>Command to end the current session.</p>
        pub fn end_session(mut self, input: crate::model::EndSessionRequest) -> Self {
            self.end_session = Some(input);
            self
        }
        pub fn set_end_session(
            mut self,
            input: std::option::Option<crate::model::EndSessionRequest>,
        ) -> Self {
            self.end_session = input;
            self
        }
        /// <p>Command to commit the specified transaction.</p>
        pub fn commit_transaction(mut self, input: crate::model::CommitTransactionRequest) -> Self {
            self.commit_transaction = Some(input);
            self
        }
        pub fn set_commit_transaction(
            mut self,
            input: std::option::Option<crate::model::CommitTransactionRequest>,
        ) -> Self {
            self.commit_transaction = input;
            self
        }
        /// <p>Command to abort the current transaction.</p>
        pub fn abort_transaction(mut self, input: crate::model::AbortTransactionRequest) -> Self {
            self.abort_transaction = Some(input);
            self
        }
        pub fn set_abort_transaction(
            mut self,
            input: std::option::Option<crate::model::AbortTransactionRequest>,
        ) -> Self {
            self.abort_transaction = input;
            self
        }
        /// <p>Command to execute a statement in the specified transaction.</p>
        pub fn execute_statement(mut self, input: crate::model::ExecuteStatementRequest) -> Self {
            self.execute_statement = Some(input);
            self
        }
        pub fn set_execute_statement(
            mut self,
            input: std::option::Option<crate::model::ExecuteStatementRequest>,
        ) -> Self {
            self.execute_statement = input;
            self
        }
        /// <p>Command to fetch a page.</p>
        pub fn fetch_page(mut self, input: crate::model::FetchPageRequest) -> Self {
            self.fetch_page = Some(input);
            self
        }
        pub fn set_fetch_page(
            mut self,
            input: std::option::Option<crate::model::FetchPageRequest>,
        ) -> Self {
            self.fetch_page = input;
            self
        }
        /// Consumes the builder and constructs a [`SendCommandInput`](crate::input::SendCommandInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::SendCommandInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::SendCommandInput {
                session_token: self.session_token,
                start_session: self.start_session,
                start_transaction: self.start_transaction,
                end_session: self.end_session,
                commit_transaction: self.commit_transaction,
                abort_transaction: self.abort_transaction,
                execute_statement: self.execute_statement,
                fetch_page: self.fetch_page,
            })
        }
    }
}
#[doc(hidden)]
pub type SendCommandInputOperationOutputAlias = crate::operation::SendCommand;
#[doc(hidden)]
pub type SendCommandInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl SendCommandInput {
    /// Consumes the builder and constructs an Operation<[`SendCommand`](crate::operation::SendCommand)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::SendCommand,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_send_command(&self).map_err(|err| {
                    smithy_http::operation::BuildError::SerializationError(err.into())
                })?;
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request =
                smithy_http::operation::Request::new(request.map(smithy_http::body::SdkBody::from));
            request
                .config_mut()
                .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ));
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.config_mut().insert(signing_config);
            request
                .config_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.config_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.config_mut().insert(region.clone());
            }
            aws_auth::provider::set_provider(
                &mut request.config_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::SendCommand::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "SendCommand",
                "qldbsession",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/").expect("formatting should succeed");
        Ok(())
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        Ok(builder.method("POST").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let builder = http::request::Builder::new();
        let builder = builder.header("Content-Type", "application/x-amz-json-1.0");
        let builder = builder.header("x-amz-target", "QLDBSession.SendCommand");
        self.update_http_builder(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = builder.header(http::header::CONTENT_LENGTH, content_length)
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`SendCommandInput`](crate::input::SendCommandInput)
    pub fn builder() -> crate::input::send_command_input::Builder {
        crate::input::send_command_input::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SendCommandInput {
    /// <p>Specifies the session token for the current command. A session token is constant
    /// throughout the life of the session.</p>
    /// <p>To obtain a session token, run the <code>StartSession</code> command. This
    /// <code>SessionToken</code> is required for every subsequent command that is issued during
    /// the current session.</p>
    pub session_token: std::option::Option<std::string::String>,
    /// <p>Command to start a new session. A session token is obtained as part of the
    /// response.</p>
    pub start_session: std::option::Option<crate::model::StartSessionRequest>,
    /// <p>Command to start a new transaction.</p>
    pub start_transaction: std::option::Option<crate::model::StartTransactionRequest>,
    /// <p>Command to end the current session.</p>
    pub end_session: std::option::Option<crate::model::EndSessionRequest>,
    /// <p>Command to commit the specified transaction.</p>
    pub commit_transaction: std::option::Option<crate::model::CommitTransactionRequest>,
    /// <p>Command to abort the current transaction.</p>
    pub abort_transaction: std::option::Option<crate::model::AbortTransactionRequest>,
    /// <p>Command to execute a statement in the specified transaction.</p>
    pub execute_statement: std::option::Option<crate::model::ExecuteStatementRequest>,
    /// <p>Command to fetch a page.</p>
    pub fetch_page: std::option::Option<crate::model::FetchPageRequest>,
}
impl std::fmt::Debug for SendCommandInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SendCommandInput");
        formatter.field("session_token", &self.session_token);
        formatter.field("start_session", &self.start_session);
        formatter.field("start_transaction", &self.start_transaction);
        formatter.field("end_session", &self.end_session);
        formatter.field("commit_transaction", &self.commit_transaction);
        formatter.field("abort_transaction", &self.abort_transaction);
        formatter.field("execute_statement", &self.execute_statement);
        formatter.field("fetch_page", &self.fetch_page);
        formatter.finish()
    }
}
