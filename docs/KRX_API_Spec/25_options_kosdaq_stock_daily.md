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
      "BAS_DD": "2024/01/05",
      "PROD_NM": "주식옵션(카카오)",
      "RGHT_TP_NM": "CALL",
      "ISU_CD": "201SA207",
      "ISU_NM": "CALL 카카오 240119 50000",
      "TDD_CLSPRC": "1440",
      "CMPPREVDD_PRC": "40",
      "TDD_OPNPRC": "1400",
      "TDD_HGPRC": "1460",
      "TDD_LWPRC": "1380",
      "IMP_VOLT": "32.45",
      "NXTDD_BAS_PRC": "1445",
      "ACC_TRDVOL": "152",
      "ACC_TRDVAL": "218880000",
      "ACC_OPNINT_QTY": "3241"
    }
  ]
}
```