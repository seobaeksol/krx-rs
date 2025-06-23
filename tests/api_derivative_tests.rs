use krx_rs::{Client, error::Error};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

/// Helper function to create a mock server with predefined response
async fn setup_derivative_test(
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

// Futures Daily Tests
#[tokio::test]
async fn test_futures_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_OPNINT_QTY": "185074",
            "ACC_TRDVAL": "7659138250000",
            "ACC_TRDVOL": "67384",
            "BAS_DD": "20240105",
            "CMPPREVDD_PRC": "-0.54",
            "ISU_CD": "167V3000",
            "ISU_NM": "10년국채   F 202403 (주간)",
            "MKT_NM": "정규",
            "PROD_NM": "10년국채 선물",
            "SETL_PRC": "113.63",
            "SPOT_PRC": "113.75",
            "TDD_CLSPRC": "113.63",
            "TDD_HGPRC": "113.88",
            "TDD_LWPRC": "113.46",
            "TDD_OPNPRC": "113.72"
        }]
    }"#;

    let (client, _server) =
        setup_derivative_test("/drv/fut_bydd_trd", "20240105", response_body, 200).await;

    let result = client
        .derivative()
        .futures_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 12));
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(0),
        Some("167V3000")
    );
}

#[tokio::test]
async fn test_futures_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_OPNINT_QTY": "185074",
            "ACC_TRDVAL": "7659138250000",
            "ACC_TRDVOL": "67384",
            "BAS_DD": "20240105",
            "CMPPREVDD_PRC": "-0.54",
            "ISU_CD": "167V3000",
            "ISU_NM": "10년국채   F 202403 (주간)",
            "MKT_NM": "정규",
            "PROD_NM": "10년국채 선물",
            "SETL_PRC": "113.63",
            "SPOT_PRC": "113.75",
            "TDD_CLSPRC": "113.63",
            "TDD_HGPRC": "113.88",
            "TDD_LWPRC": "113.46",
            "TDD_OPNPRC": "113.72"
        }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/drv/fut_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.derivative().futures_daily().latest().fetch().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_futures_daily_empty_response() {
    let response_body = r#"{"OutBlock_1": []}"#;
    let (client, _server) =
        setup_derivative_test("/drv/fut_bydd_trd", "20240105", response_body, 200).await;

    let result = client
        .derivative()
        .futures_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (0, 0));
}

#[tokio::test]
async fn test_futures_daily_missing_date() {
    let client = Client::new("test_key");
    let result = client.derivative().futures_daily().fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

// Options Daily Tests
#[tokio::test]
async fn test_options_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_OPNINT_QTY": "12345",
            "ACC_TRDVAL": "1000000000",
            "ACC_TRDVOL": "500",
            "BAS_DD": "20240105",
            "CMPPREVDD_PRC": "0.05",
            "ISU_CD": "4C7V3C295",
            "ISU_NM": "KOSPI200    C 202403 295.0",
            "PROD_NM": "KOSPI200 옵션",
            "RGHT_TP_NM": "콜",
            "TDD_CLSPRC": "7.65",
            "TDD_HGPRC": "8.00",
            "TDD_LWPRC": "7.50",
            "TDD_OPNPRC": "7.80",
            "IMP_VOLT": "15.5",
            "NXTDD_BAS_PRC": "295.0"
        }]
    }"#;

    let (client, _server) =
        setup_derivative_test("/drv/opt_bydd_trd", "20240105", response_body, 200).await;

    let result = client
        .derivative()
        .options_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 12));
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(0),
        Some("4C7V3C295")
    );
}

#[tokio::test]
async fn test_options_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_OPNINT_QTY": "12345",
            "ACC_TRDVAL": "1000000000",
            "ACC_TRDVOL": "500",
            "BAS_DD": "20240105",
            "CMPPREVDD_PRC": "0.05",
            "ISU_CD": "4C7V3C295",
            "ISU_NM": "KOSPI200    C 202403 295.0",
            "PROD_NM": "KOSPI200 옵션",
            "RGHT_TP_NM": "콜",
            "TDD_CLSPRC": "7.65",
            "TDD_HGPRC": "8.00",
            "TDD_LWPRC": "7.50",
            "TDD_OPNPRC": "7.80",
            "IMP_VOLT": "15.5",
            "NXTDD_BAS_PRC": "295.0"
        }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/drv/opt_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.derivative().options_daily().latest().fetch().await;
    assert!(result.is_ok());
}

// Equity Stock Futures Daily Tests
#[tokio::test]
async fn test_equity_stock_futures_daily() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_OPNINT_QTY": "5000",
            "ACC_TRDVAL": "500000000",
            "ACC_TRDVOL": "100",
            "BAS_DD": "20240105",
            "CMPPREVDD_PRC": "1000",
            "ISU_CD": "475F3021",
            "ISU_NM": "삼성전자    F 202403",
            "MKT_NM": "정규",
            "PROD_NM": "개별주 선물",
            "SETL_PRC": "75000",
            "SPOT_PRC": "74500",
            "TDD_CLSPRC": "75000",
            "TDD_HGPRC": "76000",
            "TDD_LWPRC": "74500",
            "TDD_OPNPRC": "75500"
        }]
    }"#;

    let (client, _server) =
        setup_derivative_test("/drv/eqsfu_stk_bydd_trd", "20240105", response_body, 200).await;

    let result = client
        .derivative()
        .equity_stock_futures_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 10));
}

// Equity KOSDAQ Futures Daily Tests
#[tokio::test]
async fn test_equity_kosdaq_futures_daily() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_OPNINT_QTY": "3000",
            "ACC_TRDVAL": "300000000",
            "ACC_TRDVOL": "50",
            "BAS_DD": "20240105",
            "CMPPREVDD_PRC": "500",
            "ISU_CD": "475F3022",
            "ISU_NM": "셀트리온    F 202403",
            "MKT_NM": "정규",
            "PROD_NM": "개별주 선물",
            "SETL_PRC": "150000",
            "SPOT_PRC": "149500",
            "TDD_CLSPRC": "150000",
            "TDD_HGPRC": "151000",
            "TDD_LWPRC": "149000",
            "TDD_OPNPRC": "149500"
        }]
    }"#;

    let (client, _server) =
        setup_derivative_test("/drv/eqkfu_ksq_bydd_trd", "20240105", response_body, 200).await;

    let result = client
        .derivative()
        .equity_kosdaq_futures_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(result.is_ok());
}

// Equity Stock Options Daily Tests
#[tokio::test]
async fn test_equity_stock_options_daily() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_OPNINT_QTY": "1000",
            "ACC_TRDVAL": "100000000",
            "ACC_TRDVOL": "10",
            "BAS_DD": "20240105",
            "CMPPREVDD_PRC": "100",
            "ISU_CD": "475O3C750",
            "ISU_NM": "삼성전자    C 202403 75000",
            "PROD_NM": "개별주 옵션",
            "RGHT_TP_NM": "콜",
            "TDD_CLSPRC": "1500",
            "TDD_HGPRC": "1600",
            "TDD_LWPRC": "1400",
            "TDD_OPNPRC": "1450",
            "IMP_VOLT": "20.5",
            "NXTDD_BAS_PRC": "75000"
        }]
    }"#;

    let (client, _server) =
        setup_derivative_test("/drv/eqsop_bydd_trd", "20240105", response_body, 200).await;

    let result = client
        .derivative()
        .equity_stock_options_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(result.is_ok());
}

// Equity KOSDAQ Options Daily Tests
#[tokio::test]
async fn test_equity_kosdaq_options_daily() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_OPNINT_QTY": "500",
            "ACC_TRDVAL": "50000000",
            "ACC_TRDVOL": "5",
            "BAS_DD": "20240105",
            "CMPPREVDD_PRC": "50",
            "ISU_CD": "475O3P150",
            "ISU_NM": "셀트리온    P 202403 150000",
            "PROD_NM": "개별주 옵션",
            "RGHT_TP_NM": "풋",
            "TDD_CLSPRC": "2000",
            "TDD_HGPRC": "2100",
            "TDD_LWPRC": "1900",
            "TDD_OPNPRC": "1950",
            "IMP_VOLT": "25.0",
            "NXTDD_BAS_PRC": "150000"
        }]
    }"#;

    let (client, _server) =
        setup_derivative_test("/drv/eqkop_bydd_trd", "20240105", response_body, 200).await;

    let result = client
        .derivative()
        .equity_kosdaq_options_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(result.is_ok());
}

// Error Handling Tests
#[tokio::test]
async fn test_derivative_api_error() {
    let (client, _server) =
        setup_derivative_test("/drv/fut_bydd_trd", "20240105", "Not Found", 404).await;

    let result = client
        .derivative()
        .futures_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(matches!(
        result,
        Err(Error::ApiError {
            status_code: 404,
            ..
        })
    ));
}

#[tokio::test]
async fn test_derivative_parsing_error() {
    let response_body = "Invalid JSON";
    let (client, _server) =
        setup_derivative_test("/drv/fut_bydd_trd", "20240105", response_body, 200).await;

    let result = client
        .derivative()
        .futures_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(matches!(result, Err(Error::Parsing { .. })));
}

#[tokio::test]
async fn test_derivative_network_error() {
    let client = Client::builder()
        .auth_key("test_key")
        .base_url("http://localhost:12345") // Non-existent server
        .build()
        .unwrap();

    let result = client
        .derivative()
        .futures_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(matches!(result, Err(Error::Network(_))));
}

// Test with null values
#[tokio::test]
async fn test_derivative_with_null_values() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_OPNINT_QTY": "",
            "ACC_TRDVAL": "",
            "ACC_TRDVOL": "",
            "BAS_DD": "20240105",
            "CMPPREVDD_PRC": "",
            "ISU_CD": "167V3000",
            "ISU_NM": "10년국채   F 202403 (주간)",
            "MKT_NM": "정규",
            "PROD_NM": "10년국채 선물",
            "SETL_PRC": "",
            "SPOT_PRC": "",
            "TDD_CLSPRC": "",
            "TDD_HGPRC": "",
            "TDD_LWPRC": "",
            "TDD_OPNPRC": ""
        }]
    }"#;

    let (client, _server) =
        setup_derivative_test("/drv/fut_bydd_trd", "20240105", response_body, 200).await;

    let result = client
        .derivative()
        .futures_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 12));

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
async fn test_derivative_multiple_records() {
    let response_body = r#"{
        "OutBlock_1": [
            {
                "ACC_OPNINT_QTY": "185074",
                "ACC_TRDVAL": "7659138250000",
                "ACC_TRDVOL": "67384",
                "BAS_DD": "20240105",
                "CMPPREVDD_PRC": "-0.54",
                "ISU_CD": "167V3000",
                "ISU_NM": "10년국채   F 202403 (주간)",
                "MKT_NM": "정규",
                "PROD_NM": "10년국채 선물",
                "SETL_PRC": "113.63",
                "SPOT_PRC": "113.75",
                "TDD_CLSPRC": "113.63",
                "TDD_HGPRC": "113.88",
                "TDD_LWPRC": "113.46",
                "TDD_OPNPRC": "113.72"
            },
            {
                "ACC_OPNINT_QTY": "11",
                "ACC_TRDVAL": "227760000",
                "ACC_TRDVOL": "2",
                "BAS_DD": "20240105",
                "CMPPREVDD_PRC": "-0.45",
                "ISU_CD": "167V6000",
                "ISU_NM": "10년국채   F 202406 (주간)",
                "MKT_NM": "정규",
                "PROD_NM": "10년국채 선물",
                "SETL_PRC": "113.88",
                "SPOT_PRC": "113.75",
                "TDD_CLSPRC": "113.88",
                "TDD_HGPRC": "113.88",
                "TDD_LWPRC": "113.88",
                "TDD_OPNPRC": "113.88"
            }
        ]
    }"#;

    let (client, _server) =
        setup_derivative_test("/drv/fut_bydd_trd", "20240105", response_body, 200).await;

    let result = client
        .derivative()
        .futures_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (2, 12));
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(0),
        Some("167V3000")
    );
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(1),
        Some("167V6000")
    );
}

// Test rate limiting
#[tokio::test]
async fn test_derivative_rate_limit() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/drv/fut_bydd_trd"))
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

    let result = client
        .derivative()
        .futures_daily()
        .date("20240105")
        .fetch()
        .await;

    match result {
        Err(Error::RateLimit { retry_after }) => {
            assert_eq!(retry_after, 60);
        }
        _ => panic!("Expected RateLimit error"),
    }
}

// Test invalid date format
#[tokio::test]
async fn test_derivative_invalid_date_format() {
    let client = Client::new("test_key");

    let result = client
        .derivative()
        .futures_daily()
        .date("2024-01-05")
        .fetch()
        .await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));

    let result = client
        .derivative()
        .options_daily()
        .date("240105")
        .fetch()
        .await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}
