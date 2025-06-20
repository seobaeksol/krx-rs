use super::{
    ApiResponse, 
    deserialize_krx_date,
    deserialize_optional_f64,
    deserialize_optional_u64,
    deserialize_optional_percentage
};
use polars::prelude::*;
use serde::Deserialize;
use chrono::NaiveDate;
use crate::error::Result;

/// KRX 지수 일별시세정보 레코드
#[derive(Debug, Deserialize)]
pub struct KrxIndexDailyRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,
    
    /// 계열구분
    #[serde(rename = "IDX_CLSS")]
    pub index_class: String,
    
    /// 지수명
    #[serde(rename = "IDX_NM")]
    pub index_name: String,
    
    /// 종가
    #[serde(rename = "CLSPRC_IDX", deserialize_with = "deserialize_optional_f64")]
    pub close_price: Option<f64>,
    
    /// 대비
    #[serde(rename = "CMPPREVDD_IDX", deserialize_with = "deserialize_optional_f64")]
    pub price_change: Option<f64>,
    
    /// 등락률 (%)
    #[serde(rename = "FLUC_RT", deserialize_with = "deserialize_optional_percentage")]
    pub fluctuation_rate: Option<f64>,
    
    /// 시가
    #[serde(rename = "OPNPRC_IDX", deserialize_with = "deserialize_optional_f64")]
    pub open_price: Option<f64>,
    
    /// 고가
    #[serde(rename = "HGPRC_IDX", deserialize_with = "deserialize_optional_f64")]
    pub high_price: Option<f64>,
    
    /// 저가
    #[serde(rename = "LWPRC_IDX", deserialize_with = "deserialize_optional_f64")]
    pub low_price: Option<f64>,
    
    /// 거래량
    #[serde(rename = "ACC_TRDVOL", deserialize_with = "deserialize_optional_u64")]
    pub trading_volume: Option<u64>,
    
    /// 거래대금
    #[serde(rename = "ACC_TRDVAL", deserialize_with = "deserialize_optional_u64")]
    pub trading_value: Option<u64>,
    
    /// 상장시가총액
    #[serde(rename = "MKTCAP", deserialize_with = "deserialize_optional_u64")]
    pub market_cap: Option<u64>,
}

/// KOSPI 지수 일별시세정보 레코드 (KRX와 동일한 구조)
pub type KospiIndexDailyRecord = KrxIndexDailyRecord;

/// KOSDAQ 지수 일별시세정보 레코드 (KRX와 동일한 구조) 
pub type KosdaqIndexDailyRecord = KrxIndexDailyRecord;

/// 채권지수 시세정보 레코드
#[derive(Debug, Deserialize)]
pub struct BondIndexDailyRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,
    
    /// 지수명
    #[serde(rename = "BND_IDX_GRP_NM")]
    pub bond_index_group_name: String,
    
    /// 총수익지수_종가
    #[serde(rename = "TOT_EARNG_IDX", deserialize_with = "deserialize_optional_f64")]
    pub total_earning_index: Option<f64>,
    
    /// 총수익지수_대비
    #[serde(rename = "TOT_EARNG_IDX_CMPPREVDD", deserialize_with = "deserialize_optional_f64")]
    pub total_earning_index_change: Option<f64>,
    
    /// 순가격지수_종가
    #[serde(rename = "NETPRC_IDX", deserialize_with = "deserialize_optional_f64")]
    pub net_price_index: Option<f64>,
    
    /// 순가격지수_대비
    #[serde(rename = "NETPRC_IDX_CMPPREVDD", deserialize_with = "deserialize_optional_f64")]
    pub net_price_index_change: Option<f64>,
    
    /// 제로재투자지수_종가
    #[serde(rename = "ZERO_REINVST_IDX", deserialize_with = "deserialize_optional_f64")]
    pub zero_reinvest_index: Option<f64>,
    
    /// 제로재투자지수_대비
    #[serde(rename = "ZERO_REINVST_IDX_CMPPREVDD", deserialize_with = "deserialize_optional_f64")]
    pub zero_reinvest_index_change: Option<f64>,
    
    /// 콜재투자지수_종가
    #[serde(rename = "CALL_REINVST_IDX", deserialize_with = "deserialize_optional_f64")]
    pub call_reinvest_index: Option<f64>,
    
    /// 콜재투자지수_대비
    #[serde(rename = "CALL_REINVST_IDX_CMPPREVDD", deserialize_with = "deserialize_optional_f64")]
    pub call_reinvest_index_change: Option<f64>,
    
    /// 시장가격지수_종가
    #[serde(rename = "MKT_PRC_IDX", deserialize_with = "deserialize_optional_f64")]
    pub market_price_index: Option<f64>,
    
    /// 시장가격지수_대비
    #[serde(rename = "MKT_PRC_IDX_CMPPREVDD", deserialize_with = "deserialize_optional_f64")]
    pub market_price_index_change: Option<f64>,
    
    /// 듀레이션
    #[serde(rename = "AVG_DURATION", deserialize_with = "deserialize_optional_f64")]
    pub average_duration: Option<f64>,
    
    /// 컨벡시티
    #[serde(rename = "AVG_CONVEXITY_PRC", deserialize_with = "deserialize_optional_f64")]
    pub average_convexity_price: Option<f64>,
    
    /// YTM
    #[serde(rename = "BND_IDX_AVG_YD", deserialize_with = "deserialize_optional_f64")]
    pub bond_index_average_yield: Option<f64>,
}

/// 파생상품지수 시세정보 레코드 (KRX 지수와 유사하지만 다른 필드들)
#[derive(Debug, Deserialize)]
pub struct DerivativeIndexDailyRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,
    
    /// 계열구분
    #[serde(rename = "IDX_CLSS")]
    pub index_class: String,
    
    /// 지수명
    #[serde(rename = "IDX_NM")]
    pub index_name: String,
    
    /// 종가
    #[serde(rename = "CLSPRC_IDX", deserialize_with = "deserialize_optional_f64")]
    pub close_price: Option<f64>,
    
    /// 대비
    #[serde(rename = "CMPPREVDD_IDX", deserialize_with = "deserialize_optional_f64")]
    pub price_change: Option<f64>,
    
    /// 등락률 (%)
    #[serde(rename = "FLUC_RT", deserialize_with = "deserialize_optional_percentage")]
    pub fluctuation_rate: Option<f64>,
    
    /// 시가
    #[serde(rename = "OPNPRC_IDX", deserialize_with = "deserialize_optional_f64")]
    pub open_price: Option<f64>,
    
    /// 고가
    #[serde(rename = "HGPRC_IDX", deserialize_with = "deserialize_optional_f64")]
    pub high_price: Option<f64>,
    
    /// 저가
    #[serde(rename = "LWPRC_IDX", deserialize_with = "deserialize_optional_f64")]
    pub low_price: Option<f64>,
}

/// KRX 지수 일별시세정보를 DataFrame으로 변환
pub fn parse_krx_index_daily(response: ApiResponse<KrxIndexDailyRecord>) -> Result<DataFrame> {
    let records = response.data;
    
    if records.is_empty() {
        return Ok(DataFrame::empty());
    }
    
    let mut dates = Vec::with_capacity(records.len());
    let mut classes = Vec::with_capacity(records.len());
    let mut names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut fluctuation_rates = Vec::with_capacity(records.len());
    let mut open_prices = Vec::with_capacity(records.len());
    let mut high_prices = Vec::with_capacity(records.len());
    let mut low_prices = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());
    let mut market_caps = Vec::with_capacity(records.len());
    
    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        classes.push(record.index_class);
        names.push(record.index_name);
        close_prices.push(record.close_price);
        price_changes.push(record.price_change);
        fluctuation_rates.push(record.fluctuation_rate);
        open_prices.push(record.open_price);
        high_prices.push(record.high_price);
        low_prices.push(record.low_price);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
        market_caps.push(record.market_cap.map(|v| v as i64));
    }
    
    let df = df! {
        "날짜" => dates,
        "계열구분" => classes,
        "지수명" => names,
        "종가" => close_prices,
        "대비" => price_changes,
        "등락률" => fluctuation_rates,
        "시가" => open_prices,
        "고가" => high_prices,
        "저가" => low_prices,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
        "시가총액" => market_caps,
    }?;
    
    Ok(df)
}

/// KOSPI 지수 일별시세정보를 DataFrame으로 변환 (KRX와 동일)
pub fn parse_kospi_index_daily(response: ApiResponse<KospiIndexDailyRecord>) -> Result<DataFrame> {
    parse_krx_index_daily(response)
}

/// KOSDAQ 지수 일별시세정보를 DataFrame으로 변환 (KRX와 동일)
pub fn parse_kosdaq_index_daily(response: ApiResponse<KosdaqIndexDailyRecord>) -> Result<DataFrame> {
    parse_krx_index_daily(response)
}

/// 채권지수 시세정보를 DataFrame으로 변환
pub fn parse_bond_index_daily(response: ApiResponse<BondIndexDailyRecord>) -> Result<DataFrame> {
    let records = response.data;
    
    if records.is_empty() {
        return Ok(DataFrame::empty());
    }
    
    let mut dates = Vec::with_capacity(records.len());
    let mut group_names = Vec::with_capacity(records.len());
    let mut total_earning_indices = Vec::with_capacity(records.len());
    let mut total_earning_changes = Vec::with_capacity(records.len());
    let mut net_price_indices = Vec::with_capacity(records.len());
    let mut net_price_changes = Vec::with_capacity(records.len());
    let mut zero_reinvest_indices = Vec::with_capacity(records.len());
    let mut zero_reinvest_changes = Vec::with_capacity(records.len());
    let mut call_reinvest_indices = Vec::with_capacity(records.len());
    let mut call_reinvest_changes = Vec::with_capacity(records.len());
    let mut market_price_indices = Vec::with_capacity(records.len());
    let mut market_price_changes = Vec::with_capacity(records.len());
    let mut durations = Vec::with_capacity(records.len());
    let mut convexities = Vec::with_capacity(records.len());
    let mut yields = Vec::with_capacity(records.len());
    
    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        group_names.push(record.bond_index_group_name);
        total_earning_indices.push(record.total_earning_index);
        total_earning_changes.push(record.total_earning_index_change);
        net_price_indices.push(record.net_price_index);
        net_price_changes.push(record.net_price_index_change);
        zero_reinvest_indices.push(record.zero_reinvest_index);
        zero_reinvest_changes.push(record.zero_reinvest_index_change);
        call_reinvest_indices.push(record.call_reinvest_index);
        call_reinvest_changes.push(record.call_reinvest_index_change);
        market_price_indices.push(record.market_price_index);
        market_price_changes.push(record.market_price_index_change);
        durations.push(record.average_duration);
        convexities.push(record.average_convexity_price);
        yields.push(record.bond_index_average_yield);
    }
    
    let df = df! {
        "날짜" => dates,
        "지수명" => group_names,
        "총수익지수" => total_earning_indices,
        "총수익지수_대비" => total_earning_changes,
        "순가격지수" => net_price_indices,
        "순가격지수_대비" => net_price_changes,
        "제로재투자지수" => zero_reinvest_indices,
        "제로재투자지수_대비" => zero_reinvest_changes,
        "콜재투자지수" => call_reinvest_indices,
        "콜재투자지수_대비" => call_reinvest_changes,
        "시장가격지수" => market_price_indices,
        "시장가격지수_대비" => market_price_changes,
        "듀레이션" => durations,
        "컨벡시티" => convexities,
        "YTM" => yields,
    }?;
    
    Ok(df)
}

/// 파생상품지수 시세정보를 DataFrame으로 변환
pub fn parse_derivative_index_daily(response: ApiResponse<DerivativeIndexDailyRecord>) -> Result<DataFrame> {
    let records = response.data;
    
    if records.is_empty() {
        return Ok(DataFrame::empty());
    }
    
    let mut dates = Vec::with_capacity(records.len());
    let mut classes = Vec::with_capacity(records.len());
    let mut names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut fluctuation_rates = Vec::with_capacity(records.len());
    let mut open_prices = Vec::with_capacity(records.len());
    let mut high_prices = Vec::with_capacity(records.len());
    let mut low_prices = Vec::with_capacity(records.len());
    
    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        classes.push(record.index_class);
        names.push(record.index_name);
        close_prices.push(record.close_price);
        price_changes.push(record.price_change);
        fluctuation_rates.push(record.fluctuation_rate);
        open_prices.push(record.open_price);
        high_prices.push(record.high_price);
        low_prices.push(record.low_price);
    }
    
    let df = df! {
        "날짜" => dates,
        "계열구분" => classes,
        "지수명" => names,
        "종가" => close_prices,
        "대비" => price_changes,
        "등락률" => fluctuation_rates,
        "시가" => open_prices,
        "고가" => high_prices,
        "저가" => low_prices,
    }?;
    
    Ok(df)
}