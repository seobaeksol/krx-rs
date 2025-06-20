use crate::{
    api::common::{latest_workday_string, validate_base_date},
    client::Client,
    data::{ApiResponse, etp::*},
    error::Result,
};
use polars::prelude::DataFrame;

/// ETP (ETF/ETN/ELW) 관련 API 엔드포인트
pub struct EtpApi<'a> {
    client: &'a Client,
}

impl<'a> EtpApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// ETF 일별매매정보 조회
    pub fn etf_daily(&self) -> EtfDailyBuilder<'a> {
        EtfDailyBuilder::new(self.client)
    }

    /// ETN 일별매매정보 조회
    pub fn etn_daily(&self) -> EtnDailyBuilder<'a> {
        EtnDailyBuilder::new(self.client)
    }

    /// ELW 일별매매정보 조회
    pub fn elw_daily(&self) -> ElwDailyBuilder<'a> {
        ElwDailyBuilder::new(self.client)
    }
}

/// ETF 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct EtfDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> EtfDailyBuilder<'a> {
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
            .get::<ApiResponse<EtfDailyRecord>>("/etp/etf_bydd_trd", &[("basDd", &base_date)])
            .await?;

        parse_etf_daily(response)
    }
}

/// ETN 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct EtnDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> EtnDailyBuilder<'a> {
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
            .get::<ApiResponse<crate::data::etp::EtnDailyRecord>>(
                "/etp/etn_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_etn_daily(response)
    }
}

/// ELW 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct ElwDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> ElwDailyBuilder<'a> {
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
            .get::<ApiResponse<crate::data::etp::ElwDailyRecord>>(
                "/etp/elw_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_elw_daily(response)
    }
}
