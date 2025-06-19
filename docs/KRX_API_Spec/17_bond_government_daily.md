## 17. 국채전문유통시장 일별매매정보

### 17.1 Description
국채전문유통시장에 상장되어있는 채권의 매매정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/bon/kts_bydd_trd`

### 17.2 Request

#### 17.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 17.3 Response

#### 17.3.1 OutBlock_1
| Name            | Type   | Description    |
|-----------------|--------|----------------|
| BAS_DD          | string | 기준일자       |
| MKT_NM          | string | 시장구분       |
| ISU_CD          | string | 종목코드       |
| ISU_NM          | string | 종목명         |
| BND_EXP_TP_NM   | string | 만기년수       |
| GOVBND_ISU_TP_NM| string | 종목구분       |
| CLSPRC          | string | 종가_가격      |
| CMPPREVDD_PRC   | string | 종가_대비      |
| CLSPRC_YD       | string | 종가_수익률    |
| OPNPRC          | string | 시가_가격      |
| OPNPRC_YD       | string | 시가_수익률    |
| HGPRC           | string | 고가_가격      |
| HGPRC_YD        | string | 고가_수익률    |
| LWPRC           | string | 저가_가격      |
| LWPRC_YD        | string | 저가_수익률    |
| ACC_TRDVOL      | string | 거래량         |
| ACC_TRDVAL      | string | 거래대금       |

### 17.4 Request Sample
```json
{"basDd": "20240105"}
```

### 17.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "BAS_DD": "__",
      "MKT_NM": "__",
      "ISU_CD": "__",
      "ISU_NM": "__",
      "BND_EXP_TP_NM": "__",
      "GOVBND_ISU_TP_NM": "__",
      "CLSPRC": "__",
      "CMPPREVDD_PRC": "__",
      "CLSPRC_YD": "__",
      "OPNPRC": "__",
      "OPNPRC_YD": "__",
      "HGPRC": "__",
      "HGPRC_YD": "__",
      "LWPRC": "__",
      "LWPRC_YD": "__",
      "ACC_TRDVOL": "__",
      "ACC_TRDVAL": "__"
    },
    {
      "BAS_DD": "__",
      "MKT_NM": "__",
      "ISU_CD": "__",
      "ISU_NM": "__",
      "BND_EXP_TP_NM": "__",
      "GOVBND_ISU_TP_NM": "__",
      "CLSPRC": "__",
      "CMPPREVDD_PRC": "__",
      "CLSPRC_YD": "__",
      "OPNPRC": "__",
      "OPNPRC_YD": "__",
      "HGPRC": "__",
      "HGPRC_YD": "__",
      "LWPRC": "__",
      "LWPRC_YD": "__",
      "ACC_TRDVOL": "__",
      "ACC_TRDVAL": "__"
    }
  ]
}
```
