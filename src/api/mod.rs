//! `krx-rs` 라이브러리의 모든 API 엔드포인트 모듈을 관리합니다.
//!
//! 이 모듈은 각 API 카테고리를 서브 모듈로 정의하여,
//! `client.stock()`, `client.index()` 와 같은 형태로 API에 접근할 수 있도록 합니다.

/// 채권(Bond) 관련 API
pub mod bond;
/// 날짜 처리 등 공통 유틸리티
pub mod common;
/// 파생상품(Derivative) 관련 API
pub mod derivative;
/// ESG 관련 API
pub mod esg;
/// ETP (ETF, ETN, ELW) 관련 API
pub mod etp;
/// 일반상품(유가, 금, 배출권) 관련 API
pub mod general;
/// 주가지수(Index) 관련 API
pub mod index;
/// 주식(Stock) 관련 API
pub mod stock;
