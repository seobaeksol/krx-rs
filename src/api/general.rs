use crate::{
    client::Client, 
    data::general::*, 
    error::Result,
    api::common::{validate_base_date, today_string}
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

    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.base_date = Some(date.into());
        self
    }

    pub fn today(mut self) -> Self {
        self.base_date = Some(today_string());
        self
    }

    pub async fn fetch(self) -> Result<DataFrame> {
        let base_date = validate_base_date(self.base_date)?;

        let response = self.client
            .get::<crate::data::ApiResponse<crate::data::general::OilDailyRecord>>(
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

    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.base_date = Some(date.into());
        self
    }

    pub fn today(mut self) -> Self {
        self.base_date = Some(today_string());
        self
    }

    pub async fn fetch(self) -> Result<DataFrame> {
        let base_date = validate_base_date(self.base_date)?;

        let response = self.client
            .get::<crate::data::ApiResponse<crate::data::general::GoldDailyRecord>>(
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

    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.base_date = Some(date.into());
        self
    }

    pub fn today(mut self) -> Self {
        self.base_date = Some(today_string());
        self
    }

    pub async fn fetch(self) -> Result<DataFrame> {
        let base_date = validate_base_date(self.base_date)?;

        let response = self.client
            .get::<crate::data::ApiResponse<crate::data::general::EmissionsDailyRecord>>(
                "/gen/ets_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_emissions_daily(response)
    }
}