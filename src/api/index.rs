use crate::{client::Client, data::index::*, error::{Error, Result}};
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
        use chrono::Local;
        self.base_date = Some(Local::now().format("%Y%m%d").to_string());
        self
    }

    /// API 호출 및 데이터 조회
    pub async fn fetch(self) -> Result<DataFrame> {
        let base_date = self.base_date
            .ok_or_else(|| Error::InvalidInput("date is required".to_string()))?;

        if !is_valid_date_format(&base_date) {
            return Err(Error::InvalidInput(
                "date must be in YYYYMMDD format".to_string()
            ));
        }

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

    pub async fn fetch(self) -> Result<DataFrame> {
        let base_date = self.base_date
            .ok_or_else(|| Error::InvalidInput("date is required".to_string()))?;

        if !is_valid_date_format(&base_date) {
            return Err(Error::InvalidInput(
                "date must be in YYYYMMDD format".to_string()
            ));
        }

        let response = self.client
            .get(
                "/idx/kospi_dd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_krx_index_daily(response) // TODO: KOSPI 전용 파서 구현 필요
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

    pub async fn fetch(self) -> Result<DataFrame> {
        let base_date = self.base_date
            .ok_or_else(|| Error::InvalidInput("date is required".to_string()))?;

        if !is_valid_date_format(&base_date) {
            return Err(Error::InvalidInput(
                "date must be in YYYYMMDD format".to_string()
            ));
        }

        let response = self.client
            .get(
                "/idx/kosdaq_dd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_krx_index_daily(response) // TODO: KOSDAQ 전용 파서 구현 필요
    }
}

/// 날짜 형식 검증 (YYYYMMDD)
fn is_valid_date_format(date: &str) -> bool {
    date.len() == 8 && date.chars().all(|c| c.is_numeric())
}