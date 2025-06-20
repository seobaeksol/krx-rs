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
      "ACC_TRDVAL": "11175754363",
      "ACC_TRDVOL": "7746580",
      "BAS_DD": "20240105",
      "OIL_NM": "휘발유",
      "WT_AVG_PRC": "1430.00",
      "WT_DIS_AVG_PRC": "1443.93"
    },
    {
      "ACC_TRDVAL": "15593509525",
      "ACC_TRDVOL": "11699817",
      "BAS_DD": "20240105",
      "OIL_NM": "경유",
      "WT_AVG_PRC": "1315.00",
      "WT_DIS_AVG_PRC": "1336.04"
    },
    {
      "ACC_TRDVAL": "5472329710",
      "ACC_TRDVOL": "5286223",
      "BAS_DD": "20240105",
      "OIL_NM": "등유",
      "WT_AVG_PRC": "1030.00",
      "WT_DIS_AVG_PRC": "1035.63"
    }
  ]
}
```