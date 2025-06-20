use crate::{
    api::common::{today_string, validate_base_date},
    client::Client,
    data::{ApiResponse, derivative::*},
    error::Result,
};
use polars::prelude::DataFrame;

/// 파생상품 관련 API 엔드포인트
pub struct DerivativeApi<'a> {
    client: &'a Client,
}

impl<'a> DerivativeApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// 선물 일별매매정보 조회
    pub fn futures_daily(&self) -> FuturesDailyBuilder<'a> {
        FuturesDailyBuilder::new(self.client)
    }

    /// 개별주식선물 일별매매정보 조회
    pub fn equity_stock_futures_daily(&self) -> EquityStockFuturesDailyBuilder<'a> {
        EquityStockFuturesDailyBuilder::new(self.client)
    }

    /// 개별주식선물(코스닥) 일별매매정보 조회
    pub fn equity_kosdaq_futures_daily(&self) -> EquityKosdaqFuturesDailyBuilder<'a> {
        EquityKosdaqFuturesDailyBuilder::new(self.client)
    }

    /// 옵션 일별매매정보 조회
    pub fn options_daily(&self) -> OptionsDailyBuilder<'a> {
        OptionsDailyBuilder::new(self.client)
    }

    /// 개별주식옵션 일별매매정보 조회
    pub fn equity_stock_options_daily(&self) -> EquityStockOptionsDailyBuilder<'a> {
        EquityStockOptionsDailyBuilder::new(self.client)
    }

    /// 개별주식옵션(코스닥) 일별매매정보 조회
    pub fn equity_kosdaq_options_daily(&self) -> EquityKosdaqOptionsDailyBuilder<'a> {
        EquityKosdaqOptionsDailyBuilder::new(self.client)
    }
}

/// 선물 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct FuturesDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> FuturesDailyBuilder<'a> {
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

        let response = self
            .client
            .get::<ApiResponse<FuturesDailyRecord>>("/drv/fut_bydd_trd", &[("basDd", &base_date)])
            .await?;

        parse_futures_daily(response)
    }
}

/// 개별주식선물 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct EquityStockFuturesDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> EquityStockFuturesDailyBuilder<'a> {
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

        let response = self
            .client
            .get::<ApiResponse<EquityStockFuturesDailyRecord>>(
                "/drv/eqsfu_stk_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_equity_stock_futures_daily(response)
    }
}

/// 개별주식선물(코스닥) 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct EquityKosdaqFuturesDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> EquityKosdaqFuturesDailyBuilder<'a> {
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

        let response = self
            .client
            .get::<ApiResponse<EquityKosdaqFuturesDailyRecord>>(
                "/drv/eqkfu_ksq_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_equity_kosdaq_futures_daily(response)
    }
}

/// 옵션 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct OptionsDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> OptionsDailyBuilder<'a> {
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

        let response = self
            .client
            .get::<ApiResponse<OptionsDailyRecord>>("/drv/opt_bydd_trd", &[("basDd", &base_date)])
            .await?;

        parse_options_daily(response)
    }
}

/// 개별주식옵션 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct EquityStockOptionsDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> EquityStockOptionsDailyBuilder<'a> {
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

        let response = self
            .client
            .get::<ApiResponse<EquityStockOptionsDailyRecord>>(
                "/drv/eqsop_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_equity_stock_options_daily(response)
    }
}

/// 개별주식옵션(코스닥) 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct EquityKosdaqOptionsDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> EquityKosdaqOptionsDailyBuilder<'a> {
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

        let response = self
            .client
            .get::<ApiResponse<EquityKosdaqOptionsDailyRecord>>(
                "/drv/eqkop_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_equity_kosdaq_options_daily(response)
    }
}
