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
{"basDd": "20240105"}
```

### 15.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "7880",
      "ACC_TRDVOL": "1",
      "BAS_DD": "20240105",
      "CMPPREVDD_IDX": "-24.19",
      "CMPPREVDD_PRC": "90",
      "FLUC_RT": "1.16",
      "FLUC_RT_IDX": "-0.54",
      "IDX_IND_NM": "CSI 300 NTR",
      "INDIC_VAL_AMT": "15647060000",
      "ISU_CD": "580048",
      "ISU_NM": "KB CSI 300 ETN",
      "LIST_SHRS": "2000000",
      "MKTCAP": "15760000000",
      "OBJ_STKPRC_IDX": "4488.91",
      "PER1SECU_INDIC_VAL": "7823.53",
      "TDD_CLSPRC": "7880",
      "TDD_HGPRC": "7880",
      "TDD_LWPRC": "7880",
      "TDD_OPNPRC": "7880"
    },
    {
      "ACC_TRDVAL": "80100",
      "ACC_TRDVOL": "10",
      "BAS_DD": "20240105",
      "CMPPREVDD_IDX": "-85.74",
      "CMPPREVDD_PRC": "-30",
      "FLUC_RT": "-0.37",
      "FLUC_RT_IDX": "-1.37",
      "IDX_IND_NM": "CSI 500 NTR",
      "INDIC_VAL_AMT": "15897940000",
      "ISU_CD": "580049",
      "ISU_NM": "KB CSI 500 ETN",
      "LIST_SHRS": "2000000",
      "MKTCAP": "16020000000",
      "OBJ_STKPRC_IDX": "6165.94",
      "PER1SECU_INDIC_VAL": "7948.97",
      "TDD_CLSPRC": "8010",
      "TDD_HGPRC": "8010",
      "TDD_LWPRC": "8010",
      "TDD_OPNPRC": "8010"
    },
    {
      "ACC_TRDVAL": "19955",
      "ACC_TRDVOL": "2",
      "BAS_DD": "20240105",
      "CMPPREVDD_IDX": "29.64",
      "CMPPREVDD_PRC": "-30",
      "FLUC_RT": "-0.30",
      "FLUC_RT_IDX": "1.84",
      "IDX_IND_NM": "코스닥 150 선물지수 TWAP형",
      "INDIC_VAL_AMT": "7142198000",
      "ISU_CD": "580030",
      "ISU_NM": "KB KOSDAQ 150 선물 ETN",
      "LIST_SHRS": "700000",
      "MKTCAP": "6975500000",
      "OBJ_STKPRC_IDX": "1642.42",
      "PER1SECU_INDIC_VAL": "10203.14",
      "TDD_CLSPRC": "9965",
      "TDD_HGPRC": "9990",
      "TDD_LWPRC": "9965",
      "TDD_OPNPRC": "9990"
    }
  ]
}
```
