# 시작하기

krx-rs를 사용하여 KRX Open API에 접근하는 방법을 안내합니다.

## 사전 준비

### 1. KRX Open API 인증키 발급

krx-rs를 사용하려면 먼저 KRX OPEN API 서비스 인증키가 필요합니다:

1. [한국거래소 정보데이터시스템 OPEN API ](https://openapi.krx.co.kr/)에 접속
2. 로그인
3. API 인증키 신청
4. API 이용신청
5. 승인 완료 후 해당 API 기능 사용 가능
    > 이용 신청한 API는 일정시간마다 일괄 승인 되는 것으로 보임 (보통 당일 내로 처리됨)

### 2. Rust 환경 설정

Rust 1.75.0 이상이 필요합니다:

```bash
# Rust 설치 (없는 경우)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 버전 확인
rustc --version
```

## 설치

새 프로젝트를 생성하고 krx-rs를 추가합니다:

```bash
cargo new my-krx-app
cd my-krx-app
```

`Cargo.toml`에 의존성 추가:

```toml
[dependencies]
krx-rs = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## 첫 번째 프로그램

`src/main.rs`:

```rust
use krx_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 클라이언트 생성 (인증키 필요)
    let client = Client::new("YOUR_AUTH_KEY");
    
    // KOSPI 일별 시세 조회
    println!("KOSPI 일별 시세 조회 중...");
    let kospi_data = client
        .stock()
        .kospi_daily()
        .date("20240105")  // YYYYMMDD 형식
        .fetch()
        .await?;
    
    println!("조회된 데이터 건수: {}", kospi_data.height());
    println!("\n상위 5개 종목:");
    println!("{}", kospi_data.head(Some(5)));
    
    Ok(())
}
```

실행:

```bash
cargo run
```

## 환경 변수 사용

인증키를 환경 변수로 관리하는 것을 권장합니다:

`.env` 파일:
```
KRX_AUTH_KEY=your_auth_key_here
```

코드에서 사용:
```rust
use std::env;

let auth_key = env::var("KRX_AUTH_KEY")
    .expect("KRX_AUTH_KEY 환경 변수가 설정되지 않았습니다");
let client = Client::new(auth_key);
```

## 날짜 처리

krx-rs는 편리한 날짜 처리 메서드를 제공합니다:

```rust
// 특정 날짜 지정
let data = client.stock().kospi_daily()
    .date("20240105")
    .fetch().await?;

// 오늘 날짜 사용
let today_data = client.stock().kospi_daily()
    .today()
    .fetch().await?;
```

## 오류 처리

모든 API 호출은 `Result`를 반환합니다:

```rust
match client.stock().kospi_daily().today().fetch().await {
    Ok(data) => {
        println!("데이터 조회 성공: {} 건", data.height());
    }
    Err(e) => {
        eprintln!("오류 발생: {}", e);
        // 오류 타입에 따른 처리
        match e {
            krx_rs::Error::Network(_) => eprintln!("네트워크 오류"),
            krx_rs::Error::ApiError { status_code, .. } => {
                eprintln!("API 오류: HTTP {}", status_code)
            }
            _ => eprintln!("기타 오류"),
        }
    }
}
```

## 다음 단계

- [API 가이드](api-guide.md) - 전체 API 사용법
- [예제 모음](examples.md) - 실전 예제
- [설정 옵션](configuration.md) - 고급 설정