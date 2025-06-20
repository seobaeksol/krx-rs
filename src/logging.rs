use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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

/// 로깅 초기화
pub fn init_logging(config: &LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&config.level))?;

    let subscriber = tracing_subscriber::registry()
        .with(env_filter);

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