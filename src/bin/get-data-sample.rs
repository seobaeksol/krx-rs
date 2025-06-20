use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde_json::Value;
use std::fs;

/// KRX API 샘플 데이터 요청기 (여러 endpoint 지원)
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// AUTH_KEY 값
    #[arg(long)]
    auth_key: String,

    /// Cookie 값 (선택)
    #[arg(long, default_value = "")]
    cookie: String,

    /// 기준일자 (YYYYMMDD, 여러 개 입력 가능)
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    dates: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();

    // (endpoint_name, endpoint_url)
    let endpoints = vec![
        (
            "krx_dd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/idx/krx_dd_trd",
        ),
        (
            "kospi_dd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/idx/kospi_dd_trd",
        ),
        (
            "kosdaq_dd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/idx/kosdaq_dd_trd",
        ),
        (
            "bon_dd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/idx/bon_dd_trd",
        ),
        (
            "drvprod_dd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/idx/drvprod_dd_trd",
        ),
        (
            "stk_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/sto/stk_bydd_trd",
        ),
        (
            "ksq_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/sto/ksq_bydd_trd",
        ),
        (
            "knx_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/sto/knx_bydd_trd",
        ),
        (
            "sw_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/sto/sw_bydd_trd",
        ),
        (
            "sr_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/sto/sr_bydd_trd",
        ),
        (
            "stk_isu_base_info",
            "http://data-dbg.krx.co.kr/svc/apis/sto/stk_isu_base_info",
        ),
        (
            "ksq_isu_base_info",
            "http://data-dbg.krx.co.kr/svc/apis/sto/ksq_isu_base_info",
        ),
        (
            "knx_isu_base_info",
            "http://data-dbg.krx.co.kr/svc/apis/sto/knx_isu_base_info",
        ),
        (
            "etf_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/etp/etf_bydd_trd",
        ),
        (
            "etn_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/etp/etn_bydd_trd",
        ),
        (
            "elw_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/etp/elw_bydd_trd",
        ),
        (
            "kts_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/bon/kts_bydd_trd",
        ),
        (
            "bnd_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/bon/bnd_bydd_trd",
        ),
        (
            "smb_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/bon/smb_bydd_trd",
        ),
        (
            "fut_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/drv/fut_bydd_trd",
        ),
        (
            "eqsfu_stk_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/drv/eqsfu_stk_bydd_trd",
        ),
        (
            "eqkfu_ksq_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/drv/eqkfu_ksq_bydd_trd",
        ),
        (
            "opt_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/drv/opt_bydd_trd",
        ),
        (
            "eqsop_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/drv/eqsop_bydd_trd",
        ),
        (
            "eqkop_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/drv/eqkop_bydd_trd",
        ),
        (
            "oil_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/gen/oil_bydd_trd",
        ),
        (
            "gold_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/gen/gold_bydd_trd",
        ),
        (
            "ets_bydd_trd",
            "http://data-dbg.krx.co.kr/svc/apis/gen/ets_bydd_trd",
        ),
        (
            "sri_bond_info",
            "http://data-dbg.krx.co.kr/svc/apis/esg/sri_bond_info",
        ),
    ];

    fs::create_dir_all("samples")?;

    for (name, url) in &endpoints {
        for date in &args.dates {
            let mut headers = HeaderMap::new();
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/json; charset=utf-8"),
            );
            headers.insert("AUTH_KEY", HeaderValue::from_str(&args.auth_key)?);
            if !args.cookie.is_empty() {
                headers.insert("Cookie", HeaderValue::from_str(&args.cookie)?);
            }
            let body = serde_json::json!({"basDd": date});
            println!("[INFO] 요청: {} {}", name, date);
            let resp = client.post(*url).headers(headers).json(&body).send();
            match resp {
                Ok(r) => {
                    if !r.status().is_success() {
                        eprintln!("[ERROR] {} {} HTTP {}", name, date, r.status());
                        continue;
                    }
                    let json: Value = match r.json() {
                        Ok(j) => j,
                        Err(e) => {
                            eprintln!("[ERROR] {} {} JSON 파싱 실패: {}", name, date, e);
                            continue;
                        }
                    };
                    let file_path = format!("samples/{}_{}.json", name, date);
                    fs::write(&file_path, serde_json::to_string_pretty(&json)?)?;
                    println!("[OK] {} -> {}", name, file_path);
                }
                Err(e) => {
                    eprintln!("[ERROR] {} {} 요청 실패: {}", name, date, e);
                }
            }
        }
    }
    println!("모든 요청 완료");
    Ok(())
}
