## 22. 주식선물(코스닥) 일별매매정보

### 22.1 Description
파생상품시장의 주식선물 중 기초자산이 코스닥시장에 속하는 주식선물의 거래정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/drv/eqkfu_ksq_bydd_trd`

### 22.2 Request

#### 22.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 22.3 Response

#### 22.3.1 OutBlock_1
| Name            | Type   | Description      |
|-----------------|--------|------------------|
| BAS_DD          | string | 기준일자         |
| PROD_NM         | string | 상품구분         |
| MKT_NM          | string | 시장구분(정규/야간) |
| ISU_CD          | string | 종목코드         |
| ISU_NM          | string | 종목명           |
| TDD_CLSPRC      | string | 종가             |
| CMPPREVDD_PRC   | string | 대비             |
| TDD_OPNPRC      | string | 시가             |
| TDD_HGPRC       | string | 고가             |
| TDD_LWPRC       | string | 저가             |
| SPOT_PRC        | string | 현물가           |
| SETL_PRC        | string | 정산가           |
| ACC_TRDVOL      | string | 거래량           |
| ACC_TRDVAL      | string | 거래대금         |
| ACC_OPNINT_QTY  | string | 미결제약정       |

### 22.4 Request Sample
```json
{"basDd": "20240105"}
```

### 22.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "ACC_OPNINT_QTY": "1458",
      "ACC_TRDVAL": "51657000",
      "ACC_TRDVOL": "75",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "1100.00",
      "ISU_CD": "1DTV1000",
      "ISU_NM": "CJ ENM     F 202401 ",
      "MKT_NM": "정규",
      "PROD_NM": "CJ ENM 선물",
      "SETL_PRC": "69000.00",
      "SPOT_PRC": "68800.00",
      "TDD_CLSPRC": "69000.00",
      "TDD_HGPRC": "69300.00",
      "TDD_LWPRC": "68200.00",
      "TDD_OPNPRC": "68500.00"
    },
    {
      "ACC_OPNINT_QTY": "5",
      "ACC_TRDVAL": "2748000",
      "ACC_TRDVOL": "4",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "1000.00",
      "ISU_CD": "1DTV2000",
      "ISU_NM": "CJ ENM     F 202402 ",
      "MKT_NM": "정규",
      "PROD_NM": "CJ ENM 선물",
      "SETL_PRC": "69000.00",
      "SPOT_PRC": "68800.00",
      "TDD_CLSPRC": "69000.00",
      "TDD_HGPRC": "69000.00",
      "TDD_LWPRC": "68500.00",
      "TDD_OPNPRC": "68500.00"
    },
    {
      "ACC_OPNINT_QTY": "0",
      "ACC_TRDVAL": "0",
      "ACC_TRDVOL": "0",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-",
      "ISU_CD": "1DTV3000",
      "ISU_NM": "CJ ENM     F 202403 ",
      "MKT_NM": "정규",
      "PROD_NM": "CJ ENM 선물",
      "SETL_PRC": "69500.00",
      "SPOT_PRC": "68800.00",
      "TDD_CLSPRC": "-",
      "TDD_HGPRC": "-",
      "TDD_LWPRC": "-",
      "TDD_OPNPRC": "-"
    }
  ]
}
```
