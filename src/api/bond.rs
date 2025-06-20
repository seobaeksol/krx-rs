use crate::{client::Client, error::Result};
use polars::prelude::DataFrame;

/// 채권 관련 API 엔드포인트
pub struct BondApi<'a> {
    client: &'a Client,
}

impl<'a> BondApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // TODO: 채권 관련 API 엔드포인트 구현
}