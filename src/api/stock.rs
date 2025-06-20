use crate::{
    client::Client, 
    data::stock::*, 
    error::Result,
    api::common::{validate_base_date, today_string}
};
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

    /// 신주인수권증권 일별매매정보 조회
    pub fn stock_warrant_daily(&self) -> StockWarrantDailyBuilder<'a> {
        StockWarrantDailyBuilder::new(self.client)
    }

    /// 신주인수권증서 일별매매정보 조회
    pub fn stock_right_daily(&self) -> StockRightDailyBuilder<'a> {
        StockRightDailyBuilder::new(self.client)
    }

    /// 유가증권 종목기본정보 조회
    pub fn kospi_base_info(&self) -> KospiBaseInfoBuilder<'a> {
        KospiBaseInfoBuilder::new(self.client)
    }

    /// 코스닥 종목기본정보 조회
    pub fn kosdaq_base_info(&self) -> KosdaqBaseInfoBuilder<'a> {
        KosdaqBaseInfoBuilder::new(self.client)
    }

    /// 코넥스 종목기본정보 조회
    pub fn konex_base_info(&self) -> KonexBaseInfoBuilder<'a> {
        KonexBaseInfoBuilder::new(self.client)
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
            .get::<crate::data::ApiResponse<crate::data::stock::KospiDailyRecord>>(
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

    pub fn today(mut self) -> Self {
        self.base_date = Some(today_string());
        self
    }

    pub async fn fetch(self) -> Result<DataFrame> {
        let base_date = validate_base_date(self.base_date)?;

        let response = self.client
            .get::<crate::data::ApiResponse<crate::data::stock::KosdaqDailyRecord>>(
                "/sto/ksq_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_kosdaq_daily(response)
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

    pub fn today(mut self) -> Self {
        self.base_date = Some(today_string());
        self
    }

    pub async fn fetch(self) -> Result<DataFrame> {
        let base_date = validate_base_date(self.base_date)?;

        let response = self.client
            .get::<crate::data::ApiResponse<crate::data::stock::KonexDailyRecord>>(
                "/sto/knx_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_konex_daily(response)
    }
}

/// 신주인수권증권 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct StockWarrantDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> StockWarrantDailyBuilder<'a> {
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
            .get::<crate::data::ApiResponse<crate::data::stock::StockWarrantDailyRecord>>(
                "/sto/sw_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_stock_warrant_daily(response)
    }
}

/// 신주인수권증서 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct StockRightDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> StockRightDailyBuilder<'a> {
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
            .get::<crate::data::ApiResponse<crate::data::stock::StockRightDailyRecord>>(
                "/sto/sr_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_stock_right_daily(response)
    }
}

/// 유가증권 종목기본정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KospiBaseInfoBuilder<'a> {
    client: &'a Client,
}

impl<'a> KospiBaseInfoBuilder<'a> {
    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn fetch(self) -> Result<DataFrame> {
        let response = self.client
            .get::<crate::data::ApiResponse<crate::data::stock::StockBaseInfoRecord>>("/sto/stk_isu_base_info", &[])
            .await?;

        parse_stock_base_info(response)
    }
}

/// 코스닥 종목기본정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KosdaqBaseInfoBuilder<'a> {
    client: &'a Client,
}

impl<'a> KosdaqBaseInfoBuilder<'a> {
    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn fetch(self) -> Result<DataFrame> {
        let response = self.client
            .get::<crate::data::ApiResponse<crate::data::stock::StockBaseInfoRecord>>("/sto/ksq_isu_base_info", &[])
            .await?;

        parse_stock_base_info(response)
    }
}

/// 코넥스 종목기본정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KonexBaseInfoBuilder<'a> {
    client: &'a Client,
}

impl<'a> KonexBaseInfoBuilder<'a> {
    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn fetch(self) -> Result<DataFrame> {
        let response = self.client
            .get::<crate::data::ApiResponse<crate::data::stock::StockBaseInfoRecord>>("/sto/knx_isu_base_info", &[])
            .await?;

        parse_stock_base_info(response)
    }
}