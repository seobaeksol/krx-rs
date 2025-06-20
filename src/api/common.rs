use crate::error::{Error, Result};
use chrono::{Duration, Local};

/// 날짜 형식 검증 (YYYYMMDD)
pub fn is_valid_date_format(date: &str) -> bool {
    date.len() == 8 && date.chars().all(|c| c.is_numeric())
}

/// 기준일자 파라미터 검증 및 변환
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
