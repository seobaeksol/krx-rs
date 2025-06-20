use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

pub mod bond;
pub mod derivative;
pub mod esg;
pub mod etp;
pub mod general;
pub mod index;
pub mod stock;

/// KRX API 공통 응답 구조
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(rename = "OutBlock_1")]
    pub data: Vec<T>,
}

/// KRX 날짜 형식(YYYYMMDD)을 NaiveDate로 변환하는 헬퍼
pub fn deserialize_krx_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y%m%d").map_err(serde::de::Error::custom)
}

/// 옵셔널 KRX 날짜 형식을 변환하는 헬퍼
pub fn deserialize_optional_krx_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() || s == "-" {
        Ok(None)
    } else {
        NaiveDate::parse_from_str(&s, "%Y%m%d")
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}

/// 문자열을 Option<f64>로 파싱하는 헬퍼 (콤마 제거, "-" 처리)
pub fn deserialize_optional_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() || s == "-" {
        Ok(None)
    } else {
        s.replace(',', "")
            .parse::<f64>()
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}

/// 문자열을 Option<u64>로 파싱하는 헬퍼 (거래량 등의 정수값용)
pub fn deserialize_optional_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() || s == "-" {
        Ok(None)
    } else {
        s.replace(',', "")
            .parse::<u64>()
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}

/// 백분율 문자열을 Option<f64>로 파싱하는 헬퍼
pub fn deserialize_optional_percentage<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() || s == "-" {
        Ok(None)
    } else {
        s.parse::<f64>().map(Some).map_err(serde::de::Error::custom)
    }
}

/// 문자열을 f64로 파싱하는 헬퍼 (필수값용)
pub fn deserialize_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.replace(',', "")
        .parse::<f64>()
        .map_err(serde::de::Error::custom)
}

/// 문자열을 u64로 파싱하는 헬퍼 (필수값용)
pub fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.replace(',', "")
        .parse::<u64>()
        .map_err(serde::de::Error::custom)
}
