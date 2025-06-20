//! # krx-rs
//!
//! KRX Open API를 위한 현대적이고 타입 안전한 Rust 클라이언트 라이브러리
//!
//! ## 데이터 제공 범위 안내
//!
//! **중요**: KRX Open API는 **2010년 이후부터 조회일 기준 전일(T-1)**까지의 데이터만 제공합니다.
//! 라이브러리를 통해 당일 데이터를 조회할 수 없으니 이용에 참고 부탁드립니다.
//!
//! ## 특징
//!
//! - **타입 안전성**: Rust의 강력한 타입 시스템 활용
//! - **비동기 지원**: tokio 기반의 비동기 API
//! - **빌더 패턴**: 직관적이고 유연한 API 구성
//! - **데이터 처리**: Polars DataFrame으로 즉시 사용 가능한 데이터 제공
//! - **명확한 오류 처리**: 상세한 오류 타입과 컨텍스트 제공
//! - **구조화된 로깅**: tracing을 통한 모니터링 및 디버깅 지원
//!
//! ## 빠른 시작
//!
//! ```rust
//! use krx_rs::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("your_auth_key");
//!     
//!     // 특정 날짜의 데이터 조회
//!     let data_by_date = client
//!         .stock()
//!         .kospi_daily()
//!         .date("20240105")
//!         .fetch()
//!         .await?;
//!     println!("20240105 데이터:\\n{}", data_by_date);
//!
//!     // 가장 최신(전일) 데이터 조회
//!     let latest_data = client
//!         .stock()
//!         .kospi_daily()
//!         .latest()
//!         .fetch()
//!         .await?;
//!     println!("가장 최신 데이터:\\n{}", latest_data);
//!         
//!     Ok(())
//! }
//! ```
//!
//! ## 로깅 설정
//!
//! ```rust
//! use krx_rs::{Client, logging::LoggingConfig};
//!
//! let logging_config = LoggingConfig {
//!     level: "debug".to_string(),
//!     json_format: false,
//!     filter_sensitive: true,
//!     file_path: None,
//! };
//!
//! let client = Client::builder()
//!     .auth_key("your_auth_key")
//!     .logging(logging_config)
//!     .build()?;
//! ```

pub mod api;
pub mod client;
pub mod data;
pub mod error;
pub mod logging;

// Re-export main types for convenience
pub use client::{Client, ClientBuilder};
pub use error::{Error, Result};
pub use logging::LoggingConfig;

// Re-export common polars types
pub use polars::prelude::DataFrame;
