# 배출권 시장 일별매매정보

## 28. 배출권 시장 일별매매정보

### 28.1 Overview
KRX 탄소배출권 시장의 매매정보 제공

### 28.2 How to Request
**Endpoint URL:** http://data-dbg.krx.co.kr/svc/apis/gen/ets_bydd_trd

```bash
curl -X GET "http://data-dbg.krx.co.kr/svc/apis/gen/ets_bydd_trd?basDd=20240105" \
     -H "Content-Type: application/json"
```

### 28.3 Request & Response

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

### 28.4 Request Sample
```json
{"basDd": "20240105"}
```

### 28.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "BAS_DD": "2024/01/05",
      "ISU_CD": "KAU24",
      "ISU_NM": "KAU24",
      "TDD_CLSPRC": "23500",
      "CMPPREVDD_PRC": "-200",
      "FLUC_RT": "-0.84",
      "TDD_OPNPRC": "23700",
      "TDD_HGPRC": "23800",
      "TDD_LWPRC": "23450",
      "ACC_TRDVOL": "85420",
      "ACC_TRDVAL": "2007370000"
    },
    {
      "BAS_DD": "2024/01/05",
      "ISU_CD": "KCU24",
      "ISU_NM": "KCU24",
      "TDD_CLSPRC": "22800",
      "CMPPREVDD_PRC": "-150",
      "FLUC_RT": "-0.65",
      "TDD_OPNPRC": "22950",
      "TDD_HGPRC": "23000",
      "TDD_LWPRC": "22750",
      "ACC_TRDVOL": "42150",
      "ACC_TRDVAL": "961020000"
    }
  ]
}
```