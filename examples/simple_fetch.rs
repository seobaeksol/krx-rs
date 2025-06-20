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
        .kospi_daily()
        .date("20240105")
        .fetch()
        .await?;
    
    // 결과 출력
    println!("\n조회 결과 (상위 10개):");
    println!("{}", df.head(Some(10)));
    
    // 거래대금 상위 5개 종목
    use polars::prelude::*;
    let top_5 = df
        .lazy()
        .sort("거래대금", SortOptions::default().with_descending(true))
        .limit(5)
        .collect()?;
    
    println!("\n💰 거래대금 TOP 5:");
    println!("{}", top_5);
    
    Ok(())
}