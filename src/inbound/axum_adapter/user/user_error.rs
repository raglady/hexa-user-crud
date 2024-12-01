use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::business::user::model::user::UserError;

pub struct AxumUserError(pub UserError);

impl IntoResponse for AxumUserError {
    fn into_response(self) -> Response {
        match self.0 {
            ref _e @ UserError::UserNotExists { id: _ } => {
                (StatusCode::NOT_FOUND, ()).into_response()
            }
            ref e @ UserError::EmailAlreadyUsed { email: _ }
            | ref e @ UserError::EmailAlreadyUsedByOther { email: _ } => {
                (StatusCode::CONFLICT, e.to_string()).into_response()
            }
            ref e @ UserError::MismatchUserId { id1: _, id2: _ } => {
                (StatusCode::BAD_REQUEST, e.to_string()).into_response()
            }
            ref e @ UserError::PerPageValueTooHigh { per_page: _ }
            | ref e @ UserError::PerPageValueTooLow { per_page: _ } => {
                (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()).into_response()
            }
            ref e @ UserError::PerPageOffsetTooLow { offset: _ } => {
                (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()).into_response()
            }
            ref e @ UserError::Unknown(ref _error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}
