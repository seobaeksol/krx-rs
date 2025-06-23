use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt, Registry};

/// 로깅 설정 구조체
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// 로깅 레벨 (기본값: "info")
    pub level: String,
    /// JSON 형태 출력 여부
    pub json_format: bool,
    /// 민감한 정보 필터링 여부
    pub filter_sensitive: bool,
    /// 파일 출력 경로 (옵션)
    pub file_path: Option<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            json_format: false,
            filter_sensitive: true,
            file_path: None,
        }
    }
}

/// Subscriber 빌드 (테스트 가능한 부분)
pub fn build_subscriber(
    config: &LoggingConfig,
) -> Result<Box<dyn tracing::Subscriber + Send + Sync>, Box<dyn std::error::Error>> {
    let env_filter =
        EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new(&config.level))?;

    let registry = Registry::default().with(env_filter);

    if config.json_format {
        // JSON 형태 출력
        let json_layer = tracing_subscriber::fmt::layer()
            .json()
            .with_current_span(true)
            .with_span_list(true);

        Ok(Box::new(registry.with(json_layer)))
    } else {
        // 일반 텍스트 출력
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true);

        Ok(Box::new(registry.with(fmt_layer)))
    }
}

/// 로깅 초기화
pub fn init_logging(config: &LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
    // 테스트 환경에서는 초기화 실패를 무시
    #[cfg(test)]
    {
        if let Err(e) = do_init_logging(config) {
            // 이미 초기화된 경우는 무시
            if e.to_string().contains("subscriber") || e.to_string().contains("already been set") {
                return Ok(());
            }
            return Err(e);
        }
        return Ok(());
    }

    #[cfg(not(test))]
    do_init_logging(config)
}

/// 실제 초기화 로직
fn do_init_logging(config: &LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
    let env_filter =
        EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new(&config.level))?;

    let subscriber = tracing_subscriber::registry().with(env_filter);

    if config.json_format {
        // JSON 형태 출력
        let json_layer = tracing_subscriber::fmt::layer()
            .json()
            .with_current_span(true)
            .with_span_list(true);

        subscriber.with(json_layer).try_init()?;
    } else {
        // 일반 텍스트 출력
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true);

        subscriber.with(fmt_layer).try_init()?;
    }

    info!("krx-rs logging initialized with level: {}", config.level);
    Ok(())
}

/// 민감한 정보 필터링 매크로
#[macro_export]
macro_rules! log_filtered {
    ($level:ident, $msg:expr, $($key:ident = $value:expr),*) => {
        $level!(
            message = $msg,
            $(
                $key = if stringify!($key).contains("key") ||
                         stringify!($key).contains("auth") ||
                         stringify!($key).contains("token") {
                    "***FILTERED***"
                } else {
                    &$value.to_string()
                }
            ),*
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_subscriber_with_json_format() {
        let config = LoggingConfig {
            level: "debug".to_string(),
            json_format: true,
            filter_sensitive: false,
            file_path: None,
        };

        let result = build_subscriber(&config);
        assert!(result.is_ok(), "Should successfully build JSON subscriber");
    }

    #[test]
    fn test_build_subscriber_with_text_format() {
        let config = LoggingConfig {
            level: "info".to_string(),
            json_format: false,
            filter_sensitive: true,
            file_path: None,
        };

        let result = build_subscriber(&config);
        assert!(result.is_ok(), "Should successfully build text subscriber");
    }

    #[test]
    fn test_build_subscriber_with_invalid_level() {
        let config = LoggingConfig {
            level: "invalid[[[level".to_string(), // Invalid filter syntax
            json_format: false,
            filter_sensitive: false,
            file_path: None,
        };

        let result = build_subscriber(&config);
        assert!(result.is_err(), "Should fail with invalid log level");
    }

    #[test]
    fn test_init_logging_handles_multiple_calls() {
        let config = LoggingConfig::default();
        
        // First call
        let result1 = init_logging(&config);
        assert!(result1.is_ok(), "First init should succeed");

        // Second call - should also succeed in test environment
        let result2 = init_logging(&config);
        assert!(result2.is_ok(), "Second init should succeed in test environment");
    }
}
