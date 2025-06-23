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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::{self, Deserializer, Visitor};
    use serde_json::Value;

    struct TestDeserializer {
        value: Value,
    }

    impl<'de> Deserializer<'de> for TestDeserializer {
        type Error = serde_json::Error;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.value {
                Value::String(s) => visitor.visit_string(s),
                _ => Err(de::Error::custom("expected string")),
            }
        }

        fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.deserialize_any(visitor)
        }

        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str
            bytes byte_buf option unit unit_struct newtype_struct seq tuple
            tuple_struct map struct enum identifier ignored_any
        }
    }

    #[test]
    fn test_deserialize_krx_date_valid() {
        let deserializer = TestDeserializer {
            value: Value::String("20240105".to_string()),
        };
        let result = deserialize_krx_date(deserializer);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 5).unwrap()
        );
    }

    #[test]
    fn test_deserialize_krx_date_invalid() {
        let deserializer = TestDeserializer {
            value: Value::String("invalid-date".to_string()),
        };
        let result = deserialize_krx_date(deserializer);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_optional_krx_date_valid() {
        let deserializer = TestDeserializer {
            value: Value::String("20240105".to_string()),
        };
        let result = deserialize_optional_krx_date(deserializer);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(NaiveDate::from_ymd_opt(2024, 1, 5).unwrap())
        );
    }

    #[test]
    fn test_deserialize_optional_krx_date_empty() {
        let deserializer = TestDeserializer {
            value: Value::String("".to_string()),
        };
        let result = deserialize_optional_krx_date(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_deserialize_optional_krx_date_dash() {
        let deserializer = TestDeserializer {
            value: Value::String("-".to_string()),
        };
        let result = deserialize_optional_krx_date(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_deserialize_optional_f64_valid() {
        let deserializer = TestDeserializer {
            value: Value::String("123.45".to_string()),
        };
        let result = deserialize_optional_f64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(123.45));
    }

    #[test]
    fn test_deserialize_optional_f64_with_comma() {
        let deserializer = TestDeserializer {
            value: Value::String("1,234.56".to_string()),
        };
        let result = deserialize_optional_f64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1234.56));
    }

    #[test]
    fn test_deserialize_optional_f64_empty() {
        let deserializer = TestDeserializer {
            value: Value::String("".to_string()),
        };
        let result = deserialize_optional_f64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_deserialize_optional_u64_valid() {
        let deserializer = TestDeserializer {
            value: Value::String("12345".to_string()),
        };
        let result = deserialize_optional_u64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(12345));
    }

    #[test]
    fn test_deserialize_optional_u64_with_comma() {
        let deserializer = TestDeserializer {
            value: Value::String("1,234,567".to_string()),
        };
        let result = deserialize_optional_u64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1234567));
    }

    #[test]
    fn test_deserialize_f64_valid() {
        let deserializer = TestDeserializer {
            value: Value::String("123.45".to_string()),
        };
        let result = deserialize_f64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 123.45);
    }

    #[test]
    fn test_deserialize_f64_invalid() {
        let deserializer = TestDeserializer {
            value: Value::String("invalid".to_string()),
        };
        let result = deserialize_f64(deserializer);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_u64_valid() {
        let deserializer = TestDeserializer {
            value: Value::String("12345".to_string()),
        };
        let result = deserialize_u64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12345);
    }

    #[test]
    fn test_deserialize_u64_invalid() {
        let deserializer = TestDeserializer {
            value: Value::String("invalid".to_string()),
        };
        let result = deserialize_u64(deserializer);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_optional_percentage_valid() {
        let deserializer = TestDeserializer {
            value: Value::String("12.5".to_string()),
        };
        let result = deserialize_optional_percentage(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(12.5));
    }

    #[test]
    fn test_deserialize_optional_percentage_dash() {
        let deserializer = TestDeserializer {
            value: Value::String("-".to_string()),
        };
        let result = deserialize_optional_percentage(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_deserialize_optional_f64_dash() {
        let deserializer = TestDeserializer {
            value: Value::String("-".to_string()),
        };
        let result = deserialize_optional_f64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_deserialize_optional_u64_dash() {
        let deserializer = TestDeserializer {
            value: Value::String("-".to_string()),
        };
        let result = deserialize_optional_u64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_deserialize_optional_u64_empty() {
        let deserializer = TestDeserializer {
            value: Value::String("".to_string()),
        };
        let result = deserialize_optional_u64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_deserialize_optional_percentage_empty() {
        let deserializer = TestDeserializer {
            value: Value::String("".to_string()),
        };
        let result = deserialize_optional_percentage(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_deserialize_optional_f64_zero() {
        let deserializer = TestDeserializer {
            value: Value::String("0".to_string()),
        };
        let result = deserialize_optional_f64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(0.0));
    }

    #[test]
    fn test_deserialize_optional_u64_zero() {
        let deserializer = TestDeserializer {
            value: Value::String("0".to_string()),
        };
        let result = deserialize_optional_u64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(0));
    }

    #[test]
    fn test_deserialize_krx_date_strict_validation() {
        let invalid_formats = vec![
            "2024-01-05", // Wrong format
            "05/01/2024", // Wrong format
            "20240230",   // Invalid date
            "20241301",   // Invalid month
        ];

        for invalid_date in invalid_formats {
            let deserializer = TestDeserializer {
                value: Value::String(invalid_date.to_string()),
            };
            let result = deserialize_krx_date(deserializer);
            assert!(
                result.is_err(),
                "Should reject invalid date format: {}",
                invalid_date
            );
        }
    }

    #[test]
    fn test_deserialize_sample_data_patterns() {
        // Test patterns found in actual KRX API sample data

        // Price field with dash (from options data where no trading occurred)
        let deserializer = TestDeserializer {
            value: Value::String("-".to_string()),
        };
        let result = deserialize_optional_f64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);

        // Volume field with zero (valid zero trading)
        let deserializer = TestDeserializer {
            value: Value::String("0".to_string()),
        };
        let result = deserialize_optional_u64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(0));

        // Large trading value with no commas (as found in sample data)
        let deserializer = TestDeserializer {
            value: Value::String("104622050000".to_string()),
        };
        let result = deserialize_optional_u64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(104622050000));

        // Decimal yield value (as found in bond data)
        let deserializer = TestDeserializer {
            value: Value::String("3.381".to_string()),
        };
        let result = deserialize_optional_f64(deserializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(3.381));

        // Valid KRX date format
        let deserializer = TestDeserializer {
            value: Value::String("20240105".to_string()),
        };
        let result = deserialize_krx_date(deserializer);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 5).unwrap()
        );
    }
}
