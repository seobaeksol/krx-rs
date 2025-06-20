# 예제 모음

실제 사용 시나리오별 krx-rs 활용 예제입니다.

## 1. 일일 상승률 TOP 10 종목 찾기

```rust
use krx_rs::Client;
use polars::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your_auth_key");
    
    // KOSPI 일별 데이터 조회
    let df = client.stock().kospi_daily().today().fetch().await?;
    
    // 상승률 TOP 10
    let top_gainers = df.lazy()
        .filter(col("등락률").gt(lit(0)))
        .sort("등락률", SortOptions::default().with_order_descending(true))
        .limit(10)
        .select([
            col("종목명"),
            col("종가"),
            col("등락률"),
            col("거래량")
        ])
        .collect()?;
    
    println!("오늘의 상승률 TOP 10:");
    println!("{}", top_gainers);
    
    Ok(())
}
```

## 2. 거래량 급증 종목 찾기

```rust
use krx_rs::Client;
use polars::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your_auth_key");
    
    // 어제와 오늘 데이터 조회
    let yesterday = "20240104";
    let today = "20240105";
    
    let df_yesterday = client.stock()
        .kospi_daily()
        .date(yesterday)
        .fetch()
        .await?;
    
    let df_today = client.stock()
        .kospi_daily()
        .date(today)
        .fetch()
        .await?;
    
    // 거래량 비교
    let volume_surge = df_today
        .join(
            &df_yesterday,
            ["종목코드"],
            ["종목코드"],
            JoinArgs::default().with_how(JoinType::Inner)
        )?
        .lazy()
        .with_column(
            (col("거래량") / col("거래량_right")).alias("거래량_비율")
        )
        .filter(col("거래량_비율").gt(lit(3.0)))  // 3배 이상 증가
        .sort("거래량_비율", SortOptions::default().with_order_descending(true))
        .select([
            col("종목명"),
            col("거래량_비율"),
            col("등락률")
        ])
        .collect()?;
    
    println!("거래량 급증 종목 (3배 이상):");
    println!("{}", volume_surge);
    
    Ok(())
}
```

## 3. 섹터별 분석

```rust
use krx_rs::Client;
use polars::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your_auth_key");
    
    let df = client.stock().kospi_daily().today().fetch().await?;
    
    // 섹터별 평균 등락률
    let sector_performance = df.lazy()
        .group_by([col("소속부")])
        .agg([
            col("등락률").mean().alias("평균_등락률"),
            col("종목명").count().alias("종목수"),
            col("거래대금").sum().alias("총_거래대금")
        ])
        .sort("평균_등락률", SortOptions::default().with_order_descending(true))
        .collect()?;
    
    println!("섹터별 성과:");
    println!("{}", sector_performance);
    
    Ok(())
}
```

## 4. ETF와 기초지수 비교

```rust
use krx_rs::Client;
use polars::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your_auth_key");
    
    // ETF 데이터
    let etf_data = client.etp()
        .etf_daily()
        .today()
        .fetch()
        .await?;
    
    // KOSPI 200 추적 ETF 찾기
    let kospi200_etf = etf_data.lazy()
        .filter(col("종목명").str().contains(lit("KOSPI 200")))
        .select([
            col("종목명"),
            col("종가"),
            col("등락률"),
            col("순자산가치(NAV)"),
            col("거래량")
        ])
        .collect()?;
    
    println!("KOSPI 200 추적 ETF:");
    println!("{}", kospi200_etf);
    
    Ok(())
}
```

## 5. 다중 시장 통합 분석

```rust
use krx_rs::Client;
use polars::prelude::*;
use futures::future::try_join3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your_auth_key");
    
    // 동시에 여러 시장 데이터 조회
    let (kospi, kosdaq, konex) = try_join3(
        client.stock().kospi_daily().today().fetch(),
        client.stock().kosdaq_daily().today().fetch(),
        client.stock().konex_daily().today().fetch()
    ).await?;
    
    // 각 시장 요약
    println!("KOSPI - 종목수: {}, 평균 등락률: {:.2}%", 
        kospi.height(),
        kospi.column("등락률")?.mean().unwrap_or(0.0)
    );
    
    println!("KOSDAQ - 종목수: {}, 평균 등락률: {:.2}%", 
        kosdaq.height(),
        kosdaq.column("등락률")?.mean().unwrap_or(0.0)
    );
    
    println!("KONEX - 종목수: {}, 평균 등락률: {:.2}%", 
        konex.height(),
        konex.column("등락률")?.mean().unwrap_or(0.0)
    );
    
    Ok(())
}
```

## 6. 시계열 데이터 수집

```rust
use krx_rs::Client;
use polars::prelude::*;
use chrono::{Local, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your_auth_key");
    
    // 최근 5일간 데이터 수집
    let mut all_data = Vec::new();
    let today = Local::now();
    
    for i in 0..5 {
        let date = today - Duration::days(i);
        let date_str = date.format("%Y%m%d").to_string();
        
        match client.stock().kospi_daily().date(&date_str).fetch().await {
            Ok(df) => {
                println!("{} 데이터 수집 완료", date_str);
                all_data.push(df);
            }
            Err(e) => {
                eprintln!("{} 데이터 수집 실패: {}", date_str, e);
            }
        }
    }
    
    // 데이터 결합
    if !all_data.is_empty() {
        let combined = concat(&all_data, UnionArgs::default())?;
        
        // CSV로 저장
        combined.write_csv("kospi_5days.csv")?;
        println!("데이터를 kospi_5days.csv에 저장했습니다.");
    }
    
    Ok(())
}
```

## 7. 실시간 모니터링 (주기적 조회)

```rust
use krx_rs::Client;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your_auth_key");
    let mut interval = interval(Duration::from_secs(300)); // 5분마다
    
    loop {
        interval.tick().await;
        
        match client.stock().kospi_daily().today().fetch().await {
            Ok(df) => {
                // 특정 종목 모니터링
                let samsung = df.lazy()
                    .filter(col("종목명").eq(lit("삼성전자")))
                    .collect()?;
                
                if samsung.height() > 0 {
                    println!("[{}] 삼성전자: {}", 
                        chrono::Local::now().format("%H:%M:%S"),
                        samsung
                    );
                }
            }
            Err(e) => eprintln!("조회 실패: {}", e),
        }
    }
}
```

## 8. 채권 수익률 곡선 분석

```rust
use krx_rs::Client;
use polars::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your_auth_key");
    
    let bonds = client.bond().kts_daily().today().fetch().await?;
    
    // 만기별 수익률 정렬
    let yield_curve = bonds.lazy()
        .filter(col("지표종목구분").eq(lit("지표")))
        .sort("잔존만기", SortOptions::default())
        .select([
            col("종목명"),
            col("잔존만기"),
            col("종가수익률")
        ])
        .collect()?;
    
    println!("국채 수익률 곡선:");
    println!("{}", yield_curve);
    
    Ok(())
}
```

## 9. 오류 처리와 재시도

```rust
use krx_rs::{Client, Error};
use tokio::time::{sleep, Duration};

async fn fetch_with_retry(
    client: &Client,
    date: &str,
    max_retries: u32
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut retries = 0;
    
    loop {
        match client.stock().kospi_daily().date(date).fetch().await {
            Ok(df) => return Ok(df),
            Err(Error::RateLimit { retry_after }) => {
                println!("Rate limit. {} 초 대기...", retry_after);
                sleep(Duration::from_secs(retry_after)).await;
            }
            Err(e) if retries < max_retries => {
                retries += 1;
                println!("시도 {}/{} 실패: {}", retries, max_retries, e);
                sleep(Duration::from_secs(2_u64.pow(retries))).await;
            }
            Err(e) => return Err(e.into()),
        }
    }
}
```