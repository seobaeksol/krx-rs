use crate::{
    api::common::{latest_workday_string, validate_base_date},
    client::Client,
    data::{ApiResponse, stock::*},
    error::Result,
};
use polars::prelude::DataFrame;

/// 주식(Stock) 관련 API 엔드포인트를 제공합니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 주식 API 접근
/// let stock_api = client.stock();
///
/// // KOSPI 일별 시세 조회
/// let stock_daily = stock_api.stock_daily().date("20240105").fetch().await?;
/// println!("KOSPI Daily: {}", stock_daily);
///
/// // KOSDAQ 종목 기본 정보 조회
/// let kosdaq_info = stock_api.kosdaq_base_info().latest().fetch().await?;
/// println!("KOSDAQ Info: {}", kosdaq_info);
/// # Ok(())
/// # }
/// ```
pub struct StockApi<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> StockApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// 유가증권(KOSPI) 전종목 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201020101)
    pub fn stock_daily(&self) -> StockDailyBuilder<'a> {
        StockDailyBuilder::new(self.client)
    }

    /// 코스닥(KOSDAQ) 전종목 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201020201)
    pub fn kosdaq_daily(&self) -> KosdaqDailyBuilder<'a> {
        KosdaqDailyBuilder::new(self.client)
    }

    /// 코넥스(KONEX) 전종목 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201020301)
    pub fn konex_daily(&self) -> KonexDailyBuilder<'a> {
        KonexDailyBuilder::new(self.client)
    }

    /// 신주인수권증권 전종목 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201020401)
    pub fn stock_warrant_daily(&self) -> StockWarrantDailyBuilder<'a> {
        StockWarrantDailyBuilder::new(self.client)
    }

    /// 신주인수권증서 전종목 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201020501)
    pub fn stock_right_daily(&self) -> StockRightDailyBuilder<'a> {
        StockRightDailyBuilder::new(self.client)
    }

    /// 유가증권(KOSPI) 종목 기본정보.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201020601)
    pub fn stock_base_info(&self) -> StockBaseInfoBuilder<'a> {
        StockBaseInfoBuilder::new(self.client)
    }

    /// 코스닥(KOSDAQ) 종목 기본정보.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201020701)
    pub fn kosdaq_base_info(&self) -> KosdaqBaseInfoBuilder<'a> {
        KosdaqBaseInfoBuilder::new(self.client)
    }

    /// 코넥스(KONEX) 종목 기본정보.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201020801)
    pub fn konex_base_info(&self) -> KonexBaseInfoBuilder<'a> {
        KonexBaseInfoBuilder::new(self.client)
    }
}

/// 유가증권(KOSPI) 전종목 일별 시세를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 2024년 1월 5일 기준 데이터 조회
/// let df_by_date = client.stock().stock_daily().date("20240105").fetch().await?;
/// println!("{}", df_by_date);
///
/// // 가장 최신(전일) 데이터 조회
/// let df_latest = client.stock().stock_daily().latest().fetch().await?;
/// println!("{}", df_latest);
/// # Ok(())
/// # }
/// ```
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct StockDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> StockDailyBuilder<'a> {
    fn new(client: &'a Client) -> Self {
        Self {
            client,
            base_date: None,
        }
    }

    /// 조회 기준일자를 설정합니다. (YYYYMMDD 형식)
    ///
    /// # Panics
    ///
    /// 잘못된 날짜 형식은 `fetch()` 시점에서 `Error::InvalidInput`을 반환합니다.
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
    ///
    /// # Returns
    ///
    /// Polars DataFrame 형태의 결과 데이터를 반환합니다.
    ///
    /// # Errors
    ///
    /// - `Error::InvalidInput`: 날짜가 지정되지 않았거나 형식이 잘못된 경우
    /// - `Error::ApiError`: KRX API 서버에서 오류를 반환한 경우
    /// - `Error::Network`: 네트워크 요청 실패 시
    /// - `Error::Parsing`: 응답 데이터 파싱 실패 시
    pub async fn fetch(self) -> Result<DataFrame> {
        let base_date = validate_base_date(self.base_date)?;

        let response = self
            .client
            .get::<ApiResponse<StockDailyRecord>>("/sto/stk_bydd_trd", &[("basDd", &base_date)])
            .await?;

        parse_stock_daily(response)
    }
}

/// 코스닥(KOSDAQ) 전종목 일별 시세를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 2024년 1월 5일 기준 데이터 조회
/// let df_by_date = client.stock().kosdaq_daily().date("20240105").fetch().await?;
/// println!("{}", df_by_date);
///
/// // 가장 최신 거래일(보통 전일)의 데이터로 설정합니다.
/// let df_latest = client.stock().kosdaq_daily().latest().fetch().await?;
/// println!("{}", df_latest);
/// # Ok(())
/// # }
/// ```
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
            .get::<ApiResponse<KosdaqDailyRecord>>("/sto/ksq_bydd_trd", &[("basDd", &base_date)])
            .await?;

        parse_kosdaq_daily(response)
    }
}

/// 코넥스(KONEX) 전종목 일별 시세를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 2024년 1월 5일 기준 데이터 조회
/// let df_by_date = client.stock().konex_daily().date("20240105").fetch().await?;
/// println!("{}", df_by_date);
///
/// // 가장 최신 거래일(보통 전일)의 데이터로 설정합니다.
/// let df_latest = client.stock().konex_daily().latest().fetch().await?;
/// println!("{}", df_latest);
/// # Ok(())
/// # }
/// ```
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
            .get::<ApiResponse<KonexDailyRecord>>("/sto/knx_bydd_trd", &[("basDd", &base_date)])
            .await?;

        parse_konex_daily(response)
    }
}

/// 신주인수권증권 전종목 일별 시세를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 2024년 1월 5일 기준 데이터 조회
/// let df_by_date = client.stock().stock_warrant_daily().date("20240105").fetch().await?;
/// println!("{}", df_by_date);
///
/// // 가장 최신 거래일(보통 전일)의 데이터로 설정합니다.
/// let df_latest = client.stock().stock_warrant_daily().latest().fetch().await?;
/// println!("{}", df_latest);
/// # Ok(())
/// # }
/// ```
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
            .get::<ApiResponse<StockWarrantDailyRecord>>(
                "/sto/sw_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_stock_warrant_daily(response)
    }
}

/// 신주인수권증서 전종목 일별 시세를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 2024년 1월 5일 기준 데이터 조회
/// let df_by_date = client.stock().stock_right_daily().date("20240105").fetch().await?;
/// println!("{}", df_by_date);
///
/// // 가장 최신 거래일(보통 전일)의 데이터로 설정합니다.
/// let df_latest = client.stock().stock_right_daily().latest().fetch().await?;
/// println!("{}", df_latest);
/// # Ok(())
/// # }
/// ```
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
            .get::<ApiResponse<StockRightDailyRecord>>("/sto/sr_bydd_trd", &[("basDd", &base_date)])
            .await?;

        parse_stock_right_daily(response)
    }
}

/// 유가증권(KOSPI) 종목 기본정보를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 2024년 1월 5일 기준 데이터 조회
/// let df_by_date = client.stock().stock_base_info().date("20240105").fetch().await?;
/// println!("{}", df_by_date);
///
/// // 가장 최신 거래일(보통 전일)의 데이터로 설정합니다.
/// let df_latest = client.stock().stock_base_info().latest().fetch().await?;
/// println!("{}", df_latest);
/// # Ok(())
/// # }
/// ```
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct StockBaseInfoBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> StockBaseInfoBuilder<'a> {
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
            .get::<ApiResponse<crate::data::stock::StockBaseInfoRecord>>(
                "/sto/stk_isu_base_info",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_stock_base_info(response)
    }
}

/// 코스닥(KOSDAQ) 종목 기본정보를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 2024년 1월 5일 기준 데이터 조회
/// let df_by_date = client.stock().kosdaq_base_info().date("20240105").fetch().await?;
/// println!("{}", df_by_date);
///
/// // 가장 최신 거래일(보통 전일)의 데이터로 설정합니다.
/// let df_latest = client.stock().kosdaq_base_info().latest().fetch().await?;
/// println!("{}", df_latest);
/// # Ok(())
/// # }
/// ```
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KosdaqBaseInfoBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> KosdaqBaseInfoBuilder<'a> {
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
            .get::<ApiResponse<crate::data::stock::StockBaseInfoRecord>>(
                "/sto/ksq_isu_base_info",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_stock_base_info(response)
    }
}

/// 코넥스(KONEX) 종목 기본정보를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 2024년 1월 5일 기준 데이터 조회
/// let df_by_date = client.stock().konex_base_info().date("20240105").fetch().await?;
/// println!("{}", df_by_date);
///
/// // 가장 최신 거래일(보통 전일)의 데이터로 설정합니다.
/// let df_latest = client.stock().konex_base_info().latest().fetch().await?;
/// println!("{}", df_latest);
/// # Ok(())
/// # }
/// ```
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KonexBaseInfoBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> KonexBaseInfoBuilder<'a> {
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
            .get::<ApiResponse<crate::data::stock::StockBaseInfoRecord>>(
                "/sto/knx_isu_base_info",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_stock_base_info(response)
    }
}
