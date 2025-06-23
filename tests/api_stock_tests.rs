use krx_rs::{Client, error::Error};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

/// Helper function to create a mock server with predefined response
async fn setup_stock_test(
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

// KOSPI Daily Tests
#[tokio::test]
async fn test_kospi_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "1144821080",
            "ACC_TRDVOL": "219508",
            "BAS_DD": "20240104",
            "CMPPREVDD_PRC": "20",
            "FLUC_RT": "0.38",
            "ISU_CD": "095570",
            "ISU_NM": "AJ네트웍스",
            "LIST_SHRS": "45252759",
            "MKTCAP": "236219401980",
            "MKT_NM": "KOSPI",
            "SECT_TP_NM": "-",
            "TDD_CLSPRC": "5220",
            "TDD_HGPRC": "5290",
            "TDD_LWPRC": "5100",
            "TDD_OPNPRC": "5200"
        }]
    }"#;

    let (client, _server) =
        setup_stock_test("/sto/stk_bydd_trd", "20240104", response_body, 200).await;

    let result = client.stock().stock_daily().date("20240104").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 15));
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(0),
        Some("095570")
    );
    assert_eq!(
        df.column("종목명").unwrap().str().unwrap().get(0),
        Some("AJ네트웍스")
    );
}

#[tokio::test]
async fn test_kospi_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "89295560",
            "ACC_TRDVOL": "5254",
            "BAS_DD": "20240104",
            "CMPPREVDD_PRC": "-30",
            "FLUC_RT": "-0.18",
            "ISU_CD": "006840",
            "ISU_NM": "AK홀딩스",
            "LIST_SHRS": "13247561",
            "MKTCAP": "224811110170",
            "MKT_NM": "KOSPI",
            "SECT_TP_NM": "-",
            "TDD_CLSPRC": "16970",
            "TDD_HGPRC": "17040",
            "TDD_LWPRC": "16940",
            "TDD_OPNPRC": "17000"
        }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sto/stk_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.stock().stock_daily().latest().fetch().await;
    assert!(result.is_ok());
}

// KOSDAQ Daily Tests
#[tokio::test]
async fn test_kosdaq_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "71497855505",
            "ACC_TRDVOL": "20877558",
            "BAS_DD": "20240104",
            "CMPPREVDD_PRC": "20",
            "FLUC_RT": "0.62",
            "ISU_CD": "060310",
            "ISU_NM": "3S",
            "LIST_SHRS": "48536642",
            "MKTCAP": "158714819340",
            "MKT_NM": "KOSDAQ",
            "SECT_TP_NM": "중견기업부",
            "TDD_CLSPRC": "3270",
            "TDD_HGPRC": "3685",
            "TDD_LWPRC": "3210",
            "TDD_OPNPRC": "3250"
        }]
    }"#;

    let (client, _server) =
        setup_stock_test("/sto/ksq_bydd_trd", "20240104", response_body, 200).await;

    let result = client.stock().kosdaq_daily().date("20240104").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 15));
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(0),
        Some("060310")
    );
    assert_eq!(
        df.column("종목명").unwrap().str().unwrap().get(0),
        Some("3S")
    );
}

#[tokio::test]
async fn test_kosdaq_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "280048410",
            "ACC_TRDVOL": "40510",
            "BAS_DD": "20240104",
            "CMPPREVDD_PRC": "-10",
            "FLUC_RT": "-0.14",
            "ISU_CD": "054620",
            "ISU_NM": "APS",
            "LIST_SHRS": "20394221",
            "MKTCAP": "141535893740",
            "MKT_NM": "KOSDAQ",
            "SECT_TP_NM": "중견기업부",
            "TDD_CLSPRC": "6940",
            "TDD_HGPRC": "7010",
            "TDD_LWPRC": "6830",
            "TDD_OPNPRC": "6950"
        }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sto/ksq_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.stock().kosdaq_daily().latest().fetch().await;
    assert!(result.is_ok());
}

// KONEX Daily Tests
#[tokio::test]
async fn test_konex_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "12612540",
            "ACC_TRDVOL": "1076",
            "BAS_DD": "20240104",
            "CMPPREVDD_PRC": "-990",
            "FLUC_RT": "-7.51",
            "ISU_CD": "278990",
            "ISU_NM": "EMB",
            "LIST_SHRS": "4729954",
            "MKTCAP": "57705438800",
            "MKT_NM": "KONEX",
            "SECT_TP_NM": "일반기업부",
            "TDD_CLSPRC": "12200",
            "TDD_HGPRC": "12500",
            "TDD_LWPRC": "11360",
            "TDD_OPNPRC": "12500"
        }]
    }"#;

    let (client, _server) =
        setup_stock_test("/sto/knx_bydd_trd", "20240105", response_body, 200).await;

    let result = client.stock().konex_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 15));
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(0),
        Some("278990")
    );
    assert_eq!(
        df.column("종목명").unwrap().str().unwrap().get(0),
        Some("EMB")
    );
}

#[tokio::test]
async fn test_konex_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "74924960",
            "ACC_TRDVOL": "33066",
            "BAS_DD": "20240104",
            "CMPPREVDD_PRC": "-380",
            "FLUC_RT": "-14.84",
            "ISU_CD": "343090",
            "ISU_NM": "HLB사이언스",
            "LIST_SHRS": "16321365",
            "MKTCAP": "35580575700",
            "MKT_NM": "KONEX",
            "SECT_TP_NM": "일반기업부",
            "TDD_CLSPRC": "2180",
            "TDD_HGPRC": "2550",
            "TDD_LWPRC": "2180",
            "TDD_OPNPRC": "2550"
            }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sto/knx_bydd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.stock().konex_daily().latest().fetch().await;
    assert!(result.is_ok());
}

// Error Handling Tests
#[tokio::test]
async fn test_stock_api_error() {
    let (client, _server) =
        setup_stock_test("/sto/stk_bydd_trd", "20240105", "Not Found", 404).await;

    let result = client.stock().stock_daily().date("20240105").fetch().await;
    assert!(matches!(
        result,
        Err(Error::ApiError {
            status_code: 404,
            ..
        })
    ));
}

#[tokio::test]
async fn test_stock_parsing_error() {
    let response_body = "Invalid JSON";
    let (client, _server) =
        setup_stock_test("/sto/stk_bydd_trd", "20240105", response_body, 200).await;

    let result = client.stock().stock_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Parsing { .. })));
}

#[tokio::test]
async fn test_stock_missing_date() {
    let client = Client::new("test_key");
    let result = client.stock().stock_daily().fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[tokio::test]
async fn test_stock_empty_response() {
    let response_body = r#"{"OutBlock_1": []}"#;
    let (client, _server) =
        setup_stock_test("/sto/stk_bydd_trd", "20240105", response_body, 200).await;

    let result = client.stock().stock_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (0, 0));
}

#[tokio::test]
async fn test_stock_with_null_values() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "",
            "ACC_TRDVOL": "-",
            "BAS_DD": "20240104",
            "CMPPREVDD_PRC": "20",
            "FLUC_RT": "0.62",
            "ISU_CD": "060310",
            "ISU_NM": "3S",
            "LIST_SHRS": "48536642",
            "MKTCAP": "",
            "MKT_NM": "KOSDAQ",
            "SECT_TP_NM": "중견기업부",
            "TDD_CLSPRC": "3270",
            "TDD_HGPRC": "3685",
            "TDD_LWPRC": "3210",
            "TDD_OPNPRC": "3250"
        }]
    }"#;

    let (client, _server) =
        setup_stock_test("/sto/ksq_bydd_trd", "20240104", response_body, 200).await;

    let result = client.stock().kosdaq_daily().date("20240104").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 15));

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

#[tokio::test]
async fn test_stock_network_error() {
    let client = Client::builder()
        .auth_key("test_key")
        .base_url("http://localhost:12345")
        .build()
        .unwrap();

    let result = client.stock().stock_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Network(_))));
}

#[tokio::test]
async fn test_stock_invalid_date_format() {
    let client = Client::new("test_key");

    let result = client
        .stock()
        .stock_daily()
        .date("2024-01-05")
        .fetch()
        .await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));

    let result = client.stock().kosdaq_daily().date("240105").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));

    let result = client.stock().konex_daily().date("invalid").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[tokio::test]
async fn test_stock_rate_limit() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sto/stk_bydd_trd"))
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

    let result = client.stock().stock_daily().date("20240105").fetch().await;

    match result {
        Err(Error::RateLimit { retry_after }) => {
            assert_eq!(retry_after, 60);
        }
        _ => panic!("Expected RateLimit error"),
    }
}

// Test multiple records
#[tokio::test]
async fn test_stock_multiple_records() {
    let response_body = r#"{
        "OutBlock_1": [
            {
                "ACC_TRDVAL": "71497855505",
                "ACC_TRDVOL": "20877558",
                "BAS_DD": "20240104",
                "CMPPREVDD_PRC": "20",
                "FLUC_RT": "0.62",
                "ISU_CD": "060310",
                "ISU_NM": "3S",
                "LIST_SHRS": "48536642",
                "MKTCAP": "158714819340",
                "MKT_NM": "KOSDAQ",
                "SECT_TP_NM": "중견기업부",
                "TDD_CLSPRC": "3270",
                "TDD_HGPRC": "3685",
                "TDD_LWPRC": "3210",
                "TDD_OPNPRC": "3250"
            },
            {
                "ACC_TRDVAL": "280048410",
                "ACC_TRDVOL": "40510",
                "BAS_DD": "20240104",
                "CMPPREVDD_PRC": "-10",
                "FLUC_RT": "-0.14",
                "ISU_CD": "054620",
                "ISU_NM": "APS",
                "LIST_SHRS": "20394221",
                "MKTCAP": "141535893740",
                "MKT_NM": "KOSDAQ",
                "SECT_TP_NM": "중견기업부",
                "TDD_CLSPRC": "6940",
                "TDD_HGPRC": "7010",
                "TDD_LWPRC": "6830",
                "TDD_OPNPRC": "6950"
            }
        ]
    }"#;

    let (client, _server) =
        setup_stock_test("/sto/ksq_bydd_trd", "20240104", response_body, 200).await;

    let result = client.stock().kosdaq_daily().date("20240104").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (2, 15));
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(0),
        Some("060310")
    );
    assert_eq!(
        df.column("종목코드").unwrap().str().unwrap().get(1),
        Some("054620")
    );
    assert_eq!(
        df.column("종목명").unwrap().str().unwrap().get(0),
        Some("3S")
    );
    assert_eq!(
        df.column("종목명").unwrap().str().unwrap().get(1),
        Some("APS")
    );
}

// Test fluctuation type handling
#[tokio::test]
async fn test_stock_fluctuation_types() {
    let response_body = r#"{
        "OutBlock_1": [
            {
                "ACC_TRDVAL": "71497855505",
                "ACC_TRDVOL": "20877558",
                "BAS_DD": "20240104",
                "CMPPREVDD_PRC": "20",
                "FLUC_RT": "0.62",
                "ISU_CD": "060310",
                "ISU_NM": "3S",
                "LIST_SHRS": "48536642",
                "MKTCAP": "158714819340",
                "MKT_NM": "KOSDAQ",
                "SECT_TP_NM": "중견기업부",
                "TDD_CLSPRC": "3270",
                "TDD_HGPRC": "3685",
                "TDD_LWPRC": "3210",
                "TDD_OPNPRC": "3250"
            },
            {
                "ACC_TRDVAL": "280048410",
                "ACC_TRDVOL": "40510",
                "BAS_DD": "20240104",
                "CMPPREVDD_PRC": "-10",
                "FLUC_RT": "-0.14",
                "ISU_CD": "054620",
                "ISU_NM": "APS",
                "LIST_SHRS": "20394221",
                "MKTCAP": "141535893740",
                "MKT_NM": "KOSDAQ",
                "SECT_TP_NM": "중견기업부",
                "TDD_CLSPRC": "6940",
                "TDD_HGPRC": "7010",
                "TDD_LWPRC": "6830",
                "TDD_OPNPRC": "6950"
            },
            {
                "ACC_TRDVAL": "157870801100",
                "ACC_TRDVOL": "8391563",
                "BAS_DD": "20240104",
                "CMPPREVDD_PRC": "0",
                "FLUC_RT": "0.00",
                "ISU_CD": "211270",
                "ISU_NM": "AP위성",
                "LIST_SHRS": "15082304",
                "MKTCAP": "269369949440",
                "MKT_NM": "KOSDAQ",
                "SECT_TP_NM": "벤처기업부",
                "TDD_CLSPRC": "17860",
                "TDD_HGPRC": "20200",
                "TDD_LWPRC": "17620",
                "TDD_OPNPRC": "18090"
            }
        ]
    }"#;

    let (client, _server) =
        setup_stock_test("/sto/ksq_bydd_trd", "20240104", response_body, 200).await;

    let result = client.stock().kosdaq_daily().date("20240104").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (3, 15));

    // Check different fluctuation types are handled correctly
    assert!(df.column("대비").unwrap().f64().unwrap().get(0).unwrap() > 0.0);
    assert!(df.column("대비").unwrap().f64().unwrap().get(1).unwrap() < 0.0);
    assert_eq!(
        df.column("대비").unwrap().f64().unwrap().get(2).unwrap(),
        0.0
    );
}
