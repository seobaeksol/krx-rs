use chrono::NaiveDate;
use krx_rs::data::{
    deserialize_f64, deserialize_krx_date, deserialize_optional_f64,
    deserialize_optional_percentage, deserialize_optional_u64, deserialize_u64,
};
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct TestKrxDate {
    #[serde(deserialize_with = "deserialize_krx_date")]
    date: NaiveDate,
}

#[derive(Deserialize)]
struct TestOptionalF64 {
    #[serde(deserialize_with = "deserialize_optional_f64")]
    value: Option<f64>,
}

#[derive(Deserialize)]
struct TestOptionalU64 {
    #[serde(deserialize_with = "deserialize_optional_u64")]
    value: Option<u64>,
}

#[derive(Deserialize)]
struct TestOptionalPercentage {
    #[serde(deserialize_with = "deserialize_optional_percentage")]
    value: Option<f64>,
}

#[derive(Deserialize)]
struct TestF64 {
    #[serde(deserialize_with = "deserialize_f64")]
    value: f64,
}

#[derive(Deserialize)]
struct TestU64 {
    #[serde(deserialize_with = "deserialize_u64")]
    value: u64,
}

#[test]
fn test_deserialize_krx_date_success() {
    let json = r#"{"date": "20240105"}"#;
    let result: TestKrxDate = serde_json::from_str(json).unwrap();

    let expected = NaiveDate::from_ymd_opt(2024, 1, 5).unwrap();
    assert_eq!(result.date, expected);

    // 다른 유효한 날짜들
    let test_cases = vec![
        ("20000101", NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        ("19991231", NaiveDate::from_ymd_opt(1999, 12, 31).unwrap()),
        ("20240229", NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()), // 윤년
    ];

    for (date_str, expected_date) in test_cases {
        let json = format!(r#"{{"date": "{}"}}"#, date_str);
        let result: TestKrxDate = serde_json::from_str(&json).unwrap();
        assert_eq!(result.date, expected_date);
    }
}

#[test]
fn test_deserialize_krx_date_invalid() {
    let invalid_dates = vec![
        r#"{"date": "2024-01-05"}"#, // 하이픈 포함
        r#"{"date": "202401"}"#,     // 너무 짧음
        r#"{"date": "202401050"}"#,  // 너무 긺
        r#"{"date": "2024010a"}"#,   // 숫자가 아닌 문자
        r#"{"date": ""}"#,           // 빈 문자열
        r#"{"date": "20241301"}"#,   // 잘못된 월
        r#"{"date": "20240132"}"#,   // 잘못된 일
    ];

    for invalid_json in invalid_dates {
        let result: Result<TestKrxDate, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err(), "Should fail for: {}", invalid_json);
    }
}

#[test]
fn test_deserialize_optional_f64_success() {
    // 유효한 숫자
    let json = r#"{"value": "123.45"}"#;
    let result: TestOptionalF64 = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(123.45));

    // 콤마가 포함된 숫자
    let json = r#"{"value": "1,234.56"}"#;
    let result: TestOptionalF64 = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(1234.56));

    // 큰 숫자
    let json = r#"{"value": "1,234,567.89"}"#;
    let result: TestOptionalF64 = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(1234567.89));

    // 음수
    let json = r#"{"value": "-123.45"}"#;
    let result: TestOptionalF64 = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(-123.45));
}

#[test]
fn test_deserialize_optional_f64_none() {
    let none_cases = vec![
        r#"{"value": ""}"#,  // 빈 문자열
        r#"{"value": "-"}"#, // 대시
    ];

    for json in none_cases {
        let result: TestOptionalF64 = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, None, "Should be None for: {}", json);
    }
}

#[test]
fn test_deserialize_optional_f64_invalid() {
    let invalid_cases = vec![
        r#"{"value": "abc"}"#,      // 숫자가 아님
        r#"{"value": "12.34.56"}"#, // 잘못된 형식
    ];

    for json in invalid_cases {
        let result: Result<TestOptionalF64, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Should fail for: {}", json);
    }
}

#[test]
fn test_deserialize_optional_u64_success() {
    // 유효한 정수
    let json = r#"{"value": "12345"}"#;
    let result: TestOptionalU64 = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(12345));

    // 콤마가 포함된 정수
    let json = r#"{"value": "1,234,567"}"#;
    let result: TestOptionalU64 = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(1234567));

    // 0
    let json = r#"{"value": "0"}"#;
    let result: TestOptionalU64 = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(0));
}

#[test]
fn test_deserialize_optional_u64_none() {
    let none_cases = vec![
        r#"{"value": ""}"#,  // 빈 문자열
        r#"{"value": "-"}"#, // 대시
    ];

    for json in none_cases {
        let result: TestOptionalU64 = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, None, "Should be None for: {}", json);
    }
}

#[test]
fn test_deserialize_optional_u64_invalid() {
    let invalid_cases = vec![
        r#"{"value": "abc"}"#,   // 숫자가 아님
        r#"{"value": "-123"}"#,  // 음수 (u64는 음수 불가)
        r#"{"value": "12.34"}"#, // 소수점
    ];

    for json in invalid_cases {
        let result: Result<TestOptionalU64, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Should fail for: {}", json);
    }
}

#[test]
fn test_deserialize_optional_percentage_success() {
    // 일반 백분율
    let json = r#"{"value": "12.34"}"#;
    let result: TestOptionalPercentage = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(12.34));

    // 음수 백분율
    let json = r#"{"value": "-5.67"}"#;
    let result: TestOptionalPercentage = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(-5.67));

    // 0
    let json = r#"{"value": "0"}"#;
    let result: TestOptionalPercentage = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(0.0));
}

#[test]
fn test_deserialize_optional_percentage_none() {
    let none_cases = vec![
        r#"{"value": ""}"#,  // 빈 문자열
        r#"{"value": "-"}"#, // 대시
    ];

    for json in none_cases {
        let result: TestOptionalPercentage = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, None, "Should be None for: {}", json);
    }
}

#[test]
fn test_deserialize_f64_success() {
    let json = r#"{"value": "123.45"}"#;
    let result: TestF64 = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, 123.45);

    // 콤마 제거 테스트
    let json = r#"{"value": "1,234.56"}"#;
    let result: TestF64 = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, 1234.56);
}

#[test]
fn test_deserialize_f64_invalid() {
    let json = r#"{"value": "abc"}"#;
    let result: Result<TestF64, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_u64_success() {
    let json = r#"{"value": "12345"}"#;
    let result: TestU64 = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, 12345);

    // 콤마 제거 테스트
    let json = r#"{"value": "1,234,567"}"#;
    let result: TestU64 = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, 1234567);
}

#[test]
fn test_deserialize_u64_invalid() {
    let invalid_cases = vec![
        r#"{"value": "abc"}"#,   // 숫자가 아님
        r#"{"value": "-123"}"#,  // 음수
        r#"{"value": "12.34"}"#, // 소수점
    ];

    for json in invalid_cases {
        let result: Result<TestU64, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Should fail for: {}", json);
    }
}
