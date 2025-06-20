use crate::{
    client::Client, 
    data::esg::*, 
    error::Result
};
use polars::prelude::DataFrame;

/// ESG 관련 API 엔드포인트
pub struct EsgApi<'a> {
    client: &'a Client,
}

impl<'a> EsgApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// 사회책임투자채권 정보 조회
    pub fn sri_bond_info(&self) -> SriBondInfoBuilder<'a> {
        SriBondInfoBuilder::new(self.client)
    }
}

/// 사회책임투자채권 정보 빌더
#[must_use = "Builder does nothing unless you call .fetch()"]
pub struct SriBondInfoBuilder<'a> {
    client: &'a Client,
}

impl<'a> SriBondInfoBuilder<'a> {
    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn fetch(self) -> Result<DataFrame> {
        let response = self.client
            .get::<crate::data::ApiResponse<crate::data::esg::SriBondInfoRecord>>("/esg/sri_bond_info", &[])
            .await?;

        parse_sri_bond_info(response)
    }
}