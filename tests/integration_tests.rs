use krx_rs::client::Client;
use krx_rs::error::Error;
use polars::prelude::*;
use std::sync::Once;

static INIT: Once = Once::new();

fn setup() -> Option<Client> {
    INIT.call_once(|| {
        dotenvy::dotenv().ok();
    });

    match std::env::var("KRX_API_KEY") {
        Ok(key) => Some(Client::new(&key)),
        Err(_) => {
            println!("Skipping integration test: KRX_API_KEY not set.");
            None
        }
    }
}

async fn run_fetch_test(result: Result<DataFrame, Error>) {
    assert!(result.is_ok(), "Fetch failed: {:?}", result.err());
    let df = result.unwrap();
    assert!(!df.is_empty(), "DataFrame should not be empty");
    println!("Schema: {:?}", df.schema());
    println!("DataFrame (first 5 rows):\n{}", df.head(Some(5)));
}

// --- Stock API Tests ---

#[tokio::test]
#[ignore]
async fn test_stock_kospi_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.stock().kospi_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_kospi_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.stock().kospi_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_kosdaq_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.stock().kosdaq_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_kosdaq_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.stock().kosdaq_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_konex_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.stock().konex_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_konex_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.stock().konex_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_warrant_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .stock()
                .stock_warrant_daily()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_warrant_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.stock().stock_warrant_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_right_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .stock()
                .stock_right_daily()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_right_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.stock().stock_right_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_kospi_base_info_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .stock()
                .kospi_base_info()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_kospi_base_info_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.stock().kospi_base_info().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_kosdaq_base_info_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .stock()
                .kosdaq_base_info()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_kosdaq_base_info_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.stock().kosdaq_base_info().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_konex_base_info_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .stock()
                .konex_base_info()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_stock_konex_base_info_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.stock().konex_base_info().latest().fetch().await).await;
    }
}

// --- Index API Tests ---

#[tokio::test]
#[ignore]
async fn test_index_krx_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.index().krx_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_index_krx_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.index().krx_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_index_kospi_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.index().kospi_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_index_kospi_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.index().kospi_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_index_kosdaq_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.index().kosdaq_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_index_kosdaq_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.index().kosdaq_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_index_bond_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.index().bond_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_index_bond_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.index().bond_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_index_derivative_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .index()
                .derivative_daily()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_index_derivative_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.index().derivative_daily().latest().fetch().await).await;
    }
}

// --- Bond API Tests ---

#[tokio::test]
#[ignore]
async fn test_bond_kts_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.bond().kts_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_bond_kts_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.bond().kts_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_bond_bond_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.bond().bond_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_bond_bond_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.bond().bond_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_bond_small_bond_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .bond()
                .small_bond_daily()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_bond_small_bond_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.bond().small_bond_daily().latest().fetch().await).await;
    }
}

// --- ETP API Tests ---

#[tokio::test]
#[ignore]
async fn test_etp_etf_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.etp().etf_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_etp_etf_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.etp().etf_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_etp_etn_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.etp().etn_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_etp_etn_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.etp().etn_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_etp_elw_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.etp().elw_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_etp_elw_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.etp().elw_daily().latest().fetch().await).await;
    }
}

// --- Derivative API Tests ---

#[tokio::test]
#[ignore]
async fn test_derivative_futures_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .derivative()
                .futures_daily()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_derivative_futures_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.derivative().futures_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_derivative_equity_stock_futures_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .derivative()
                .equity_stock_futures_daily()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_derivative_equity_stock_futures_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .derivative()
                .equity_stock_futures_daily()
                .latest()
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_derivative_equity_kosdaq_futures_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .derivative()
                .equity_kosdaq_futures_daily()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_derivative_equity_kosdaq_futures_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .derivative()
                .equity_kosdaq_futures_daily()
                .latest()
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_derivative_options_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .derivative()
                .options_daily()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_derivative_options_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.derivative().options_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_derivative_equity_stock_options_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .derivative()
                .equity_stock_options_daily()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_derivative_equity_stock_options_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .derivative()
                .equity_stock_options_daily()
                .latest()
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_derivative_equity_kosdaq_options_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .derivative()
                .equity_kosdaq_options_daily()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_derivative_equity_kosdaq_options_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .derivative()
                .equity_kosdaq_options_daily()
                .latest()
                .fetch()
                .await,
        )
        .await;
    }
}

// --- General API Tests ---

#[tokio::test]
#[ignore]
async fn test_general_oil_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.general().oil_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_general_oil_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.general().oil_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_general_gold_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.general().gold_daily().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_general_gold_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.general().gold_daily().latest().fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_general_emissions_daily_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(
            client
                .general()
                .emissions_daily()
                .date("20240105")
                .fetch()
                .await,
        )
        .await;
    }
}

#[tokio::test]
#[ignore]
async fn test_general_emissions_daily_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.general().emissions_daily().latest().fetch().await).await;
    }
}

// --- ESG API Tests ---

#[tokio::test]
#[ignore]
async fn test_esg_sri_bond_info_by_date() {
    if let Some(client) = setup() {
        run_fetch_test(client.esg().sri_bond_info().date("20240105").fetch().await).await;
    }
}

#[tokio::test]
#[ignore]
async fn test_esg_sri_bond_info_latest() {
    if let Some(client) = setup() {
        run_fetch_test(client.esg().sri_bond_info().latest().fetch().await).await;
    }
}
