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
{"basDd": "20240105"}
```

### 4.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "AVG_CONVEXITY_PRC": "80.091",
      "AVG_DURATION": "5.238",
      "BAS_DD": "20240105",
      "BND_IDX_AVG_YD": "3.555",
      "BND_IDX_GRP_NM": "KRX 채권지수",
      "CALL_REINVST_IDX": "187.17",
      "CALL_REINVST_IDX_CMPPREVDD": "-0.28",
      "MKT_PRC_IDX": "101.06",
      "MKT_PRC_IDX_CMPPREVDD": "-0.16",
      "NETPRC_IDX": "100.36",
      "NETPRC_IDX_CMPPREVDD": "-0.16",
      "TOT_EARNG_IDX": "188.33",
      "TOT_EARNG_IDX_CMPPREVDD": "-0.29",
      "ZERO_REINVST_IDX": "182.01",
      "ZERO_REINVST_IDX_CMPPREVDD": "-0.28"
    },
    {
      "AVG_CONVEXITY_PRC": "10.623",
      "AVG_DURATION": "2.794",
      "BAS_DD": "20240105",
      "BND_IDX_AVG_YD": "3.330",
      "BND_IDX_GRP_NM": "KTB 지수",
      "CALL_REINVST_IDX": "-",
      "CALL_REINVST_IDX_CMPPREVDD": "-",
      "MKT_PRC_IDX": "10313.04",
      "MKT_PRC_IDX_CMPPREVDD": "-11.82",
      "NETPRC_IDX": "10419.18",
      "NETPRC_IDX_CMPPREVDD": "-15.09",
      "TOT_EARNG_IDX": "15066.74",
      "TOT_EARNG_IDX_CMPPREVDD": "-17.26",
      "ZERO_REINVST_IDX": "-",
      "ZERO_REINVST_IDX_CMPPREVDD": "-"
    },
    {
      "AVG_CONVEXITY_PRC": "23.676",
      "AVG_DURATION": "4.004",
      "BAS_DD": "20240105",
      "BND_IDX_AVG_YD": "3.328",
      "BND_IDX_GRP_NM": "국고채프라임지수",
      "CALL_REINVST_IDX": "-",
      "CALL_REINVST_IDX_CMPPREVDD": "-",
      "MKT_PRC_IDX": "-",
      "MKT_PRC_IDX_CMPPREVDD": "-",
      "NETPRC_IDX": "105.97",
      "NETPRC_IDX_CMPPREVDD": "-0.24",
      "TOT_EARNG_IDX": "187.19",
      "TOT_EARNG_IDX_CMPPREVDD": "-0.37",
      "ZERO_REINVST_IDX": "-",
      "ZERO_REINVST_IDX_CMPPREVDD": "-"
    }
  ]
}
```
