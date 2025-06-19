## 15. ETN 일별매매정보

### 15.1 Description
ETN(상장지수증권)의 매매정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/etp/etn_bydd_trd`

### 15.2 Request

#### 15.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 15.3 Response

#### 15.3.1 OutBlock_1
| Name                  | Type   | Description            |
|-----------------------|--------|------------------------|
| BAS_DD                | string | 기준일자               |
| ISU_CD                | string | 종목코드               |
| ISU_NM                | string | 종목명                 |
| TDD_CLSPRC            | string | 종가                   |
| CMPPREVDD_PRC         | string | 대비                   |
| FLUC_RT               | string | 등락률                 |
| PER1SECU_INDIC_VAL    | string | 지표가치(IV)           |
| TDD_OPNPRC            | string | 시가                   |
| TDD_HGPRC             | string | 고가                   |
| TDD_LWPRC             | string | 저가                   |
| ACC_TRDVOL            | string | 거래량                 |
| ACC_TRDVAL            | string | 거래대금               |
| MKTCAP                | string | 시가총액               |
| INDIC_VAL_AMT         | string | 지표가치총액           |
| LIST_SHRS             | string | 상장증권수             |
| IDX_IND_NM            | string | 기초지수_지수명        |
| OBJ_STKPRC_IDX        | string | 기초지수_종가          |
| CMPPREVDD_IDX         | string | 기초지수_대비          |
| FLUC_RT_IDX           | string | 기초지수_등락률        |

### 15.4 Request Sample
```json
{P240105"}
```

### 15.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "BAS_DD": "__",
      "ISU_CD": "__",
      "ISU_NM": "__",
      "TDD_CLSPRC": "-",
      "CMPPREVDD_PRC": "-",
      "FLUC_RT": "-",
      "PER1SECU_INDIC_VAL": "-",
      "TDD_OPNPRC": "-",
      "TDD_HGPRC": "-",
      "TDD_LWPRC": "-",
      "ACC_TRDVOL": "-",
      "ACC_TRDVAL": "-",
      "MKTCAP": "-",
      "INDIC_VAL_AMT": "-",
      "LIST_SHRS": "-",
      "IDX_IND_NM": "__",
      "OBJ_STKPRC_IDX": "-",
      "CMPPREVDD_IDX": "-",
      "FLUC_RT_IDX": "-"
    },
    {
      "BAS_DD": "__",
      "ISU_CD": "__",
      "ISU_NM": "__",
      "TDD_CLSPRC": "-",
      "CMPPREVDD_PRC": "-",
      "FLUC_RT": "-",
      "PER1SECU_INDIC_VAL": "-",
      "TDD_OPNPRC": "-",
      "TDD_HGPRC": "-",
      "TDD_LWPRC": "-",
      "ACC_TRDVOL": "-",
      "ACC_TRDVAL": "-",
      "MKTCAP": "-",
      "INDIC_VAL_AMT": "-",
      "LIST_SHRS": "-",
      "IDX_IND_NM": "__",
      "OBJ_STKPRC_IDX": "-",
      "CMPPREVDD_IDX": "-",
      "FLUC_RT_IDX": "-"
    }
  ]
}
```
