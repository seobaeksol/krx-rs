use krx_rs::{Client, error::Error};
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::{method, path, query_param, header}};
use serde_json::json;

async fn setup_mock_server() -> MockServer {
    MockServer::start().await
}

#[tokio::test]
async fn test_client_creation() {
    let client = Client::new("test_auth_key");
    
    // 클라이언트가 성공적으로 생성되어야 함
    let _stock_api = client.stock();
    let _index_api = client.index();
}

#[tokio::test]
async fn test_client_builder() {
    let client = Client::builder()
        .auth_key("test_key")
        .base_url("http://test.example.com")
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("test-agent")
        .build()
        .unwrap();
    
    let _stock_api = client.stock();
}

#[tokio::test]
async fn test_client_builder_missing_auth_key() {
    let result = Client::builder()
        .base_url("http://test.example.com")
        .build();
    
    assert!(result.is_err());
    if let Err(Error::InvalidInput(msg)) = result {
        assert_eq!(msg, "auth_key is required");
    } else {
        panic!("Expected InvalidInput error");
    }
}

#[tokio::test]
async fn test_successful_api_call() {
    let mock_server = setup_mock_server().await;
    
    let mock_response = json!({
        "OutBlock_1": [
            {
                "ACC_TRDVAL": "8252309091974",
                "ACC_TRDVOL": "139123344",
                "BAS_DD": "20240105",
                "CLSPRC_IDX": "1608.11",
                "CMPPREVDD_IDX": "-0.15",
                "FLUC_RT": "-0.01",
                "HGPRC_IDX": "1612.19",
                "IDX_CLSS": "KRX",
                "IDX_NM": "KRX 300",
                "LWPRC_IDX": "1603.75",
                "MKTCAP": "1912238454981230",
                "OPNPRC_IDX": "1608.71"
            }
        ]
    });
    
    Mock::given(method("GET"))
        .and(path("/idx/krx_dd_trd"))
        .and(query_param("basDd", "20240105"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;
    
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let result = client.index()
        .krx_daily()
        .date("20240105")
        .fetch()
        .await;
    
    if let Err(e) = &result {
        eprintln!("Error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_api_error_404() {
    let mock_server = setup_mock_server().await;
    
    Mock::given(method("GET"))
        .and(path("/idx/krx_dd_trd"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
        .mount(&mock_server)
        .await;
    
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let result = client.index()
        .krx_daily()
        .date("20240105")
        .fetch()
        .await;
    
    assert!(result.is_err());
    if let Err(Error::ApiError { status_code, message }) = result {
        assert_eq!(status_code, 404);
        assert_eq!(message, "Not Found");
    } else {
        panic!("Expected ApiError");
    }
}

#[tokio::test]
async fn test_rate_limit_error() {
    let mock_server = setup_mock_server().await;
    
    Mock::given(method("GET"))
        .and(path("/idx/krx_dd_trd"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "60")
                .set_body_string("Rate limit exceeded")
        )
        .mount(&mock_server)
        .await;
    
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let result = client.index()
        .krx_daily()
        .date("20240105")
        .fetch()
        .await;
    
    assert!(result.is_err());
    if let Err(Error::RateLimit { retry_after }) = result {
        assert_eq!(retry_after, 60);
    } else {
        panic!("Expected RateLimit error");
    }
}

#[tokio::test]
async fn test_json_parsing_error() {
    let mock_server = setup_mock_server().await;
    
    Mock::given(method("GET"))
        .and(path("/idx/krx_dd_trd"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Invalid JSON"))
        .mount(&mock_server)
        .await;
    
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let result = client.index()
        .krx_daily()
        .date("20240105")
        .fetch()
        .await;
    
    assert!(result.is_err());
    if let Err(Error::Parsing { details, source: _, response_body }) = result {
        assert!(details.contains("Failed to deserialize response"));
        assert_eq!(response_body, "Invalid JSON");
    } else {
        panic!("Expected Parsing error");
    }
}

#[tokio::test]
async fn test_stock_kospi_daily_api() {
    let mock_server = setup_mock_server().await;
    
    let mock_response = json!({
        "OutBlock_1": [
            {
                "BAS_DD": "20240105",
                "ISU_CD": "005930",
                "ISU_NM": "삼성전자",
                "MKT_NM": "코스피",
                "SECT_TP_NM": "전기전자",
                "TDD_CLSPRC": "75000",
                "CMPPREVDD_PRC": "1000",
                "FLUC_RT": "1.35",
                "TDD_OPNPRC": "74500",
                "TDD_HGPRC": "75500",
                "TDD_LWPRC": "74000",
                "ACC_TRDVOL": "12345678",
                "ACC_TRDVAL": "987654321000",
                "MKTCAP": "450000000000000",
                "LIST_SHRS": "5969782550"
            }
        ]
    });
    
    Mock::given(method("GET"))
        .and(path("/sto/stk_bydd_trd"))
        .and(query_param("basDd", "20240105"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;
    
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let result = client.stock()
        .kospi_daily()
        .date("20240105")
        .fetch()
        .await;
    
    if let Err(e) = &result {
        eprintln!("Stock API Error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_kts_daily_api() {
    let mock_server = setup_mock_server().await;
    
    let mock_response = json!({
        "OutBlock_1": [
            {
                "ACC_TRDVAL": "104622050000",
                "ACC_TRDVOL": "103000000000",
                "BAS_DD": "20240105",
                "BND_EXP_TP_NM": "2",
                "CLSPRC": "10158.0",
                "CLSPRC_YD": "3.381",
                "CMPPREVDD_PRC": "-4.5",
                "GOVBND_ISU_TP_NM": "지표",
                "HGPRC": "10158.5",
                "HGPRC_YD": "3.378",
                "ISU_CD": "KR103503GD97",
                "ISU_NM": "국고03625-2509(23-8)",
                "LWPRC": "10155.5",
                "LWPRC_YD": "3.397",
                "MKT_NM": "국채전문유통시장",
                "OPNPRC": "10157.0",
                "OPNPRC_YD": "3.388"
            }
        ]
    });
    
    Mock::given(method("GET"))
        .and(path("/bon/kts_bydd_trd"))
        .and(query_param("basDd", "20240105"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;
    
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let result = client.bond()
        .kts_daily()
        .date("20240105")
        .fetch()
        .await;
    
    if let Err(e) = &result {
        eprintln!("Bond API Error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_etp_etf_daily_api() {
    let mock_server = setup_mock_server().await;
    
    let mock_response = json!({
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
                "TDD_LWPRC": "10070",
                "TDD_OPNPRC": "10080"
            }
        ]
    });
    
    Mock::given(method("GET"))
        .and(path("/etp/etf_bydd_trd"))
        .and(query_param("basDd", "20240105"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;
    
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let result = client.etp()
        .etf_daily()
        .date("20240105")
        .fetch()
        .await;
    
    if let Err(e) = &result {
        eprintln!("ETP API Error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_derivative_futures_daily_api() {
    let mock_server = setup_mock_server().await;
    
    let mock_response = json!({
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
            }
        ]
    });
    
    Mock::given(method("GET"))
        .and(path("/drv/fut_bydd_trd"))
        .and(query_param("basDd", "20240105"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;
    
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let result = client.derivative()
        .futures_daily()
        .date("20240105")
        .fetch()
        .await;
    
    if let Err(e) = &result {
        eprintln!("Derivative API Error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_general_oil_daily_api() {
    let mock_server = setup_mock_server().await;
    
    let mock_response = json!({
        "OutBlock_1": [
            {
                "ACC_TRDVAL": "11175754363",
                "ACC_TRDVOL": "7746580",
                "BAS_DD": "20240105",
                "OIL_NM": "휘발유",
                "WT_AVG_PRC": "1430.00",
                "WT_DIS_AVG_PRC": "1443.93"
            }
        ]
    });
    
    Mock::given(method("GET"))
        .and(path("/gen/oil_bydd_trd"))
        .and(query_param("basDd", "20240105"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;
    
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let result = client.general()
        .oil_daily()
        .date("20240105")
        .fetch()
        .await;
    
    if let Err(e) = &result {
        eprintln!("General API Error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_esg_sri_bond_info_api() {
    let mock_server = setup_mock_server().await;
    
    let mock_response = json!({
        "OutBlock_1": [
            {
                "BAS_DD": "20240105",
                "BND_TP_NM": "회사채",
                "ISUR_NM": "신한은행",
                "ISU_AMT": "400000000000",
                "ISU_CD": "KR6000011B58",
                "ISU_DD": "20210506",
                "ISU_NM": "신한은행25-05-이10갑후(녹)",
                "ISU_RT": "2.58000",
                "LIST_AMT": "400000000000",
                "LIST_DD": "20210507",
                "REDMPT_DD": "20310506",
                "SRI_BND_TP_NM": "녹색채권"
            }
        ]
    });
    
    Mock::given(method("GET"))
        .and(path("/esg/sri_bond_info"))
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;
    
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();
    
    let result = client.esg()
        .sri_bond_info()
        .fetch()
        .await;
    
    if let Err(e) = &result {
        eprintln!("ESG API Error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_network_error() {
    // 존재하지 않는 서버에 요청
    let client = Client::builder()
        .auth_key("test_key")
        .base_url("http://nonexistent.example.com")
        .timeout(std::time::Duration::from_millis(100))
        .build()
        .unwrap();
    
    let result = client.index()
        .krx_daily()
        .date("20240105")
        .fetch()
        .await;
    
    assert!(result.is_err());
    if let Err(Error::Network(_)) = result {
        // 네트워크 오류 발생
    } else {
        panic!("Expected Network error");
    }
}