# krx-rs: KRX Open API Rust 클라이언트 상세 설계 명세서

## 📋 목차

1. [개요](#1-개요)
2. [프로젝트 구조](#2-프로젝트-구조)
3. [핵심 의존성](#3-핵심-의존성)
4. [오류 처리](#4-오류-처리)
5. [로깅 전략](#5-로깅-전략)
6. [핵심 클라이언트](#6-핵심-클라이언트)
7. [API 빌더 패턴](#7-api-빌더-패턴)
8. [데이터 구조체](#8-데이터-구조체)
9. [사용 예제](#9-사용-예제)
10. [개발 가이드라인](#10-개발-가이드라인)

---

## 1. 개요

`krx-rs`는 한국거래소(KRX) Open API를 위한 현대적이고 타입 안전한 Rust 클라이언트 라이브러리입니다.

### 주요 특징
- ✅ **타입 안전성**: Rust의 강력한 타입 시스템 활용
- ✅ **비동기 지원**: tokio 기반의 비동기 API
- ✅ **빌더 패턴**: 직관적이고 유연한 API 구성
- ✅ **데이터 처리**: Polars DataFrame으로 즉시 사용 가능한 데이터 제공
- ✅ **명확한 오류 처리**: 상세한 오류 타입과 컨텍스트 제공

---

## 2. 프로젝트 구조

```
krx-rs/
├── Cargo.toml              # 프로젝트 설정 및 의존성
├── src/
│   ├── lib.rs             # 라이브러리 진입점
│   ├── client.rs          # HTTP 클라이언트 핵심 로직
│   ├── error.rs           # 오류 타입 정의
│   ├── data/              # API 응답 데이터 구조체
│   │   ├── mod.rs         # 공통 데이터 타입
│   │   ├── stock.rs       # 주식 관련 응답
│   │   ├── index.rs       # 지수 관련 응답
│   │   ├── bond.rs        # 채권 관련 응답
│   │   ├── etp.rs         # ETP (ETF/ETN/ELW) 응답
│   │   ├── derivative.rs  # 파생상품 응답
│   │   └── general.rs     # 일반상품 응답
│   └── api/               # API 엔드포인트 빌더
│       ├── mod.rs         # API 모듈 진입점
│       ├── stock.rs       # 주식 API
│       ├── index.rs       # 지수 API
│       ├── bond.rs        # 채권 API
│       ├── etp.rs         # ETP API
│       ├── derivative.rs  # 파생상품 API
│       └── general.rs     # 일반상품 API
├── examples/              # 사용 예제
│   ├── simple_fetch.rs    # 기본 사용법
│   └── advanced_query.rs  # 고급 기능
└── tests/                 # 테스트 코드
    ├── integration/       # 통합 테스트
    └── unit/             # 단위 테스트
```

---

## 3. 핵심 의존성

### Cargo.toml

```toml
[package]
name = "krx-rs"
version = "0.1.0"
edition = "2024"
authors = ["Your Name <your.email@example.com>"]
license = "MIT OR Apache-2.0"
description = "KRX Open API를 위한 Rust 클라이언트"
repository = "https://github.com/yourusername/krx-rs"
keywords = ["krx", "korea-exchange", "stock", "finance", "api"]
categories = ["api-bindings", "finance"]

[dependencies]
# 비동기 런타임
tokio = { version = "1.45", features = ["full"] }

# HTTP 클라이언트
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }

# 데이터 처리
polars = { version = "0.49", features = ["lazy", "serde", "json", "temporal"] }

# 직렬화/역직렬화
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 오류 처리
thiserror = "2.0"

# 날짜/시간 처리
chrono = { version = "0.4", features = ["serde"] }

# 로깅
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

[dev-dependencies]
# 테스트용
tokio-test = "0.4"
wiremock = "0.6"
pretty_assertions = "1.4"

# CLI 예제용
clap = { version = "4.0", features = ["derive"] }
```

---

## 4. 오류 처리

### src/error.rs

```rust
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
```

---

## 5. 로깅 전략

### 5.1 로깅 프레임워크 선택

`krx-rs`는 **tracing** 크레이트를 사용하여 구조화된 로깅을 제공합니다.

#### tracing 선택 이유
- ✅ **구조화된 로깅**: JSON 형태의 구조화된 로그 출력
- ✅ **비동기 지원**: tokio와 완벽한 호환성
- ✅ **성능**: 낮은 오버헤드와 고성능
- ✅ **유연성**: 다양한 출력 포맷과 필터링 지원
- ✅ **분산 추적**: span을 통한 요청 추적

### 5.2 로깅 레벨 정의

| 레벨 | 용도 | 예시 |
|------|------|------|
| **ERROR** | 심각한 오류 | API 인증 실패, 네트워크 연결 오류 |
| **WARN** | 경고 상황 | Rate limiting 발생, 재시도 시도 |
| **INFO** | 일반 정보 | API 요청 시작/완료, 주요 작업 상태 |
| **DEBUG** | 디버깅 정보 | 요청/응답 상세 내용, 파라미터 값 |
| **TRACE** | 상세 추적 | 함수 진입/종료, 내부 처리 과정 |

### 5.3 로깅 구현

#### src/logging.rs

```rust
use tracing::{info, warn, error, debug, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use std::env;

/// 로깅 설정 구조체
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// 로깅 레벨 (기본값: "info")
    pub level: String,
    /// JSON 형태 출력 여부
    pub json_format: bool,
    /// 민감한 정보 필터링 여부
    pub filter_sensitive: bool,
    /// 파일 출력 경로 (옵션)
    pub file_path: Option<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            json_format: false,
            filter_sensitive: true,
            file_path: None,
        }
    }
}

/// 로깅 초기화
pub fn init_logging(config: &LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&config.level))?;

    let subscriber = tracing_subscriber::registry()
        .with(env_filter);

    if config.json_format {
        // JSON 형태 출력
        let json_layer = tracing_subscriber::fmt::layer()
            .json()
            .with_current_span(true)
            .with_span_list(true);
        
        subscriber.with(json_layer).try_init()?;
    } else {
        // 일반 텍스트 출력
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true);
            
        subscriber.with(fmt_layer).try_init()?;
    }

    info!("krx-rs logging initialized with level: {}", config.level);
    Ok(())
}

/// 민감한 정보 필터링 매크로
#[macro_export]
macro_rules! log_filtered {
    ($level:ident, $msg:expr, $($key:ident = $value:expr),*) => {
        $level!(
            message = $msg,
            $(
                $key = if stringify!($key).contains("key") || 
                         stringify!($key).contains("auth") || 
                         stringify!($key).contains("token") {
                    "***FILTERED***"
                } else {
                    &$value.to_string()
                }
            ),*
        );
    };
}
```

### 5.4 클라이언트 로깅 통합

#### src/client.rs (로깅 추가)

```rust
use tracing::{info, warn, error, debug, trace, instrument, Span};
use crate::logging::LoggingConfig;

impl Client {
    /// 로깅 설정과 함께 클라이언트 생성
    pub fn with_logging(auth_key: impl Into<String>, logging_config: LoggingConfig) -> Result<Self> {
        crate::logging::init_logging(&logging_config)?;
        Ok(Self::new(auth_key))
    }

    /// HTTP GET 요청 (로깅 포함)
    #[instrument(skip(self, params), fields(endpoint = %endpoint))]
    pub(crate) async fn get<T>(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let start_time = std::time::Instant::now();
        
        // 요청 시작 로깅
        info!(
            endpoint = %endpoint,
            params_count = params.len(),
            "Starting API request"
        );
        
        debug!(
            url = %url,
            params = ?params,
            "Request details"
        );

        let response = self.http_client
            .get(&url)
            .query(params)
            .send()
            .await
            .map_err(|e| {
                error!(
                    endpoint = %endpoint,
                    error = %e,
                    duration_ms = start_time.elapsed().as_millis(),
                    "Network request failed"
                );
                Error::Network(e)
            })?;

        let status_code = response.status().as_u16();
        let duration = start_time.elapsed();

        if response.status().is_success() {
            let body = response.text().await?;
            
            debug!(
                endpoint = %endpoint,
                status_code = status_code,
                response_size = body.len(),
                duration_ms = duration.as_millis(),
                "Received successful response"
            );

            // 응답 파싱
            match serde_json::from_str(&body) {
                Ok(parsed) => {
                    info!(
                        endpoint = %endpoint,
                        status_code = status_code,
                        duration_ms = duration.as_millis(),
                        "API request completed successfully"
                    );
                    Ok(parsed)
                }
                Err(e) => {
                    error!(
                        endpoint = %endpoint,
                        error = %e,
                        response_body = %body.chars().take(500).collect::<String>(),
                        "Failed to parse response"
                    );
                    Err(Error::Parsing {
                        details: format!("Failed to deserialize response from {}", endpoint),
                        source: e,
                        response_body: body,
                    })
                }
            }
        } else {
            let message = response.text().await.unwrap_or_default();
            
            // Rate limiting 특별 처리
            if status_code == 429 {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60);
                
                warn!(
                    endpoint = %endpoint,
                    retry_after = retry_after,
                    duration_ms = duration.as_millis(),
                    "Rate limit exceeded"
                );
                
                return Err(Error::RateLimit { retry_after });
            }
            
            error!(
                endpoint = %endpoint,
                status_code = status_code,
                error_message = %message,
                duration_ms = duration.as_millis(),
                "API request failed"
            );
            
            Err(Error::ApiError {
                status_code,
                message,
            })
        }
    }
}

impl ClientBuilder {
    /// 로깅 설정 추가
    pub fn logging(mut self, config: LoggingConfig) -> Self {
        self.logging_config = Some(config);
        self
    }
    
    pub fn build(self) -> Result<Client> {
        // 로깅 초기화
        if let Some(config) = &self.logging_config {
            crate::logging::init_logging(config)?;
        }
        
        // ... 기존 빌드 로직
    }
}
```

### 5.5 사용 예제

#### 기본 로깅 설정

```rust
use krx_rs::{Client, logging::LoggingConfig};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 설정
    let logging_config = LoggingConfig {
        level: "debug".to_string(),
        json_format: false,
        filter_sensitive: true,
        file_path: None,
    };
    
    // 클라이언트 생성 (로깅 포함)
    let client = Client::builder()
        .auth_key(std::env::var("KRX_API_KEY")?)
        .logging(logging_config)
        .build()?;
    
    info!("Starting KRX data collection");
    
    // API 호출 (자동으로 로깅됨)
    let data = client
        .stock()
        .stock_daily()
        .date("20240105")
        .fetch()
        .await?;
        
    info!(record_count = data.height(), "Data collection completed");
    
    Ok(())
}
```

#### 프로덕션 환경 설정

```rust
// 환경 변수 기반 로깅 설정
let logging_config = LoggingConfig {
    level: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    json_format: std::env::var("LOG_FORMAT").unwrap_or_default() == "json",
    filter_sensitive: true,
    file_path: std::env::var("LOG_FILE").ok(),
};

// 구조화된 로깅으로 메트릭 수집
use tracing::{info_span, Instrument};

async fn collect_market_data() -> Result<(), Error> {
    let span = info_span!("market_data_collection", 
        market = "kospi", 
        date = "20240105"
    );
    
    async move {
        // API 호출들...
        info!(metrics.api_calls = 1, metrics.records = 100, "Collection completed");
    }
    .instrument(span)
    .await
}
```

### 5.6 로깅 모범 사례

1. **민감한 정보 보호**
   - 인증키, 토큰 등은 자동 필터링
   - 개인정보는 해시화 또는 마스킹

2. **성능 모니터링**
   - API 응답 시간 측정
   - 처리량 및 오류율 추적

3. **구조화된 로깅**
   - 일관된 필드명 사용
   - 검색 가능한 키-값 형태

4. **환경별 설정**
   - 개발: DEBUG 레벨, 텍스트 형태
   - 프로덕션: INFO 레벨, JSON 형태

---

## 6. 핵심 클라이언트

### src/client.rs

```rust
use crate::{api, error::{Error, Result}};
use reqwest::{Client as HttpClient, header::{HeaderMap, HeaderValue}};
use serde::de::DeserializeOwned;
use std::time::Duration;

const BASE_URL: &str = "http://data-dbg.krx.co.kr/svc/apis";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// KRX API 클라이언트
pub struct Client {
    http_client: HttpClient,
    auth_key: String,
    base_url: String,
}

impl Client {
    /// 새로운 클라이언트 인스턴스 생성
    ///
    /// # Arguments
    /// * `auth_key` - KRX Open API 인증키
    ///
    /// # Example
    /// ```
    /// let client = Client::new("your_auth_key");
    /// ```
    pub fn new(auth_key: impl Into<String>) -> Self {
        Self::builder()
            .auth_key(auth_key)
            .build()
            .expect("Failed to build client with default settings")
    }

    /// 클라이언트 빌더 반환
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// 내부 HTTP GET 요청 처리
    pub(crate) async fn get<T>(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let response = self.http_client
            .get(&url)
            .query(params)
            .send()
            .await?;

        if response.status().is_success() {
            let body = response.text().await?;
            serde_json::from_str(&body).map_err(|e| Error::Parsing {
                details: format!("Failed to deserialize response from {}", endpoint),
                source: e,
                response_body: body,
            })
        } else {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            
            // 속도 제한 체크
            if status == 429 {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60);
                
                return Err(Error::RateLimit { retry_after });
            }
            
            Err(Error::ApiError {
                status_code: status,
                message,
            })
        }
    }

    /// 주식 API 접근
    pub fn stock(&self) -> api::stock::StockApi {
        api::stock::StockApi::new(self)
    }

    /// 지수 API 접근
    pub fn index(&self) -> api::index::IndexApi {
        api::index::IndexApi::new(self)
    }

    /// 채권 API 접근
    pub fn bond(&self) -> api::bond::BondApi {
        api::bond::BondApi::new(self)
    }

    /// ETP API 접근
    pub fn etp(&self) -> api::etp::EtpApi {
        api::etp::EtpApi::new(self)
    }

    /// 파생상품 API 접근
    pub fn derivative(&self) -> api::derivative::DerivativeApi {
        api::derivative::DerivativeApi::new(self)
    }

    /// 일반상품 API 접근
    pub fn general(&self) -> api::general::GeneralApi {
        api::general::GeneralApi::new(self)
    }
}

/// 클라이언트 빌더
#[derive(Default)]
pub struct ClientBuilder {
    auth_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    user_agent: Option<String>,
}

impl ClientBuilder {
    /// 인증키 설정
    pub fn auth_key(mut self, key: impl Into<String>) -> Self {
        self.auth_key = Some(key.into());
        self
    }

    /// 기본 URL 재정의 (테스트용)
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// 요청 타임아웃 설정
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// User-Agent 헤더 설정
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// 클라이언트 빌드
    pub fn build(self) -> Result<Client> {
        let auth_key = self.auth_key
            .ok_or_else(|| Error::InvalidInput("auth_key is required".to_string()))?;

        let mut headers = HeaderMap::new();
        headers.insert("AUTH_KEY", HeaderValue::from_str(&auth_key)
            .map_err(|_| Error::InvalidInput("Invalid auth_key format".to_string()))?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let http_client = HttpClient::builder()
            .default_headers(headers)
            .timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
            .user_agent(self.user_agent.unwrap_or_else(|| 
                format!("krx-rs/{}", env!("CARGO_PKG_VERSION"))
            ))
            .build()?;

        Ok(Client {
            http_client,
            auth_key,
            base_url: self.base_url.unwrap_or_else(|| BASE_URL.to_string()),
        })
    }
}
```

---

## 7. API 빌더 패턴

### src/api/stock.rs

```rust
use crate::{client::Client, data::stock::*, error::Result};
use polars::prelude::DataFrame;

/// 주식 관련 API 엔드포인트
pub struct StockApi<'a> {
    client: &'a Client,
}

impl<'a> StockApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// 유가증권 일별매매정보 조회
    pub fn stock_daily(&self) -> KospiDailyBuilder<'a> {
        KospiDailyBuilder::new(self.client)
    }

    /// 코스닥 일별매매정보 조회
    pub fn kosdaq_daily(&self) -> KosdaqDailyBuilder<'a> {
        KosdaqDailyBuilder::new(self.client)
    }

    /// 코넥스 일별매매정보 조회
    pub fn konex_daily(&self) -> KonexDailyBuilder<'a> {
        KonexDailyBuilder::new(self.client)
    }

    /// 유가증권 종목기본정보 조회
    pub fn kospi_info(&self) -> KospiInfoBuilder<'a> {
        KospiInfoBuilder::new(self.client)
    }
}

/// 유가증권 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KospiDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> KospiDailyBuilder<'a> {
    fn new(client: &'a Client) -> Self {
        Self {
            client,
            base_date: None,
        }
    }

    /// 조회 기준일자 설정 (YYYYMMDD)
    ///
    /// # Example
    /// ```
    /// .date("20240105")
    /// ```
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.base_date = Some(date.into());
        self
    }

    /// 오늘 날짜로 설정
    pub fn today(mut self) -> Self {
        use chrono::Local;
        self.base_date = Some(Local::now().format("%Y%m%d").to_string());
        self
    }

    /// API 호출 및 데이터 조회
    pub async fn fetch(self) -> Result<DataFrame> {
        let base_date = self.base_date
            .ok_or_else(|| Error::InvalidInput("date is required".to_string()))?;

        // 날짜 형식 검증
        if !is_valid_date_format(&base_date) {
            return Err(Error::InvalidInput(
                "date must be in YYYYMMDD format".to_string()
            ));
        }

        let response: ApiResponse<KospiDailyRecord> = self.client
            .get(
                "/sto/stk_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_stock_daily(response)
    }
}

/// 날짜 형식 검증 (YYYYMMDD)
fn is_valid_date_format(date: &str) -> bool {
    date.len() == 8 && date.chars().all(|c| c.is_numeric())
}
```

---

## 8. 데이터 구조체

### src/data/mod.rs

```rust
use serde::{Deserialize, Deserializer};

pub mod stock;
pub mod index;
pub mod bond;
pub mod etp;
pub mod derivative;
pub mod general;

/// KRX API 공통 응답 구조
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(rename = "OutBlock_1")]
    pub data: Vec<T>,
}

/// 문자열을 Option<f64>로 파싱하는 헬퍼
pub fn deserialize_optional_number<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() || s == "-" {
        Ok(None)
    } else {
        s.replace(',', "")
            .parse::<f64>()
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}
```

### src/data/stock.rs

```rust
use super::{ApiResponse, deserialize_optional_number};
use polars::prelude::*;
use serde::Deserialize;
use crate::error::Result;

/// 유가증권 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct KospiDailyRecord {
    /// 기준일자 (YYYY/MM/DD)
    #[serde(rename = "BAS_DD")]
    pub base_date: String,
    
    /// 종목코드
    #[serde(rename = "ISU_CD")]
    pub issue_code: String,
    
    /// 종목명
    #[serde(rename = "ISU_NM")]
    pub issue_name: String,
    
    /// 시장구분
    #[serde(rename = "MKT_NM")]
    pub market_name: String,
    
    /// 소속부
    #[serde(rename = "SECT_TP_NM")]
    pub sector_type: String,
    
    /// 종가
    #[serde(rename = "TDD_CLSPRC")]
    pub close_price: String,
    
    /// 대비
    #[serde(rename = "CMPPREVDD_PRC")]
    pub price_change: String,
    
    /// 등락률
    #[serde(rename = "FLUC_RT")]
    pub fluctuation_rate: String,
    
    /// 시가
    #[serde(rename = "TDD_OPNPRC")]
    pub open_price: String,
    
    /// 고가
    #[serde(rename = "TDD_HGPRC")]
    pub high_price: String,
    
    /// 저가
    #[serde(rename = "TDD_LWPRC")]
    pub low_price: String,
    
    /// 거래량
    #[serde(rename = "ACC_TRDVOL")]
    pub trading_volume: String,
    
    /// 거래대금
    #[serde(rename = "ACC_TRDVAL")]
    pub trading_value: String,
    
    /// 시가총액
    #[serde(rename = "MKTCAP")]
    pub market_cap: String,
    
    /// 상장주식수
    #[serde(rename = "LIST_SHRS")]
    pub listed_shares: String,
}

/// KOSPI 일별매매정보를 DataFrame으로 변환
pub fn parse_stock_daily(response: ApiResponse<KospiDailyRecord>) -> Result<DataFrame> {
    let records = response.data;
    
    if records.is_empty() {
        return Ok(DataFrame::empty());
    }
    
    // 각 필드를 벡터로 수집
    let mut dates = Vec::with_capacity(records.len());
    let mut codes = Vec::with_capacity(records.len());
    let mut names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut changes = Vec::with_capacity(records.len());
    let mut change_rates = Vec::with_capacity(records.len());
    let mut volumes = Vec::with_capacity(records.len());
    let mut values = Vec::with_capacity(records.len());
    let mut market_caps = Vec::with_capacity(records.len());
    
    for record in records {
        dates.push(record.base_date);
        codes.push(record.issue_code);
        names.push(record.issue_name);
        close_prices.push(parse_number(&record.close_price));
        changes.push(parse_number(&record.price_change));
        change_rates.push(parse_number(&record.fluctuation_rate));
        volumes.push(parse_number(&record.trading_volume));
        values.push(parse_number(&record.trading_value));
        market_caps.push(parse_number(&record.market_cap));
    }
    
    // DataFrame 생성
    let df = df! {
        "날짜" => &dates,
        "종목코드" => &codes,
        "종목명" => &names,
        "종가" => &close_prices,
        "대비" => &changes,
        "등락률" => &change_rates,
        "거래량" => &volumes,
        "거래대금" => &values,
        "시가총액" => &market_caps,
    }?;
    
    Ok(df)
}

/// 문자열을 숫자로 파싱 (콤마 제거)
fn parse_number(s: &str) -> Option<f64> {
    if s.is_empty() || s == "-" {
        None
    } else {
        s.replace(',', "").parse().ok()
    }
}
```

---

## 9. 사용 예제

### examples/simple_fetch.rs

```rust
use krx_rs::{Client, Error};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 환경 변수에서 인증키 읽기
    let auth_key = env::var("KRX_API_KEY")
        .expect("KRX_API_KEY 환경 변수를 설정해주세요");
    
    // 클라이언트 생성
    let client = Client::new(auth_key);
    
    // KOSPI 일별 매매정보 조회
    println!("📊 2024년 1월 5일 KOSPI 시장 데이터 조회 중...");
    
    let df = client
        .stock()
        .stock_daily()
        .date("20240105")
        .fetch()
        .await?;
    
    // 결과 출력
    println!("\n조회 결과 (상위 10개):");
    println!("{}", df.head(Some(10)));
    
    // 거래대금 상위 5개 종목
    let top_5 = df
        .lazy()
        .sort("거래대금", SortOptions::default().with_descending(true))
        .limit(5)
        .collect()?;
    
    println!("\n💰 거래대금 TOP 5:");
    println!("{}", top_5);
    
    Ok(())
}
```

### examples/advanced_query.rs

```rust
use krx_rs::{Client, Error};
use polars::prelude::*;
use chrono::{Local, Duration};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::builder()
        .auth_key(std::env::var("KRX_API_KEY")?)
        .timeout(std::time::Duration::from_secs(60))
        .build()?;
    
    // 최근 5일간의 데이터 수집
    let mut all_data = Vec::new();
    let today = Local::now().date_naive();
    
    for i in 0..5 {
        let date = today - Duration::days(i);
        let date_str = date.format("%Y%m%d").to_string();
        
        match client.stock().stock_daily().date(&date_str).fetch().await {
            Ok(df) => {
                println!("✅ {} 데이터 수집 완료", date_str);
                all_data.push(df);
            }
            Err(e) => {
                eprintln!("❌ {} 데이터 수집 실패: {}", date_str, e);
            }
        }
        
        // Rate limit 방지
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    // 데이터 병합
    if !all_data.is_empty() {
        let combined = concat(&all_data, true, true)?;
        
        // 분석: 종목별 평균 거래량
        let analysis = combined
            .lazy()
            .group_by(&[col("종목명")])
            .agg(&[
                col("거래량").mean().alias("평균거래량"),
                col("등락률").mean().alias("평균등락률"),
            ])
            .sort("평균거래량", SortOptions::default().with_descending(true))
            .limit(10)
            .collect()?;
        
        println!("\n📈 최근 5일 평균 거래량 TOP 10:");
        println!("{}", analysis);
    }
    
    Ok(())
}
```

---

## 10. 개발 가이드라인

### 코드 스타일
- Rust 공식 스타일 가이드 준수 (`cargo fmt`)
- Clippy 경고 모두 해결 (`cargo clippy`)
- 모든 공개 API에 문서화 주석 작성

### 테스트
- 각 API 엔드포인트별 단위 테스트 작성
- Mock 서버를 사용한 통합 테스트
- 실제 API 호출 예제는 `examples/` 디렉토리에

### 버전 관리
- Semantic Versioning 준수
- 변경사항은 CHANGELOG.md에 기록
- Breaking changes는 메이저 버전 업데이트

### 성능 고려사항
- 대량 데이터 처리 시 스트리밍 지원
- 연결 재사용을 위한 Client 재사용 권장
- Rate limiting 자동 처리

### 보안
- 인증키는 환경 변수나 설정 파일로 관리
- HTTPS 사용 (가능한 경우)
- 민감한 정보는 로그에 남기지 않음 (자동 필터링 적용)

### 로깅 가이드라인
- 적절한 로깅 레벨 선택 (개발시 DEBUG, 운영시 INFO)
- 구조화된 로깅으로 검색 및 분석 편의성 제공
- 성능에 영향을 주지 않는 효율적인 로깅
- 민감한 정보 자동 필터링으로 보안 강화

---

이 설계서는 `krx-rs` 라이브러리의 전체적인 구조와 구현 방향을 제시합니다. 각 모듈과 기능은 확장 가능하도록 설계되었으며, 사용자 친화적인 API를 제공하는 것을 목표로 합니다.