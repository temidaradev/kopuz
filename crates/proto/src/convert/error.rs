use super::macros::enum_conversion;
use crate::*;

enum_conversion!(error_code_to_proto, error_code_from_proto, api::ErrorCode, ErrorCode,
    default api::ErrorCode::Internal, unspecified ErrorCode::Unspecified, {
        api::ErrorCode::InvalidInput => ErrorCode::InvalidInput,
        api::ErrorCode::Unauthorized => ErrorCode::Unauthorized,
        api::ErrorCode::NotFound => ErrorCode::NotFound,
        api::ErrorCode::Conflict => ErrorCode::Conflict,
        api::ErrorCode::SourceAuthExpired => ErrorCode::SourceAuthExpired,
        api::ErrorCode::SourceUnreachable => ErrorCode::SourceUnreachable,
        api::ErrorCode::Unsupported => ErrorCode::Unsupported,
        api::ErrorCode::Internal => ErrorCode::Internal,
    }
);

pub fn error_body_to_proto(value: &api::ErrorBody) -> ErrorBody {
    ErrorBody {
        code: error_code_to_proto(value.code) as i32,
        message: value.message.clone(),
    }
}

pub fn error_body_from_proto(value: &ErrorBody) -> api::ErrorBody {
    api::ErrorBody {
        code: error_code_from_proto(value.code),
        message: value.message.clone(),
    }
}
