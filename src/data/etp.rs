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
    let mut open_prices = Vec::with_capacity(records.len());
    let mut high_prices = Vec::with_capacity(records.len());
    let mut low_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut fluctuation_rates = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());
    let mut market_caps = Vec::with_capacity(records.len());
    let mut listed_shares = Vec::with_capacity(records.len());
    let mut navs = Vec::with_capacity(records.len());
    let mut index_indicator_names = Vec::with_capacity(records.len());
    let mut objective_stock_price_indices = Vec::with_capacity(records.len());
    let mut index_changes = Vec::with_capacity(records.len());
    let mut index_fluctuation_rates = Vec::with_capacity(records.len());
    let mut investment_asset_net_total_amounts = Vec::with_capacity(records.len());

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
        market_caps.push(record.market_cap.map(|v| v as i64));
        listed_shares.push(record.listed_shares.map(|v| v as i64));
        navs.push(record.nav);
        index_indicator_names.push(record.index_indicator_name);
        objective_stock_price_indices.push(record.objective_stock_price_index);
        index_changes.push(record.index_change);
        index_fluctuation_rates.push(record.index_fluctuation_rate);
        investment_asset_net_total_amounts
            .push(record.investment_asset_net_total_amount.map(|v| v as i64));
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
        "시가총액" => market_caps,
        "상장주식수" => listed_shares,
        "NAV" => navs,
        "기초지수명" => index_indicator_names,
        "목적지수" => objective_stock_price_indices,
        "지수대비" => index_changes,
        "지수등락률" => index_fluctuation_rates,
        "투자자산순자산총액" => investment_asset_net_total_amounts,
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
    let mut open_prices = Vec::with_capacity(records.len());
    let mut high_prices = Vec::with_capacity(records.len());
    let mut low_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut fluctuation_rates = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());
    let mut market_caps = Vec::with_capacity(records.len());
    let mut listed_shares = Vec::with_capacity(records.len());
    let mut index_indicator_names = Vec::with_capacity(records.len());
    let mut objective_stock_price_indices = Vec::with_capacity(records.len());
    let mut index_changes = Vec::with_capacity(records.len());
    let mut index_fluctuation_rates = Vec::with_capacity(records.len());
    let mut indicative_value_amounts = Vec::with_capacity(records.len());
    let mut per_security_indicative_values = Vec::with_capacity(records.len());

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
        market_caps.push(record.market_cap.map(|v| v as i64));
        listed_shares.push(record.listed_shares.map(|v| v as i64));
        index_indicator_names.push(record.index_indicator_name);
        objective_stock_price_indices.push(record.objective_stock_price_index);
        index_changes.push(record.index_change);
        index_fluctuation_rates.push(record.index_fluctuation_rate);
        indicative_value_amounts.push(record.indicative_value_amount.map(|v| v as i64));
        per_security_indicative_values.push(record.per_security_indicative_value);
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
        "시가총액" => market_caps,
        "상장주식수" => listed_shares,
        "기초지수명" => index_indicator_names,
        "목적지수" => objective_stock_price_indices,
        "지수대비" => index_changes,
        "지수등락률" => index_fluctuation_rates,
        "지시가격금액" => indicative_value_amounts,
        "1증권당지시가격" => per_security_indicative_values,
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
    let mut open_prices = Vec::with_capacity(records.len());
    let mut high_prices = Vec::with_capacity(records.len());
    let mut low_prices = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());
    let mut market_caps = Vec::with_capacity(records.len());
    let mut listed_shares = Vec::with_capacity(records.len());
    let mut underlying_names = Vec::with_capacity(records.len());
    let mut underlying_prices = Vec::with_capacity(records.len());
    let mut underlying_price_changes = Vec::with_capacity(records.len());
    let mut underlying_fluctuation_rates = Vec::with_capacity(records.len());

    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        close_prices.push(record.close_price);
        open_prices.push(record.open_price);
        high_prices.push(record.high_price);
        low_prices.push(record.low_price);
        price_changes.push(record.price_change);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
        market_caps.push(record.market_cap.map(|v| v as i64));
        listed_shares.push(record.listed_shares.map(|v| v as i64));
        underlying_names.push(record.underlying_name);
        underlying_prices.push(record.underlying_price);
        underlying_price_changes.push(record.underlying_price_change);
        underlying_fluctuation_rates.push(record.underlying_fluctuation_rate);
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
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
        "시가총액" => market_caps,
        "상장주식수" => listed_shares,
        "기초자산명" => underlying_names,
        "기초자산가격" => underlying_prices,
        "기초자산대비" => underlying_price_changes,
        "기초자산등락률" => underlying_fluctuation_rates,
    }?;

    Ok(df)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_etf_daily() {
        let record = EtfDailyRecord {
            base_date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            issue_code: "069500".to_string(),
            issue_name: "KODEX 200".to_string(),
            close_price: Some(35000.0),
            price_change: Some(150.0),
            fluctuation_rate: Some(0.43),
            nav: Some(35010.0),
            open_price: Some(34850.0),
            high_price: Some(35100.0),
            low_price: Some(34800.0),
            trading_volume: Some(1000000),
            trading_value: Some(35000000000),
            market_cap: Some(5000000000000),
            listed_shares: Some(142857142),
            index_indicator_name: "KOSPI 200".to_string(),
            objective_stock_price_index: Some(350.10),
            index_change: Some(1.50),
            index_fluctuation_rate: Some(0.43),
            investment_asset_net_total_amount: Some(5000100000000),
        };
        let response = ApiResponse { data: vec![record] };
        let df = parse_etf_daily(response).unwrap();

        assert_eq!(df.shape(), (1, 19));
        assert_eq!(
            df.column("종가").unwrap().f64().unwrap().get(0),
            Some(35000.0)
        );
        assert_eq!(
            df.column("NAV").unwrap().f64().unwrap().get(0),
            Some(35010.0)
        );
        assert_eq!(
            df.column("기초지수명").unwrap().str().unwrap().get(0),
            Some("KOSPI 200")
        );
    }
}
