
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    /// 401
    #[error("Unauthorized")]
    Unauthorized,

    /// 403
    #[error("Forbidden")]
    Forbidden,

    /// 404
    #[error("Not Found")]
    NotFound,

    /// 500
    #[error("Internal Server Error")]
    InternalServerError,

    /// Deserialize error
    #[error("Deserialize Error")]
    DeserializeError,

    /// Request Error
    #[error("Http Request Error")]
    RequestError,
}
