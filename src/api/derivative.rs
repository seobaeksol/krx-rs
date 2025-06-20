use crate::{client::Client, error::Result};
use polars::prelude::DataFrame;

/// 파생상품 관련 API 엔드포인트
pub struct DerivativeApi<'a> {
    client: &'a Client,
}

impl<'a> DerivativeApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // TODO: 파생상품 관련 API 엔드포인트 구현
}