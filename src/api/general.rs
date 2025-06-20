use crate::{client::Client, error::Result};
use polars::prelude::DataFrame;

/// 일반상품 관련 API 엔드포인트
pub struct GeneralApi<'a> {
    client: &'a Client,
}

impl<'a> GeneralApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // TODO: 일반상품 관련 API 엔드포인트 구현
}