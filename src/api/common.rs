use crate::error::{Error, Result};

/// 날짜 형식 검증 (YYYYMMDD)
pub fn is_valid_date_format(date: &str) -> bool {
    date.len() == 8 && date.chars().all(|c| c.is_numeric())
}

/// 기준일자 파라미터 검증 및 변환
pub fn validate_base_date(base_date: Option<String>) -> Result<String> {
    let date = base_date
        .ok_or_else(|| Error::InvalidInput("date is required".to_string()))?;
    
    if !is_valid_date_format(&date) {
        return Err(Error::InvalidInput(
            "date must be in YYYYMMDD format".to_string()
        ));
    }
    
    Ok(date)
}

/// 오늘 날짜를 YYYYMMDD 형식으로 반환
pub fn today_string() -> String {
    use chrono::Local;
    Local::now().format("%Y%m%d").to_string()
}

/// 기본 빌더 매크로 - 공통 날짜 설정 메서드들을 생성
#[macro_export]
macro_rules! impl_date_builder_methods {
    ($builder_type:ty) => {
        impl<'a> $builder_type {
            /// 조회 기준일자 설정 (YYYYMMDD)
            ///
            /// # Example
            /// ```
            /// .date("20240105")
            /// ```
            pub fn date(mut self, date: impl Into<String>) -> Self {
                self.base_date = Some(date.into());
                self
            }

            /// 오늘 날짜로 설정
            pub fn today(mut self) -> Self {
                self.base_date = Some(crate::api::common::today_string());
                self
            }
        }
    };
}