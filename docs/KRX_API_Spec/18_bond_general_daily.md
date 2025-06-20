## 18. 일반채권시장 일별매매정보

### 18.1 Description
일반채권시장에 상장되어있는 채권의 매매정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/bon/bnd_bydd_trd`

### 18.2 Request

#### 18.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 18.3 Response

#### 18.3.1 OutBlock_1
| Name            | Type   | Description    |
|-----------------|--------|----------------|
| BAS_DD          | string | 기준일자       |
| MKT_NM          | string | 시장구분       |
| ISU_CD          | string | 종목코드       |
| ISU_NM          | string | 종목명         |
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

### 18.4 Request Sample
```json
{"basDd": "20240105"}
```

### 18.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "188460",
      "ACC_TRDVOL": "200000",
      "BAS_DD": "20240105",
      "CLSPRC": "9423.0",
      "CLSPRC_YD": "3.617",
      "CMPPREVDD_PRC": "",
      "HGPRC": "9423.0",
      "HGPRC_YD": "3.617",
      "ISU_CD": "KR101501DC14",
      "ISU_NM": "국민주택1종22-01",
      "LWPRC": "9423.0",
      "LWPRC_YD": "3.617",
      "MKT_NM": "일반채권시장",
      "OPNPRC": "9423.0",
      "OPNPRC_YD": "3.617"
    },
    {
      "ACC_TRDVAL": "34295467",
      "ACC_TRDVOL": "36540000",
      "BAS_DD": "20240105",
      "CLSPRC": "9386.0",
      "CLSPRC_YD": "3.652",
      "CMPPREVDD_PRC": "",
      "HGPRC": "9386.0",
      "HGPRC_YD": "3.652",
      "ISU_CD": "KR101501DC22",
      "ISU_NM": "국민주택1종22-02",
      "LWPRC": "9385.5",
      "LWPRC_YD": "3.654",
      "MKT_NM": "일반채권시장",
      "OPNPRC": "9385.5",
      "OPNPRC_YD": "3.654"
    },
    {
      "ACC_TRDVAL": "162668354",
      "ACC_TRDVOL": "175084000",
      "BAS_DD": "20240105",
      "CLSPRC": "9310.0",
      "CLSPRC_YD": "3.537",
      "CMPPREVDD_PRC": "11.5",
      "HGPRC": "9310.0",
      "HGPRC_YD": "3.537",
      "ISU_CD": "KR101501DC63",
      "ISU_NM": "국민주택1종22-06",
      "LWPRC": "9283.5",
      "LWPRC_YD": "3.621",
      "MKT_NM": "일반채권시장",
      "OPNPRC": "9291.0",
      "OPNPRC_YD": "3.597"
    }
  ]
}
```
