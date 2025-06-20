use crate::{
    api::common::{latest_workday_string, validate_base_date},
    client::Client,
    data::{ApiResponse, derivative::*},
    error::Result,
};
use polars::prelude::DataFrame;

/// 파생상품(선물, 옵션) 관련 API 엔드포인트를 제공합니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 파생상품 API 접근
/// let deriv_api = client.derivative();
///
/// // 선물 일별 시세 조회
/// let futures = deriv_api.futures_daily().date("20240105").fetch().await?;
/// println!("선물: {}", futures);
///
/// // 옵션 최신 정보 조회
/// let options = deriv_api.options_daily().latest().fetch().await?;
/// println!("옵션: {}", options);
/// # Ok(())
/// # }
/// ```
pub struct DerivativeApi<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> DerivativeApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// 선물 전종목 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201050101)
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

/// 선물 전종목 일별 시세를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// let df = client.derivative().futures_daily().date("20240105").fetch().await?;
/// println!("{}", df);
/// # Ok(())
/// # }
/// ```
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

    /// 조회 기준일자를 설정합니다. (YYYYMMDD 형식)
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.base_date = Some(date.into());
        self
    }

    /// 가장 최신 거래일(보통 전일)의 데이터로 기준일자를 설정합니다.
    pub fn latest(mut self) -> Self {
        self.base_date = Some(latest_workday_string());
        self
    }

    /// 설정된 파라미터로 API를 호출하여 데이터를 가져옵니다.
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
            .get::<ApiResponse<EquityKosdaqOptionsDailyRecord>>(
                "/drv/eqkop_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_equity_kosdaq_options_daily(response)
    }
}
