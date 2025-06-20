use crate::{
    api::common::{latest_workday_string, validate_base_date},
    client::Client,
    data::{ApiResponse, index::*},
    error::Result,
};
use polars::prelude::DataFrame;

/// 주가지수(Index) 관련 API 엔드포인트를 제공합니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 주가지수 API 접근
/// let index_api = client.index();
///
/// // KRX 지수 일별 시세 조회
/// let krx_index = index_api.krx_daily().date("20240105").fetch().await?;
/// println!("KRX Index: {}", krx_index);
///
/// // KOSPI 지수 최신 정보 조회
/// let kospi_index = index_api.kospi_daily().latest().fetch().await?;
/// println!("KOSPI Index: {}", kospi_index);
/// # Ok(())
/// # }
/// ```
pub struct IndexApi<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> IndexApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// KRX 지수 전종목 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201010101)
    pub fn krx_daily(&self) -> KrxIndexDailyBuilder<'a> {
        KrxIndexDailyBuilder::new(self.client)
    }

    /// 코스피 지수 전종목 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201010102)
    pub fn kospi_daily(&self) -> KospiIndexDailyBuilder<'a> {
        KospiIndexDailyBuilder::new(self.client)
    }

    /// 코스닥 지수 전종목 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201010103)
    pub fn kosdaq_daily(&self) -> KosdaqIndexDailyBuilder<'a> {
        KosdaqIndexDailyBuilder::new(self.client)
    }

    /// 채권 지수 전종목 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201010104)
    pub fn bond_daily(&self) -> BondIndexDailyBuilder<'a> {
        BondIndexDailyBuilder::new(self.client)
    }

    /// 파생상품 지수 전종목 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201010105)
    pub fn derivative_daily(&self) -> DerivativeIndexDailyBuilder<'a> {
        DerivativeIndexDailyBuilder::new(self.client)
    }
}

/// KRX 지수 전종목 일별 시세를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// let df = client.index().krx_daily().date("20240105").fetch().await?;
/// println!("{}", df);
/// # Ok(())
/// # }
/// ```
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
            .get::<ApiResponse<KrxIndexDailyRecord>>("/idx/krx_dd_trd", &[("basDd", &base_date)])
            .await?;

        parse_krx_index_daily(response)
    }
}

/// 코스피 지수 전종목 일별 시세를 조회하는 빌더입니다.
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
            .get::<ApiResponse<KospiIndexDailyRecord>>(
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
            .get::<ApiResponse<KosdaqIndexDailyRecord>>(
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
            .get::<ApiResponse<BondIndexDailyRecord>>("/idx/bon_dd_trd", &[("basDd", &base_date)])
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
            .get::<ApiResponse<DerivativeIndexDailyRecord>>(
                "/idx/drvprod_dd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_derivative_index_daily(response)
    }
}
