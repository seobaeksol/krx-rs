use krx_rs::{Client, error::Error};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

/// Helper function to create a mock server with predefined response
async fn setup_esg_test(endpoint: &str, date: &str, response_body: &str, status: u16) -> (Client, MockServer) {
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

// SRI Bond Info Tests
#[tokio::test]
async fn test_sri_bond_info_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20240105",
            "ISUR_NM": "한국정부",
            "ISU_CD": "KR103502GD29",
            "SRI_BND_TP_NM": "사회적 채권",
            "ISU_NM": "국고04250-2912(22-7)",
            "LIST_DD": "20220531",
            "ISU_DD": "20220531",
            "REDMPT_DD": "20291205",
            "ISU_RT": "4.25",
            "ISU_AMT": "1000000000000",
            "LIST_AMT": "1000000000000",
            "BND_TP_NM": "국고채"
        }]
    }"#;

    let (client, _server) = setup_esg_test("/esg/sri_bond_info", "20240105", response_body, 200).await;
    
    let result = client.esg().sri_bond_info().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 12));
    assert_eq!(df.column("표준코드").unwrap().str().unwrap().get(0), Some("KR103502GD29"));
    assert_eq!(df.column("종목명").unwrap().str().unwrap().get(0), Some("국고04250-2912(22-7)"));
}

#[tokio::test]
async fn test_sri_bond_info_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20240105",
            "ISUR_NM": "한국정부",
            "ISU_CD": "KR103502GD29",
            "SRI_BND_TP_NM": "사회적 채권",
            "ISU_NM": "국고04250-2912(22-7)",
            "LIST_DD": "20220531",
            "ISU_DD": "20220531",
            "REDMPT_DD": "20291205",
            "ISU_RT": "4.25",
            "ISU_AMT": "1000000000000",
            "LIST_AMT": "1000000000000",
            "BND_TP_NM": "국고채"
        }]
    }"#;

    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/esg/sri_bond_info"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.esg().sri_bond_info().latest().fetch().await;
    assert!(result.is_ok());
}


// Error Handling Tests
#[tokio::test]
async fn test_esg_api_error() {
    let (client, _server) = setup_esg_test("/esg/sri_bond_info", "20240105", "Not Found", 404).await;
    
    let result = client.esg().sri_bond_info().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::ApiError { status_code: 404, .. })));
}

#[tokio::test]
async fn test_esg_parsing_error() {
    let response_body = "Invalid JSON";
    let (client, _server) = setup_esg_test("/esg/sri_bond_info", "20240105", response_body, 200).await;
    
    let result = client.esg().sri_bond_info().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Parsing { .. })));
}

#[tokio::test]
async fn test_esg_missing_date() {
    let client = Client::new("test_key");
    let result = client.esg().sri_bond_info().fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[tokio::test]
async fn test_esg_empty_response() {
    let response_body = r#"{"OutBlock_1": []}"#;
    let (client, _server) = setup_esg_test("/esg/sri_bond_info", "20240105", response_body, 200).await;
    
    let result = client.esg().sri_bond_info().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (0, 0));
}

#[tokio::test]
async fn test_esg_with_null_values() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20240105",
            "ISUR_NM": "한국정부",
            "ISU_CD": "KR103502GD29",
            "SRI_BND_TP_NM": "사회적 채권",
            "ISU_NM": "국고04250-2912(22-7)",
            "LIST_DD": "",
            "ISU_DD": "",
            "REDMPT_DD": "",
            "ISU_RT": "",
            "ISU_AMT": "",
            "LIST_AMT": "",
            "BND_TP_NM": "국고채"
        }]
    }"#;

    let (client, _server) = setup_esg_test("/esg/sri_bond_info", "20240105", response_body, 200).await;
    
    let result = client.esg().sri_bond_info().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 12));
    
    // Check that null values are properly handled
    assert!(df.column("표면이자율").unwrap().f64().unwrap().get(0).is_none());
    assert!(df.column("발행금액").unwrap().i64().unwrap().get(0).is_none());
}

#[tokio::test]
async fn test_esg_network_error() {
    let client = Client::builder()
        .auth_key("test_key")
        .base_url("http://localhost:12345")
        .build()
        .unwrap();
    
    let result = client.esg().sri_bond_info().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Network(_))));
}

#[tokio::test]
async fn test_esg_invalid_date_format() {
    let client = Client::new("test_key");
    
    let result = client.esg().sri_bond_info().date("2024-01-05").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
    
    // Only test sri_bond_info as k_esg_index_daily doesn't exist yet
    let result = client.esg().sri_bond_info().date("240105").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
    
    let result = client.esg().sri_bond_info().date("invalid").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[tokio::test]
async fn test_esg_rate_limit() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/esg/sri_bond_info"))
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

    let result = client.esg().sri_bond_info().date("20240105").fetch().await;
    
    match result {
        Err(Error::RateLimit { retry_after }) => {
            assert_eq!(retry_after, 60);
        }
        _ => panic!("Expected RateLimit error"),
    }
}

// Test multiple records for SRI Bond
#[tokio::test]
async fn test_sri_bond_multiple_records() {
    let response_body = r#"{
        "OutBlock_1": [
            {
                "BAS_DD": "20240105",
                "ISUR_NM": "한국정부",
                "ISU_CD": "KR103502GD29",
                "SRI_BND_TP_NM": "사회적 채권",
                "ISU_NM": "국고04250-2912(22-7)",
                "LIST_DD": "20220531",
                "ISU_DD": "20220531",
                "REDMPT_DD": "20291205",
                "ISU_RT": "4.25",
                "ISU_AMT": "1000000000000",
                "LIST_AMT": "1000000000000",
                "BND_TP_NM": "국고채"
            },
            {
                "BAS_DD": "20240105",
                "ISUR_NM": "한국정부",
                "ISU_CD": "KR103503GE28",
                "SRI_BND_TP_NM": "환경 채권",
                "ISU_NM": "국고03500-2812(23-5)",
                "LIST_DD": "20230315",
                "ISU_DD": "20230315",
                "REDMPT_DD": "20281205",
                "ISU_RT": "3.50",
                "ISU_AMT": "800000000000",
                "LIST_AMT": "800000000000",
                "BND_TP_NM": "국고채"
            }
        ]
    }"#;

    let (client, _server) = setup_esg_test("/esg/sri_bond_info", "20240105", response_body, 200).await;
    
    let result = client.esg().sri_bond_info().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (2, 12));
    assert_eq!(df.column("표준코드").unwrap().str().unwrap().get(0), Some("KR103502GD29"));
    assert_eq!(df.column("표준코드").unwrap().str().unwrap().get(1), Some("KR103503GE28"));
    assert_eq!(df.column("채권종류").unwrap().str().unwrap().get(0), Some("사회적 채권"));
    assert_eq!(df.column("채권종류").unwrap().str().unwrap().get(1), Some("환경 채권"));
}


// Test fluctuation type handling for ESG bonds
#[tokio::test]
async fn test_esg_bond_fluctuation_types() {
    let response_body = r#"{
        "OutBlock_1": [
            {
                "BAS_DD": "20240105",
                "ISUR_NM": "테스트기관1",
                "ISU_CD": "TEST01",
                "SRI_BND_TP_NM": "사회적 채권",
                "ISU_NM": "사회적 ESG 채권",
                "LIST_DD": "20220101",
                "ISU_DD": "20220101",
                "REDMPT_DD": "20270101",
                "ISU_RT": "4.0",
                "ISU_AMT": "1000000000",
                "LIST_AMT": "1000000000",
                "BND_TP_NM": "회사채"
            },
            {
                "BAS_DD": "20240105",
                "ISUR_NM": "테스트기관2",
                "ISU_CD": "TEST02",
                "SRI_BND_TP_NM": "환경 채권",
                "ISU_NM": "환경 ESG 채권",
                "LIST_DD": "20220101",
                "ISU_DD": "20220101",
                "REDMPT_DD": "20270101",
                "ISU_RT": "3.5",
                "ISU_AMT": "800000000",
                "LIST_AMT": "800000000",
                "BND_TP_NM": "회사채"
            },
            {
                "BAS_DD": "20240105",
                "ISUR_NM": "테스트기관3",
                "ISU_CD": "TEST03",
                "SRI_BND_TP_NM": "지배구조 채권",
                "ISU_NM": "지배구조 ESG 채권",
                "LIST_DD": "20220101",
                "ISU_DD": "20220101",
                "REDMPT_DD": "20270101",
                "ISU_RT": "3.8",
                "ISU_AMT": "900000000",
                "LIST_AMT": "900000000",
                "BND_TP_NM": "회사채"
            }
        ]
    }"#;

    let (client, _server) = setup_esg_test("/esg/sri_bond_info", "20240105", response_body, 200).await;
    
    let result = client.esg().sri_bond_info().date("20240105").fetch().await;
    assert!(result.is_ok());
    
    let df = result.unwrap();
    assert_eq!(df.shape(), (3, 12));
    
    // Check different ESG bond types are handled correctly
    assert_eq!(df.column("채권종류").unwrap().str().unwrap().get(0), Some("사회적 채권"));
    assert_eq!(df.column("채권종류").unwrap().str().unwrap().get(1), Some("환경 채권"));
    assert_eq!(df.column("채권종류").unwrap().str().unwrap().get(2), Some("지배구조 채권"));
}