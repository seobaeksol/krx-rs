# 주식옵션(코스닥) 일별매매정보

## 25. 주식옵션(코스닥) 일별매매정보

### 25.1 Overview
파생상품시장의 주식옵션 중 기초자산이 코스닥시장에 속하는 주식옵션의 거래정보 제공

### 25.2 How to Request
**Endpoint URL:** http://data-dbg.krx.co.kr/svc/apis/drv/eqkop_bydd_trd

```bash
curl -X GET "http://data-dbg.krx.co.kr/svc/apis/drv/eqkop_bydd_trd?basDd=20240105" \
     -H "Content-Type: application/json"
```

### 25.3 Request & Response

| Name            | Type   | Description      |
|-----------------|--------|------------------|
| basDd           | string | 기준일자         |

| Name            | Type   | Description      |
|-----------------|--------|------------------|
| BAS_DD          | string | 기준일자         |
| PROD_NM         | string | 상품구분         |
| RGHT_TP_NM      | string | 권리유형(CALL/PUT)|
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

### 25.4 Request Sample
```json
{"basDd": "20240105"}
```

### 25.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "ACC_OPNINT_QTY": "0",
      "ACC_TRDVAL": "0",
      "ACC_TRDVOL": "0",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-",
      "IMP_VOLT": "45.10",
      "ISU_CD": "2CLV1056",
      "ISU_NM": "씨젠       C 202401    17,000(  10)",
      "NXTDD_BAS_PRC": "6400.00",
      "PROD_NM": "씨젠 옵션",
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
      "IMP_VOLT": "45.10",
      "ISU_CD": "2CLV1059",
      "ISU_NM": "씨젠       C 202401    17,500(  10)",
      "NXTDD_BAS_PRC": "5900.00",
      "PROD_NM": "씨젠 옵션",
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
      "IMP_VOLT": "45.10",
      "ISU_CD": "2CLV1054",
      "ISU_NM": "씨젠       C 202401    18,000(  10)",
      "NXTDD_BAS_PRC": "5400.00",
      "PROD_NM": "씨젠 옵션",
      "RGHT_TP_NM": "CALL",
      "TDD_CLSPRC": "-",
      "TDD_HGPRC": "-",
      "TDD_LWPRC": "-",
      "TDD_OPNPRC": "-"
    }
  ]
}
```