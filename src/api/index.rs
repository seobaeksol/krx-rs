use crate::{
    client::Client, 
    data::index::*, 
    error::{Error, Result},
    api::common::{validate_base_date, today_string}
};
use polars::prelude::DataFrame;

/// 지수 관련 API 엔드포인트
pub struct IndexApi<'a> {
    client: &'a Client,
}

impl<'a> IndexApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// KRX 지수 일별시세정보 조회
    pub fn krx_daily(&self) -> KrxIndexDailyBuilder<'a> {
        KrxIndexDailyBuilder::new(self.client)
    }

    /// KOSPI 지수 일별시세정보 조회
    pub fn kospi_daily(&self) -> KospiIndexDailyBuilder<'a> {
        KospiIndexDailyBuilder::new(self.client)
    }

    /// KOSDAQ 지수 일별시세정보 조회
    pub fn kosdaq_daily(&self) -> KosdaqIndexDailyBuilder<'a> {
        KosdaqIndexDailyBuilder::new(self.client)
    }

    /// 채권지수 시세정보 조회
    pub fn bond_daily(&self) -> BondIndexDailyBuilder<'a> {
        BondIndexDailyBuilder::new(self.client)
    }

    /// 파생상품지수 시세정보 조회
    pub fn derivative_daily(&self) -> DerivativeIndexDailyBuilder<'a> {
        DerivativeIndexDailyBuilder::new(self.client)
    }
}

/// KRX 지수 일별시세정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KrxIndexDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> KrxIndexDailyBuilder<'a> {
    fn new(client: &'a Client) -> Self {
        Self {
            client,
            base_date: None,
        }
    }

    /// 조회 기준일자 설정 (YYYYMMDD)
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.base_date = Some(date.into());
        self
    }

    /// 오늘 날짜로 설정
    pub fn today(mut self) -> Self {
        self.base_date = Some(today_string());
        self
    }

    /// API 호출 및 데이터 조회
    pub async fn fetch(self) -> Result<DataFrame> {
        let base_date = validate_base_date(self.base_date)?;

        let response = self.client
            .get(
                "/idx/krx_dd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_krx_index_daily(response)
    }
}

/// KOSPI 지수 일별시세정보 빌더 (기본 구조)
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KospiIndexDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> KospiIndexDailyBuilder<'a> {
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
                "/idx/kospi_dd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_kospi_index_daily(response)
    }
}

/// KOSDAQ 지수 일별시세정보 빌더 (기본 구조)
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KosdaqIndexDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> KosdaqIndexDailyBuilder<'a> {
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
                "/idx/kosdaq_dd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_kosdaq_index_daily(response)
    }
}

/// 채권지수 시세정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct BondIndexDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> BondIndexDailyBuilder<'a> {
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
                "/idx/bon_dd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_bond_index_daily(response)
    }
}

/// 파생상품지수 시세정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct DerivativeIndexDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> DerivativeIndexDailyBuilder<'a> {
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
                "/idx/drvprod_dd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_derivative_index_daily(response)
    }
}