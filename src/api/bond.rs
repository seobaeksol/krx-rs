use crate::{
    client::Client, 
    data::bond::*, 
    error::Result,
    api::common::{validate_base_date, today_string}
};
use polars::prelude::DataFrame;

/// 채권 관련 API 엔드포인트
pub struct BondApi<'a> {
    client: &'a Client,
}

impl<'a> BondApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// 국채전문유통시장 일별매매정보 조회
    pub fn kts_daily(&self) -> KtsDailyBuilder<'a> {
        KtsDailyBuilder::new(self.client)
    }

    /// 일반채권 일별매매정보 조회
    pub fn bond_daily(&self) -> BondDailyBuilder<'a> {
        BondDailyBuilder::new(self.client)
    }

    /// 소액채권 일별매매정보 조회
    pub fn small_bond_daily(&self) -> SmallBondDailyBuilder<'a> {
        SmallBondDailyBuilder::new(self.client)
    }
}

/// 국채전문유통시장 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KtsDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> KtsDailyBuilder<'a> {
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
            .get(
                "/bon/kts_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_kts_daily(response)
    }
}

/// 일반채권 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct BondDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> BondDailyBuilder<'a> {
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
            .get(
                "/bon/bnd_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_bond_daily(response)
    }
}

/// 소액채권 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct SmallBondDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> SmallBondDailyBuilder<'a> {
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
            .get(
                "/bon/smb_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_small_bond_daily(response)
    }
}