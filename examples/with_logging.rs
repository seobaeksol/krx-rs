use krx_rs::{Client, Error, LoggingConfig};
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 로깅 설정
    let logging_config = LoggingConfig {
        level: "debug".to_string(),
        json_format: false,
        filter_sensitive: true,
        file_path: None,
    };

    // 환경 변수에서 인증키 읽기
    let auth_key = env::var("KRX_API_KEY").expect("KRX_API_KEY 환경 변수를 설정해주세요");

    // 클라이언트 생성 (로깅 포함)
    let client = Client::builder()
        .auth_key(auth_key)
        .logging(logging_config)
        .build()?;

    info!("Starting KRX data collection");

    // KRX 지수 데이터 조회
    println!("📈 KRX 지수 데이터 조회 중...");

    let index_data = client.index().krx_daily().date("20240105").fetch().await?;

    info!(
        record_count = index_data.height(),
        "Index data collection completed"
    );

    println!("\n지수 데이터 (상위 5개):");
    println!("{}", index_data.head(Some(5)));

    // 주식 데이터 조회
    println!("\n📊 주식 데이터 조회 중...");

    let stock_data = client
        .stock()
        .stock_daily()
        .date("20240105")
        .fetch()
        .await?;

    info!(
        record_count = stock_data.height(),
        "Stock data collection completed"
    );

    println!("\n주식 데이터 (상위 3개):");
    println!("{}", stock_data.head(Some(3)));

    Ok(())
}
