# 석유시장 일별매매정보

## 26. 석유시장 일별매매정보

### 26.1 Overview
KRX 석유시장의 매매정보 제공

### 26.2 How to Request
**Endpoint URL:** http://data-dbg.krx.co.kr/svc/apis/gen/oil_bydd_trd

```bash
curl -X GET "http://data-dbg.krx.co.kr/svc/apis/gen/oil_bydd_trd?basDd=20240105" \
     -H "Content-Type: application/json"
```

### 26.3 Request & Response

| Name            | Type   | Description      |
|-----------------|--------|------------------|
| basDd           | string | 기준일자         |

| Name            | Type   | Description      |
|-----------------|--------|------------------|
| BAS_DD          | string | 기준일자         |
| OIL_NM          | string | 유종구분         |
| WT_AVG_PRC      | string | 가중평균가격_경쟁|
| WT_DIS_AVG_PRC  | string | 가중평균가격_협의|
| ACC_TRDVOL      | string | 거래량           |
| ACC_TRDVAL      | string | 거래대금         |

### 26.4 Request Sample
```json
{"basDd": "20240105"}
```

### 26.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "BAS_DD": "2024/01/05",
      "OIL_NM": "경유",
      "WT_AVG_PRC": "1485.23",
      "WT_DIS_AVG_PRC": "1482.15",
      "ACC_TRDVOL": "125000",
      "ACC_TRDVAL": "185653750000"
    },
    {
      "BAS_DD": "2024/01/05",
      "OIL_NM": "휘발유",
      "WT_AVG_PRC": "1612.45",
      "WT_DIS_AVG_PRC": "1610.25",
      "ACC_TRDVOL": "98000",
      "ACC_TRDVAL": "158020100000"
    }
  ]
}
```