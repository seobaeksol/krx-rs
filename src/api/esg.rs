use crate::{
    api::common::{latest_workday_string, validate_base_date},
    client::Client,
    data::{ApiResponse, esg::*},
    error::Result,
};
use polars::prelude::DataFrame;

/// ESG 관련 API 엔드포인트를 제공합니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // ESG API 접근
/// let esg_api = client.esg();
///
/// // 사회책임투자채권 정보 조회
/// let sri_bond = esg_api.sri_bond_info().date("20240105").fetch().await?;
/// println!("SRI Bond: {}", sri_bond);
/// # Ok(())
/// # }
/// ```
pub struct EsgApi<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> EsgApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// 사회책임투자(SRI) 채권 종목정보.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201080101)
    pub fn sri_bond_info(&self) -> SriBondInfoBuilder<'a> {
        SriBondInfoBuilder::new(self.client)
    }
}

/// 사회책임투자(SRI) 채권 종목정보를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// let df = client.esg().sri_bond_info().date("20240105").fetch().await?;
/// println!("{}", df);
/// # Ok(())
/// # }
/// ```
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct SriBondInfoBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> SriBondInfoBuilder<'a> {
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
            .get::<ApiResponse<SriBondInfoRecord>>("/esg/sri_bond_info", &[("basDd", &base_date)])
            .await?;

        parse_sri_bond_info(response)
    }
}
