use super::{
    ApiResponse, deserialize_krx_date, deserialize_optional_f64, deserialize_optional_percentage,
    deserialize_optional_u64,
};
use crate::error::Result;
use chrono::NaiveDate;
use polars::prelude::*;
use serde::Deserialize;

/// 유가증권 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct KospiDailyRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,

    /// 종목코드
    #[serde(rename = "ISU_CD")]
    pub issue_code: String,

    /// 종목명
    #[serde(rename = "ISU_NM")]
    pub issue_name: String,

    /// 시장구분
    #[serde(rename = "MKT_NM")]
    pub market_name: String,

    /// 소속부
    #[serde(rename = "SECT_TP_NM")]
    pub sector_type: String,

    /// 종가
    #[serde(rename = "TDD_CLSPRC", deserialize_with = "deserialize_optional_f64")]
    pub close_price: Option<f64>,

    /// 대비
    #[serde(
        rename = "CMPPREVDD_PRC",
        deserialize_with = "deserialize_optional_f64"
    )]
    pub price_change: Option<f64>,

    /// 등락률 (%)
    #[serde(
        rename = "FLUC_RT",
        deserialize_with = "deserialize_optional_percentage"
    )]
    pub fluctuation_rate: Option<f64>,

    /// 시가
    #[serde(rename = "TDD_OPNPRC", deserialize_with = "deserialize_optional_f64")]
    pub open_price: Option<f64>,

    /// 고가
    #[serde(rename = "TDD_HGPRC", deserialize_with = "deserialize_optional_f64")]
    pub high_price: Option<f64>,

    /// 저가
    #[serde(rename = "TDD_LWPRC", deserialize_with = "deserialize_optional_f64")]
    pub low_price: Option<f64>,

    /// 거래량
    #[serde(rename = "ACC_TRDVOL", deserialize_with = "deserialize_optional_u64")]
    pub trading_volume: Option<u64>,

    /// 거래대금
    #[serde(rename = "ACC_TRDVAL", deserialize_with = "deserialize_optional_u64")]
    pub trading_value: Option<u64>,

    /// 시가총액
    #[serde(rename = "MKTCAP", deserialize_with = "deserialize_optional_u64")]
    pub market_cap: Option<u64>,

    /// 상장주식수
    #[serde(rename = "LIST_SHRS", deserialize_with = "deserialize_optional_u64")]
    pub listed_shares: Option<u64>,
}

/// 코스닥 일별매매정보 레코드 (KOSPI와 동일한 구조)
pub type KosdaqDailyRecord = KospiDailyRecord;

/// 코넥스 일별매매정보 레코드 (KOSPI와 동일한 구조)
pub type KonexDailyRecord = KospiDailyRecord;

/// 신주인수권증권 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct StockWarrantDailyRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,

    /// 시장구분
    #[serde(rename = "MKT_NM")]
    pub market_name: String,

    /// 종목코드
    #[serde(rename = "ISU_CD")]
    pub issue_code: String,

    /// 종목명
    #[serde(rename = "ISU_NM")]
    pub issue_name: String,

    /// 종가
    #[serde(rename = "TDD_CLSPRC", deserialize_with = "deserialize_optional_f64")]
    pub close_price: Option<f64>,

    /// 대비
    #[serde(
        rename = "CMPPREVDD_PRC",
        deserialize_with = "deserialize_optional_f64"
    )]
    pub price_change: Option<f64>,

    /// 등락률 (%)
    #[serde(
        rename = "FLUC_RT",
        deserialize_with = "deserialize_optional_percentage"
    )]
    pub fluctuation_rate: Option<f64>,

    /// 시가
    #[serde(rename = "TDD_OPNPRC", deserialize_with = "deserialize_optional_f64")]
    pub open_price: Option<f64>,

    /// 고가
    #[serde(rename = "TDD_HGPRC", deserialize_with = "deserialize_optional_f64")]
    pub high_price: Option<f64>,

    /// 저가
    #[serde(rename = "TDD_LWPRC", deserialize_with = "deserialize_optional_f64")]
    pub low_price: Option<f64>,

    /// 거래량
    #[serde(rename = "ACC_TRDVOL", deserialize_with = "deserialize_optional_u64")]
    pub trading_volume: Option<u64>,

    /// 거래대금
    #[serde(rename = "ACC_TRDVAL", deserialize_with = "deserialize_optional_u64")]
    pub trading_value: Option<u64>,

    /// 시가총액
    #[serde(rename = "MKTCAP", deserialize_with = "deserialize_optional_u64")]
    pub market_cap: Option<u64>,

    /// 상장증권수
    #[serde(rename = "LIST_SHRS", deserialize_with = "deserialize_optional_u64")]
    pub listed_shares: Option<u64>,

    /// 행사가격
    #[serde(rename = "EXER_PRC", deserialize_with = "deserialize_optional_f64")]
    pub exercise_price: Option<f64>,

    /// 존속기간_시작일
    #[serde(rename = "EXST_STRT_DD")]
    pub existence_start_date: String,

    /// 존속기간_종료일
    #[serde(rename = "EXST_END_DD")]
    pub existence_end_date: String,

    /// 목적주권_종목코드
    #[serde(rename = "TARSTK_ISU_SRT_CD")]
    pub target_stock_code: String,

    /// 목적주권_종목명
    #[serde(rename = "TARSTK_ISU_NM")]
    pub target_stock_name: String,

    /// 목적주권_종가
    #[serde(
        rename = "TARSTK_ISU_PRSNT_PRC",
        deserialize_with = "deserialize_optional_f64"
    )]
    pub target_stock_price: Option<f64>,
}

/// 신주인수권증서 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct StockRightDailyRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,

    /// 시장구분
    #[serde(rename = "MKT_NM")]
    pub market_name: String,

    /// 종목코드
    #[serde(rename = "ISU_CD")]
    pub issue_code: String,

    /// 종목명
    #[serde(rename = "ISU_NM")]
    pub issue_name: String,

    /// 종가
    #[serde(rename = "TDD_CLSPRC", deserialize_with = "deserialize_optional_f64")]
    pub close_price: Option<f64>,

    /// 대비
    #[serde(
        rename = "CMPPREVDD_PRC",
        deserialize_with = "deserialize_optional_f64"
    )]
    pub price_change: Option<f64>,

    /// 등락률 (%)
    #[serde(
        rename = "FLUC_RT",
        deserialize_with = "deserialize_optional_percentage"
    )]
    pub fluctuation_rate: Option<f64>,

    /// 시가
    #[serde(rename = "TDD_OPNPRC", deserialize_with = "deserialize_optional_f64")]
    pub open_price: Option<f64>,

    /// 고가
    #[serde(rename = "TDD_HGPRC", deserialize_with = "deserialize_optional_f64")]
    pub high_price: Option<f64>,

    /// 저가
    #[serde(rename = "TDD_LWPRC", deserialize_with = "deserialize_optional_f64")]
    pub low_price: Option<f64>,

    /// 거래량
    #[serde(rename = "ACC_TRDVOL", deserialize_with = "deserialize_optional_u64")]
    pub trading_volume: Option<u64>,

    /// 거래대금
    #[serde(rename = "ACC_TRDVAL", deserialize_with = "deserialize_optional_u64")]
    pub trading_value: Option<u64>,

    /// 시가총액
    #[serde(rename = "MKTCAP", deserialize_with = "deserialize_optional_u64")]
    pub market_cap: Option<u64>,

    /// 상장증서수
    #[serde(rename = "LIST_SHRS", deserialize_with = "deserialize_optional_u64")]
    pub listed_shares: Option<u64>,

    /// 신주발행가
    #[serde(rename = "ISU_PRC", deserialize_with = "deserialize_optional_f64")]
    pub issue_price: Option<f64>,

    /// 상장폐지일
    #[serde(rename = "DELIST_DD")]
    pub delisting_date: String,

    /// 목적주권_종목코드
    #[serde(rename = "TARSTK_ISU_SRT_CD")]
    pub target_stock_code: String,

    /// 목적주권_종목명
    #[serde(rename = "TARSTK_ISU_NM")]
    pub target_stock_name: String,

    /// 목적주권_종가
    #[serde(
        rename = "TARSTK_ISU_PRSNT_PRC",
        deserialize_with = "deserialize_optional_f64"
    )]
    pub target_stock_price: Option<f64>,
}

/// 종목기본정보 레코드 (유가증권/코스닥/코넥스 공통 구조)
#[derive(Debug, Deserialize)]
pub struct StockBaseInfoRecord {
    /// 표준코드
    #[serde(rename = "ISU_CD")]
    pub issue_code: String,

    /// 단축코드
    #[serde(rename = "ISU_SRT_CD")]
    pub issue_short_code: String,

    /// 한글 종목명
    #[serde(rename = "ISU_NM")]
    pub issue_name: String,

    /// 한글 종목약명
    #[serde(rename = "ISU_ABBRV")]
    pub issue_abbreviation: String,

    /// 영문 종목명
    #[serde(rename = "ISU_ENG_NM")]
    pub issue_english_name: String,

    /// 상장일
    #[serde(rename = "LIST_DD")]
    pub listing_date: String,

    /// 시장구분
    #[serde(rename = "MKT_TP_NM")]
    pub market_type: String,

    /// 증권구분
    #[serde(rename = "SECUGRP_NM")]
    pub security_group: String,

    /// 소속부
    #[serde(rename = "SECT_TP_NM")]
    pub sector_type: String,

    /// 주식종류
    #[serde(rename = "KIND_STKCERT_TP_NM")]
    pub stock_type: String,

    /// 액면가
    #[serde(rename = "PARVAL", deserialize_with = "deserialize_optional_f64")]
    pub par_value: Option<f64>,

    /// 상장주식수
    #[serde(rename = "LIST_SHRS", deserialize_with = "deserialize_optional_u64")]
    pub listed_shares: Option<u64>,
}

/// KOSPI 일별매매정보를 DataFrame으로 변환
pub fn parse_kospi_daily(response: ApiResponse<KospiDailyRecord>) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    // 각 필드를 벡터로 수집
    let mut dates = Vec::with_capacity(records.len());
    let mut codes = Vec::with_capacity(records.len());
    let mut names = Vec::with_capacity(records.len());
    let mut market_names = Vec::with_capacity(records.len());
    let mut sector_types = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut fluctuation_rates = Vec::with_capacity(records.len());
    let mut open_prices = Vec::with_capacity(records.len());
    let mut high_prices = Vec::with_capacity(records.len());
    let mut low_prices = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());
    let mut market_caps = Vec::with_capacity(records.len());
    let mut listed_shares = Vec::with_capacity(records.len());

    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        codes.push(record.issue_code);
        names.push(record.issue_name);
        market_names.push(record.market_name);
        sector_types.push(record.sector_type);
        close_prices.push(record.close_price);
        price_changes.push(record.price_change);
        fluctuation_rates.push(record.fluctuation_rate);
        open_prices.push(record.open_price);
        high_prices.push(record.high_price);
        low_prices.push(record.low_price);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
        market_caps.push(record.market_cap.map(|v| v as i64));
        listed_shares.push(record.listed_shares.map(|v| v as i64));
    }

    // DataFrame 생성
    let df = df! {
        "날짜" => dates,
        "종목코드" => codes,
        "종목명" => names,
        "시장구분" => market_names,
        "소속부" => sector_types,
        "종가" => close_prices,
        "대비" => price_changes,
        "등락률" => fluctuation_rates,
        "시가" => open_prices,
        "고가" => high_prices,
        "저가" => low_prices,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
        "시가총액" => market_caps,
        "상장주식수" => listed_shares,
    }?;

    Ok(df)
}

/// 코스닥 일별매매정보를 DataFrame으로 변환 (KOSPI와 동일)
pub fn parse_kosdaq_daily(response: ApiResponse<KosdaqDailyRecord>) -> Result<DataFrame> {
    parse_kospi_daily(response)
}

/// 코넥스 일별매매정보를 DataFrame으로 변환 (KOSPI와 동일)
pub fn parse_konex_daily(response: ApiResponse<KonexDailyRecord>) -> Result<DataFrame> {
    parse_kospi_daily(response)
}

/// 신주인수권증권 일별매매정보를 DataFrame으로 변환
pub fn parse_stock_warrant_daily(
    response: ApiResponse<StockWarrantDailyRecord>,
) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut dates = Vec::with_capacity(records.len());
    let mut market_names = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut exercise_prices = Vec::with_capacity(records.len());
    let mut target_stock_names = Vec::with_capacity(records.len());
    let mut target_stock_prices = Vec::with_capacity(records.len());

    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        market_names.push(record.market_name);
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        close_prices.push(record.close_price);
        exercise_prices.push(record.exercise_price);
        target_stock_names.push(record.target_stock_name);
        target_stock_prices.push(record.target_stock_price);
    }

    let df = df! {
        "날짜" => dates,
        "시장구분" => market_names,
        "종목코드" => issue_codes,
        "종목명" => issue_names,
        "종가" => close_prices,
        "행사가격" => exercise_prices,
        "목적주권명" => target_stock_names,
        "목적주권가격" => target_stock_prices,
    }?;

    Ok(df)
}

/// 신주인수권증서 일별매매정보를 DataFrame으로 변환
pub fn parse_stock_right_daily(response: ApiResponse<StockRightDailyRecord>) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut dates = Vec::with_capacity(records.len());
    let mut market_names = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut issue_prices = Vec::with_capacity(records.len());
    let mut target_stock_names = Vec::with_capacity(records.len());
    let mut target_stock_prices = Vec::with_capacity(records.len());

    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        market_names.push(record.market_name);
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        close_prices.push(record.close_price);
        issue_prices.push(record.issue_price);
        target_stock_names.push(record.target_stock_name);
        target_stock_prices.push(record.target_stock_price);
    }

    let df = df! {
        "날짜" => dates,
        "시장구분" => market_names,
        "종목코드" => issue_codes,
        "종목명" => issue_names,
        "종가" => close_prices,
        "신주발행가" => issue_prices,
        "목적주권명" => target_stock_names,
        "목적주권가격" => target_stock_prices,
    }?;

    Ok(df)
}

/// 종목기본정보를 DataFrame으로 변환
pub fn parse_stock_base_info(response: ApiResponse<StockBaseInfoRecord>) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_short_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut issue_abbreviations = Vec::with_capacity(records.len());
    let mut issue_english_names = Vec::with_capacity(records.len());
    let mut listing_dates = Vec::with_capacity(records.len());
    let mut market_types = Vec::with_capacity(records.len());
    let mut security_groups = Vec::with_capacity(records.len());
    let mut sector_types = Vec::with_capacity(records.len());
    let mut stock_types = Vec::with_capacity(records.len());
    let mut par_values = Vec::with_capacity(records.len());
    let mut listed_shares = Vec::with_capacity(records.len());

    for record in records {
        issue_codes.push(record.issue_code);
        issue_short_codes.push(record.issue_short_code);
        issue_names.push(record.issue_name);
        issue_abbreviations.push(record.issue_abbreviation);
        issue_english_names.push(record.issue_english_name);
        listing_dates.push(Some(record.listing_date));
        market_types.push(record.market_type);
        security_groups.push(record.security_group);
        sector_types.push(record.sector_type);
        stock_types.push(record.stock_type);
        par_values.push(record.par_value);
        listed_shares.push(record.listed_shares);
    }

    let df = df! {
        "표준코드" => issue_codes,
        "단축코드" => issue_short_codes,
        "종목명" => issue_names,
        "종목약명" => issue_abbreviations,
        "영문명" => issue_english_names,
        "상장일" => listing_dates,
        "시장구분" => market_types,
        "증권구분" => security_groups,
        "소속부" => sector_types,
        "주식종류" => stock_types,
        "액면가" => par_values,
        "상장주식수" => listed_shares,
    }?;

    Ok(df)
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_parse_kospi_daily() {
        let record = KospiDailyRecord {
            base_date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            issue_code: "005930".to_string(),
            issue_name: "삼성전자".to_string(),
            market_name: "KOSPI".to_string(),
            sector_type: "우량기업부".to_string(),
            close_price: Some(79500.0),
            price_change: Some(100.0),
            fluctuation_rate: Some(0.13),
            open_price: Some(79000.0),
            high_price: Some(80000.0),
            low_price: Some(78000.0),
            trading_volume: Some(1000000),
            trading_value: Some(79500000000),
            market_cap: Some(474500000000000),
            listed_shares: Some(5969782550),
        };
        let response = ApiResponse { data: vec![record] };

        let df = parse_kospi_daily(response).unwrap();

        assert_eq!(df.shape(), (1, 15));
        assert_eq!(
            df.column("종가").unwrap().f64().unwrap().get(0).unwrap(),
            79500.0
        );
    }

    #[test]
    fn test_parse_stock_base_info() {
        let record = StockBaseInfoRecord {
            issue_code: "005930".to_string(),
            issue_short_code: "KR7005930003".to_string(),
            issue_name: "삼성전자".to_string(),
            issue_abbreviation: "삼성전자".to_string(),
            issue_english_name: "SamsungElec".to_string(),
            listing_date: "19750611".to_string(),
            market_type: "KOSPI".to_string(),
            security_group: "주권".to_string(),
            sector_type: "우량기업부".to_string(),
            stock_type: "보통주".to_string(),
            par_value: Some(100.0),
            listed_shares: Some(5969782550),
        };
        let response = ApiResponse { data: vec![record] };

        let df = parse_stock_base_info(response).unwrap();

        assert_eq!(df.shape(), (1, 12));
        assert_eq!(
            df.column("액면가").unwrap().f64().unwrap().get(0).unwrap(),
            100.0
        );
        assert_eq!(
            df.column("상장주식수")
                .unwrap()
                .u64()
                .unwrap()
                .get(0)
                .unwrap(),
            5969782550
        );
    }
}
