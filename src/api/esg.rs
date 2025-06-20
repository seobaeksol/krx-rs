use crate::{
    api::common::{latest_workday_string, validate_base_date},
    client::Client,
    data::{ApiResponse, esg::*},
    error::Result,
};
use polars::prelude::DataFrame;

/// ESG 관련 API 엔드포인트
pub struct EsgApi<'a> {
    client: &'a Client,
}

impl<'a> EsgApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// 사회책임투자채권 정보 조회
    pub fn sri_bond_info(&self) -> SriBondInfoBuilder<'a> {
        SriBondInfoBuilder::new(self.client)
    }
}

/// 사회책임투자채권 정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct SriBondInfoBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> SriBondInfoBuilder<'a> {
    fn new(client: &'a Client) -> Self {
        Self {
            client,
            base_date: None,
        }
    }

    /// 조회 기준일자 설정 (YYYYMMDD).
    ///
    /// KRX 데이터는 2010년 이후부터 조회일 기준 전일까지만 제공됩니다.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.base_date = Some(date.into());
        self
    }

    /// 가장 최신 거래일(보통 전일)의 데이터로 설정합니다.
    pub fn latest(mut self) -> Self {
        self.base_date = Some(latest_workday_string());
        self
    }

    pub async fn fetch(self) -> Result<DataFrame> {
        let base_date = validate_base_date(self.base_date)?;
        let response = self
            .client
            .get::<ApiResponse<SriBondInfoRecord>>("/esg/sri_bond_info", &[("basDd", &base_date)])
            .await?;

        parse_sri_bond_info(response)
    }
}
