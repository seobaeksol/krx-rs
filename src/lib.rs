//!
//! # krx-rs: 한국거래소 Open API Rust 클라이언트
//!
//! **한국거래소(KRX) Open API를 위한 비공식 Rust 라이브러리**
//!
//! ---
//!
//! ## ✨ 주요 특징
//!
//! - **모든 KRX 엔드포인트 지원**: 주식, 지수, 채권, 파생, ETF/ETN, ESG 등
//! - **비동기(async) 지원**: `tokio` 기반 고성능 HTTP 클라이언트
//! - **데이터 분석 친화적**: [polars](https://pola-rs.github.io/polars/) DataFrame 반환
//! - **빌더 패턴**: 직관적이고 안전한 API 체이닝
//! - **구조화된 로깅**: `tracing` 기반 실무형 로깅
//! - **명확한 에러 처리**: 상세한 오류 타입 제공
//!
//! ---
//!
//! ## 🚀 빠른 시작 (Quick Start)
//!
//! ```rust,no_run
//! use krx_rs::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), krx_rs::error::Error> {
//!     // KRX 인증키로 클라이언트 생성
//!     let client = Client::new("your_auth_key");
//!
//!     // KOSPI 일별 시세 조회 (특정일)
//!     let df = client.stock().stock_daily().date("20240105").fetch().await?;
//!     println!("{}", df);
//!     Ok(())
//! }
//! ```
//!
//! ---
//!
//! ## ⚡ 고급 사용법 (Advanced)
//!
//! ```rust,no_run
//! use krx_rs::{Client, logging::LoggingConfig};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), krx_rs::error::Error> {
//!     let logging_config = LoggingConfig {
//!         level: "debug".to_string(),
//!         json_format: false,
//!         filter_sensitive: true,
//!         file_path: None,
//!     };
//!     let client = Client::builder()
//!         .auth_key("your_auth_key")
//!         .timeout(Duration::from_secs(30))
//!         .user_agent("my-krx-app/1.0")
//!         .logging(logging_config)
//!         .build()?;
//!     let df = client.stock().kosdaq_daily().latest().fetch().await?;
//!     println!("{}", df);
//!     Ok(())
//! }
//! ```
//!
//! ---
//!
//! ## 📅 지원 데이터 범위
//!
//! | 구분         | 범위                        |
//! |--------------|-----------------------------|
//! | 지원 기간    | 2010년 ~ 전일(T-1)           |
//! | 실시간 데이터| ❌ 미지원 (최신: 전일 종가)   |
//! | 인증키 필요  | ✅ (KRX Open API 회원가입)    |
//!
//! > **중요:** 모든 엔드포인트는 `.date()` 또는 `.latest()`로 기준일자를 반드시 지정해야 합니다.
//!
//! ---
//!
//! ## 🗂️ 지원 엔드포인트 및 하위 기능 (계층 구조)
//!
//! | Client 메서드         | 하위 기능(Builder/메서드)                       | 설명                                 |
//! |----------------------|-----------------------------------------------|--------------------------------------|
//! | `client.stock()`     | `stock_daily()`, `kosdaq_daily()`, `konex_daily()`,<br>`stock_warrant_daily()`, `stock_right_daily()`,<br>`stock_base_info()`, `kosdaq_base_info()`, `konex_base_info()` | 주식(일별시세, 기본정보 등)           |
//! | `client.index()`     | `krx_daily()`, `stock_daily()`, `kosdaq_daily()`,<br>`bond_daily()`, `derivative_daily()` | 주가지수(KRX, KOSPI, KOSDAQ 등)       |
//! | `client.bond()`      | `kts_daily()`, `bond_daily()`, `small_bond_daily()` | 채권(국고채, 일반채권, 소액채권 등)    |
//! | `client.etp()`       | `etf_daily()`, `etn_daily()`, `elw_daily()`        | ETP (ETF, ETN, ELW)                   |
//! | `client.derivative()`| `futures_daily()`, `equity_stock_futures_daily()`,<br>`equity_kosdaq_futures_daily()`,<br>`options_daily()`, `equity_stock_options_daily()`,<br>`equity_kosdaq_options_daily()` | 파생상품(선물, 옵션 등)               |
//! | `client.general()`   | `oil_daily()`, `gold_daily()`, `emissions_daily()`  | 일반상품(유가, 금, 배출권 등)          |
//! | `client.esg()`       | `sri_bond_info()`                                 | ESG/사회책임투자채권                  |
//!
//! ---
//!
//! ## 🔗 참고 자료
//!
//! - [KRX 정보데이터시스템(Open API)](https://data.krx.co.kr/)
//! - [공식 GitHub 저장소](https://github.com/finfra/krx-rs)
//! - [docs.rs 문서](https://docs.rs/krx-rs)
//!
//! ---
//!
//! _문의/기여/이슈는 GitHub에서 환영합니다!_

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
