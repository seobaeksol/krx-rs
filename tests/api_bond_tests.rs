use krx_rs::{Client, error::Error};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

/// Helper function to create a mock server with predefined response
async fn setup_bond_test(endpoint: &str, date: &str, response_body: &str, status: u16) -> (Client, MockServer) {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path(endpoint))
        .and(query_param("basDd", date))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(status).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    (client, mock_server)
}

// KTS Daily Tests
#[tokio::test]
async fn test_kts_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20240105",
            "ISU_CD": "KR103501GC38",
            "ISU_NM": "국고03500-2603(23-1)",
            "MKT_NM": "KTS",
            "GOVBND_ISU_TP_NM": "경과",
            "BND_EXP_TP_NM": "장기",
            "CLSPRC": "10000.0",
            "CLSPRC_YD": "3.5",
            "OPNPRC": "9990.0",
            "OPNPRC_YD": "3.51",
            "HGPRC": "10010.0",
            "HGPRC_YD": "3.49",
            "LWPRC": "9980.0",
            "LWPRC_YD": "3.52",
            "CMPPREVDD_PRC": "10.0",
            "ACC_TRDVOL": "1000",
            "ACC_TRDVAL": "10000000"
        }]
    }"#;

    let (client, _server) = setup_bond_test("/bon/kts_bydd_trd", "20240105", response_body, 200).await;
    
    let result = client.bond().kts_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 17));
    assert_eq!(df.column("종목코드").unwrap().str().unwrap().get(0), Some("KR103501GC38"));
}

#[tokio::test]
async fn test_kts_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20240105",
            "ISU_CD": "KR103501GC38",
            "ISU_NM": "국고03500-2603(23-1)",
            "MKT_NM": "KTS",
            "GOVBND_ISU_TP_NM": "경과",
            "BND_EXP_TP_NM": "장기",
            "CLSPRC": "10000.0",
            "CLSPRC_YD": "3.5",
            "OPNPRC": "9990.0",
            "OPNPRC_YD": "3.51",
            "HGPRC": "10010.0",
            "HGPRC_YD": "3.49",
            "LWPRC": "9980.0",
            "LWPRC_YD": "3.52",
            "CMPPREVDD_PRC": "10.0",
            "ACC_TRDVOL": "1000",
            "ACC_TRDVAL": "10000000"
        }]
    }"#;

    let mock_server = MockServer::start().await;
    
    // We need to check what date the latest() method generates
    Mock::given(method("GET"))
        .and(path("/bon/kts_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.bond().kts_daily().latest().fetch().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_kts_daily_empty_response() {
    let response_body = r#"{"OutBlock_1": []}"#;
    let (client, _server) = setup_bond_test("/bon/kts_bydd_trd", "20240105", response_body, 200).await;
    
    let result = client.bond().kts_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (0, 0)); // Empty DataFrame
}

#[tokio::test]
async fn test_kts_daily_missing_date() {
    let client = Client::new("test_key");
    let result = client.bond().kts_daily().fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[tokio::test]
async fn test_kts_daily_api_error() {
    let (client, _server) = setup_bond_test("/bon/kts_bydd_trd", "20240105", "Not Found", 404).await;
    
    let result = client.bond().kts_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::ApiError { status_code: 404, .. })));
}

// Bond Daily Tests
#[tokio::test]
async fn test_bond_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20240105",
            "ISU_CD": "KR203801GC59",
            "ISU_NM": "국민주택1종(03-5)",
            "MKT_NM": "일반",
            "CLSPRC": "10050.0",
            "CLSPRC_YD": "4.1",
            "OPNPRC": "10045.0",
            "OPNPRC_YD": "4.11",
            "HGPRC": "10055.0",
            "HGPRC_YD": "4.09",
            "LWPRC": "10040.0",
            "LWPRC_YD": "4.12",
            "CMPPREVDD_PRC": "5",
            "ACC_TRDVOL": "500",
            "ACC_TRDVAL": "5025000"
        }]
    }"#;

    let (client, _server) = setup_bond_test("/bon/bnd_bydd_trd", "20240105", response_body, 200).await;
    
    let result = client.bond().bond_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 15));
    assert_eq!(df.column("종목코드").unwrap().str().unwrap().get(0), Some("KR203801GC59"));
}

#[tokio::test]
async fn test_bond_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20240105",
            "ISU_CD": "KR203801GC59",
            "ISU_NM": "국민주택1종(03-5)",
            "MKT_NM": "일반",
            "CLSPRC": "10050.0",
            "CLSPRC_YD": "4.1",
            "OPNPRC": "10045.0",
            "OPNPRC_YD": "4.11",
            "HGPRC": "10055.0",
            "HGPRC_YD": "4.09",
            "LWPRC": "10040.0",
            "LWPRC_YD": "4.12",
            "CMPPREVDD_PRC": "5",
            "ACC_TRDVOL": "500",
            "ACC_TRDVAL": "5025000"
        }]
    }"#;

    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/bon/bnd_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.bond().bond_daily().latest().fetch().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_daily_empty_response() {
    let response_body = r#"{"OutBlock_1": []}"#;
    let (client, _server) = setup_bond_test("/bon/bnd_bydd_trd", "20240105", response_body, 200).await;
    
    let result = client.bond().bond_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (0, 0));
}

#[tokio::test]
async fn test_bond_daily_missing_date() {
    let client = Client::new("test_key");
    let result = client.bond().bond_daily().fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[tokio::test]
async fn test_bond_daily_parsing_error() {
    let response_body = "Invalid JSON";
    let (client, _server) = setup_bond_test("/bon/bnd_bydd_trd", "20240105", response_body, 200).await;
    
    let result = client.bond().bond_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Parsing { .. })));
}

// Small Bond Daily Tests
#[tokio::test]
async fn test_small_bond_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20240105",
            "ISU_CD": "KR203801GC59",
            "ISU_NM": "소액채권",
            "MKT_NM": "소액",
            "CLSPRC": "1000.0",
            "CLSPRC_YD": "3.8",
            "OPNPRC": "999.0",
            "OPNPRC_YD": "3.81",
            "HGPRC": "1001.0",
            "HGPRC_YD": "3.79",
            "LWPRC": "998.0",
            "LWPRC_YD": "3.82",
            "CMPPREVDD_PRC": "1",
            "ACC_TRDVOL": "100",
            "ACC_TRDVAL": "100000"
        }]
    }"#;

    let (client, _server) = setup_bond_test("/bon/smb_bydd_trd", "20240105", response_body, 200).await;
    
    let result = client.bond().small_bond_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 15));
    assert_eq!(df.column("종목명").unwrap().str().unwrap().get(0), Some("소액채권"));
}

#[tokio::test]
async fn test_small_bond_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20240105",
            "ISU_CD": "KR203801GC59",
            "ISU_NM": "소액채권",
            "MKT_NM": "소액",
            "CLSPRC": "1000.0",
            "CLSPRC_YD": "3.8",
            "OPNPRC": "999.0",
            "OPNPRC_YD": "3.81",
            "HGPRC": "1001.0",
            "HGPRC_YD": "3.79",
            "LWPRC": "998.0",
            "LWPRC_YD": "3.82",
            "CMPPREVDD_PRC": "1",
            "ACC_TRDVOL": "100",
            "ACC_TRDVAL": "100000"
        }]
    }"#;

    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/bon/smb_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.bond().small_bond_daily().latest().fetch().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_small_bond_daily_empty_response() {
    let response_body = r#"{"OutBlock_1": []}"#;
    let (client, _server) = setup_bond_test("/bon/smb_bydd_trd", "20240105", response_body, 200).await;
    
    let result = client.bond().small_bond_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (0, 0));
}

#[tokio::test]
async fn test_small_bond_daily_missing_date() {
    let client = Client::new("test_key");
    let result = client.bond().small_bond_daily().fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[tokio::test]
async fn test_small_bond_daily_network_error() {
    let client = Client::builder()
        .auth_key("test_key")
        .base_url("http://localhost:12345") // Non-existent server
        .build()
        .unwrap();
    
    let result = client.bond().small_bond_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Network(_))));
}

// Test edge cases for date validation
#[tokio::test]
async fn test_invalid_date_format() {
    let client = Client::new("test_key");
    
    // Test with invalid date format
    let result = client.bond().kts_daily().date("2024-01-05").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
    
    let result = client.bond().bond_daily().date("240105").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
    
    let result = client.bond().small_bond_daily().date("invalid").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

// Test multiple records response
#[tokio::test]
async fn test_kts_daily_multiple_records() {
    let response_body = r#"{
        "OutBlock_1": [
            {
                "BAS_DD": "20240105",
                "ISU_CD": "KR103501GC38",
                "ISU_NM": "국고03500-2603(23-1)",
                "MKT_NM": "KTS",
                "GOVBND_ISU_TP_NM": "경과",
                "BND_EXP_TP_NM": "장기",
                "CLSPRC": "10000.0",
                "CLSPRC_YD": "3.5",
                "OPNPRC": "9990.0",
                "OPNPRC_YD": "3.51",
                "HGPRC": "10010.0",
                "HGPRC_YD": "3.49",
                "LWPRC": "9980.0",
                "LWPRC_YD": "3.52",
                "CMPPREVDD_PRC": "10.0",
                "ACC_TRDVOL": "1000",
                "ACC_TRDVAL": "10000000"
            },
            {
                "BAS_DD": "20240105",
                "ISU_CD": "KR103502GD34",
                "ISU_NM": "국고03500-2803(24-2)",
                "MKT_NM": "KTS",
                "GOVBND_ISU_TP_NM": "신규",
                "BND_EXP_TP_NM": "장기",
                "CLSPRC": "10100.0",
                "CLSPRC_YD": "3.4",
                "OPNPRC": "10090.0",
                "OPNPRC_YD": "3.41",
                "HGPRC": "10110.0",
                "HGPRC_YD": "3.39",
                "LWPRC": "10080.0",
                "LWPRC_YD": "3.42",
                "CMPPREVDD_PRC": "15.0",
                "ACC_TRDVOL": "2000",
                "ACC_TRDVAL": "20200000"
            }
        ]
    }"#;

    let (client, _server) = setup_bond_test("/bon/kts_bydd_trd", "20240105", response_body, 200).await;
    
    let result = client.bond().kts_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (2, 17));
}

// Test rate limiting
#[tokio::test]
async fn test_bond_rate_limit() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/bon/kts_bydd_trd"))
        .and(query_param("basDd", "20240105"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "60"))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.bond().kts_daily().date("20240105").fetch().await;
    
    match result {
        Err(Error::RateLimit { retry_after }) => {
            assert_eq!(retry_after, 60);
        }
        _ => panic!("Expected RateLimit error"),
    }
}

// Test parsing error in parse_kts_daily
#[tokio::test]
async fn test_kts_daily_parse_error() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "invalid-date",
            "ISU_CD": "KR103501GC38",
            "ISU_NM": "국고03500-2603(23-1)",
            "MKT_NM": "KTS",
            "GOVBND_ISU_TP_NM": "경과",
            "BND_EXP_TP_NM": "장기",
            "CLSPRC": "10000.0",
            "CLSPRC_YD": "3.5",
            "OPNPRC": "9990.0",
            "OPNPRC_YD": "3.51",
            "HGPRC": "10010.0",
            "HGPRC_YD": "3.49",
            "LWPRC": "9980.0",
            "LWPRC_YD": "3.52",
            "CMPPREVDD_PRC": "10.0",
            "ACC_TRDVOL": "1000",
            "ACC_TRDVAL": "10000000"
        }]
    }"#;

    let (client, _server) = setup_bond_test("/bon/kts_bydd_trd", "20240105", response_body, 200).await;
    
    let result = client.bond().kts_daily().date("20240105").fetch().await;
    assert!(result.is_err());
}

// Test network timeout error
#[tokio::test]
async fn test_bond_network_timeout() {
    let mock_server = MockServer::start().await;
    
    // Create a mock that delays response to trigger timeout
    Mock::given(method("GET"))
        .and(path("/bon/kts_bydd_trd"))
        .and(query_param("basDd", "20240105"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string(r#"{"OutBlock_1": []}"#)
            .set_delay(std::time::Duration::from_secs(5)))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .timeout(std::time::Duration::from_millis(100)) // Very short timeout
        .build()
        .unwrap();

    let result = client.bond().kts_daily().date("20240105").fetch().await;
    assert!(result.is_err());
}

// Test with null/missing values in response
#[tokio::test]
async fn test_bond_daily_with_null_values() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20240105",
            "ISU_CD": "KR203801GC59",
            "ISU_NM": "국민주택1종(03-5)",
            "MKT_NM": "일반",
            "CLSPRC": "",
            "CLSPRC_YD": "",
            "OPNPRC": "10045.0",
            "OPNPRC_YD": "4.11",
            "HGPRC": "",
            "HGPRC_YD": "",
            "LWPRC": "10040.0",
            "LWPRC_YD": "4.12",
            "CMPPREVDD_PRC": "5",
            "ACC_TRDVOL": "",
            "ACC_TRDVAL": ""
        }]
    }"#;

    let (client, _server) = setup_bond_test("/bon/bnd_bydd_trd", "20240105", response_body, 200).await;
    
    let result = client.bond().bond_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 15));
    
    // Check that null values are properly handled
    assert!(df.column("종가").unwrap().f64().unwrap().get(0).is_none());
    assert!(df.column("거래량").unwrap().i64().unwrap().get(0).is_none());
}