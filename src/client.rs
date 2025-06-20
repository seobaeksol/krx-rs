use crate::{
    api,
    error::{Error, Result},
    logging::LoggingConfig,
};
use reqwest::{
    Client as HttpClient,
    header::{HeaderMap, HeaderValue},
};
use serde::de::DeserializeOwned;
use std::time::Duration;
use tracing::{debug, error, info, instrument, warn};

const BASE_URL: &str = "http://data-dbg.krx.co.kr/svc/apis";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// KRX API 클라이언트
pub struct Client {
    http_client: HttpClient,
    auth_key: String,
    base_url: String,
}

impl Client {
    /// 새로운 클라이언트 인스턴스 생성
    ///
    /// # Arguments
    /// * `auth_key` - KRX Open API 인증키
    ///
    /// # Example
    /// ```
    /// use krx_rs::Client;
    /// let client = Client::new("your_auth_key");
    /// ```
    pub fn new(auth_key: impl Into<String>) -> Self {
        Self::builder()
            .auth_key(auth_key)
            .build()
            .expect("Failed to build client with default settings")
    }

    /// 로깅 설정과 함께 클라이언트 생성
    pub fn with_logging(
        auth_key: impl Into<String>,
        logging_config: LoggingConfig,
    ) -> Result<Self> {
        crate::logging::init_logging(&logging_config)
            .map_err(|e| Error::InvalidInput(format!("Failed to initialize logging: {}", e)))?;
        Ok(Self::new(auth_key))
    }

    /// 클라이언트 빌더 반환
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// 내부 HTTP GET 요청 처리
    #[instrument(skip(self, params), fields(endpoint = %endpoint))]
    pub(crate) async fn get<T>(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let start_time = std::time::Instant::now();

        // 요청 시작 로깅
        info!(
            endpoint = %endpoint,
            params_count = params.len(),
            "Starting API request"
        );

        debug!(
            url = %url,
            params = ?params,
            "Request details"
        );

        let response = self
            .http_client
            .get(&url)
            .header("AUTH_KEY", &self.auth_key)
            .query(params)
            .send()
            .await
            .map_err(|e| {
                error!(
                    endpoint = %endpoint,
                    error = %e,
                    duration_ms = start_time.elapsed().as_millis(),
                    "Network request failed"
                );
                Error::Network(e)
            })?;

        let status_code = response.status().as_u16();
        let duration = start_time.elapsed();

        if response.status().is_success() {
            let body = response.text().await?;

            debug!(
                endpoint = %endpoint,
                status_code = status_code,
                response_size = body.len(),
                duration_ms = duration.as_millis(),
                "Received successful response"
            );

            // 응답 파싱
            match serde_json::from_str(&body) {
                Ok(parsed) => {
                    info!(
                        endpoint = %endpoint,
                        status_code = status_code,
                        duration_ms = duration.as_millis(),
                        "API request completed successfully"
                    );
                    Ok(parsed)
                }
                Err(e) => {
                    error!(
                        endpoint = %endpoint,
                        error = %e,
                        response_body = %body.chars().take(500).collect::<String>(),
                        "Failed to parse response"
                    );
                    Err(Error::Parsing {
                        details: format!("Failed to deserialize response from {}", endpoint),
                        source: e,
                        response_body: body,
                    })
                }
            }
        } else {
            // Rate limiting 특별 처리
            if status_code == 429 {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60);

                warn!(
                    endpoint = %endpoint,
                    retry_after = retry_after,
                    duration_ms = duration.as_millis(),
                    "Rate limit exceeded"
                );

                return Err(Error::RateLimit { retry_after });
            }

            let message = response.text().await.unwrap_or_default();

            error!(
                endpoint = %endpoint,
                status_code = status_code,
                error_message = %message,
                duration_ms = duration.as_millis(),
                "API request failed"
            );

            Err(Error::ApiError {
                status_code,
                message,
            })
        }
    }

    /// 주식 API 접근
    pub fn stock(&self) -> api::stock::StockApi {
        api::stock::StockApi::new(self)
    }

    /// 지수 API 접근
    pub fn index(&self) -> api::index::IndexApi {
        api::index::IndexApi::new(self)
    }

    /// 채권 API 접근
    pub fn bond(&self) -> api::bond::BondApi {
        api::bond::BondApi::new(self)
    }

    /// ETP API 접근
    pub fn etp(&self) -> api::etp::EtpApi {
        api::etp::EtpApi::new(self)
    }

    /// 파생상품 API 접근
    pub fn derivative(&self) -> api::derivative::DerivativeApi {
        api::derivative::DerivativeApi::new(self)
    }

    /// 일반상품 API 접근
    pub fn general(&self) -> api::general::GeneralApi {
        api::general::GeneralApi::new(self)
    }

    /// ESG API 접근
    pub fn esg(&self) -> api::esg::EsgApi {
        api::esg::EsgApi::new(self)
    }

    /// 현재 클라이언트의 기본 URL을 반환합니다. (테스트용)
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }
}

/// 클라이언트 빌더
#[derive(Default)]
pub struct ClientBuilder {
    auth_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    user_agent: Option<String>,
    logging_config: Option<LoggingConfig>,
}

impl ClientBuilder {
    /// 인증키 설정
    pub fn auth_key(mut self, key: impl Into<String>) -> Self {
        self.auth_key = Some(key.into());
        self
    }

    /// 기본 URL 재정의 (테스트용)
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// 요청 타임아웃 설정
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// User-Agent 헤더 설정
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// 로깅 설정 추가
    pub fn logging(mut self, config: LoggingConfig) -> Self {
        self.logging_config = Some(config);
        self
    }

    /// 클라이언트 빌드
    pub fn build(self) -> Result<Client> {
        let auth_key = self
            .auth_key
            .ok_or_else(|| Error::InvalidInput("auth_key is required".to_string()))?;

        // 로깅 초기화
        if let Some(config) = &self.logging_config {
            crate::logging::init_logging(config)
                .map_err(|e| Error::InvalidInput(format!("Failed to initialize logging: {}", e)))?;
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            "AUTH_KEY",
            HeaderValue::from_str(&auth_key)
                .map_err(|_| Error::InvalidInput("Invalid auth_key format".to_string()))?,
        );
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let http_client = HttpClient::builder()
            .default_headers(headers)
            .timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
            .user_agent(
                self.user_agent
                    .unwrap_or_else(|| format!("krx-rs/{}", env!("CARGO_PKG_VERSION"))),
            )
            .build()?;

        let base_url = self.base_url.unwrap_or_else(|| BASE_URL.to_string());

        Ok(Client {
            http_client,
            auth_key,
            base_url,
        })
    }
}
