use crate::error::{Error, Result};
use chrono::{Duration, Local};

/// 날짜 문자열이 "YYYYMMDD" 형식인지 검증합니다.
pub fn is_valid_date_format(date: &str) -> bool {
    date.len() == 8 && date.chars().all(|c| c.is_numeric())
}

/// `Option<String>` 형태의 날짜를 검증하고, 유효하면 `String`을 반환합니다.
///
/// # Errors
/// - 날짜가 `None`일 경우 `Error::InvalidInput` 반환.
/// - 날짜 형식이 유효하지 않을 경우 `Error::InvalidInput` 반환.
pub fn validate_base_date(base_date: Option<String>) -> Result<String> {
    let date = base_date.ok_or_else(|| {
        Error::InvalidInput("date is required, call `date()` or `latest()`".to_string())
    })?;

    if !is_valid_date_format(&date) {
        return Err(Error::InvalidInput(
            "date must be in YYYYMMDD format".to_string(),
        ));
    }

    Ok(date)
}

/// KRX API 조회를 위한 가장 최신 거래일(보통 전일)의 날짜 문자열을 생성합니다.
pub fn latest_workday_string() -> String {
    let yesterday = Local::now() - Duration::days(1);
    yesterday.format("%Y%m%d").to_string()
}

/// 기본 빌더 매크로 - 공통 날짜 설정 메서드들을 생성
#[macro_export]
macro_rules! impl_date_builder_methods {
    ($builder_type:ty) => {
        impl<'a> $builder_type {
            /// 조회 기준일자 설정 (YYYYMMDD).
            ///
            /// KRX 데이터는 2010년 이후부터 조회일 기준 전일까지만 제공됩니다.
            ///
            /// # Example
            /// ```
            /// .date("20240105")
            /// ```
            pub fn date(mut self, date: impl Into<String>) -> Self {
                self.base_date = Some(date.into());
                self
            }

            /// 가장 최신 거래일(보통 전일)의 데이터로 설정합니다.
            pub fn latest(mut self) -> Self {
                self.base_date = Some($crate::api::common::latest_workday_string());
                self
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_date_format() {
        assert!(is_valid_date_format("20240105"));
        assert!(is_valid_date_format("19991231"));
        assert!(!is_valid_date_format("2024-01-05"));
        assert!(!is_valid_date_format("240105"));
        assert!(!is_valid_date_format("2024010a"));
        assert!(!is_valid_date_format(""));
    }

    #[test]
    fn test_validate_base_date_none() {
        let result = validate_base_date(None);
        assert!(result.is_err());
        match result {
            Err(Error::InvalidInput(msg)) => {
                assert_eq!(msg, "date is required, call `date()` or `latest()`");
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_validate_base_date_invalid_format() {
        let result = validate_base_date(Some("2024-01-05".to_string()));
        assert!(result.is_err());
        match result {
            Err(Error::InvalidInput(msg)) => {
                assert_eq!(msg, "date must be in YYYYMMDD format");
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_validate_base_date_valid() {
        let result = validate_base_date(Some("20240105".to_string()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "20240105");
    }

    #[test]
    fn test_latest_workday_string() {
        let date = latest_workday_string();
        assert_eq!(date.len(), 8);
        assert!(date.chars().all(|c| c.is_numeric()));
    }
}
