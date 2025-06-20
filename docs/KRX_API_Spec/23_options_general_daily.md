## 23. 옵션 일별매매정보 (주식옵션外)

### 23.1 Description
파생상품시장의 옵션 중 주식옵션을 제외한 옵션의 매매정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/drv/opt_bydd_trd`

### 23.2 Request

#### 23.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 23.3 Response

#### 23.3.1 OutBlock_1
| Name            | Type   | Description      |
|-----------------|--------|------------------|
| BAS_DD          | string | 기준일자         |
| PROD_NM         | string | 상품구분         |
| RGHT_TP_NM      | string | 권리유형(CALL/PUT) |
| ISU_CD          | string | 종목코드         |
| ISU_NM          | string | 종목명           |
| TDD_CLSPRC      | string | 종가             |
| CMPPREVDD_PRC   | string | 대비             |
| TDD_OPNPRC      | string | 시가             |
| TDD_HGPRC       | string | 고가             |
| TDD_LWPRC       | string | 저가             |
| IMP_VOLT        | string | 내재변동성       |
| NXTDD_BAS_PRC   | string | 익일정산가       |
| ACC_TRDVOL      | string | 거래량           |
| ACC_TRDVAL      | string | 거래대금         |
| ACC_OPNINT_QTY  | string | 미결제약정       |

### 23.4 Request Sample
```json
{"basDd": "20240105"}
```

### 23.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "ACC_OPNINT_QTY": "5",
      "ACC_TRDVAL": "0",
      "ACC_TRDVOL": "0",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-",
      "IMP_VOLT": "3.00",
      "ISU_CD": "205V1225",
      "ISU_NM": "미니코스피 C 202401 225.0 (정규)",
      "NXTDD_BAS_PRC": "122.30",
      "PROD_NM": "미니코스피200 옵션",
      "RGHT_TP_NM": "CALL",
      "TDD_CLSPRC": "-",
      "TDD_HGPRC": "-",
      "TDD_LWPRC": "-",
      "TDD_OPNPRC": "-"
    },
    {
      "ACC_OPNINT_QTY": "0",
      "ACC_TRDVAL": "0",
      "ACC_TRDVOL": "0",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-",
      "IMP_VOLT": "3.00",
      "ISU_CD": "205V1227",
      "ISU_NM": "미니코스피 C 202401 227.5 (정규)",
      "NXTDD_BAS_PRC": "119.80",
      "PROD_NM": "미니코스피200 옵션",
      "RGHT_TP_NM": "CALL",
      "TDD_CLSPRC": "-",
      "TDD_HGPRC": "-",
      "TDD_LWPRC": "-",
      "TDD_OPNPRC": "-"
    },
    {
      "ACC_OPNINT_QTY": "0",
      "ACC_TRDVAL": "0",
      "ACC_TRDVOL": "0",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-",
      "IMP_VOLT": "3.00",
      "ISU_CD": "205V1230",
      "ISU_NM": "미니코스피 C 202401 230.0 (정규)",
      "NXTDD_BAS_PRC": "117.30",
      "PROD_NM": "미니코스피200 옵션",
      "RGHT_TP_NM": "CALL",
      "TDD_CLSPRC": "-",
      "TDD_HGPRC": "-",
      "TDD_LWPRC": "-",
      "TDD_OPNPRC": "-"
    }
  ]
}
```
