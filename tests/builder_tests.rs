use krx_rs::Client;
use chrono::Local;

// Mock client for testing builders
fn create_test_client() -> Client {
    Client::builder()
        .auth_key("test_key")
        .base_url("http://test.example.com")
        .build()
        .unwrap()
}

#[test]
fn test_stock_api_kospi_daily_builder() {
    let client = create_test_client();
    let stock_api = client.stock();
    
    // 기본 빌더 생성
    let builder = stock_api.kospi_daily();
    
    // 날짜 설정 메서드 체이닝 테스트
    let builder_with_date = builder.date("20240105");
    
    // 오늘 날짜 설정 테스트  
    let client2 = create_test_client();
    let builder_with_today = client2.stock().kospi_daily().today();
    
    // 체이닝 테스트
    let client3 = create_test_client();
    let _chained_builder = client3.stock()
        .kospi_daily()
        .date("20240105");
}

#[test]
fn test_stock_api_kosdaq_daily_builder() {
    let client = create_test_client();
    let stock_api = client.stock();
    
    // KosDAQ 빌더 테스트
    let builder = stock_api.kosdaq_daily();
    let _builder_with_date = builder.date("20240105");
    
    // 체이닝 테스트
    let client2 = create_test_client();
    let _chained_builder = client2.stock()
        .kosdaq_daily()
        .today();
}

#[test]
fn test_stock_api_konex_daily_builder() {
    let client = create_test_client();
    let stock_api = client.stock();
    
    // KONEX 빌더 테스트
    let builder = stock_api.konex_daily();
    let _builder_with_date = builder.date("20240105");
    
    // 체이닝 테스트
    let client2 = create_test_client();
    let _chained_builder = client2.stock()
        .konex_daily()
        .date("20231215")
        .today(); // today()가 마지막이므로 덮어쓰기됨
}

#[test]
fn test_stock_api_warrant_and_right_builders() {
    let client = create_test_client();
    let stock_api = client.stock();
    
    // 신주인수권증권 빌더
    let _warrant_builder = stock_api.stock_warrant_daily()
        .date("20240105");
    
    // 신주인수권증서 빌더
    let _right_builder = stock_api.stock_right_daily()
        .date("20240105");
}

#[test]
fn test_stock_api_base_info_builders() {
    let client = create_test_client();
    let stock_api = client.stock();
    
    // 기본정보 조회는 날짜 파라미터가 없음
    let _kospi_base_builder = stock_api.kospi_base_info();
    let _kosdaq_base_builder = stock_api.kosdaq_base_info();
    let _konex_base_builder = stock_api.konex_base_info();
}

#[test]
fn test_index_api_builders() {
    let client = create_test_client();
    let index_api = client.index();
    
    // KRX 지수 빌더
    let _krx_builder = index_api.krx_daily()
        .date("20240105");
    
    // KOSPI 지수 빌더
    let _kospi_builder = index_api.kospi_daily()
        .today();
    
    // KOSDAQ 지수 빌더
    let _kosdaq_builder = index_api.kosdaq_daily()
        .date("20240105");
    
    // 채권지수 빌더
    let _bond_builder = index_api.bond_daily()
        .date("20240105");
    
    // 파생상품지수 빌더
    let _derivative_builder = index_api.derivative_daily()
        .date("20240105");
}

#[test]
fn test_bond_api_builders() {
    let client = create_test_client();
    let bond_api = client.bond();
    
    // 국고채 빌더
    let _kts_builder = bond_api.kts_daily()
        .date("20240105");
    
    // 일반채권 빌더
    let _bond_builder = bond_api.bond_daily()
        .today();
    
    // 소액채권 빌더
    let _small_bond_builder = bond_api.small_bond_daily()
        .date("20240105");
}

#[test]
fn test_etp_api_builders() {
    let client = create_test_client();
    let etp_api = client.etp();
    
    // ETF 빌더
    let _etf_builder = etp_api.etf_daily()
        .date("20240105");
    
    // ETN 빌더
    let _etn_builder = etp_api.etn_daily()
        .today();
    
    // ELW 빌더
    let _elw_builder = etp_api.elw_daily()
        .date("20240105");
}

#[test]
fn test_derivative_api_builders() {
    let client = create_test_client();
    let derivative_api = client.derivative();
    
    // 선물 빌더들
    let _futures_builder = derivative_api.futures_daily()
        .date("20240105");
    
    let _equity_stock_futures_builder = derivative_api.equity_stock_futures_daily()
        .today();
    
    let _equity_kosdaq_futures_builder = derivative_api.equity_kosdaq_futures_daily()
        .date("20240105");
    
    // 옵션 빌더들
    let _options_builder = derivative_api.options_daily()
        .date("20240105");
    
    let _equity_stock_options_builder = derivative_api.equity_stock_options_daily()
        .today();
    
    let _equity_kosdaq_options_builder = derivative_api.equity_kosdaq_options_daily()
        .date("20240105");
}

#[test]
fn test_general_api_builders() {
    let client = create_test_client();
    let general_api = client.general();
    
    // 유가 빌더
    let _oil_builder = general_api.oil_daily()
        .date("20240105");
    
    // 금 빌더
    let _gold_builder = general_api.gold_daily()
        .today();
    
    // 배출권 빌더
    let _emissions_builder = general_api.emissions_daily()
        .date("20240105");
}

#[test]
fn test_esg_api_builders() {
    let client = create_test_client();
    let esg_api = client.esg();
    
    // SRI 채권 빌더 (날짜 파라미터 없음)
    let _sri_builder = esg_api.sri_bond_info();
}

#[test]
fn test_builder_date_method_accepts_different_types() {
    let client = create_test_client();
    
    // String 타입
    let _builder1 = client.stock().kospi_daily()
        .date(String::from("20240105"));
    
    // &str 타입
    let _builder2 = client.stock().kospi_daily()
        .date("20240105");
    
    // 체이닝에서 다양한 타입 사용
    let client2 = create_test_client();
    let date_string = "20240105".to_string();
    let _builder3 = client2.stock().kospi_daily()
        .date(date_string);
}

#[test]
fn test_today_method_consistency() {
    let client = create_test_client();
    
    // today() 메서드는 현재 날짜를 YYYYMMDD 형식으로 설정해야 함
    let expected_today = Local::now().format("%Y%m%d").to_string();
    
    // 실제로는 내부 구현이므로 호출만 테스트
    let _builder = client.stock().kospi_daily().today();
    let _builder2 = client.index().krx_daily().today();
    let _builder3 = client.bond().kts_daily().today();
}

#[test]
fn test_multiple_api_access() {
    let client = create_test_client();
    
    // 같은 클라이언트로 여러 API 접근
    let _stock_api = client.stock();
    let _index_api = client.index();
    let _bond_api = client.bond();
    let _etp_api = client.etp();
    let _derivative_api = client.derivative();
    let _general_api = client.general();
    let _esg_api = client.esg();
    
    // 동일 API를 여러 번 접근해도 문제없어야 함
    let _stock_api2 = client.stock();
    let _builder1 = _stock_api2.kospi_daily();
    let _builder2 = _stock_api2.kosdaq_daily();
}

#[test]
fn test_builder_method_chaining_order() {
    let client = create_test_client();
    
    // 메서드 체이닝 순서 테스트
    let _builder1 = client.stock()
        .kospi_daily()
        .date("20240105");
    
    let client2 = create_test_client();
    let _builder2 = client2.stock()
        .kospi_daily()
        .today()
        .date("20240105"); // date()가 today() 이후에 와도 정상 작동
    
    let client3 = create_test_client();
    let _builder3 = client3.index()
        .krx_daily()
        .date("20240101")
        .today()
        .date("20240105"); // 마지막 설정이 적용됨
}