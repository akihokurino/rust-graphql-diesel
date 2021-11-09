use crate::ddb::DaoError;
use convert_case::{Case, Casing};
use juniper::{graphql_value, FieldError};
use strum_macros::Display as StrumDisplay;
use thiserror::Error;

#[derive(StrumDisplay, Debug)]
pub enum FieldErrorCode {
    BadRequest,
    UnAuthenticate,
    NotFound,
    Forbidden,
    Internal,
}

#[derive(Error, Debug, Clone)]
pub enum APIError {
    #[error("api error: {0}")]
    Text(String),
}

pub struct FieldErrorWithCode {
    err: APIError,
    code: FieldErrorCode,
}

impl FieldErrorWithCode {
    pub fn bad_request() -> Self {
        FieldErrorWithCode {
            err: APIError::Text("エラーです".to_string()),
            code: FieldErrorCode::BadRequest,
        }
    }

    pub fn un_authenticate() -> Self {
        FieldErrorWithCode {
            err: APIError::Text("エラーです".to_string()),
            code: FieldErrorCode::UnAuthenticate,
        }
    }

    pub fn forbidden() -> Self {
        FieldErrorWithCode {
            err: APIError::Text("エラーです".to_string()),
            code: FieldErrorCode::Forbidden,
        }
    }
}

impl From<DaoError> for FieldErrorWithCode {
    fn from(err: DaoError) -> Self {
        FieldErrorWithCode {
            err: APIError::Text("エラーです".to_string()),
            code: match err {
                DaoError::NotFound => FieldErrorCode::NotFound,
                DaoError::Forbidden => FieldErrorCode::Forbidden,
                DaoError::Internal(_) => FieldErrorCode::Internal,
            },
        }
    }
}

impl From<FieldErrorWithCode> for FieldError {
    fn from(v: FieldErrorWithCode) -> Self {
        let code = v.code.to_string().to_case(Case::UpperSnake);

        FieldError::new(
            v.err,
            graphql_value!({
                "code": code,
            }),
        )
    }
}
