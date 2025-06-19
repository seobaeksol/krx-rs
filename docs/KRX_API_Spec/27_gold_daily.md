# 금시장 일별매매정보

## 27. 금시장 일별매매정보

### 27.1 Overview
KRX 금시장 매매정보 제공

### 27.2 How to Request
**Endpoint URL:** http://data-dbg.krx.co.kr/svc/apis/gen/gold_bydd_trd

```bash
curl -X GET "http://data-dbg.krx.co.kr/svc/apis/gen/gold_bydd_trd?basDd=20240105" \
     -H "Content-Type: application/json"
```

### 27.3 Request & Response

| Name            | Type   | Description      |
|-----------------|--------|------------------|
| basDd           | string | 기준일자         |

| Name            | Type   | Description      |
|-----------------|--------|------------------|
| BAS_DD          | string | 기준일자         |
| ISU_CD          | string | 종목코드         |
| ISU_NM          | string | 종목명           |
| TDD_CLSPRC      | string | 종가             |
| CMPPREVDD_PRC   | string | 대비             |
| FLUC_RT         | string | 등락률           |
| TDD_OPNPRC      | string | 시가             |
| TDD_HGPRC       | string | 고가             |
| TDD_LWPRC       | string | 저가             |
| ACC_TRDVOL      | string | 거래량           |
| ACC_TRDVAL      | string | 거래대금         |

### 27.4 Request Sample
```json
{"basDd": "20240105"}
```

### 27.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "BAS_DD": "2024/01/05",
      "ISU_CD": "KRG001",
      "ISU_NM": "금 1kg",
      "TDD_CLSPRC": "85250000",
      "CMPPREVDD_PRC": "520000",
      "FLUC_RT": "0.61",
      "TDD_OPNPRC": "84800000",
      "TDD_HGPRC": "85350000",
      "TDD_LWPRC": "84750000",
      "ACC_TRDVOL": "152",
      "ACC_TRDVAL": "12958000000"
    }
  ]
}
```