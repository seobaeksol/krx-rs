use krx_rs::{Client, LoggingConfig, Error};
use tracing::info;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 설정
    let logging_config = LoggingConfig {
        level: "debug".to_string(),
        json_format: false,
        filter_sensitive: true,
        file_path: None,
    };
    
    // 클라이언트 생성 (테스트용 - 실제 API 호출하지 않음)
    let client = Client::builder()
        .auth_key("test_key")
        .logging(logging_config)
        .build()?;
    
    info!("Testing type conversions with sample data");
    
    // 샘플 JSON 파일 읽기
    let sample_stock_data = fs::read_to_string("docs/KRX_API_Spec/samples/stk_bydd_trd_20240105.json")?;
    let sample_index_data = fs::read_to_string("docs/KRX_API_Spec/samples/krx_dd_trd_20240105.json")?;
    
    // 타입 변환 테스트
    println!("🧪 Testing KRX type conversions...\n");
    
    // 주식 데이터 파싱 테스트
    println!("📊 Stock data parsing:");
    match serde_json::from_str::<krx_rs::data::ApiResponse<krx_rs::data::stock::KospiDailyRecord>>(&sample_stock_data) {
        Ok(response) => {
            println!("✅ Successfully parsed {} stock records", response.data.len());
            
            if let Some(first_record) = response.data.first() {
                println!("📋 First record:");
                println!("  - 날짜: {}", first_record.base_date);
                println!("  - 종목명: {}", first_record.issue_name);
                println!("  - 종가: {:?}", first_record.close_price);
                println!("  - 등락률: {:?}%", first_record.fluctuation_rate);
                println!("  - 거래량: {:?}", first_record.trading_volume);
            }
            
            // DataFrame 변환 테스트
            match krx_rs::data::stock::parse_kospi_daily(response) {
                Ok(df) => {
                    println!("✅ DataFrame conversion successful!");
                    println!("📏 DataFrame shape: {} rows × {} columns", df.height(), df.width());
                    println!("📑 Columns: {:?}", df.get_column_names());
                    println!("🔍 First 3 rows:");
                    println!("{}", df.head(Some(3)));
                }
                Err(e) => println!("❌ DataFrame conversion failed: {}", e),
            }
        }
        Err(e) => println!("❌ Stock data parsing failed: {}", e),
    }
    
    println!("\n{}\n", "=".repeat(50));
    
    // 지수 데이터 파싱 테스트
    println!("📈 Index data parsing:");
    match serde_json::from_str::<krx_rs::data::ApiResponse<krx_rs::data::index::KrxIndexDailyRecord>>(&sample_index_data) {
        Ok(response) => {
            println!("✅ Successfully parsed {} index records", response.data.len());
            
            if let Some(first_record) = response.data.first() {
                println!("📋 First record:");
                println!("  - 날짜: {}", first_record.base_date);
                println!("  - 지수명: {}", first_record.index_name);
                println!("  - 종가: {:?}", first_record.close_price);
                println!("  - 등락률: {:?}%", first_record.fluctuation_rate);
                println!("  - 거래량: {:?}", first_record.trading_volume);
            }
            
            // DataFrame 변환 테스트
            match krx_rs::data::index::parse_krx_index_daily(response) {
                Ok(df) => {
                    println!("✅ DataFrame conversion successful!");
                    println!("📏 DataFrame shape: {} rows × {} columns", df.height(), df.width());
                    println!("📑 Columns: {:?}", df.get_column_names());
                    println!("🔍 First 3 rows:");
                    println!("{}", df.head(Some(3)));
                }
                Err(e) => println!("❌ DataFrame conversion failed: {}", e),
            }
        }
        Err(e) => println!("❌ Index data parsing failed: {}", e),
    }
    
    Ok(())
}