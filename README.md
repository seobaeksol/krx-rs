# krx-rs

[![Crates.io](https://img.shields.io/crates/v/krx-rs.svg)](https://crates.io/crates/krx-rs)
[![Documentation](https://docs.rs/krx-rs/badge.svg)](https://docs.rs/krx-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

KRX(한국거래소) Open API를 위한 현대적이고 타입 안전한 Rust 클라이언트 라이브러리입니다.
> API 인증키를 발급 받아야합니다. [시작하기](docs/getting-started.md) 참고.

## 특징

- 🦀 **타입 안전성**: Rust의 강력한 타입 시스템을 활용한 안전한 API
- ⚡ **비동기 지원**: Tokio 기반의 완전한 비동기 지원
- 🏗️ **빌더 패턴**: 직관적이고 유연한 API 구성
- 📊 **Polars 통합**: DataFrame으로 즉시 사용 가능한 데이터 제공
- 🎯 **명확한 오류 처리**: 상세한 오류 타입과 컨텍스트 제공
- 📝 **구조화된 로깅**: tracing을 통한 모니터링 및 디버깅 지원

## 설치

`Cargo.toml`에 다음을 추가하세요:

```toml
[dependencies]
krx-rs = "0.1.0"
```

## 빠른 시작

```rust
use krx_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 클라이언트 생성
    let client = Client::new("your_auth_key");
    
    // KOSPI 일별 데이터 조회
    let data = client
        .stock()
        .kospi_daily()
        .date("20240105")
        .fetch()
        .await?;
        
    println!("{}", data);
    Ok(())
}
```

## 주요 API

### 주식 (Stock)
```rust
// KOSPI 일별 시세
let kospi_daily = client.stock().kospi_daily().today().fetch().await?;

// KOSDAQ 종목 기본정보
let kosdaq_info = client.stock().kosdaq_base_info().fetch().await?;
```

### 지수 (Index)
```rust
// KRX 지수
let krx_index = client.index().krx_daily().date("20240105").fetch().await?;

// KOSPI 지수
let kospi_index = client.index().kospi_daily().today().fetch().await?;
```

### 채권 (Bond)
```rust
// 국채 전문시장
let treasury = client.bond().kts_daily().today().fetch().await?;
```

### ETP (ETF/ETN/ELW)
```rust
// ETF 일별 시세
let etf = client.etp().etf_daily().date("20240105").fetch().await?;
```

### 파생상품 (Derivatives)
```rust
// 선물 일별 시세
let futures = client.derivative().futures_daily().today().fetch().await?;
```

### 일반상품 (General)
```rust
// 유가 정보
let oil = client.general().oil_daily().today().fetch().await?;
```

### ESG
```rust
// SRI 채권 정보
let sri_bonds = client.esg().sri_bond_info().fetch().await?;
```

## 고급 기능

### 로깅 설정

```rust
use krx_rs::{Client, logging::LoggingConfig};

let logging_config = LoggingConfig {
    level: "debug".to_string(),
    json_format: false,
    filter_sensitive: true,
    file_path: Some("krx.log".to_string()),
};

let client = Client::builder()
    .auth_key("your_auth_key")
    .logging(logging_config)
    .build()?;
```

### 사용자 정의 설정

```rust
use std::time::Duration;

let client = Client::builder()
    .auth_key("your_auth_key")
    .timeout(Duration::from_secs(60))
    .user_agent("MyApp/1.0")
    .build()?;
```

## 데이터 처리

모든 API 응답은 [Polars DataFrame](https://github.com/pola-rs/polars)으로 반환됩니다:

```rust
let df = client.stock().kospi_daily().today().fetch().await?;

// DataFrame 작업
let filtered = df.lazy()
    .filter(col("등락률").gt(lit(2.0)))
    .select([col("종목명"), col("종가"), col("등락률")])
    .sort("등락률", SortOptions::default().with_order_descending(true))
    .collect()?;
```

## 예제

더 많은 예제는 [examples](examples/) 디렉토리를 참조하세요:

- `simple_fetch.rs` - 기본 데이터 조회
- `with_logging.rs` - 로깅 설정 예제
- `type_conversion_test.rs` - 타입 변환 예제

## API 문서

전체 API 문서는 [docs.rs](https://docs.rs/krx-rs)에서 확인할 수 있습니다.

## 요구사항

- Rust 1.75.0 이상
- KRX Open API 인증키 ([신청하기](https://openapi.krx.co.kr))

## 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.

## 기여

기여를 환영합니다! 이슈나 풀 리퀘스트를 자유롭게 제출해주세요.

## 참고

이 라이브러리는 KRX(한국거래소)와 공식적으로 제휴하거나 승인받은 것이 아닙니다.