use crate::{
    api::common::{latest_workday_string, validate_base_date},
    client::Client,
    data::{ApiResponse, general::*},
    error::Result,
};
use polars::prelude::DataFrame;

/// 일반상품(유가, 금, 배출권) 관련 API 엔드포인트를 제공합니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 일반상품 API 접근
/// let general_api = client.general();
///
/// // 유가 일별 시세 조회
/// let oil = general_api.oil_daily().date("20240105").fetch().await?;
/// println!("유가: {}", oil);
///
/// // 금 최신 정보 조회
/// let gold = general_api.gold_daily().latest().fetch().await?;
/// println!("금: {}", gold);
/// # Ok(())
/// # }
/// ```
pub struct GeneralApi<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> GeneralApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// 유가증권시장 석유(오일) 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201070101)
    pub fn oil_daily(&self) -> OilDailyBuilder<'a> {
        OilDailyBuilder::new(self.client)
    }

    /// KRX 금시장 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201070201)
    pub fn gold_daily(&self) -> GoldDailyBuilder<'a> {
        GoldDailyBuilder::new(self.client)
    }

    /// 배출권시장 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201070301)
    pub fn emissions_daily(&self) -> EmissionsDailyBuilder<'a> {
        EmissionsDailyBuilder::new(self.client)
    }
}

/// 유가증권시장 석유(오일) 일별 시세를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// let df = client.general().oil_daily().date("20240105").fetch().await?;
/// println!("{}", df);
/// # Ok(())
/// # }
/// ```
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct OilDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> OilDailyBuilder<'a> {
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
            .get::<ApiResponse<OilDailyRecord>>("/gen/oil_bydd_trd", &[("basDd", &base_date)])
            .await?;

        parse_oil_daily(response)
    }
}

/// 금 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct GoldDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> GoldDailyBuilder<'a> {
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
            .get::<ApiResponse<crate::data::general::GoldDailyRecord>>(
                "/gen/gold_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_gold_daily(response)
    }
}

/// 배출권 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct EmissionsDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> EmissionsDailyBuilder<'a> {
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
            .get::<ApiResponse<EmissionsDailyRecord>>("/gen/ets_bydd_trd", &[("basDd", &base_date)])
            .await?;

        parse_emissions_daily(response)
    }
}
