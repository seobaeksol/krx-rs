use super::{
    ApiResponse, deserialize_krx_date, deserialize_optional_f64, deserialize_optional_krx_date,
    deserialize_optional_u64,
};
use crate::error::Result;
use chrono::NaiveDate;
use polars::prelude::*;
use serde::Deserialize;

/// 사회책임투자채권 정보 레코드
#[derive(Debug, Deserialize)]
pub struct SriBondInfoRecord {
    /// 기준일자
    #[serde(rename = "BAS_DD", deserialize_with = "deserialize_krx_date")]
    pub base_date: NaiveDate,

    /// 발행기관
    #[serde(rename = "ISUR_NM")]
    pub issuer_name: String,

    /// 표준코드
    #[serde(rename = "ISU_CD")]
    pub issue_code: String,

    /// 채권종류
    #[serde(rename = "SRI_BND_TP_NM")]
    pub sri_bond_type: String,

    /// 종목명
    #[serde(rename = "ISU_NM")]
    pub issue_name: String,

    /// 상장일
    #[serde(rename = "LIST_DD", deserialize_with = "deserialize_optional_krx_date")]
    pub listing_date: Option<NaiveDate>,

    /// 발행일
    #[serde(rename = "ISU_DD", deserialize_with = "deserialize_optional_krx_date")]
    pub issue_date: Option<NaiveDate>,

    /// 상환일
    #[serde(
        rename = "REDMPT_DD",
        deserialize_with = "deserialize_optional_krx_date"
    )]
    pub redemption_date: Option<NaiveDate>,

    /// 표면이자율 (%)
    #[serde(rename = "ISU_RT", deserialize_with = "deserialize_optional_f64")]
    pub issue_rate: Option<f64>,

    /// 발행금액
    #[serde(rename = "ISU_AMT", deserialize_with = "deserialize_optional_u64")]
    pub issue_amount: Option<u64>,

    /// 상장금액
    #[serde(rename = "LIST_AMT", deserialize_with = "deserialize_optional_u64")]
    pub listing_amount: Option<u64>,

    /// 채권유형
    #[serde(rename = "BND_TP_NM")]
    pub bond_type: String,
}

/// 사회책임투자채권 정보를 DataFrame으로 변환
pub fn parse_sri_bond_info(response: ApiResponse<SriBondInfoRecord>) -> Result<DataFrame> {
    let records = response.data;

    if records.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut base_dates = Vec::with_capacity(records.len());
    let mut issuer_names = Vec::with_capacity(records.len());
    let mut issue_codes = Vec::with_capacity(records.len());
    let mut sri_bond_types = Vec::with_capacity(records.len());
    let mut issue_names = Vec::with_capacity(records.len());
    let mut listing_dates = Vec::with_capacity(records.len());
    let mut issue_dates = Vec::with_capacity(records.len());
    let mut redemption_dates = Vec::with_capacity(records.len());
    let mut issue_rates = Vec::with_capacity(records.len());
    let mut issue_amounts = Vec::with_capacity(records.len());
    let mut listing_amounts = Vec::with_capacity(records.len());
    let mut bond_types = Vec::with_capacity(records.len());

    for record in records {
        base_dates.push(record.base_date.format("%Y-%m-%d").to_string());
        issuer_names.push(record.issuer_name);
        issue_codes.push(record.issue_code);
        sri_bond_types.push(record.sri_bond_type);
        issue_names.push(record.issue_name);
        listing_dates.push(
            record
                .listing_date
                .map(|d| d.format("%Y-%m-%d").to_string()),
        );
        issue_dates.push(record.issue_date.map(|d| d.format("%Y-%m-%d").to_string()));
        redemption_dates.push(
            record
                .redemption_date
                .map(|d| d.format("%Y-%m-%d").to_string()),
        );
        issue_rates.push(record.issue_rate);
        issue_amounts.push(record.issue_amount.map(|v| v as i64));
        listing_amounts.push(record.listing_amount.map(|v| v as i64));
        bond_types.push(record.bond_type);
    }

    let df = df! {
        "기준일자" => base_dates,
        "발행기관" => issuer_names,
        "표준코드" => issue_codes,
        "채권종류" => sri_bond_types,
        "종목명" => issue_names,
        "상장일" => listing_dates,
        "발행일" => issue_dates,
        "상환일" => redemption_dates,
        "표면이자율" => issue_rates,
        "발행금액" => issue_amounts,
        "상장금액" => listing_amounts,
        "채권유형" => bond_types,
    }?;

    Ok(df)
}
