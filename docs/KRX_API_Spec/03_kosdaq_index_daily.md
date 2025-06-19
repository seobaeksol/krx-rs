## 3. KOSDAQ 시리즈 일별시세정보

### 3.1 Description
KOSDAQ 시리즈 지수의 시세정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/idx/kosdaq_dd_trd`

### 3.2 Request

#### 3.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 3.3 Response

#### 3.3.1 OutBlock_1
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

### 3.4 Request Sample
```json
{P240105"}
```

### 3.5 
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "10173764925209",
      "ACC_TRDVOL": "1047923558",
      "BAS_DD": "20240105",
      "CLSPRC_IDX": "-",
      "CMPPREVDD_IDX": "-",
      "FLUC_RT": "-",
      "HGPRC_IDX": "-",
      "IDX_CLSS": "KOSDAQ",
      "IDX_NM": "코스닥 (외국주포함)",
      "LWPRC_IDX": "-",
      "MKTCAP": "426191476679294",
      "OPNPRC_IDX": "-"
    },
    {
      "ACC_TRDVAL": "10151817875099",
      "ACC_TRDVOL": "1004129628",
      "BAS_DD": "20240105",
      "CLSPRC_IDX": "878.33",
      "CMPPREVDD_IDX": "12.08",
      "FLUC_RT": "1.39",
      "HGPRC_IDX": "878.98",
      "IDX_CLSS": "KOSDAQ",
      "IDX_NM": "코스닥",
      "LWPRC_IDX": "864.86",
      "MKTCAP": "423306961831493",
      "OPNPRC_IDX": "866.81"
    },
    {
      "ACC_TRDVAL": "3549536873730",
      "ACC_TRDVOL": "74099369",
      "BAS_DD": "20240105",
      "CLSPRC_IDX": "1399.33",
      "CMPPREVDD_IDX": "27.57",
      "FLUC_RT": "2.01",
      "HGPRC_IDX": "1402.03",
      "IDX_CLSS": "KOSDAQ",
      "IDX_NM": "코스닥 150",
      "LWPRC_IDX": "1367.85",
      "MKTCAP": "210499603857705",
      "OPNPRC_IDX": "1370.99"
    }
  ]
}

