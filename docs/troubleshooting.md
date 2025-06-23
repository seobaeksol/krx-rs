# 문제 해결 가이드

krx-rs 사용 중 발생할 수 있는 일반적인 문제와 해결 방법을 안내합니다.

## 일반적인 오류

### 1. 인증 오류

**증상:**
```
Error: ApiError { status_code: 401, message: "Unauthorized" }
```

**해결방법:**
- AUTH_KEY가 올바른지 확인
- KRX Open API 계정이 활성화되어 있는지 확인
- 인증키가 만료되지 않았는지 확인

```rust
// 환경 변수 확인
println!("AUTH_KEY: {}", std::env::var("KRX_AUTH_KEY").unwrap_or("Not set".to_string()));
```

### 2. Rate Limiting

**증상:**
```
Error: RateLimit { retry_after: 60 }
```

**해결방법:**
- API 호출 빈도를 줄이기
- retry_after 시간만큼 대기 후 재시도

```rust
use tokio::time::{sleep, Duration};

match result {
    Err(krx_rs::Error::RateLimit { retry_after }) => {
        println!("Rate limit 발생. {}초 대기", retry_after);
        sleep(Duration::from_secs(retry_after)).await;
        // 재시도
    }
    _ => {}
}
```

### 3. 네트워크 오류

**증상:**
```
Error: Network(...)
```

**해결방법:**
- 인터넷 연결 확인
- 프록시/방화벽 설정 확인
- 타임아웃 시간 늘리기

```rust
let client = Client::builder()
    .auth_key("your_key")
    .timeout(Duration::from_secs(60))  // 타임아웃 증가
    .build()?;
```

### 4. 날짜 형식 오류

**증상:**
```
Error: InvalidInput("date must be in YYYYMMDD format")
```

**해결방법:**
- 날짜는 반드시 "YYYYMMDD" 형식 사용
- 예: "20240105" (O), "2024-01-05" (X)

```rust
// 올바른 형식
client.stock().stock_daily().date("20240105").fetch().await?;

// chrono 사용 시
use chrono::Local;
let date_str = Local::now().format("%Y%m%d").to_string();
client.stock().stock_daily().date(&date_str).fetch().await?;
```

### 5. 데이터 없음

**증상:**
- 빈 DataFrame 반환
- `df.height() == 0`

**원인:**
- 휴장일 데이터 조회
- 아직 생성되지 않은 당일 데이터 조회
- 잘못된 날짜 입력

**해결방법:**
```rust
let df = client.stock().stock_daily().date("20240105").fetch().await?;

if df.height() == 0 {
    println!("데이터가 없습니다. 날짜를 확인하세요.");
    // 영업일 확인 로직 추가
}
```

## 컴파일 오류

### 1. async 런타임 오류

**증상:**
```
error: `main` function is not allowed to be `async`
```

**해결방법:**
```rust
// Tokio 런타임 매크로 추가
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ...
}
```

### 2. 타입 오류

**증상:**
```
error: the trait `From<krx_rs::Error>` is not implemented
```

**해결방법:**
```rust
// Box<dyn Error> 사용
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ...
}

// 또는 krx_rs::Result 사용
use krx_rs::Result;
async fn fetch_data() -> Result<DataFrame> {
    // ...
}
```

## DataFrame 관련 문제

### 1. 컬럼명 오류

**증상:**
```
PolarsError: column not found
```

**해결방법:**
```rust
// 컬럼명 확인
println!("컬럼 목록: {:?}", df.get_column_names());

// 안전한 컬럼 접근
if let Ok(column) = df.column("종가") {
    // 작업 수행
}
```

### 2. 타입 변환 오류

**해결방법:**
```rust
// 숫자 컬럼 안전하게 처리
let mean = df.column("등락률")?
    .cast(&DataType::Float64)?
    .mean()
    .unwrap_or(0.0);
```

## 성능 문제

### 1. 느린 응답 속도

**해결방법:**
- 동시 요청 활용
- 필요한 데이터만 조회
- 캐싱 구현

```rust
use futures::future::try_join_all;

// 병렬 처리
let tasks = dates.iter().map(|date| {
    client.stock().stock_daily().date(date).fetch()
});

let results = try_join_all(tasks).await?;
```

### 2. 메모리 사용량

**해결방법:**
- Lazy evaluation 활용
- 필요한 컬럼만 선택

```rust
let filtered = df.lazy()
    .select([col("종목명"), col("종가")])  // 필요한 컬럼만
    .filter(col("거래량").gt(lit(1000000)))
    .collect()?;
```

## 디버깅 팁

### 1. 상세 로깅 활성화

```rust
use krx_rs::logging::LoggingConfig;

let client = Client::builder()
    .auth_key("your_key")
    .logging(LoggingConfig {
        level: "debug".to_string(),
        json_format: false,
        filter_sensitive: false,
        file_path: Some("debug.log".to_string()),
    })
    .build()?;
```

### 2. 환경 변수 로깅

```bash
RUST_LOG=krx_rs=debug cargo run
```

### 3. 요청/응답 확인

로그에서 다음 정보 확인:
- 요청 URL
- 요청 파라미터
- 응답 상태 코드
- 에러 메시지

## 추가 도움말

### 커뮤니티 지원

- GitHub Issues: https://github.com/suyoungkim/krx-rs/issues
- 문서: https://docs.rs/krx-rs

### KRX API 관련

- KRX Open API 문서: https://data.krx.co.kr
- API 제한사항 확인
- 서비스 점검 시간 확인

### 버전 호환성

```toml
# Cargo.toml
[dependencies]
krx-rs = "0.1"  # 최신 버전 사용
tokio = { version = "1", features = ["full"] }
```