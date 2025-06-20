use super::{
    ApiResponse, 
    deserialize_krx_date,
    deserialize_optional_f64,
    deserialize_optional_u64
};
use polars::prelude::*;
use serde::Deserialize;
use chrono::NaiveDate;
use crate::error::Result;

/// 선물 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct FuturesDailyRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,
    
    /// 종목코드
    #[serde(rename = "ISU_CD")]
    pub issue_code: String,
    
    /// 종목명
    #[serde(rename = "ISU_NM")]
    pub issue_name: String,
    
    /// 상품명
    #[serde(rename = "PROD_NM")]
    pub product_name: String,
    
    /// 시장구분
    #[serde(rename = "MKT_NM")]
    pub market_name: String,
    
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
    
    /// 정산가격
    #[serde(rename = "SETL_PRC", deserialize_with = "deserialize_optional_f64")]
    pub settlement_price: Option<f64>,
    
    /// 현물가격
    #[serde(rename = "SPOT_PRC", deserialize_with = "deserialize_optional_f64")]
    pub spot_price: Option<f64>,
    
    /// 대비
    #[serde(rename = "CMPPREVDD_PRC", deserialize_with = "deserialize_optional_f64")]
    pub price_change: Option<f64>,
    
    /// 거래량
    #[serde(rename = "ACC_TRDVOL", deserialize_with = "deserialize_optional_u64")]
    pub trading_volume: Option<u64>,
    
    /// 거래대금
    #[serde(rename = "ACC_TRDVAL", deserialize_with = "deserialize_optional_u64")]
    pub trading_value: Option<u64>,
    
    /// 미결제약정수량
    #[serde(rename = "ACC_OPNINT_QTY")]
    pub open_interest_quantity: String,
}

/// 옵션 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct OptionsDailyRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,
    
    /// 종목코드
    #[serde(rename = "ISU_CD")]
    pub issue_code: String,
    
    /// 종목명
    #[serde(rename = "ISU_NM")]
    pub issue_name: String,
    
    /// 상품명
    #[serde(rename = "PROD_NM")]
    pub product_name: String,
    
    /// 권리구분
    #[serde(rename = "RGHT_TP_NM")]
    pub right_type: String,
    
    /// 종가
    #[serde(rename = "TDD_CLSPRC")]
    pub close_price: String,
    
    /// 시가
    #[serde(rename = "TDD_OPNPRC")]
    pub open_price: String,
    
    /// 고가
    #[serde(rename = "TDD_HGPRC")]
    pub high_price: String,
    
    /// 저가
    #[serde(rename = "TDD_LWPRC")]
    pub low_price: String,
    
    /// 대비
    #[serde(rename = "CMPPREVDD_PRC")]
    pub price_change: String,
    
    /// 거래량
    #[serde(rename = "ACC_TRDVOL", deserialize_with = "deserialize_optional_u64")]
    pub trading_volume: Option<u64>,
    
    /// 거래대금
    #[serde(rename = "ACC_TRDVAL", deserialize_with = "deserialize_optional_u64")]
    pub trading_value: Option<u64>,
    
    /// 미결제약정수량
    #[serde(rename = "ACC_OPNINT_QTY", deserialize_with = "deserialize_optional_u64")]
    pub open_interest_quantity: Option<u64>,
    
    /// 내재변동성
    #[serde(rename = "IMP_VOLT", deserialize_with = "deserialize_optional_f64")]
    pub implied_volatility: Option<f64>,
    
    /// 익일기준가격
    #[serde(rename = "NXTDD_BAS_PRC", deserialize_with = "deserialize_optional_f64")]
    pub next_day_base_price: Option<f64>,
}

/// 개별주식선물 일별매매정보 레코드 (KOSDAQ)
#[derive(Debug, Deserialize)]
pub struct EquityKosdaqFuturesDailyRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,
    
    /// 종목코드
    #[serde(rename = "ISU_CD")]
    pub issue_code: String,
    
    /// 종목명
    #[serde(rename = "ISU_NM")]
    pub issue_name: String,
    
    /// 상품명
    #[serde(rename = "PROD_NM")]
    pub product_name: String,
    
    /// 시장구분
    #[serde(rename = "MKT_NM")]
    pub market_name: String,
    
    /// 종가
    #[serde(rename = "TDD_CLSPRC", deserialize_with = "deserialize_optional_f64")]
    pub close_price: Option<f64>,
    
    /// 정산가격
    #[serde(rename = "SETL_PRC", deserialize_with = "deserialize_optional_f64")]
    pub settlement_price: Option<f64>,
    
    /// 현물가격
    #[serde(rename = "SPOT_PRC", deserialize_with = "deserialize_optional_f64")]
    pub spot_price: Option<f64>,
    
    /// 대비
    #[serde(rename = "CMPPREVDD_PRC", deserialize_with = "deserialize_optional_f64")]
    pub price_change: Option<f64>,
    
    /// 거래량
    #[serde(rename = "ACC_TRDVOL", deserialize_with = "deserialize_optional_u64")]
    pub trading_volume: Option<u64>,
    
    /// 거래대금
    #[serde(rename = "ACC_TRDVAL", deserialize_with = "deserialize_optional_u64")]
    pub trading_value: Option<u64>,
    
    /// 미결제약정수량
    #[serde(rename = "ACC_OPNINT_QTY")]
    pub open_interest_quantity: String,
}

/// 개별주식선물 일별매매정보 레코드
pub type EquityStockFuturesDailyRecord = EquityKosdaqFuturesDailyRecord;

/// 개별주식옵션 일별매매정보 레코드 (KOSDAQ)
pub type EquityKosdaqOptionsDailyRecord = OptionsDailyRecord;

/// 개별주식옵션 일별매매정보 레코드
pub type EquityStockOptionsDailyRecord = OptionsDailyRecord;

/// 선물 일별매매정보를 DataFrame으로 변환
pub fn parse_futures_daily(response: ApiResponse<FuturesDailyRecord>) -> Result<DataFrame> {
    let records = response.data;
    
    if records.is_empty() {
        return Ok(DataFrame::empty());
    }
    
    let mut dates = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut product_names = Vec::with_capacity(records.len());
    let mut market_names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut settlement_prices = Vec::with_capacity(records.len());
    let mut spot_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());
    let mut open_interest_quantities = Vec::with_capacity(records.len());
    
    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        product_names.push(record.product_name);
        market_names.push(record.market_name);
        close_prices.push(record.close_price);
        settlement_prices.push(record.settlement_price);
        spot_prices.push(record.spot_price);
        price_changes.push(record.price_change);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
        open_interest_quantities.push(record.open_interest_quantity);
    }
    
    let df = df! {
        "날짜" => dates,
        "종목코드" => issue_codes,
        "종목명" => issue_names,
        "상품명" => product_names,
        "시장구분" => market_names,
        "종가" => close_prices,
        "정산가격" => settlement_prices,
        "현물가격" => spot_prices,
        "대비" => price_changes,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
        "미결제약정수량" => open_interest_quantities,
    }?;
    
    Ok(df)
}

/// 옵션 일별매매정보를 DataFrame으로 변환
pub fn parse_options_daily(response: ApiResponse<OptionsDailyRecord>) -> Result<DataFrame> {
    let records = response.data;
    
    if records.is_empty() {
        return Ok(DataFrame::empty());
    }
    
    let mut dates = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut product_names = Vec::with_capacity(records.len());
    let mut right_types = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());
    let mut open_interest_quantities = Vec::with_capacity(records.len());
    let mut implied_volatilities = Vec::with_capacity(records.len());
    let mut next_day_base_prices = Vec::with_capacity(records.len());
    
    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        product_names.push(record.product_name);
        right_types.push(record.right_type);
        close_prices.push(record.close_price);
        price_changes.push(record.price_change);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
        open_interest_quantities.push(record.open_interest_quantity.map(|v| v as i64));
        implied_volatilities.push(record.implied_volatility);
        next_day_base_prices.push(record.next_day_base_price);
    }
    
    let df = df! {
        "날짜" => dates,
        "종목코드" => issue_codes,
        "종목명" => issue_names,
        "상품명" => product_names,
        "권리구분" => right_types,
        "종가" => close_prices,
        "대비" => price_changes,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
        "미결제약정수량" => open_interest_quantities,
        "내재변동성" => implied_volatilities,
        "익일기준가격" => next_day_base_prices,
    }?;
    
    Ok(df)
}

/// 개별주식선물 일별매매정보를 DataFrame으로 변환 (KOSDAQ)
pub fn parse_equity_kosdaq_futures_daily(response: ApiResponse<EquityKosdaqFuturesDailyRecord>) -> Result<DataFrame> {
    let records = response.data;
    
    if records.is_empty() {
        return Ok(DataFrame::empty());
    }
    
    let mut dates = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut product_names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut settlement_prices = Vec::with_capacity(records.len());
    let mut spot_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());
    
    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        product_names.push(record.product_name);
        close_prices.push(record.close_price);
        settlement_prices.push(record.settlement_price);
        spot_prices.push(record.spot_price);
        price_changes.push(record.price_change);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
    }
    
    let df = df! {
        "날짜" => dates,
        "종목코드" => issue_codes,
        "종목명" => issue_names,
        "상품명" => product_names,
        "종가" => close_prices,
        "정산가격" => settlement_prices,
        "현물가격" => spot_prices,
        "대비" => price_changes,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
    }?;
    
    Ok(df)
}

/// 개별주식선물 일별매매정보를 DataFrame으로 변환
pub fn parse_equity_stock_futures_daily(response: ApiResponse<EquityStockFuturesDailyRecord>) -> Result<DataFrame> {
    parse_equity_kosdaq_futures_daily(response)
}

/// 개별주식옵션 일별매매정보를 DataFrame으로 변환 (KOSDAQ)
pub fn parse_equity_kosdaq_options_daily(response: ApiResponse<EquityKosdaqOptionsDailyRecord>) -> Result<DataFrame> {
    parse_options_daily(response)
}

/// 개별주식옵션 일별매매정보를 DataFrame으로 변환
pub fn parse_equity_stock_options_daily(response: ApiResponse<EquityStockOptionsDailyRecord>) -> Result<DataFrame> {
    parse_options_daily(response)
}