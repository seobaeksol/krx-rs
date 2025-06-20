## 19. 소액채권시장 일별매매정보

### 19.1 Description
소액채권시장에 상장되어있는 채권의 매매정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/bon/smb_bydd_trd`

### 19.2 Request

#### 19.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 19.3 Response

#### 19.3.1 OutBlock_1
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

### 19.4 Request Sample
```json
{"basDd": "20240105"}
```

### 19.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "721882835",
      "ACC_TRDVOL": "806573000",
      "BAS_DD": "20240105",
      "CLSPRC": "8950.0",
      "CLSPRC_YD": "3.582",
      "CMPPREVDD_PRC": "-16.0",
      "HGPRC": "8950.0",
      "HGPRC_YD": "3.582",
      "ISU_CD": "KR101501DDC7",
      "ISU_NM": "국민주택1종23-12",
      "LWPRC": "8950.0",
      "LWPRC_YD": "3.582",
      "MKT_NM": "소액채권시장",
      "OPNPRC": "8950.0",
      "OPNPRC_YD": "3.582"
    },
    {
      "ACC_TRDVAL": "144154175",
      "ACC_TRDVOL": "153046000",
      "BAS_DD": "20240105",
      "CLSPRC": "9424.0",
      "CLSPRC_YD": "3.669",
      "CMPPREVDD_PRC": "24.0",
      "HGPRC": "9424.0",
      "HGPRC_YD": "3.669",
      "ISU_CD": "KR2101012E13",
      "ISU_NM": "세종지역개발24-01",
      "LWPRC": "9405.0",
      "LWPRC_YD": "3.710",
      "MKT_NM": "소액채권시장",
      "OPNPRC": "9406.0",
      "OPNPRC_YD": "3.708"
    },
    {
      "ACC_TRDVAL": "5125048919",
      "ACC_TRDVOL": "5601678000",
      "BAS_DD": "20240105",
      "CLSPRC": "9163.0",
      "CLSPRC_YD": "3.704",
      "CMPPREVDD_PRC": "13.0",
      "HGPRC": "9163.0",
      "HGPRC_YD": "3.704",
      "ISU_CD": "KR2001024DC8",
      "ISU_NM": "서울도시철도23-12",
      "LWPRC": "9130.0",
      "LWPRC_YD": "3.757",
      "MKT_NM": "소액채권시장",
      "OPNPRC": "9148.0",
      "OPNPRC_YD": "3.728"
    }
  ]
}
```
