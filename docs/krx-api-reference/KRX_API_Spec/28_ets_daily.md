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
      "ACC_TRDVAL": "4805945740",
      "ACC_TRDVOL": "581003",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-410",
      "FLUC_RT": "-4.61",
      "ISU_CD": "05003230",
      "ISU_NM": "KAU23",
      "TDD_CLSPRC": "8490",
      "TDD_HGPRC": "8840",
      "TDD_LWPRC": "8200",
      "TDD_OPNPRC": "8200"
    },
    {
      "ACC_TRDVAL": "0",
      "ACC_TRDVOL": "0",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "0",
      "FLUC_RT": "0.00",
      "ISU_CD": "05003240",
      "ISU_NM": "KAU24",
      "TDD_CLSPRC": "8610",
      "TDD_HGPRC": "0",
      "TDD_LWPRC": "0",
      "TDD_OPNPRC": "0"
    },
    {
      "ACC_TRDVAL": "0",
      "ACC_TRDVOL": "0",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "0",
      "FLUC_RT": "0.00",
      "ISU_CD": "05003250",
      "ISU_NM": "KAU25",
      "TDD_CLSPRC": "8610",
      "TDD_HGPRC": "0",
      "TDD_LWPRC": "0",
      "TDD_OPNPRC": "0"
    }
  ]
}
```