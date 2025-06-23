# API 가이드

krx-rs가 제공하는 모든 API 엔드포인트와 사용법을 설명합니다.

## API 구조

krx-rs는 KRX Open API를 7개의 주요 카테고리로 구성합니다:

1. **Stock** - 주식 관련 API
2. **Index** - 지수 관련 API
3. **Bond** - 채권 관련 API
4. **ETP** - ETF, ETN, ELW 관련 API
5. **Derivative** - 파생상품 관련 API
6. **General** - 일반상품 (유가, 금, 배출권) API
7. **ESG** - ESG 관련 API

## Stock API

### KOSPI 일별 시세
```rust
let data = client.stock()
    .stock_daily()
    .date("20240105")
    .fetch()
    .await?;
```

### KOSDAQ 일별 시세
```rust
let data = client.stock()
    .kosdaq_daily()
    .today()
    .fetch()
    .await?;
```

### KONEX 일별 시세
```rust
let data = client.stock()
    .konex_daily()
    .date("20240105")
    .fetch()
    .await?;
```

### 신주인수권 관련
```rust
// 신주인수권증권
let warrant = client.stock()
    .stock_warrant_daily()
    .today()
    .fetch()
    .await?;

// 신주인수권증서
let right = client.stock()
    .stock_right_daily()
    .today()
    .fetch()
    .await?;
```

### 종목 기본정보
```rust
// KOSPI 종목 정보
let kospi_info = client.stock()
    .stock_base_info()
    .fetch()
    .await?;

// KOSDAQ 종목 정보
let kosdaq_info = client.stock()
    .kosdaq_base_info()
    .fetch()
    .await?;

// KONEX 종목 정보
let konex_info = client.stock()
    .konex_base_info()
    .fetch()
    .await?;
```

## Index API

### KRX 지수
```rust
let krx_index = client.index()
    .krx_daily()
    .date("20240105")
    .fetch()
    .await?;
```

### KOSPI/KOSDAQ 지수
```rust
// KOSPI 지수
let kospi = client.index()
    .stock_daily()
    .today()
    .fetch()
    .await?;

// KOSDAQ 지수
let kosdaq = client.index()
    .kosdaq_daily()
    .today()
    .fetch()
    .await?;
```

### 채권/파생상품 지수
```rust
// 채권지수
let bond_index = client.index()
    .bond_daily()
    .today()
    .fetch()
    .await?;

// 파생상품지수
let derivative_index = client.index()
    .derivative_daily()
    .today()
    .fetch()
    .await?;
```

## Bond API

### 국채 전문유통시장
```rust
let kts = client.bond()
    .kts_daily()
    .date("20240105")
    .fetch()
    .await?;
```

### 일반채권
```rust
let bond = client.bond()
    .bond_daily()
    .today()
    .fetch()
    .await?;
```

### 소액채권
```rust
let small_bond = client.bond()
    .small_bond_daily()
    .today()
    .fetch()
    .await?;
```

## ETP API

### ETF
```rust
let etf = client.etp()
    .etf_daily()
    .date("20240105")
    .fetch()
    .await?;
```

### ETN
```rust
let etn = client.etp()
    .etn_daily()
    .today()
    .fetch()
    .await?;
```

### ELW
```rust
let elw = client.etp()
    .elw_daily()
    .today()
    .fetch()
    .await?;
```

## Derivative API

### 선물
```rust
// 일반 선물
let futures = client.derivative()
    .futures_daily()
    .date("20240105")
    .fetch()
    .await?;

// 개별주식선물 (KOSPI)
let stock_futures = client.derivative()
    .equity_stock_futures_daily()
    .today()
    .fetch()
    .await?;

// 개별주식선물 (KOSDAQ)
let kosdaq_futures = client.derivative()
    .equity_kosdaq_futures_daily()
    .today()
    .fetch()
    .await?;
```

### 옵션
```rust
// 일반 옵션
let options = client.derivative()
    .options_daily()
    .date("20240105")
    .fetch()
    .await?;

// 개별주식옵션 (KOSPI)
let stock_options = client.derivative()
    .equity_stock_options_daily()
    .today()
    .fetch()
    .await?;

// 개별주식옵션 (KOSDAQ)
let kosdaq_options = client.derivative()
    .equity_kosdaq_options_daily()
    .today()
    .fetch()
    .await?;
```

## General API

### 유가
```rust
let oil = client.general()
    .oil_daily()
    .date("20240105")
    .fetch()
    .await?;
```

### 금
```rust
let gold = client.general()
    .gold_daily()
    .today()
    .fetch()
    .await?;
```

### 배출권
```rust
let emissions = client.general()
    .emissions_daily()
    .today()
    .fetch()
    .await?;
```

## ESG API

### SRI 채권 정보
```rust
let sri_bonds = client.esg()
    .sri_bond_info()
    .fetch()
    .await?;
```

## DataFrame 활용

모든 API는 Polars DataFrame을 반환합니다:

```rust
let df = client.stock().stock_daily().today().fetch().await?;

// 컬럼 확인
println!("컬럼: {:?}", df.get_column_names());

// 통계 정보
println!("평균 등락률: {:?}", 
    df.column("등락률")?.mean());

// 필터링
let rising_stocks = df.lazy()
    .filter(col("등락률").gt(lit(0)))
    .collect()?;

// 정렬
let top_gainers = df.lazy()
    .sort("등락률", SortOptions::default().with_order_descending(true))
    .limit(10)
    .collect()?;

// CSV로 저장
df.write_csv("stock_daily.csv")?;
```

## 빌더 패턴

모든 API는 빌더 패턴을 사용합니다:

```rust
// 기본 사용
let data = client.stock().stock_daily().fetch().await?;

// 날짜 지정
let data = client.stock()
    .stock_daily()
    .date("20240105")
    .fetch()
    .await?;

// 오늘 날짜
let data = client.stock()
    .stock_daily()
    .today()
    .fetch()
    .await?;
```

## 비동기 처리

모든 API 호출은 비동기입니다:

```rust
// 동시에 여러 API 호출
let (kospi, kosdaq) = tokio::join!(
    client.stock().stock_daily().today().fetch(),
    client.stock().kosdaq_daily().today().fetch()
);

let kospi_data = kospi?;
let kosdaq_data = kosdaq?;
```

## 오류 처리

상세한 오류 타입을 제공합니다:

```rust
use krx_rs::Error;

match client.stock().stock_daily().fetch().await {
    Ok(data) => println!("성공"),
    Err(e) => match e {
        Error::InvalidInput(msg) => println!("잘못된 입력: {}", msg),
        Error::Network(e) => println!("네트워크 오류: {}", e),
        Error::ApiError { status_code, message } => {
            println!("API 오류 {}: {}", status_code, message)
        }
        Error::RateLimit { retry_after } => {
            println!("요청 제한. {} 초 후 재시도", retry_after)
        }
        Error::Parsing { details, .. } => {
            println!("파싱 오류: {}", details)
        }
    }
}
```