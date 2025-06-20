use crate::{client::Client, data::stock::*, error::{Error, Result}};
use polars::prelude::DataFrame;

/// 주식 관련 API 엔드포인트
pub struct StockApi<'a> {
    client: &'a Client,
}

impl<'a> StockApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// 유가증권 일별매매정보 조회
    pub fn kospi_daily(&self) -> KospiDailyBuilder<'a> {
        KospiDailyBuilder::new(self.client)
    }

    /// 코스닥 일별매매정보 조회
    pub fn kosdaq_daily(&self) -> KosdaqDailyBuilder<'a> {
        KosdaqDailyBuilder::new(self.client)
    }

    /// 코넥스 일별매매정보 조회
    pub fn konex_daily(&self) -> KonexDailyBuilder<'a> {
        KonexDailyBuilder::new(self.client)
    }

    /// 유가증권 종목기본정보 조회
    pub fn kospi_info(&self) -> KospiInfoBuilder<'a> {
        KospiInfoBuilder::new(self.client)
    }
}

/// 유가증권 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KospiDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> KospiDailyBuilder<'a> {
    fn new(client: &'a Client) -> Self {
        Self {
            client,
            base_date: None,
        }
    }

    /// 조회 기준일자 설정 (YYYYMMDD)
    ///
    /// # Example
    /// ```
    /// .date("20240105")
    /// ```
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

        // 날짜 형식 검증
        if !is_valid_date_format(&base_date) {
            return Err(Error::InvalidInput(
                "date must be in YYYYMMDD format".to_string()
            ));
        }

        let response = self.client
            .get(
                "/sto/stk_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_kospi_daily(response)
    }
}

/// 코스닥 일별매매정보 빌더 (기본 구조)
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KosdaqDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> KosdaqDailyBuilder<'a> {
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
                "/sto/stk_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_kospi_daily(response) // TODO: 별도 파서 구현 필요
    }
}

/// 코넥스 일별매매정보 빌더 (기본 구조)
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KonexDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> KonexDailyBuilder<'a> {
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
                "/sto/stk_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_kospi_daily(response) // TODO: 별도 파서 구현 필요
    }
}

/// 유가증권 종목기본정보 빌더 (기본 구조)
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KospiInfoBuilder<'a> {
    client: &'a Client,
}

impl<'a> KospiInfoBuilder<'a> {
    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn fetch(self) -> Result<DataFrame> {
        // TODO: 실제 API 엔드포인트 구현
        Err(Error::InvalidInput("Not implemented yet".to_string()))
    }
}

/// 날짜 형식 검증 (YYYYMMDD)
fn is_valid_date_format(date: &str) -> bool {
    date.len() == 8 && date.chars().all(|c| c.is_numeric())
}