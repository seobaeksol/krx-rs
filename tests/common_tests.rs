use chrono::Local;
use krx_rs::api::common::{is_valid_date_format, today_string, validate_base_date};
use krx_rs::error::Error;

#[test]
fn test_is_valid_date_format() {
    // 유효한 날짜 형식
    assert!(is_valid_date_format("20240105"));
    assert!(is_valid_date_format("19991231"));
    assert!(is_valid_date_format("20000101"));

    // 유효하지 않은 형식
    assert!(!is_valid_date_format("2024-01-05")); // 하이픈 포함
    assert!(!is_valid_date_format("202401")); // 너무 짧음
    assert!(!is_valid_date_format("202401050")); // 너무 긺
    assert!(!is_valid_date_format("2024010a")); // 숫자가 아닌 문자 포함
    assert!(!is_valid_date_format("")); // 빈 문자열
    assert!(!is_valid_date_format("abcdefgh")); // 모두 문자
}

#[test]
fn test_validate_base_date_success() {
    // 유효한 날짜
    let result = validate_base_date(Some("20240105".to_string()));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "20240105");

    let result = validate_base_date(Some("19991231".to_string()));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "19991231");
}

#[test]
fn test_validate_base_date_none() {
    let result = validate_base_date(None);
    assert!(result.is_err());

    if let Err(Error::InvalidInput(msg)) = result {
        assert_eq!(msg, "date is required");
    } else {
        panic!("Expected InvalidInput error");
    }
}

#[test]
fn test_validate_base_date_invalid_format() {
    let test_cases = vec![
        "2024-01-05", // 하이픈 포함
        "202401",     // 너무 짧음
        "202401050",  // 너무 긺
        "2024010a",   // 숫자가 아닌 문자
        "",           // 빈 문자열
        "abcdefgh",   // 모두 문자
    ];

    for invalid_date in test_cases {
        let result = validate_base_date(Some(invalid_date.to_string()));
        assert!(result.is_err());

        if let Err(Error::InvalidInput(msg)) = result {
            assert_eq!(msg, "date must be in YYYYMMDD format");
        } else {
            panic!("Expected InvalidInput error for: {}", invalid_date);
        }
    }
}

#[test]
fn test_today_string() {
    let today = today_string();

    // 길이가 8이어야 함 (YYYYMMDD)
    assert_eq!(today.len(), 8);

    // 모든 문자가 숫자여야 함
    assert!(today.chars().all(|c| c.is_numeric()));

    // 실제 오늘 날짜와 일치하는지 확인
    let expected = Local::now().format("%Y%m%d").to_string();
    assert_eq!(today, expected);

    // 날짜 형식 검증 통과해야 함
    assert!(is_valid_date_format(&today));
}

#[test]
fn test_today_string_format() {
    let today = today_string();

    // YYYY 부분 (연도) 검증
    let year: i32 = today[0..4].parse().expect("Year should be numeric");
    assert!(year >= 2024 && year <= 2100);

    // MM 부분 (월) 검증
    let month: u32 = today[4..6].parse().expect("Month should be numeric");
    assert!(month >= 1 && month <= 12);

    // DD 부분 (일) 검증
    let day: u32 = today[6..8].parse().expect("Day should be numeric");
    assert!(day >= 1 && day <= 31);
}
