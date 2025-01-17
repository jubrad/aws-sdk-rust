// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CommitTransactionInput {
    /// <p>The transaction to commit.</p>
    #[doc(hidden)]
    pub transaction_id: std::option::Option<std::string::String>,
}
impl CommitTransactionInput {
    /// <p>The transaction to commit.</p>
    pub fn transaction_id(&self) -> std::option::Option<&str> {
        self.transaction_id.as_deref()
    }
}
impl CommitTransactionInput {
    /// Creates a new builder-style object to manufacture [`CommitTransactionInput`](crate::operation::commit_transaction::CommitTransactionInput).
    pub fn builder() -> crate::operation::commit_transaction::builders::CommitTransactionInputBuilder
    {
        crate::operation::commit_transaction::builders::CommitTransactionInputBuilder::default()
    }
}

/// A builder for [`CommitTransactionInput`](crate::operation::commit_transaction::CommitTransactionInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct CommitTransactionInputBuilder {
    pub(crate) transaction_id: std::option::Option<std::string::String>,
}
impl CommitTransactionInputBuilder {
    /// <p>The transaction to commit.</p>
    pub fn transaction_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.transaction_id = Some(input.into());
        self
    }
    /// <p>The transaction to commit.</p>
    pub fn set_transaction_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.transaction_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CommitTransactionInput`](crate::operation::commit_transaction::CommitTransactionInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::commit_transaction::CommitTransactionInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::commit_transaction::CommitTransactionInput {
                transaction_id: self.transaction_id,
            },
        )
    }
}
