use crate::{client::Client, error::Result};
use polars::prelude::DataFrame;

/// ETP (ETF/ETN/ELW) 관련 API 엔드포인트
pub struct EtpApi<'a> {
    client: &'a Client,
}

impl<'a> EtpApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // TODO: ETP 관련 API 엔드포인트 구현
}