use krx_rs::{Client, error::Error};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

/// Helper function to create a mock server with predefined response
async fn setup_etp_test(
    endpoint: &str,
    date: &str,
    response_body: &str,
    status: u16,
) -> (Client, MockServer) {
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

// ETF Daily Tests
#[tokio::test]
async fn test_etf_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "286826330",
            "ACC_TRDVOL": "28458",
            "BAS_DD": "20240105",
            "CMPPREVDD_IDX": "0.05",
            "CMPPREVDD_PRC": "5",
            "FLUC_RT": "0.05",
            "FLUC_RT_IDX": "0.04",
            "IDX_IND_NM": "KIS 11월 만기자동연장회사채(AA-이상) 총수익지수",
            "INVSTASST_NETASST_TOTAMT": "131011910572",
            "ISU_CD": "473440",
            "ISU_NM": "ACE 11월만기자동연장회사채AA-이상액티브",
            "LIST_SHRS": "13000000",
            "MKTCAP": "131040000000",
            "NAV": "10077.84",
            "OBJ_STKPRC_IDX": "113.63",
            "TDD_CLSPRC": "10080",
            "TDD_HGPRC": "10080",
            "TDD_LWPRC": "10075",
            "TDD_OPNPRC": "10080"
        }]
    }"#;

    let (client, _server) =
        setup_etp_test("/etp/etf_bydd_trd", "20240105", response_body, 200).await;

    let result = client.etp().etf_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 19));
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(0),
        Some("473440")
    );
}

#[tokio::test]
async fn test_etf_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "286826330",
            "ACC_TRDVOL": "28458",
            "BAS_DD": "20240105",
            "CMPPREVDD_IDX": "0.05",
            "CMPPREVDD_PRC": "5",
            "FLUC_RT": "0.05",
            "FLUC_RT_IDX": "0.04",
            "IDX_IND_NM": "KIS 11월 만기자동연장회사채(AA-이상) 총수익지수",
            "INVSTASST_NETASST_TOTAMT": "131011910572",
            "ISU_CD": "473440",
            "ISU_NM": "ACE 11월만기자동연장회사채AA-이상액티브",
            "LIST_SHRS": "13000000",
            "MKTCAP": "131040000000",
            "NAV": "10077.84",
            "OBJ_STKPRC_IDX": "113.63",
            "TDD_CLSPRC": "10080",
            "TDD_HGPRC": "10080",
            "TDD_LWPRC": "10075",
            "TDD_OPNPRC": "10080"
        }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/etp/etf_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.etp().etf_daily().latest().fetch().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_etf_daily_empty_response() {
    let response_body = r#"{"OutBlock_1": []}"#;
    let (client, _server) =
        setup_etp_test("/etp/etf_bydd_trd", "20240105", response_body, 200).await;

    let result = client.etp().etf_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (0, 0));
}

#[tokio::test]
async fn test_etf_daily_missing_date() {
    let client = Client::new("test_key");
    let result = client.etp().etf_daily().fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

// ETN Daily Tests
#[tokio::test]
async fn test_etn_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "5000000",
            "ACC_TRDVOL": "500",
            "BAS_DD": "20240105",
            "CMPPREVDD_IDX": "0.1",
            "CMPPREVDD_PRC": "10",
            "FLUC_RT": "0.1",
            "FLUC_RT_IDX": "0.1",
            "IDX_IND_NM": "테스트 지수",
            "INDIC_VAL_AMT": "15691960000",
            "ISU_CD": "580123",
            "ISU_NM": "테스트 ETN",
            "LIST_SHRS": "1000000",
            "MKTCAP": "10000000000",
            "NAV": "10000.0",
            "OBJ_STKPRC_IDX": "100.0",
            "PER1SECU_INDIC_VAL": "7845.98",
            "TDD_CLSPRC": "10010",
            "TDD_HGPRC": "10020",
            "TDD_LWPRC": "10000",
            "TDD_OPNPRC": "10005"
        }]
    }"#;

    let (client, _server) =
        setup_etp_test("/etp/etn_bydd_trd", "20240105", response_body, 200).await;

    let result = client.etp().etn_daily().date("20240105").fetch().await;
    assert!(result.is_ok(), "{:?}", result.unwrap_err());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 19));
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(0),
        Some("580123")
    );
}

#[tokio::test]
async fn test_etn_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "5000000",
            "ACC_TRDVOL": "500",
            "BAS_DD": "20240105",
            "CMPPREVDD_IDX": "0.1",
            "CMPPREVDD_PRC": "10",
            "FLUC_RT": "0.1",
            "FLUC_RT_IDX": "0.1",
            "IDX_IND_NM": "테스트 지수",
            "INDIC_VAL_AMT": "15691960000",
            "ISU_CD": "580123",
            "ISU_NM": "테스트 ETN",
            "LIST_SHRS": "1000000",
            "MKTCAP": "10000000000",
            "NAV": "10000.0",
            "OBJ_STKPRC_IDX": "100.0",
            "PER1SECU_INDIC_VAL": "7845.98",
            "TDD_CLSPRC": "10010",
            "TDD_HGPRC": "10020",
            "TDD_LWPRC": "10000",
            "TDD_OPNPRC": "10005"
        }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/etp/etn_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.etp().etn_daily().latest().fetch().await;
    assert!(result.is_ok());
}

// ELW Daily Tests
#[tokio::test]
async fn test_elw_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "1000000",
            "ACC_TRDVOL": "100",
            "BAS_DD": "20240105",
            "CMPPREVDD_PRC": "5",
            "CMPPREVDD_PRC_ULY": "-400",
            "FLUC_RT_ULY": "-0.52",
            "ISU_CD": "580001",
            "ISU_NM": "테스트 ELW",
            "LIST_SHRS": "15800000",
            "MKTCAP": "158000000",
            "TDD_CLSPRC": "1005",
            "TDD_HGPRC": "1010",
            "TDD_LWPRC": "1000",
            "TDD_OPNPRC": "1002",
            "ULY_NM": "테스트",
            "ULY_PRC": "76,600"
        }]
    }"#;

    let (client, _server) =
        setup_etp_test("/etp/elw_bydd_trd", "20240105", response_body, 200).await;

    let result = client.etp().elw_daily().date("20240105").fetch().await;
    assert!(result.is_ok(), "{:?}", result.unwrap_err());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 16));
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(0),
        Some("580001")
    );
}

#[tokio::test]
async fn test_elw_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "1000000",
            "ACC_TRDVOL": "100",
            "BAS_DD": "20240105",
            "CMPPREVDD_PRC": "5",
            "CMPPREVDD_PRC_ULY": "-400",
            "FLUC_RT_ULY": "-0.52",
            "ISU_CD": "580001",
            "ISU_NM": "테스트 ELW",
            "LIST_SHRS": "15800000",
            "MKTCAP": "158000000",
            "TDD_CLSPRC": "1005",
            "TDD_HGPRC": "1010",
            "TDD_LWPRC": "1000",
            "TDD_OPNPRC": "1002",
            "ULY_NM": "테스트",
            "ULY_PRC": "76,600"
        }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/etp/elw_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.etp().elw_daily().latest().fetch().await;
    assert!(result.is_ok());
}

// Error Handling Tests
#[tokio::test]
async fn test_etp_api_error() {
    let (client, _server) = setup_etp_test("/etp/etf_bydd_trd", "20240105", "Not Found", 404).await;

    let result = client.etp().etf_daily().date("20240105").fetch().await;
    assert!(matches!(
        result,
        Err(Error::ApiError {
            status_code: 404,
            ..
        })
    ));
}

#[tokio::test]
async fn test_etp_parsing_error() {
    let response_body = "Invalid JSON";
    let (client, _server) =
        setup_etp_test("/etp/etf_bydd_trd", "20240105", response_body, 200).await;

    let result = client.etp().etf_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Parsing { .. })));
}

#[tokio::test]
async fn test_etp_network_error() {
    let client = Client::builder()
        .auth_key("test_key")
        .base_url("http://localhost:12345") // Non-existent server
        .build()
        .unwrap();

    let result = client.etp().etf_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Network(_))));
}

// Test with null values
#[tokio::test]
async fn test_etp_with_null_values() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "",
            "ACC_TRDVOL": "",
            "BAS_DD": "20240105",
            "CMPPREVDD_IDX": "",
            "CMPPREVDD_PRC": "",
            "FLUC_RT": "",
            "FLUC_RT_IDX": "",
            "IDX_IND_NM": "테스트 지수",
            "INVSTASST_NETASST_TOTAMT": "",
            "ISU_CD": "473440",
            "ISU_NM": "테스트 ETF",
            "LIST_SHRS": "",
            "MKTCAP": "",
            "NAV": "",
            "OBJ_STKPRC_IDX": "",
            "TDD_CLSPRC": "",
            "TDD_HGPRC": "",
            "TDD_LWPRC": "",
            "TDD_OPNPRC": ""
        }]
    }"#;

    let (client, _server) =
        setup_etp_test("/etp/etf_bydd_trd", "20240105", response_body, 200).await;

    let result = client.etp().etf_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 19));

    // Check that null values are properly handled
    assert!(df.column("거래량").unwrap().i64().unwrap().get(0).is_none());
    assert!(
        df.column("거래대금")
            .unwrap()
            .i64()
            .unwrap()
            .get(0)
            .is_none()
    );
}

// Test multiple records
#[tokio::test]
async fn test_etp_multiple_records() {
    let response_body = r#"{
        "OutBlock_1": [
            {
                "ACC_TRDVAL": "286826330",
                "ACC_TRDVOL": "28458",
                "BAS_DD": "20240105",
                "CMPPREVDD_IDX": "0.05",
                "CMPPREVDD_PRC": "5",
                "FLUC_RT": "0.05",
                "FLUC_RT_IDX": "0.04",
                "IDX_IND_NM": "KIS 11월 만기자동연장회사채(AA-이상) 총수익지수",
                "INVSTASST_NETASST_TOTAMT": "131011910572",
                "ISU_CD": "473440",
                "ISU_NM": "ACE 11월만기자동연장회사채AA-이상액티브",
                "LIST_SHRS": "13000000",
                "MKTCAP": "131040000000",
                "NAV": "10077.84",
                "OBJ_STKPRC_IDX": "113.63",
                "TDD_CLSPRC": "10080",
                "TDD_HGPRC": "10080",
                "TDD_LWPRC": "10075",
                "TDD_OPNPRC": "10080"
            },
            {
                "ACC_TRDVAL": "263621860",
                "ACC_TRDVOL": "7452",
                "BAS_DD": "20240105",
                "CMPPREVDD_IDX": "-0.85",
                "CMPPREVDD_PRC": "-125",
                "FLUC_RT": "-0.35",
                "FLUC_RT_IDX": "-0.75",
                "IDX_IND_NM": "다른 지수",
                "INVSTASST_NETASST_TOTAMT": "100000000000",
                "ISU_CD": "473441",
                "ISU_NM": "다른 ETF",
                "LIST_SHRS": "10000000",
                "MKTCAP": "100000000000",
                "NAV": "10000.0",
                "OBJ_STKPRC_IDX": "100.0",
                "TDD_CLSPRC": "10000",
                "TDD_HGPRC": "10100",
                "TDD_LWPRC": "9900",
                "TDD_OPNPRC": "10050"
            }
        ]
    }"#;

    let (client, _server) =
        setup_etp_test("/etp/etf_bydd_trd", "20240105", response_body, 200).await;

    let result = client.etp().etf_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (2, 19));
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(0),
        Some("473440")
    );
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(1),
        Some("473441")
    );
}

// Test rate limiting
#[tokio::test]
async fn test_etp_rate_limit() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/etp/etf_bydd_trd"))
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

    let result = client.etp().etf_daily().date("20240105").fetch().await;

    match result {
        Err(Error::RateLimit { retry_after }) => {
            assert_eq!(retry_after, 60);
        }
        _ => panic!("Expected RateLimit error"),
    }
}

// Test invalid date format
#[tokio::test]
async fn test_etp_invalid_date_format() {
    let client = Client::new("test_key");

    let result = client.etp().etf_daily().date("2024-01-05").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));

    let result = client.etp().etn_daily().date("240105").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));

    let result = client.etp().elw_daily().date("invalid").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}
