# 설정 가이드

krx-rs의 다양한 설정 옵션을 설명합니다.

## 클라이언트 빌더

`Client::builder()`를 사용하여 다양한 옵션을 설정할 수 있습니다:

```rust
use krx_rs::{Client, logging::LoggingConfig};
use std::time::Duration;

let client = Client::builder()
    .auth_key("your_auth_key")
    .timeout(Duration::from_secs(60))
    .user_agent("MyApp/1.0")
    .base_url("http://custom.api.url")  // 테스트용
    .logging(LoggingConfig::default())
    .build()?;
```

## 타임아웃 설정

API 요청의 타임아웃을 설정할 수 있습니다:

```rust
use std::time::Duration;

let client = Client::builder()
    .auth_key("your_auth_key")
    .timeout(Duration::from_secs(30))  // 기본값: 30초
    .build()?;
```

## User-Agent 설정

커스텀 User-Agent를 설정할 수 있습니다:

```rust
let client = Client::builder()
    .auth_key("your_auth_key")
    .user_agent("MyTradingBot/2.0")
    .build()?;
```

## 로깅 설정

### 기본 로깅

```rust
use krx_rs::logging::LoggingConfig;

let config = LoggingConfig {
    level: "info".to_string(),     // trace, debug, info, warn, error
    json_format: false,            // JSON 형식 출력
    filter_sensitive: true,        // 민감한 정보 필터링
    file_path: None,              // 파일 로깅 비활성화
};

let client = Client::with_logging("your_auth_key", config)?;
```

### 파일 로깅

```rust
let config = LoggingConfig {
    level: "debug".to_string(),
    json_format: true,
    filter_sensitive: true,
    file_path: Some("krx-api.log".to_string()),
};
```

### 로그 레벨

- `trace`: 매우 상세한 디버깅 정보
- `debug`: 디버깅 정보
- `info`: 일반 정보 (기본값)
- `warn`: 경고 메시지
- `error`: 오류 메시지만

### 환경 변수로 로그 레벨 설정

```bash
RUST_LOG=krx_rs=debug cargo run
```

## 민감한 정보 필터링

`filter_sensitive: true` 설정 시 로그에서 다음 정보가 마스킹됩니다:

- AUTH_KEY 헤더
- 개인정보로 의심되는 데이터
- API 응답의 일부 민감한 필드

## 재시도 정책

Rate limiting 발생 시 자동으로 처리됩니다:

```rust
match result {
    Err(krx_rs::Error::RateLimit { retry_after }) => {
        println!("{}초 후에 재시도하세요", retry_after);
        tokio::time::sleep(Duration::from_secs(retry_after)).await;
        // 재시도
    }
    _ => {}
}
```

## 프록시 설정

현재 버전에서는 시스템 프록시 설정을 자동으로 따릅니다. 
향후 버전에서 명시적 프록시 설정을 지원할 예정입니다.

## 성능 최적화

### 연결 재사용

krx-rs는 내부적으로 연결 풀을 사용하여 성능을 최적화합니다.

### 동시 요청

여러 API를 동시에 호출할 수 있습니다:

```rust
use futures::future::join_all;

let tasks = vec![
    client.stock().stock_daily().today().fetch(),
    client.stock().kosdaq_daily().today().fetch(),
    client.index().krx_daily().today().fetch(),
];

let results = join_all(tasks).await;
```

### DataFrame 최적화

큰 데이터셋의 경우 lazy evaluation을 활용하세요:

```rust
let optimized = df.lazy()
    .filter(col("거래량").gt(lit(1000000)))
    .select([col("종목명"), col("종가"), col("거래량")])
    .collect()?;
```

## 환경별 설정

### 개발 환경

```rust
#[cfg(debug_assertions)]
let client = Client::builder()
    .auth_key("test_key")
    .logging(LoggingConfig {
        level: "debug".to_string(),
        json_format: false,
        filter_sensitive: false,
        file_path: Some("debug.log".to_string()),
    })
    .build()?;
```

### 프로덕션 환경

```rust
#[cfg(not(debug_assertions))]
let client = Client::builder()
    .auth_key(std::env::var("KRX_AUTH_KEY")?)
    .timeout(Duration::from_secs(10))
    .logging(LoggingConfig {
        level: "warn".to_string(),
        json_format: true,
        filter_sensitive: true,
        file_path: Some("/var/log/krx-api.log".to_string()),
    })
    .build()?;
```

## 테스트 환경

Mock 서버를 사용한 테스트:

```rust
#[cfg(test)]
let client = Client::builder()
    .auth_key("mock_key")
    .base_url(&mock_server.uri())
    .build()?;
```