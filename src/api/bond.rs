use crate::{
    api::common::{latest_workday_string, validate_base_date},
    client::Client,
    data::{
        ApiResponse,
        bond::{KtsDailyRecord, *},
    },
    error::Result,
};
use polars::prelude::DataFrame;

/// 채권(Bond) 관련 API 엔드포인트를 제공합니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// // 채권 API 접근
/// let bond_api = client.bond();
///
/// // 국고채 일별 시세 조회
/// let kts_daily = bond_api.kts_daily().date("20240105").fetch().await?;
/// println!("국고채: {}", kts_daily);
///
/// // 일반채권 최신 정보 조회
/// let bond_daily = bond_api.bond_daily().latest().fetch().await?;
/// println!("일반채권: {}", bond_daily);
/// # Ok(())
/// # }
/// ```
pub struct BondApi<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> BondApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// 국고채(KTS) 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201040101)
    pub fn kts_daily(&self) -> KtsDailyBuilder<'a> {
        KtsDailyBuilder::new(self.client)
    }

    /// 일반채권 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201040201)
    pub fn bond_daily(&self) -> BondDailyBuilder<'a> {
        BondDailyBuilder::new(self.client)
    }

    /// 소액채권 일별 시세.
    ///
    /// [API 명세](https://data.krx.co.kr/contents/MDC/MDI/mdiLoader/index.cmd?menuId=MDC0201040301)
    pub fn small_bond_daily(&self) -> SmallBondDailyBuilder<'a> {
        SmallBondDailyBuilder::new(self.client)
    }
}

/// 국고채(KTS) 일별 시세를 조회하는 빌더입니다.
///
/// # 예시
/// ```rust,no_run
/// # use krx_rs::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), krx_rs::error::Error> {
/// # let client = Client::new("YOUR_AUTH_KEY");
/// let df = client.bond().kts_daily().date("20240105").fetch().await?;
/// println!("{}", df);
/// # Ok(())
/// # }
/// ```
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct KtsDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> KtsDailyBuilder<'a> {
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
            .get::<ApiResponse<KtsDailyRecord>>("/bon/kts_bydd_trd", &[("basDd", &base_date)])
            .await?;

        parse_kts_daily(response)
    }
}

/// 일반채권 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct BondDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> BondDailyBuilder<'a> {
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
            .get::<ApiResponse<crate::data::bond::BondDailyRecord>>(
                "/bon/bnd_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_bond_daily(response)
    }
}

/// 소액채권 일별매매정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct SmallBondDailyBuilder<'a> {
    client: &'a Client,
    base_date: Option<String>,
}

impl<'a> SmallBondDailyBuilder<'a> {
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
            .get::<ApiResponse<crate::data::bond::SmallBondDailyRecord>>(
                "/bon/smb_bydd_trd",
                &[("basDd", &base_date)],
            )
            .await?;

        parse_small_bond_daily(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::common::latest_workday_string;

    #[test]
    fn test_kts_daily_builder_creation() {
        let client = Client::new("test_key");
        let builder = client.bond().kts_daily();
        assert!(builder.base_date.is_none());
    }

    #[test]
    fn test_kts_daily_builder_with_date() {
        let client = Client::new("test_key");
        let builder = client.bond().kts_daily().date("20240105");
        assert_eq!(builder.base_date, Some("20240105".to_string()));
    }

    #[test]
    fn test_kts_daily_builder_with_latest() {
        let client = Client::new("test_key");
        let builder = client.bond().kts_daily().latest();
        assert!(builder.base_date.is_some());
        assert_eq!(builder.base_date, Some(latest_workday_string()));
    }

    #[test]
    fn test_bond_daily_builder_creation() {
        let client = Client::new("test_key");
        let builder = client.bond().bond_daily();
        assert!(builder.base_date.is_none());
    }

    #[test]
    fn test_bond_daily_builder_with_date() {
        let client = Client::new("test_key");
        let builder = client.bond().bond_daily().date("20240105");
        assert_eq!(builder.base_date, Some("20240105".to_string()));
    }

    #[test]
    fn test_bond_daily_builder_with_latest() {
        let client = Client::new("test_key");
        let builder = client.bond().bond_daily().latest();
        assert!(builder.base_date.is_some());
        assert_eq!(builder.base_date, Some(latest_workday_string()));
    }

    #[test]
    fn test_small_bond_daily_builder_creation() {
        let client = Client::new("test_key");
        let builder = client.bond().small_bond_daily();
        assert!(builder.base_date.is_none());
    }

    #[test]
    fn test_small_bond_daily_builder_with_date() {
        let client = Client::new("test_key");
        let builder = client.bond().small_bond_daily().date("20240105");
        assert_eq!(builder.base_date, Some("20240105".to_string()));
    }

    #[test]
    fn test_small_bond_daily_builder_with_latest() {
        let client = Client::new("test_key");
        let builder = client.bond().small_bond_daily().latest();
        assert!(builder.base_date.is_some());
        assert_eq!(builder.base_date, Some(latest_workday_string()));
    }

    #[test]
    fn test_bond_api_creation() {
        let client = Client::new("test_key");
        let _bond_api = client.bond();
        // This test ensures BondApi can be created successfully
    }

    #[test]
    fn test_builder_date_overwrite() {
        let client = Client::new("test_key");
        let builder = client.bond().kts_daily()
            .date("20240105")
            .date("20240106");
        assert_eq!(builder.base_date, Some("20240106".to_string()));
    }

    #[test]
    fn test_builder_latest_overwrite() {
        let client = Client::new("test_key");
        let builder = client.bond().kts_daily()
            .date("20240105")
            .latest();
        assert_eq!(builder.base_date, Some(latest_workday_string()));
    }
}
