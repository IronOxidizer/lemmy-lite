use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

pub type LemmyLiteResult<T> = ::std::result::Result<T, LemmyLiteError>;

#[derive(Debug)]

pub enum LemmyLiteError {
    InternalError(String),
}

impl std::fmt::Display for LemmyLiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LemmyLiteError::InternalError(s) => write!(f, "{}", s),
        }
    }
}

impl actix_web::error::ResponseError for LemmyLiteError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            LemmyLiteError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<url::ParseError> for LemmyLiteError {
    fn from(value: url::ParseError) -> Self {
        LemmyLiteError::InternalError(value.to_string())
    }
}

impl From<awc::error::JsonPayloadError> for LemmyLiteError {
    fn from(value: awc::error::JsonPayloadError) -> Self {
        LemmyLiteError::InternalError(value.to_string())
    }
}

impl From<actix_web::error::Error> for LemmyLiteError {
    fn from(value: actix_web::error::Error) -> Self {
        LemmyLiteError::InternalError(value.to_string())
    }
}

impl From<awc::error::SendRequestError> for LemmyLiteError {
    fn from(value: awc::error::SendRequestError) -> Self {
        LemmyLiteError::InternalError(value.to_string())
    }
}
