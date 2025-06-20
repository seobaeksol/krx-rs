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
      "ACC_TRDVAL": "104622050000",
      "ACC_TRDVOL": "103000000000",
      "BAS_DD": "20240105",
      "BND_EXP_TP_NM": "2",
      "CLSPRC": "10158.0",
      "CLSPRC_YD": "3.381",
      "CMPPREVDD_PRC": "-4.5",
      "GOVBND_ISU_TP_NM": "지표",
      "HGPRC": "10158.5",
      "HGPRC_YD": "3.378",
      "ISU_CD": "KR103503GD97",
      "ISU_NM": "국고03625-2509(23-8)",
      "LWPRC": "10155.5",
      "LWPRC_YD": "3.397",
      "MKT_NM": "국채전문유통시장",
      "OPNPRC": "10157.0",
      "OPNPRC_YD": "3.388"
    },
    {
      "ACC_TRDVAL": "1832976000000",
      "ACC_TRDVOL": "1798000000000",
      "BAS_DD": "20240105",
      "BND_EXP_TP_NM": "3",
      "CLSPRC": "10193.5",
      "CLSPRC_YD": "3.284",
      "CMPPREVDD_PRC": "-14.0",
      "GOVBND_ISU_TP_NM": "지표",
      "HGPRC": "10198.0",
      "HGPRC_YD": "3.268",
      "ISU_CD": "KR103501GDC8",
      "ISU_NM": "국고03875-2612(23-10)",
      "LWPRC": "10192.5",
      "LWPRC_YD": "3.288",
      "MKT_NM": "국채전문유통시장",
      "OPNPRC": "10198.0",
      "OPNPRC_YD": "3.268"
    },
    {
      "ACC_TRDVAL": "677047150000",
      "ACC_TRDVOL": "664000000000",
      "BAS_DD": "20240105",
      "BND_EXP_TP_NM": "5",
      "CLSPRC": "10194.0",
      "CLSPRC_YD": "3.315",
      "CMPPREVDD_PRC": "-24.5",
      "GOVBND_ISU_TP_NM": "지표",
      "HGPRC": "10202.0",
      "HGPRC_YD": "3.296",
      "ISU_CD": "KR103501GD99",
      "ISU_NM": "국고03500-2809(23-6)",
      "LWPRC": "10191.0",
      "LWPRC_YD": "3.322",
      "MKT_NM": "국채전문유통시장",
      "OPNPRC": "10195.5",
      "OPNPRC_YD": "3.311"
    }
  ]
}
```
