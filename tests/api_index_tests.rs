use krx_rs::{Client, error::Error};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

/// Helper function to create a mock server with predefined response
async fn setup_index_test(
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

// KRX Daily Tests
#[tokio::test]
async fn test_krx_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "303487254430",
            "ACC_TRDVOL": "13130984",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "791.44",
            "CMPPREVDD_IDX": "-14.72",
            "FLUC_RT": "-1.83",
            "HGPRC_IDX": "802.59",
            "IDX_CLSS": "KRX",
            "IDX_NM": "KRX 300 금융",
            "LWPRC_IDX": "790.62",
            "MKTCAP": "164626089910150",
            "OPNPRC_IDX": "802.59"
        },
        {
            "ACC_TRDVAL": "530844445692",
            "ACC_TRDVOL": "10106979",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "1396.49",
            "CMPPREVDD_IDX": "-20.39",
            "FLUC_RT": "-1.44",
            "HGPRC_IDX": "1406.99",
            "IDX_CLSS": "KRX",
            "IDX_NM": "KRX 300 자유소비재",
            "LWPRC_IDX": "1389.83",
            "MKTCAP": "165686295621020",
            "OPNPRC_IDX": "1405.94"
        }]
    }"#;

    let (client, _server) =
        setup_index_test("/idx/krx_dd_trd", "20240105", response_body, 200).await;

    let result = client.index().krx_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (2, 12));
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(0),
        Some("KRX 300 금융")
    );
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(1),
        Some("KRX 300 자유소비재")
    );
}

#[tokio::test]
async fn test_krx_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "1517511119710",
            "ACC_TRDVOL": "22052144",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "2754.78",
            "CMPPREVDD_IDX": "-133.83",
            "FLUC_RT": "-4.63",
            "HGPRC_IDX": "2861.38",
            "IDX_CLSS": "KRX",
            "IDX_NM": "KRX 300 헬스케어",
            "LWPRC_IDX": "2745.21",
            "MKTCAP": "165131020421770",
            "OPNPRC_IDX": "2859.41"
        },
        {
            "ACC_TRDVAL": "535533829540",
            "ACC_TRDVOL": "7815392",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "1527.17",
            "CMPPREVDD_IDX": "5.18",
            "FLUC_RT": "0.34",
            "HGPRC_IDX": "1531.75",
            "IDX_CLSS": "KRX",
            "IDX_NM": "KRX 300 커뮤니케이션서비스",
            "LWPRC_IDX": "1511.23",
            "MKTCAP": "136341568947160",
            "OPNPRC_IDX": "1514.46"
        }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/idx/krx_dd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.index().krx_daily().latest().fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (2, 12));
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(0),
        Some("KRX 300 헬스케어")
    );
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(1),
        Some("KRX 300 커뮤니케이션서비스")
    );
}

// KOSPI Daily Tests
#[tokio::test]
async fn test_stock_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "8992273956601",
            "ACC_TRDVOL": "770176172",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "-",
            "CMPPREVDD_IDX": "-",
            "FLUC_RT": "-",
            "HGPRC_IDX": "-",
            "IDX_CLSS": "KOSPI",
            "IDX_NM": "코스피 (외국주포함)",
            "LWPRC_IDX": "-",
            "MKTCAP": "2081862659594752",
            "OPNPRC_IDX": "-"
        },
        {
            "ACC_TRDVAL": "8986436535976",
            "ACC_TRDVOL": "768208666",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "2587.02",
            "CMPPREVDD_IDX": "-20.29",
            "FLUC_RT": "-0.78",
            "HGPRC_IDX": "2602.64",
            "IDX_CLSS": "KOSPI",
            "IDX_NM": "코스피",
            "LWPRC_IDX": "2580.09",
            "MKTCAP": "2080804000032312",
            "OPNPRC_IDX": "2592.44"
        },
        {
            "ACC_TRDVAL": "5817672265149",
            "ACC_TRDVOL": "103140948",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "348.07",
            "CMPPREVDD_IDX": "-3.13",
            "FLUC_RT": "-0.89",
            "HGPRC_IDX": "350.76",
            "IDX_CLSS": "KOSPI",
            "IDX_NM": "코스피 200",
            "LWPRC_IDX": "347.13",
            "MKTCAP": "1814393097178020",
            "OPNPRC_IDX": "349.10"
        }]
    }"#;

    let (client, _server) =
        setup_index_test("/idx/kospi_dd_trd", "20240105", response_body, 200).await;

    let result = client.index().stock_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (3, 12));
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(0),
        Some("코스피 (외국주포함)")
    );
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(1),
        Some("코스피")
    );
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(2),
        Some("코스피 200")
    );
}

#[tokio::test]
async fn test_stock_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "8992273956601",
            "ACC_TRDVOL": "770176172",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "-",
            "CMPPREVDD_IDX": "-",
            "FLUC_RT": "-",
            "HGPRC_IDX": "-",
            "IDX_CLSS": "KOSPI",
            "IDX_NM": "코스피 (외국주포함)",
            "LWPRC_IDX": "-",
            "MKTCAP": "2081862659594752",
            "OPNPRC_IDX": "-"
        },
        {
            "ACC_TRDVAL": "8986436535976",
            "ACC_TRDVOL": "768208666",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "2587.02",
            "CMPPREVDD_IDX": "-20.29",
            "FLUC_RT": "-0.78",
            "HGPRC_IDX": "2602.64",
            "IDX_CLSS": "KOSPI",
            "IDX_NM": "코스피",
            "LWPRC_IDX": "2580.09",
            "MKTCAP": "2080804000032312",
            "OPNPRC_IDX": "2592.44"
        },
        {
            "ACC_TRDVAL": "5817672265149",
            "ACC_TRDVOL": "103140948",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "348.07",
            "CMPPREVDD_IDX": "-3.13",
            "FLUC_RT": "-0.89",
            "HGPRC_IDX": "350.76",
            "IDX_CLSS": "KOSPI",
            "IDX_NM": "코스피 200",
            "LWPRC_IDX": "347.13",
            "MKTCAP": "1814393097178020",
            "OPNPRC_IDX": "349.10"
        }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/idx/kospi_dd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.index().stock_daily().latest().fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (3, 12));
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(0),
        Some("코스피 (외국주포함)")
    );
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(1),
        Some("코스피")
    );
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(2),
        Some("코스피 200")
    );

    assert_eq!(
        df.column("거래량").unwrap().i64().unwrap().get(0),
        Some(770176172)
    );
    assert_eq!(
        df.column("거래량").unwrap().i64().unwrap().get(1),
        Some(768208666)
    );
    assert_eq!(
        df.column("거래량").unwrap().i64().unwrap().get(2),
        Some(103140948)
    );
}

// KOSDAQ Daily Tests
#[tokio::test]
async fn test_kosdaq_daily_with_date() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "10106508745144",
            "ACC_TRDVOL": "1107985837",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "866.25",
            "CMPPREVDD_IDX": "-5.32",
            "FLUC_RT": "-0.61",
            "HGPRC_IDX": "872.79",
            "IDX_CLSS": "KOSDAQ",
            "IDX_NM": "코스닥",
            "LWPRC_IDX": "860.45",
            "MKTCAP": "417129376707379",
            "OPNPRC_IDX": "865.35"
        }]
    }"#;

    let (client, _server) =
        setup_index_test("/idx/kosdaq_dd_trd", "20240105", response_body, 200).await;

    let result = client.index().kosdaq_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 12));
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(0),
        Some("코스닥")
    );

    assert_eq!(
        df.column("거래량").unwrap().i64().unwrap().get(0),
        Some(1107985837)
    );
}

#[tokio::test]
async fn test_kosdaq_daily_with_latest() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "10152118117016",
            "ACC_TRDVOL": "1224264897",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "-",
            "CMPPREVDD_IDX": "-",
            "FLUC_RT": "-",
            "HGPRC_IDX": "-",
            "IDX_CLSS": "KOSDAQ",
            "IDX_NM": "코스닥 (외국주포함)",
            "LWPRC_IDX": "-",
            "MKTCAP": "420014902125688",
            "OPNPRC_IDX": "-"
        },
        {
            "ACC_TRDVAL": "10106508745144",
            "ACC_TRDVOL": "1107985837",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "866.25",
            "CMPPREVDD_IDX": "-5.32",
            "FLUC_RT": "-0.61",
            "HGPRC_IDX": "872.79",
            "IDX_CLSS": "KOSDAQ",
            "IDX_NM": "코스닥",
            "LWPRC_IDX": "860.45",
            "MKTCAP": "417129376707379",
            "OPNPRC_IDX": "865.35"
        },
        {
            "ACC_TRDVAL": "2987895803110",
            "ACC_TRDVOL": "111103169",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "1371.76",
            "CMPPREVDD_IDX": "-11.97",
            "FLUC_RT": "-0.87",
            "HGPRC_IDX": "1387.47",
            "IDX_CLSS": "KOSDAQ",
            "IDX_NM": "코스닥 150",
            "LWPRC_IDX": "1359.18",
            "MKTCAP": "205720880941870",
            "OPNPRC_IDX": "1367.90"
        }]
    }"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/idx/kosdaq_dd_trd"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let result = client.index().kosdaq_daily().latest().fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (3, 12));
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(0),
        Some("코스닥 (외국주포함)")
    );
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(1),
        Some("코스닥")
    );
    assert_eq!(
        df.column("지수명").unwrap().str().unwrap().get(2),
        Some("코스닥 150")
    );
}

// Error Handling Tests
#[tokio::test]
async fn test_index_api_error() {
    let (client, _server) = setup_index_test("/idx/krx_dd_trd", "20240105", "Not Found", 404).await;

    let result = client.index().krx_daily().date("20240105").fetch().await;
    assert!(matches!(
        result,
        Err(Error::ApiError {
            status_code: 404,
            ..
        })
    ));
}

#[tokio::test]
async fn test_index_parsing_error() {
    let response_body = "Invalid JSON";
    let (client, _server) =
        setup_index_test("/idx/krx_dd_trd", "20240105", response_body, 200).await;

    let result = client.index().krx_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Parsing { .. })));
}

#[tokio::test]
async fn test_index_missing_date() {
    let client = Client::new("test_key");
    let result = client.index().krx_daily().fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[tokio::test]
async fn test_index_empty_response() {
    let response_body = r#"{"OutBlock_1": []}"#;
    let (client, _server) =
        setup_index_test("/idx/krx_dd_trd", "20240105", response_body, 200).await;

    let result = client.index().krx_daily().date("20240105").fetch().await;
    assert!(result.is_ok());

    let df = result.unwrap();
    assert_eq!(df.shape(), (0, 0));
}

#[tokio::test]
async fn test_index_with_null_values() {
    let response_body = r#"{
        "OutBlock_1": [{
            "ACC_TRDVAL": "-",
            "ACC_TRDVOL": "-",
            "BAS_DD": "20240104",
            "CLSPRC_IDX": "0.00",
            "CMPPREVDD_IDX": "0.00",
            "FLUC_RT": "0.00",
            "HGPRC_IDX": "0.00",
            "IDX_CLSS": "KRX",
            "IDX_NM": "KRX 300 금융",
            "LWPRC_IDX": "0.00",
            "MKTCAP": "-",
            "OPNPRC_IDX": "0.00"
        }]
    }"#;

    let (client, _server) =
        setup_index_test("/idx/krx_dd_trd", "20240105", response_body, 200).await;

    let result = client.index().krx_daily().date("20240105").fetch().await;
    assert!(result.is_ok(), "{:?}", result.unwrap_err());

    let df = result.unwrap();
    assert_eq!(df.shape(), (1, 12));

    // Check that null values are properly handled
    assert!(df.column("거래량").unwrap().i64().unwrap().get(0).is_none());
    assert_eq!(df.column("고가").unwrap().f64().unwrap().get(0), Some(0.0));
}

#[tokio::test]
async fn test_index_network_error() {
    let client = Client::builder()
        .auth_key("test_key")
        .base_url("http://localhost:12345")
        .build()
        .unwrap();

    let result = client.index().krx_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Network(_))));
}

#[tokio::test]
async fn test_index_invalid_date_format() {
    let client = Client::new("test_key");

    let result = client.index().krx_daily().date("2024-01-05").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));

    let result = client.index().stock_daily().date("240105").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));

    let result = client.index().kosdaq_daily().date("invalid").fetch().await;
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[tokio::test]
async fn test_index_rate_limit() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/idx/krx_dd_trd"))
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

    let result = client.index().krx_daily().date("20240105").fetch().await;

    match result {
        Err(Error::RateLimit { retry_after }) => {
            assert_eq!(retry_after, 60);
        }
        _ => panic!("Expected RateLimit error"),
    }
}
