## 2. KOSPI 시리즈 일별시세정보

### 2.1 Description
KOSPI 시리즈 지수의 시세정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/idx/kospi_dd_trd`

### 2.2 Request

#### 2.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 2.3 Response

#### 2.3.1 OutBlock_1
| Name            | Type   | Description      |
|-----------------|--------|------------------|
| BAS_DD          | string | 기준일자         |
| IDX_CLSS        | string | 계열구분         |
| IDX_NM          | string | 지수명           |
| CLSPRC_IDX      | string | 종가             |
| CMPPREVDD_IDX   | string | 대비             |
| FLUC_RT         | string | 등락률           |
| OPNPRC_IDX      | string | 시가             |
| HGPRC_IDX       | string | 고가             |
| LWPRC_IDX       | string | 저가             |
| ACC_TRDVOL      | string | 거래량           |
| ACC_TRDVAL      | string | 거래대금         |
| MKTCAP          | string | 상장시가총액     |

### 2.4 Request Sample
```json
{P240105"}
```

### 2.5 
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "8384472928787",
      "ACC_TRDVOL": "522289892",
      "BAS_DD": "20240105",
      "CLSPRC_IDX": "-",
      "CMPPREVDD_IDX": "-",
      "FLUC_RT": "-",
      "HGPRC_IDX": "-",
      "IDX_CLSS": "KOSPI",
      "IDX_NM": "코스피 (외국주포함)",
      "LWPRC_IDX": "-",
      "MKTCAP": "2075148802709824",
      "OPNPRC_IDX": "-"
    },
    {
      "ACC_TRDVAL": "8379290435057",
      "ACC_TRDVOL": "520480370",
      "BAS_DD": "20240105",
      "CLSPRC_IDX": "2578.08",
      "CMPPREVDD_IDX": "-8.94",
      "FLUC_RT": "-0.35",
      "HGPRC_IDX": "2592.29",
      "IDX_CLSS": "KOSPI",
      "IDX_NM": "코스피",
      "LWPRC_IDX": "2572.60",
      "MKTCAP": "2074105333144984",
      "OPNPRC_IDX": "2586.89"
    },
    {
      "ACC_TRDVAL": "5723728835744",
      "ACC_TRDVOL": "82152940",
      "BAS_DD": "20240105",
      "CLSPRC_IDX": "347.22",
      "CMPPREVDD_IDX": "-0.85",
      "FLUC_RT": "-0.24",
      "HGPRC_IDX": "348.87",
      "IDX_CLSS": "KOSPI",
      "IDX_NM": "코스피 200",
      "LWPRC_IDX": "346.32",
      "MKTCAP": "1809009724361440",
      "OPNPRC_IDX": "348.22"
    }
  ]
}

