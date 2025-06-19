## 4. 채권지수 시세정보

### 4.1 Description
채권지수의 시세정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/idx/bon_dd_trd`

### 4.2 Request

#### 4.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 4.3 Response

#### 4.3.1 OutBlock_1
| Name                      | Type   | Description           |
|---------------------------|--------|-----------------------|
| BAS_DD                    | string | 기준일자              |
| BND_IDX_GRP_NM            | string | 지수명                |
| TOT_EARNG_IDX             | string | 총수익지수_종가       |
| TOT_EARNG_IDX_CMPPREVDD   | string | 총수익지수_대비       |
| NETPRC_IDX                | string | 순가격지수_종가       |
| NETPRC_IDX_CMPPREVDD      | string | 순가격지수_대비       |
| ZERO_REINVST_IDX          | string | 제로재투자지수_종가   |
| ZERO_REINVST_IDX_CMPPREVDD| string | 제로재투자지수_대비   |
| CALL_REINVST_IDX          | string | 콜재투자지수_종가     |
| CALL_REINVST_IDX_CMPPREVDD| string | 콜재투자지수_대비     |
| MKT_PRC_IDX               | string | 시장가격지수_종가     |
| MKT_PRC_IDX_CMPPREVDD     | string | 시장가격지수_대비     |
| AVG_DURATION              | string | 듀레이션              |
| AVG_CONVEXITY_PRC         | string | 컨벡시티              |
| BND_IDX_AVG_YD            | string | YTM                   |

### 4.4 Request Sample
```json
{P240105"}
```

### 4.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "BAS_DD": "__",
      "BND_IDX_GRP_NM": "__",
      "TOT_EARNG_IDX": "-",
      "TOT_EARNG_IDX_CMPPREVDD": "-",
      "NETPRC_IDX": "-",
      "NETPRC_IDX_CMPPREVDD": "-",
      "ZERO_REINVST_IDX": "-",
      "ZERO_REINVST_IDX_CMPPREVDD": "-",
      "CALL_REINVST_IDX": "-",
      "CALL_REINVST_IDX_CMPPREVDD": "-",
      "MKT_PRC_IDX": "-",
      "MKT_PRC_IDX_CMPPREVDD": "-",
      "AVG_DURATION": "-",
      "AVG_CONVEXITY_PRC": "-",
      "BND_IDX_AVG_YD": "-"
    },
    {
      "BAS_DD": "__",
      "BND_IDX_GRP_NM": "__",
      "TOT_EARNG_IDX": "-",
      "TOT_EARNG_IDX_CMPPREVDD": "-",
      "NETPRC_IDX": "-",
      "NETPRC_IDX_CMPPREVDD": "-",
      "ZERO_REINVST_IDX": "-",
      "ZERO_REINVST_IDX_CMPPREVDD": "-",
      "CALL_REINVST_IDX": "-",
      "CALL_REINVST_IDX_CMPPREVDD": "-",
      "MKT_PRC_IDX": "-",
      "MKT_PRC_IDX_CMPPREVDD": "-",
      "AVG_DURATION": "-",
      "AVG_CONVEXITY_PRC": "-",
      "BND_IDX_AVG_YD": "-"
    }
  ]
}
```
