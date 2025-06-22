use krx_rs::{Client, error::Error};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

// 헬퍼 함수: 테스트 클라이언트와 모의 서버를 설정합니다.
async fn setup_test_client(
    path_and_query: &str,
    response_file: &str,
    status_code: u16,
) -> (Client, MockServer) {
    let mock_server = MockServer::start().await;
    let mock_response = std::fs::read_to_string(format!(
        "docs/krx-api-reference/KRX_API_Spec/samples/{response_file}"
    ))
    .unwrap_or_else(|_| "{}".to_string()); // 샘플 파일 없으면 빈 JSON 객체 사용

    let mut path_parts = path_and_query.split('?');
    let path_str = path_parts.next().unwrap();
    let query_str = path_parts.next();

    let mut mock_builder = Mock::given(method("GET")).and(path(path_str));

    if let Some(q_str) = query_str {
        if !q_str.is_empty() {
            for part in q_str.split('&') {
                let mut kv = part.split('=');
                if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
                    mock_builder = mock_builder.and(query_param(key, value));
                }
            }
        }
    }

    mock_builder
        .and(header("AUTH_KEY", "test_key"))
        .respond_with(ResponseTemplate::new(status_code).set_body_string(&mock_response))
        .mount(&mock_server)
        .await;

    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    (client, mock_server)
}

#[tokio::test]
async fn test_client_creation() {
    let client = Client::new("test_auth_key");
    assert_eq!(client.get_base_url(), "http://data-dbg.krx.co.kr/svc/apis");
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
    assert_eq!(client.get_base_url(), "http://test.example.com");
}

#[tokio::test]
async fn test_client_builder_missing_auth_key() {
    let result = Client::builder().build();
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[tokio::test]
async fn test_successful_api_call() {
    let (client, _mock_server) = setup_test_client(
        "/idx/krx_dd_trd?basDd=20240105",
        "krx_dd_trd_20240105.json",
        200,
    )
    .await;
    let result = client.index().krx_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_api_error_404() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/idx/krx_dd_trd"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
        .mount(&mock_server)
        .await;
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
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
async fn test_rate_limit_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/idx/krx_dd_trd"))
        .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "60"))
        .mount(&mock_server)
        .await;
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
    let result = client.index().krx_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::RateLimit { .. })));
}

#[tokio::test]
async fn test_json_parsing_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/idx/krx_dd_trd"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Invalid JSON"))
        .mount(&mock_server)
        .await;
    let client = Client::builder()
        .auth_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
    let result = client.index().krx_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Parsing { .. })));
}

#[tokio::test]
async fn test_network_error() {
    let client = Client::builder()
        .auth_key("test_key")
        .base_url("http://localhost:12345")
        .build()
        .unwrap();
    let result = client.index().krx_daily().date("20240105").fetch().await;
    assert!(matches!(result, Err(Error::Network(_))));
}

// --- API Endpoint Tests ---

#[tokio::test]
async fn test_stock_kospi_daily_api() {
    let (client, _server) = setup_test_client(
        "/sto/stk_bydd_trd?basDd=20240105",
        "stk_bydd_trd_20240105.json",
        200,
    )
    .await;
    let result = client.stock().kospi_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_kts_daily_api() {
    let (client, _server) = setup_test_client(
        "/bon/kts_bydd_trd?basDd=20240105",
        "kts_bydd_trd_20240105.json",
        200,
    )
    .await;
    let result = client.bond().kts_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_etp_etf_daily_api() {
    let (client, _server) = setup_test_client(
        "/etp/etf_bydd_trd?basDd=20240105",
        "etf_bydd_trd_20240105.json",
        200,
    )
    .await;
    let result = client.etp().etf_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_derivative_futures_daily_api() {
    let (client, _server) = setup_test_client(
        "/drv/fut_bydd_trd?basDd=20240105",
        "fut_bydd_trd_20240105.json",
        200,
    )
    .await;
    let result = client
        .derivative()
        .futures_daily()
        .date("20240105")
        .fetch()
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_general_oil_daily_api() {
    let (client, _server) = setup_test_client(
        "/gen/oil_bydd_trd?basDd=20240105",
        "oil_bydd_trd_20240105.json",
        200,
    )
    .await;
    let result = client.general().oil_daily().date("20240105").fetch().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_esg_sri_bond_info_api() {
    let (client, _server) = setup_test_client(
        "/esg/sri_bond_info?basDd=20240105",
        "sri_bond_info_20240105.json",
        200,
    )
    .await;
    let result = client.esg().sri_bond_info().date("20240105").fetch().await;
    assert!(result.is_ok());
}
