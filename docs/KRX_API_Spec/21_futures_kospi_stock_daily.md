## 21. 주식선물(유가) 일별매매정보

### 21.1 Description
파생상품시장의 주식선물 중 기초자산이 유가증권시장에 속하는 주식선물의 거래정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/drv/eqsfu_stk_bydd_trd`

### 21.2 Request

#### 21.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 21.3 Response

#### 21.3.1 OutBlock_1
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

### 21.4 Request Sample
```json
{"basDd": "20240105"}
```

### 21.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "BAS_DD": "__",
      "PROD_NM": "__",
      "MKT_NM": "__",
      "ISU_CD": "__",
      "ISU_NM": "__",
      "TDD_CLSPRC": "-",
      "CMPPREVDD_PRC": "-",
      "TDD_OPNPRC": "-",
      "TDD_HGPRC": "-",
      "TDD_LWPRC": "-",
      "SPOT_PRC": "-",
      "SETL_PRC": "0.00",
      "ACC_TRDVOL": "-",
      "ACC_TRDVAL": "-",
      "ACC_OPNINT_QTY": "-"
    },
    {
      "BAS_DD": "__",
      "PROD_NM": "__",
      "MKT_NM": "__",
      "ISU_CD": "__",
      "ISU_NM": "__",
      "TDD_CLSPRC": "-",
      "CMPPREVDD_PRC": "-",
      "TDD_OPNPRC": "-",
      "TDD_HGPRC": "-",
      "TDD_LWPRC": "-",
      "SPOT_PRC": "-",
      "SETL_PRC": "0.00",
      "ACC_TRDVOL": "-",
      "ACC_TRDVAL": "-",
      "ACC_OPNINT_QTY": "-"
    }
  ]
}
```
