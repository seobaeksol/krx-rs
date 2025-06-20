use super::{
    ApiResponse, deserialize_krx_date, deserialize_optional_f64, deserialize_optional_percentage,
    deserialize_optional_u64,
};
use crate::error::Result;
use chrono::NaiveDate;
use polars::prelude::*;
use serde::Deserialize;

/// ETF 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct EtfDailyRecord {
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

    /// 시가총액
    #[serde(rename = "MKTCAP", deserialize_with = "deserialize_optional_u64")]
    pub market_cap: Option<u64>,

    /// 상장주식수
    #[serde(rename = "LIST_SHRS", deserialize_with = "deserialize_optional_u64")]
    pub listed_shares: Option<u64>,

    /// NAV (순자산가치)
    #[serde(rename = "NAV", deserialize_with = "deserialize_optional_f64")]
    pub nav: Option<f64>,

    /// 기초지수명
    #[serde(rename = "IDX_IND_NM")]
    pub index_indicator_name: String,

    /// 목적지수
    #[serde(
        rename = "OBJ_STKPRC_IDX",
        deserialize_with = "deserialize_optional_f64"
    )]
    pub objective_stock_price_index: Option<f64>,

    /// 지수대비
    #[serde(
        rename = "CMPPREVDD_IDX",
        deserialize_with = "deserialize_optional_f64"
    )]
    pub index_change: Option<f64>,

    /// 지수등락률
    #[serde(rename = "FLUC_RT_IDX", deserialize_with = "deserialize_optional_f64")]
    pub index_fluctuation_rate: Option<f64>,

    /// 투자자산순자산총액
    #[serde(
        rename = "INVSTASST_NETASST_TOTAMT",
        deserialize_with = "deserialize_optional_u64"
    )]
    pub investment_asset_net_total_amount: Option<u64>,
}

/// ETN 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct EtnDailyRecord {
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

    /// 시가총액
    #[serde(rename = "MKTCAP", deserialize_with = "deserialize_optional_u64")]
    pub market_cap: Option<u64>,

    /// 상장주식수
    #[serde(rename = "LIST_SHRS", deserialize_with = "deserialize_optional_u64")]
    pub listed_shares: Option<u64>,

    /// 기초지수명
    #[serde(rename = "IDX_IND_NM")]
    pub index_indicator_name: String,

    /// 목적지수
    #[serde(
        rename = "OBJ_STKPRC_IDX",
        deserialize_with = "deserialize_optional_f64"
    )]
    pub objective_stock_price_index: Option<f64>,

    /// 지수대비
    #[serde(
        rename = "CMPPREVDD_IDX",
        deserialize_with = "deserialize_optional_f64"
    )]
    pub index_change: Option<f64>,

    /// 지수등락률
    #[serde(rename = "FLUC_RT_IDX", deserialize_with = "deserialize_optional_f64")]
    pub index_fluctuation_rate: Option<f64>,

    /// 지시가격금액
    #[serde(
        rename = "INDIC_VAL_AMT",
        deserialize_with = "deserialize_optional_u64"
    )]
    pub indicative_value_amount: Option<u64>,

    /// 1증권당지시가격
    #[serde(
        rename = "PER1SECU_INDIC_VAL",
        deserialize_with = "deserialize_optional_f64"
    )]
    pub per_security_indicative_value: Option<f64>,
}

/// ELW 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct ElwDailyRecord {
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

    /// 기초자산명
    #[serde(rename = "ULY_NM")]
    pub underlying_name: String,

    /// 기초자산가격
    #[serde(rename = "ULY_PRC")]
    pub underlying_price: String,

    /// 기초자산대비
    #[serde(
        rename = "CMPPREVDD_PRC_ULY",
        deserialize_with = "deserialize_optional_f64"
    )]
    pub underlying_price_change: Option<f64>,

    /// 기초자산등락률
    #[serde(
        rename = "FLUC_RT_ULY",
        deserialize_with = "deserialize_optional_percentage"
    )]
    pub underlying_fluctuation_rate: Option<f64>,
}

/// ETF 일별매매정보를 DataFrame으로 변환
pub fn parse_etf_daily(response: ApiResponse<EtfDailyRecord>) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut dates = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut fluctuation_rates = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());
    let mut navs = Vec::with_capacity(records.len());
    let mut index_names = Vec::with_capacity(records.len());
    let mut objective_indices = Vec::with_capacity(records.len());

    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        close_prices.push(record.close_price);
        price_changes.push(record.price_change);
        fluctuation_rates.push(record.fluctuation_rate);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
        navs.push(record.nav);
        index_names.push(record.index_indicator_name);
        objective_indices.push(record.objective_stock_price_index);
    }

    let df = df! {
        "날짜" => dates,
        "종목코드" => issue_codes,
        "종목명" => issue_names,
        "종가" => close_prices,
        "대비" => price_changes,
        "등락률" => fluctuation_rates,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
        "NAV" => navs,
        "기초지수명" => index_names,
        "목적지수" => objective_indices,
    }?;

    Ok(df)
}

/// ETN 일별매매정보를 DataFrame으로 변환
pub fn parse_etn_daily(response: ApiResponse<EtnDailyRecord>) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut dates = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut fluctuation_rates = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());
    let mut index_names = Vec::with_capacity(records.len());
    let mut indicative_values = Vec::with_capacity(records.len());

    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        close_prices.push(record.close_price);
        price_changes.push(record.price_change);
        fluctuation_rates.push(record.fluctuation_rate);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
        index_names.push(record.index_indicator_name);
        indicative_values.push(record.per_security_indicative_value);
    }

    let df = df! {
        "날짜" => dates,
        "종목코드" => issue_codes,
        "종목명" => issue_names,
        "종가" => close_prices,
        "대비" => price_changes,
        "등락률" => fluctuation_rates,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
        "기초지수명" => index_names,
        "1증권당지시가격" => indicative_values,
    }?;

    Ok(df)
}

/// ELW 일별매매정보를 DataFrame으로 변환
pub fn parse_elw_daily(response: ApiResponse<ElwDailyRecord>) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut dates = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());
    let mut underlying_names = Vec::with_capacity(records.len());
    let mut underlying_prices = Vec::with_capacity(records.len());
    let mut underlying_fluctuation_rates = Vec::with_capacity(records.len());

    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        close_prices.push(record.close_price);
        price_changes.push(record.price_change);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
        underlying_names.push(record.underlying_name);
        underlying_prices.push(record.underlying_price);
        underlying_fluctuation_rates.push(record.underlying_fluctuation_rate);
    }

    let df = df! {
        "날짜" => dates,
        "종목코드" => issue_codes,
        "종목명" => issue_names,
        "종가" => close_prices,
        "대비" => price_changes,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
        "기초자산명" => underlying_names,
        "기초자산가격" => underlying_prices,
        "기초자산등락률" => underlying_fluctuation_rates,
    }?;

    Ok(df)
}
