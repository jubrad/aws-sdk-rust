// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AccessDeniedException(crate::error::AccessDeniedException),
    ConcurrentLimitExceededException(crate::error::ConcurrentLimitExceededException),
    ConflictException(crate::error::ConflictException),
    InternalServerException(crate::error::InternalServerException),
    RequestThrottledException(crate::error::RequestThrottledException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    ValidationException(crate::error::ValidationException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConcurrentLimitExceededException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::RequestThrottledException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CompleteSnapshotError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CompleteSnapshotError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CompleteSnapshotErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CompleteSnapshotErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CompleteSnapshotErrorKind::RequestThrottledException(inner) => {
                    Error::RequestThrottledException(inner)
                }
                crate::error::CompleteSnapshotErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CompleteSnapshotErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::CompleteSnapshotErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CompleteSnapshotErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetSnapshotBlockError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetSnapshotBlockError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetSnapshotBlockErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetSnapshotBlockErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetSnapshotBlockErrorKind::RequestThrottledException(inner) => {
                    Error::RequestThrottledException(inner)
                }
                crate::error::GetSnapshotBlockErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetSnapshotBlockErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::GetSnapshotBlockErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::GetSnapshotBlockErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListChangedBlocksError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListChangedBlocksError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListChangedBlocksErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListChangedBlocksErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListChangedBlocksErrorKind::RequestThrottledException(inner) => {
                    Error::RequestThrottledException(inner)
                }
                crate::error::ListChangedBlocksErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListChangedBlocksErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::ListChangedBlocksErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListChangedBlocksErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListSnapshotBlocksError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListSnapshotBlocksError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListSnapshotBlocksErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListSnapshotBlocksErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListSnapshotBlocksErrorKind::RequestThrottledException(inner) => {
                    Error::RequestThrottledException(inner)
                }
                crate::error::ListSnapshotBlocksErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListSnapshotBlocksErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::ListSnapshotBlocksErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListSnapshotBlocksErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutSnapshotBlockError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutSnapshotBlockError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutSnapshotBlockErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::PutSnapshotBlockErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::PutSnapshotBlockErrorKind::RequestThrottledException(inner) => {
                    Error::RequestThrottledException(inner)
                }
                crate::error::PutSnapshotBlockErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::PutSnapshotBlockErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::PutSnapshotBlockErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::PutSnapshotBlockErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartSnapshotError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StartSnapshotError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartSnapshotErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::StartSnapshotErrorKind::ConcurrentLimitExceededException(inner) => {
                    Error::ConcurrentLimitExceededException(inner)
                }
                crate::error::StartSnapshotErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::StartSnapshotErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StartSnapshotErrorKind::RequestThrottledException(inner) => {
                    Error::RequestThrottledException(inner)
                }
                crate::error::StartSnapshotErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::StartSnapshotErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::StartSnapshotErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::StartSnapshotErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
