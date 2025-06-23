use krx_rs::{Client, error::Error};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

/// Helper function to create a mock server with predefined response
async fn setup_general_test(
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

// Oil Daily Tests
#[tokio::test]
async fn test_oil_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20200414",
            "OIL_NM": "휘발유",
            "WT_AVG_PRC": "1170.00",
            "WT_DIS_AVG_PRC": "1077.63",
            "ACC_TRDVOL": "14097962",
            "ACC_TRDVAL": "15571094984"
        }, {
            "BAS_DD": "20200414",
            "OIL_NM": "경유",
            "WT_AVG_PRC": "980.00",
            "WT_DIS_AVG_PRC": "965.02",
            "ACC_TRDVOL": "26672877",
            "ACC_TRDVAL": "25880611936"
        }, {
            "BAS_DD": "20200414",
            "OIL_NM": "등유",
            "WT_AVG_PRC": "475.00",
            "WT_DIS_AVG_PRC": "435.62",
            "ACC_TRDVOL": "5993714",
            "ACC_TRDVAL": "2633056816"
        }]
    }"#;

    let (client, _server) =
        setup_general_test("/gen/oil_bydd_trd", "20240105", response_body, 200).await;

    let result = client.general().oil_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (3, 6));
    assert_eq!(
        df.column("유종명").unwrap().str().unwrap().get(0),
        Some("휘발유")
    );
    assert_eq!(
        df.column("유종명").unwrap().str().unwrap().get(1),
        Some("경유")
    );
    assert_eq!(
        df.column("유종명").unwrap().str().unwrap().get(2),
        Some("등유")
    );
}

#[tokio::test]
async fn test_oil_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20200414",
            "OIL_NM": "휘발유",
            "WT_AVG_PRC": "1170.00",
            "WT_DIS_AVG_PRC": "1077.63",
            "ACC_TRDVOL": "14097962",
            "ACC_TRDVAL": "15571094984"
        }, {
            "BAS_DD": "20200414",
            "OIL_NM": "경유",
            "WT_AVG_PRC": "980.00",
            "WT_DIS_AVG_PRC": "965.02",
            "ACC_TRDVOL": "26672877",
            "ACC_TRDVAL": "25880611936"
        }, {
            "BAS_DD": "20200414",
            "OIL_NM": "등유",
            "WT_AVG_PRC": "475.00",
            "WT_DIS_AVG_PRC": "435.62",
            "ACC_TRDVOL": "5993714",
            "ACC_TRDVAL": "2633056816"
        }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/gen/oil_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.general().oil_daily().latest().fetch().await;
    assert!(result.is_ok(), "{:?}", result.unwrap_err());

    let df = result.unwrap();
    assert_eq!(df.shape(), (3, 6));
    assert_eq!(
        df.column("유종명").unwrap().str().unwrap().get(0),
        Some("휘발유")
    );
    assert_eq!(
        df.column("유종명").unwrap().str().unwrap().get(1),
        Some("경유")
    );
    assert_eq!(
        df.column("유종명").unwrap().str().unwrap().get(2),
        Some("등유")
    );
}

// Gold Daily Tests
#[tokio::test]
async fn test_gold_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20200414",
            "ISU_CD": "04020000",
            "ISU_NM": "금 99.99K",
            "TDD_CLSPRC": "67740",
            "CMPPREVDD_PRC": "1590",
            "FLUC_RT": "2.40",
            "TDD_OPNPRC": "66920",
            "TDD_HGPRC": "68240",
            "TDD_LWPRC": "66920",
            "ACC_TRDVOL": "143645",
            "ACC_TRDVAL": "9738771640"
        }, {
            "BAS_DD": "20200414",
            "ISU_CD": "04020100",
            "ISU_NM": "미니금 100g",
            "TDD_CLSPRC": "67800",
            "CMPPREVDD_PRC": "1200",
            "FLUC_RT": "1.80",
            "TDD_OPNPRC": "67440",
            "TDD_HGPRC": "68400",
            "TDD_LWPRC": "67400",
            "ACC_TRDVOL": "7264",
            "ACC_TRDVAL": "492674720"
        }]
    }"#;

    let (client, _server) =
        setup_general_test("/gen/gold_bydd_trd", "20240105", response_body, 200).await;

    let result = client.general().gold_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (2, 11));
    assert_eq!(
        df.column("종목명").unwrap().str().unwrap().get(0),
        Some("금 99.99K")
    );
    assert_eq!(
        df.column("종목명").unwrap().str().unwrap().get(1),
        Some("미니금 100g")
    );

    assert_eq!(
        df.column("종가").unwrap().f64().unwrap().get(0),
        Some(67740.0)
    );
    assert_eq!(
        df.column("종가").unwrap().f64().unwrap().get(1),
        Some(67800.0)
    );
}

#[tokio::test]
async fn test_gold_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "300000000",
            "ACC_TRDVOL": "500",
            "BAS_DD": "20240105",
            "CMPPREVDD_PRC": "10.00",
            "FLUC_RT": "0.5",
            "ISU_CD": "GOLD01",
            "ISU_NM": "골드연계증권",
            "TDD_CLSPRC": "2010.00",
            "TDD_HGPRC": "2020.00",
            "TDD_LWPRC": "2000.00",
            "TDD_OPNPRC": "2005.00"
        }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/gen/gold_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.general().gold_daily().latest().fetch().await;
    assert!(result.is_ok());
}

// Emissions Daily Tests
#[tokio::test]
async fn test_emissions_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20200414",
            "ISU_CD": "05002190",
            "ISU_NM": "KAU19",
            "TDD_CLSPRC": "40500",
            "CMPPREVDD_PRC": "0",
            "FLUC_RT": "0.00",
            "TDD_OPNPRC": "40500",
            "TDD_HGPRC": "40500",
            "TDD_LWPRC": "40500",
            "ACC_TRDVOL": "54194",
            "ACC_TRDVAL": "2194857000"
        }, {
            "BAS_DD": "20200414",
            "ISU_CD": "05002200",
            "ISU_NM": "KAU20",
            "TDD_CLSPRC": "41500",
            "CMPPREVDD_PRC": "0",
            "FLUC_RT": "0.00",
            "TDD_OPNPRC": "0",
            "TDD_HGPRC": "0",
            "TDD_LWPRC": "0",
            "ACC_TRDVOL": "0",
            "ACC_TRDVAL": "0"
        }]
    }"#;

    let (client, _server) =
        setup_general_test("/gen/ets_bydd_trd", "20200414", response_body, 200).await;

    let result = client
        .general()
        .emissions_daily()
        .date("20200414")
        .fetch()
        .await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (2, 11));
    assert_eq!(
        df.column("종목명").unwrap().str().unwrap().get(0),
        Some("KAU19")
    );
    assert_eq!(
        df.column("종목명").unwrap().str().unwrap().get(1),
        Some("KAU20")
    );
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(0),
        Some("05002190")
    );
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(1),
        Some("05002200")
    );
}

// Error Handling Tests
#[tokio::test]
async fn test_general_api_error() {
    let (client, _server) =
        setup_general_test("/gen/oil_bydd_trd", "20240105", "Not Found", 404).await;

    let result = client.general().oil_daily().date("20240105").fetch().await;
    assert!(matches!(
        result,
        Err(Error::ApiError {
            status_code: 404,
            ..
        })
    ));
}

#[tokio::test]
async fn test_general_parsing_error() {
    let response_body = "Invalid JSON";
    let (client, _server) =
        setup_general_test("/gen/oil_bydd_trd", "20240105", response_body, 200).await;

    let result = client.general().oil_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Parsing { .. })));
}

#[tokio::test]
async fn test_general_missing_date() {
    let client = Client::new("test_key");
    let result = client.general().oil_daily().fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[tokio::test]
async fn test_general_empty_response() {
    let response_body = r#"{"OutBlock_1": []}"#;
    let (client, _server) =
        setup_general_test("/gen/oil_bydd_trd", "20240105", response_body, 200).await;

    let result = client.general().oil_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (0, 0));
}

#[tokio::test]
async fn test_general_with_null_values() {
    let response_body = r#"{
        "OutBlock_1": [{
            "BAS_DD": "20200414",
            "OIL_NM": "휘발유",
            "WT_AVG_PRC": "",
            "WT_DIS_AVG_PRC": "-",
            "ACC_TRDVOL": "0",
            "ACC_TRDVAL": "0"
        }, {
            "BAS_DD": "20200414",
            "OIL_NM": "경유",
            "WT_AVG_PRC": "980.00",
            "WT_DIS_AVG_PRC": "965.02",
            "ACC_TRDVOL": "-",
            "ACC_TRDVAL": "25880611936"
        }, {
            "BAS_DD": "20200414",
            "OIL_NM": "등유",
            "WT_AVG_PRC": "0.00",
            "WT_DIS_AVG_PRC": "0.62",
            "ACC_TRDVOL": "-",
            "ACC_TRDVAL": "-"
        }]
    }"#;

    let (client, _server) =
        setup_general_test("/gen/oil_bydd_trd", "20240105", response_body, 200).await;

    let result = client.general().oil_daily().date("20240105").fetch().await;
    assert!(result.is_ok(), "{:?}", result.unwrap_err());

    let df = result.unwrap();
    assert_eq!(df.shape(), (3, 6));

    // Debug: Print the actual DataFrame structure
    println!("DataFrame columns: {:?}", df.get_column_names());
    println!("DataFrame shape: {:?}", df.shape());

    // Debug: Print the actual values in the trading volume column
    let trading_volume_col = df.column("거래량").unwrap();
    println!(
        "Trading volume column type: {:?}",
        trading_volume_col.dtype()
    );
    println!("Trading volume values: {:?}", trading_volume_col);

    // Debug: Check each row individually
    for i in 0..3 {
        let vol_value = trading_volume_col.i64().unwrap().get(i);
        println!("Row {}: trading volume = {:?}", i, vol_value);
    }

    // Check that null values are properly handled
    // First row should have trading volume as None or 0
    let first_row_volume = df.column("거래량").unwrap().i64().unwrap().get(0);
    println!("First row volume: {:?}", first_row_volume);

    // Let's check if it's 0 instead of None
    assert!(
        first_row_volume == Some(0),
        "Expected Some(0), got {:?}",
        first_row_volume
    );

    let second_row_value = df.column("거래량").unwrap().i64().unwrap().get(1);
    println!("Second row value: {:?}", second_row_value);

    assert!(
        second_row_value.is_none(),
        "Expected None, got {:?}",
        second_row_value
    );
}

#[tokio::test]
async fn test_general_network_error() {
    let client = Client::builder()
        .auth_key("test_key")
        .base_url("http://localhost:12345")
        .build()
        .unwrap();

    let result = client.general().oil_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Network(_))));
}

#[tokio::test]
async fn test_general_invalid_date_format() {
    let client = Client::new("test_key");

    let result = client
        .general()
        .oil_daily()
        .date("2024-01-05")
        .fetch()
        .await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[tokio::test]
async fn test_general_rate_limit() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/gen/oil_bydd_trd"))
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

    let result = client.general().oil_daily().date("20240105").fetch().await;

    match result {
        Err(Error::RateLimit { retry_after }) => {
            assert_eq!(retry_after, 60);
        }
        _ => panic!("Expected RateLimit error"),
    }
}
