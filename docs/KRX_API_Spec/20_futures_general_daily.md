## 20. 선물 일별매매정보 (주식선물外)

### 20.1 Description
파생상품시장의 선물 중 주식선물을 제외한 선물의 매매정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/drv/fut_bydd_trd`

### 20.2 Request

#### 20.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 20.3 Response

#### 20.3.1 OutBlock_1
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

### 20.4 Request Sample
```json
{"basDd": "20240105"}
```

### 20.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "ACC_OPNINT_QTY": "185074",
      "ACC_TRDVAL": "7659138250000",
      "ACC_TRDVOL": "67384",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-0.54",
      "ISU_CD": "167V3000",
      "ISU_NM": "10년국채   F 202403 (주간)",
      "MKT_NM": "정규",
      "PROD_NM": "10년국채 선물",
      "SETL_PRC": "113.63",
      "SPOT_PRC": "113.75",
      "TDD_CLSPRC": "113.63",
      "TDD_HGPRC": "113.88",
      "TDD_LWPRC": "113.46",
      "TDD_OPNPRC": "113.72"
    },
    {
      "ACC_OPNINT_QTY": "11",
      "ACC_TRDVAL": "227760000",
      "ACC_TRDVOL": "2",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-0.45",
      "ISU_CD": "167V6000",
      "ISU_NM": "10년국채   F 202406 (주간)",
      "MKT_NM": "정규",
      "PROD_NM": "10년국채 선물",
      "SETL_PRC": "113.88",
      "SPOT_PRC": "113.75",
      "TDD_CLSPRC": "113.88",
      "TDD_HGPRC": "113.88",
      "TDD_LWPRC": "113.88",
      "TDD_OPNPRC": "113.88"
    },
    {
      "ACC_OPNINT_QTY": "-",
      "ACC_TRDVAL": "227570000",
      "ACC_TRDVOL": "1",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-0.17",
      "ISU_CD": "467V3V6S",
      "ISU_NM": "10년국채   SP 2403-2406 (주간)",
      "MKT_NM": "정규",
      "PROD_NM": "10년국채 선물",
      "SETL_PRC": "0.00",
      "SPOT_PRC": "113.75",
      "TDD_CLSPRC": "-0.17",
      "TDD_HGPRC": "-0.17",
      "TDD_LWPRC": "-0.17",
      "TDD_OPNPRC": "-0.17"
    }
  ]
}
```
