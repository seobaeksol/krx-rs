use crate::{
    api::common::{latest_workday_string, validate_base_date},
    client::Client,
    data::{ApiResponse, general::*},
    error::Result,
};
use polars::prelude::DataFrame;

/// 일반상품 관련 API 엔드포인트
pub struct GeneralApi<'a> {
    client: &'a Client,
}

impl<'a> GeneralApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// 석유류 일별매매정보 조회
    pub fn oil_daily(&self) -> OilDailyBuilder<'a> {
        OilDailyBuilder::new(self.client)
    }

    /// 금 일별매매정보 조회
    pub fn gold_daily(&self) -> GoldDailyBuilder<'a> {
        GoldDailyBuilder::new(self.client)
    }

    /// 배출권 일별매매정보 조회
    pub fn emissions_daily(&self) -> EmissionsDailyBuilder<'a> {
        EmissionsDailyBuilder::new(self.client)
    }
}

/// 석유류 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct OilDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> OilDailyBuilder<'a> {
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
            .get::<ApiResponse<crate::data::general::OilDailyRecord>>(
                "/gen/oil_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_oil_daily(response)
    }
}

/// 금 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct GoldDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> GoldDailyBuilder<'a> {
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
            .get::<ApiResponse<crate::data::general::GoldDailyRecord>>(
                "/gen/gold_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_gold_daily(response)
    }
}

/// 배출권 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct EmissionsDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> EmissionsDailyBuilder<'a> {
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
            .get::<ApiResponse<EmissionsDailyRecord>>("/gen/ets_bydd_trd", &[("basDd", &base_date)])
            .await?;

        parse_emissions_daily(response)
    }
}
