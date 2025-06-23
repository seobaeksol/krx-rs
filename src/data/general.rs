use super::{
    ApiResponse, deserialize_krx_date, deserialize_optional_f64, deserialize_optional_percentage,
    deserialize_optional_u64,
};
use crate::error::Result;
use chrono::NaiveDate;
use polars::prelude::*;
use serde::Deserialize;

/// 석유류 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct OilDailyRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,

    /// 유종명
    #[serde(rename = "OIL_NM")]
    pub oil_name: String,

    /// 가중평균가격
    #[serde(rename = "WT_AVG_PRC", deserialize_with = "deserialize_optional_f64")]
    pub weighted_average_price: Option<f64>,

    /// 가중할인평균가격
    #[serde(
        rename = "WT_DIS_AVG_PRC",
        deserialize_with = "deserialize_optional_f64"
    )]
    pub weighted_discount_average_price: Option<f64>,

    /// 거래량
    #[serde(rename = "ACC_TRDVOL", deserialize_with = "deserialize_optional_u64")]
    pub trading_volume: Option<u64>,

    /// 거래대금
    #[serde(rename = "ACC_TRDVAL", deserialize_with = "deserialize_optional_u64")]
    pub trading_value: Option<u64>,
}

/// 금 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct GoldDailyRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,

    /// 종목코드
    #[serde(rename = "ISU_CD")]
    pub issue_code: String,

    /// 종목명
    #[serde(rename = "ISU_NM")]
    pub issue_name: String,

    /// 종가
    #[serde(rename = "TDD_CLSPRC", deserialize_with = "deserialize_optional_f64")]
    pub close_price: Option<f64>,

    /// 시가
    #[serde(rename = "TDD_OPNPRC", deserialize_with = "deserialize_optional_f64")]
    pub open_price: Option<f64>,

    /// 고가
    #[serde(rename = "TDD_HGPRC", deserialize_with = "deserialize_optional_f64")]
    pub high_price: Option<f64>,

    /// 저가
    #[serde(rename = "TDD_LWPRC", deserialize_with = "deserialize_optional_f64")]
    pub low_price: Option<f64>,

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

    /// 거래량
    #[serde(rename = "ACC_TRDVOL", deserialize_with = "deserialize_optional_u64")]
    pub trading_volume: Option<u64>,

    /// 거래대금
    #[serde(rename = "ACC_TRDVAL", deserialize_with = "deserialize_optional_u64")]
    pub trading_value: Option<u64>,
}

/// 배출권 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct EmissionsDailyRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,

    /// 종목코드
    #[serde(rename = "ISU_CD")]
    pub issue_code: String,

    /// 종목명
    #[serde(rename = "ISU_NM")]
    pub issue_name: String,

    /// 종가
    #[serde(rename = "TDD_CLSPRC", deserialize_with = "deserialize_optional_f64")]
    pub close_price: Option<f64>,

    /// 시가
    #[serde(rename = "TDD_OPNPRC", deserialize_with = "deserialize_optional_f64")]
    pub open_price: Option<f64>,

    /// 고가
    #[serde(rename = "TDD_HGPRC", deserialize_with = "deserialize_optional_f64")]
    pub high_price: Option<f64>,

    /// 저가
    #[serde(rename = "TDD_LWPRC", deserialize_with = "deserialize_optional_f64")]
    pub low_price: Option<f64>,

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

    /// 거래량
    #[serde(rename = "ACC_TRDVOL", deserialize_with = "deserialize_optional_u64")]
    pub trading_volume: Option<u64>,

    /// 거래대금
    #[serde(rename = "ACC_TRDVAL", deserialize_with = "deserialize_optional_u64")]
    pub trading_value: Option<u64>,
}

/// 석유류 일별매매정보를 DataFrame으로 변환
pub fn parse_oil_daily(response: ApiResponse<OilDailyRecord>) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut dates = Vec::with_capacity(records.len());
    let mut oil_names = Vec::with_capacity(records.len());
    let mut weighted_avg_prices = Vec::with_capacity(records.len());
    let mut weighted_discount_avg_prices = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());

    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        oil_names.push(record.oil_name);
        weighted_avg_prices.push(record.weighted_average_price);
        weighted_discount_avg_prices.push(record.weighted_discount_average_price);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
    }

    let df = df! {
        "날짜" => dates,
        "유종명" => oil_names,
        "가중평균가격" => weighted_avg_prices,
        "가중할인평균가격" => weighted_discount_avg_prices,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
    }?;

    Ok(df)
}

/// 금 일별매매정보를 DataFrame으로 변환
pub fn parse_gold_daily(response: ApiResponse<GoldDailyRecord>) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut dates = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut open_prices = Vec::with_capacity(records.len());
    let mut high_prices = Vec::with_capacity(records.len());
    let mut low_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut fluctuation_rates = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());

    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        close_prices.push(record.close_price);
        open_prices.push(record.open_price);
        high_prices.push(record.high_price);
        low_prices.push(record.low_price);
        price_changes.push(record.price_change);
        fluctuation_rates.push(record.fluctuation_rate);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
    }

    let df = df! {
        "날짜" => dates,
        "종목코드" => issue_codes,
        "종목명" => issue_names,
        "종가" => close_prices,
        "시가" => open_prices,
        "고가" => high_prices,
        "저가" => low_prices,
        "대비" => price_changes,
        "등락률" => fluctuation_rates,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
    }?;

    Ok(df)
}

/// 배출권 일별매매정보를 DataFrame으로 변환
pub fn parse_emissions_daily(response: ApiResponse<EmissionsDailyRecord>) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut dates = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut open_prices = Vec::with_capacity(records.len());
    let mut high_prices = Vec::with_capacity(records.len());
    let mut low_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut fluctuation_rates = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());

    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        close_prices.push(record.close_price);
        open_prices.push(record.open_price);
        high_prices.push(record.high_price);
        low_prices.push(record.low_price);
        price_changes.push(record.price_change);
        fluctuation_rates.push(record.fluctuation_rate);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
    }

    let df = df! {
        "날짜" => dates,
        "종목코드" => issue_codes,
        "종목명" => issue_names,
        "종가" => close_prices,
        "시가" => open_prices,
        "고가" => high_prices,
        "저가" => low_prices,
        "대비" => price_changes,
        "등락률" => fluctuation_rates,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
    }?;

    Ok(df)
}
