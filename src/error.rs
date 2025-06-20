use thiserror::Error;

/// krx-rs 라이브러리의 모든 오류를 포함하는 열거형
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// 네트워크 요청 실패
    #[error("Network request failed: {0}")]
    Network(#[from] reqwest::Error),

    /// API 응답 파싱 오류
    #[error("Failed to parse API response: {details}")]
    Parsing {
        details: String,
        #[source]
        source: serde_json::Error,
        response_body: String,
    },

    /// API 서버 오류 응답
    #[error("API error (status {status_code}): {message}")]
    ApiError {
        status_code: u16,
        message: String,
    },

    /// 잘못된 입력 파라미터
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// 데이터프레임 변환 오류
    #[error("DataFrame operation failed")]
    DataFrame(#[from] polars::prelude::PolarsError),

    /// 인증 오류
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// 속도 제한 초과
    #[error("Rate limit exceeded, retry after {retry_after} seconds")]
    RateLimit { retry_after: u64 },
}

pub type Result<T> = std::result::Result<T, Error>;