use super::{
    ApiResponse, deserialize_krx_date, deserialize_optional_f64, deserialize_optional_u64,
};
use crate::error::Result;
use chrono::NaiveDate;
use polars::prelude::*;
use serde::Deserialize;

/// 일반채권 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct BondDailyRecord {
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

    /// 종가
    #[serde(rename = "CLSPRC", deserialize_with = "deserialize_optional_f64")]
    pub close_price: Option<f64>,

    /// 종가수익률
    #[serde(rename = "CLSPRC_YD", deserialize_with = "deserialize_optional_f64")]
    pub close_price_yield: Option<f64>,

    /// 시가
    #[serde(rename = "OPNPRC", deserialize_with = "deserialize_optional_f64")]
    pub open_price: Option<f64>,

    /// 시가수익률
    #[serde(rename = "OPNPRC_YD", deserialize_with = "deserialize_optional_f64")]
    pub open_price_yield: Option<f64>,

    /// 고가
    #[serde(rename = "HGPRC", deserialize_with = "deserialize_optional_f64")]
    pub high_price: Option<f64>,

    /// 고가수익률
    #[serde(rename = "HGPRC_YD", deserialize_with = "deserialize_optional_f64")]
    pub high_price_yield: Option<f64>,

    /// 저가
    #[serde(rename = "LWPRC", deserialize_with = "deserialize_optional_f64")]
    pub low_price: Option<f64>,

    /// 저가수익률
    #[serde(rename = "LWPRC_YD", deserialize_with = "deserialize_optional_f64")]
    pub low_price_yield: Option<f64>,

    /// 대비
    #[serde(rename = "CMPPREVDD_PRC")]
    pub price_change: String,

    /// 거래량
    #[serde(rename = "ACC_TRDVOL", deserialize_with = "deserialize_optional_u64")]
    pub trading_volume: Option<u64>,

    /// 거래대금
    #[serde(rename = "ACC_TRDVAL", deserialize_with = "deserialize_optional_u64")]
    pub trading_value: Option<u64>,
}

/// 소액채권 일별매매정보 레코드 (일반채권과 동일한 구조)
pub type SmallBondDailyRecord = BondDailyRecord;

/// 국채전문유통시장 일별매매정보 레코드
#[derive(Debug, Deserialize)]
pub struct KtsDailyRecord {
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

    /// 국채발행유형
    #[serde(rename = "GOVBND_ISU_TP_NM")]
    pub government_bond_issue_type: String,

    /// 채권만기유형
    #[serde(rename = "BND_EXP_TP_NM")]
    pub bond_expiry_type: String,

    /// 종가
    #[serde(rename = "CLSPRC", deserialize_with = "deserialize_optional_f64")]
    pub close_price: Option<f64>,

    /// 종가수익률
    #[serde(rename = "CLSPRC_YD", deserialize_with = "deserialize_optional_f64")]
    pub close_price_yield: Option<f64>,

    /// 시가
    #[serde(rename = "OPNPRC", deserialize_with = "deserialize_optional_f64")]
    pub open_price: Option<f64>,

    /// 시가수익률
    #[serde(rename = "OPNPRC_YD", deserialize_with = "deserialize_optional_f64")]
    pub open_price_yield: Option<f64>,

    /// 고가
    #[serde(rename = "HGPRC", deserialize_with = "deserialize_optional_f64")]
    pub high_price: Option<f64>,

    /// 고가수익률
    #[serde(rename = "HGPRC_YD", deserialize_with = "deserialize_optional_f64")]
    pub high_price_yield: Option<f64>,

    /// 저가
    #[serde(rename = "LWPRC", deserialize_with = "deserialize_optional_f64")]
    pub low_price: Option<f64>,

    /// 저가수익률
    #[serde(rename = "LWPRC_YD", deserialize_with = "deserialize_optional_f64")]
    pub low_price_yield: Option<f64>,

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
}

/// 일반채권 일별매매정보를 DataFrame으로 변환
pub fn parse_bond_daily(response: ApiResponse<BondDailyRecord>) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut dates = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut market_names = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut close_yields = Vec::with_capacity(records.len());
    let mut open_prices = Vec::with_capacity(records.len());
    let mut open_yields = Vec::with_capacity(records.len());
    let mut high_prices = Vec::with_capacity(records.len());
    let mut high_yields = Vec::with_capacity(records.len());
    let mut low_prices = Vec::with_capacity(records.len());
    let mut low_yields = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());

    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        market_names.push(record.market_name);
        close_prices.push(record.close_price);
        close_yields.push(record.close_price_yield);
        open_prices.push(record.open_price);
        open_yields.push(record.open_price_yield);
        high_prices.push(record.high_price);
        high_yields.push(record.high_price_yield);
        low_prices.push(record.low_price);
        low_yields.push(record.low_price_yield);
        price_changes.push(record.price_change);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
    }

    let df = df! {
        "날짜" => dates,
        "종목코드" => issue_codes,
        "종목명" => issue_names,
        "시장구분" => market_names,
        "종가" => close_prices,
        "종가수익률" => close_yields,
        "시가" => open_prices,
        "시가수익률" => open_yields,
        "고가" => high_prices,
        "고가수익률" => high_yields,
        "저가" => low_prices,
        "저가수익률" => low_yields,
        "대비" => price_changes,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
    }?;

    Ok(df)
}

/// 소액채권 일별매매정보를 DataFrame으로 변환 (일반채권과 동일)
pub fn parse_small_bond_daily(response: ApiResponse<SmallBondDailyRecord>) -> Result<DataFrame> {
    parse_bond_daily(response)
}

/// 국채전문유통시장 일별매매정보를 DataFrame으로 변환
pub fn parse_kts_daily(response: ApiResponse<KtsDailyRecord>) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut dates = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut market_names = Vec::with_capacity(records.len());
    let mut bond_issue_types = Vec::with_capacity(records.len());
    let mut bond_expiry_types = Vec::with_capacity(records.len());
    let mut close_prices = Vec::with_capacity(records.len());
    let mut close_yields = Vec::with_capacity(records.len());
    let mut open_prices = Vec::with_capacity(records.len());
    let mut open_yields = Vec::with_capacity(records.len());
    let mut high_prices = Vec::with_capacity(records.len());
    let mut high_yields = Vec::with_capacity(records.len());
    let mut low_prices = Vec::with_capacity(records.len());
    let mut low_yields = Vec::with_capacity(records.len());
    let mut price_changes = Vec::with_capacity(records.len());
    let mut trading_volumes = Vec::with_capacity(records.len());
    let mut trading_values = Vec::with_capacity(records.len());

    for record in records {
        dates.push(record.base_date.format("%Y-%m-%d").to_string());
        issue_codes.push(record.issue_code);
        issue_names.push(record.issue_name);
        market_names.push(record.market_name);
        bond_issue_types.push(record.government_bond_issue_type);
        bond_expiry_types.push(record.bond_expiry_type);
        close_prices.push(record.close_price);
        close_yields.push(record.close_price_yield);
        open_prices.push(record.open_price);
        open_yields.push(record.open_price_yield);
        high_prices.push(record.high_price);
        high_yields.push(record.high_price_yield);
        low_prices.push(record.low_price);
        low_yields.push(record.low_price_yield);
        price_changes.push(record.price_change);
        trading_volumes.push(record.trading_volume.map(|v| v as i64));
        trading_values.push(record.trading_value.map(|v| v as i64));
    }

    let df = df! {
        "날짜" => dates,
        "종목코드" => issue_codes,
        "종목명" => issue_names,
        "시장구분" => market_names,
        "국채발행유형" => bond_issue_types,
        "채권만기유형" => bond_expiry_types,
        "종가" => close_prices,
        "종가수익률" => close_yields,
        "시가" => open_prices,
        "시가수익률" => open_yields,
        "고가" => high_prices,
        "고가수익률" => high_yields,
        "저가" => low_prices,
        "저가수익률" => low_yields,
        "대비" => price_changes,
        "거래량" => trading_volumes,
        "거래대금" => trading_values,
    }?;

    Ok(df)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_kts_daily() {
        let record = KtsDailyRecord {
            base_date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            issue_code: "KR103501GC38".to_string(),
            issue_name: "국고03500-2603(23-1)".to_string(),
            market_name: "KTS".to_string(),
            government_bond_issue_type: "경과".to_string(),
            bond_expiry_type: "장기".to_string(),
            close_price: Some(10000.0),
            close_price_yield: Some(3.5),
            open_price: Some(9990.0),
            open_price_yield: Some(3.51),
            high_price: Some(10010.0),
            high_price_yield: Some(3.49),
            low_price: Some(9980.0),
            low_price_yield: Some(3.52),
            price_change: Some(10.0),
            trading_volume: Some(1000),
            trading_value: Some(10000000),
        };
        let response = ApiResponse { data: vec![record] };
        let df = parse_kts_daily(response).unwrap();

        assert_eq!(df.shape(), (1, 17));
        assert_eq!(
            df.column("종가").unwrap().f64().unwrap().get(0),
            Some(10000.0)
        );
        assert_eq!(
            df.column("종가수익률").unwrap().f64().unwrap().get(0),
            Some(3.5)
        );
    }

    #[test]
    fn test_parse_bond_daily() {
        let record = BondDailyRecord {
            base_date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            issue_code: "KR203801GC59".to_string(),
            issue_name: "국민주택1종(03-5)".to_string(),
            market_name: "일반".to_string(),
            close_price: Some(10050.0),
            close_price_yield: Some(4.1),
            open_price: Some(10045.0),
            open_price_yield: Some(4.11),
            high_price: Some(10055.0),
            high_price_yield: Some(4.09),
            low_price: Some(10040.0),
            low_price_yield: Some(4.12),
            price_change: "5".to_string(),
            trading_volume: Some(500),
            trading_value: Some(5025000),
        };
        let response = ApiResponse { data: vec![record] };
        let df: DataFrame = parse_bond_daily(response).unwrap();

        assert_eq!(df.shape(), (1, 15));
        assert_eq!(
            df.column("종가").unwrap().f64().unwrap().get(0),
            Some(10050.0)
        );
        assert_eq!(
            df.column("종가수익률").unwrap().f64().unwrap().get(0),
            Some(4.1)
        );
        assert_eq!(
            df.column("시가").unwrap().f64().unwrap().get(0),
            Some(10045.0)
        );
        assert_eq!(
            df.column("시가수익률").unwrap().f64().unwrap().get(0),
            Some(4.11)
        );
        assert_eq!(
            df.column("고가").unwrap().f64().unwrap().get(0),
            Some(10055.0)
        );
        assert_eq!(
            df.column("고가수익률").unwrap().f64().unwrap().get(0),
            Some(4.09)
        );
        assert_eq!(
            df.column("저가").unwrap().f64().unwrap().get(0),
            Some(10040.0)
        );
        assert_eq!(
            df.column("저가수익률").unwrap().f64().unwrap().get(0),
            Some(4.12)
        );
    }
}
