## 5. 파생상품지수 시세정보

### 5.1 Description
파생상품지수의 시세정보를 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/idx/drvprod_dd_trd`

### 5.2 Request

#### 5.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 5.3 Response

#### 5.3.1 OutBlock_1
| Name            | Type   | Description |
|-----------------|--------|-------------|
| BAS_DD          | string | 기준일자    |
| IDX_CLSS        | string | 계열구분    |
| IDX_NM          | string | 지수명      |
| CLSPRC_IDX      | string | 종가        |
| CMPPREVDD_IDX   | string | 대비        |
| FLUC_RT         | string | 등락률      |
| OPNPRC_IDX      | string | 시가        |
| HGPRC_IDX       | string | 고가        |
| LWPRC_IDX       | string | 저가        |

### 5.4 Request Sample
```json
{"basDd": "20240105"}
```

### 5.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "BAS_DD": "__",
      "IDX_CLSS": "__",
      "IDX_NM": "__",
      "CLSPRC_IDX": "-",
      "CMPPREVDD_IDX": "-",
      "FLUC_RT": "-",
      "OPNPRC_IDX": "-",
      "HGPRC_IDX": "-",
      "LWPRC_IDX": "-"
    },
    {
      "BAS_DD": "__",
      "IDX_CLSS": "__",
      "IDX_NM": "__",
      "CLSPRC_IDX": "-",
      "CMPPREVDD_IDX": "-",
      "FLUC_RT": "-",
      "OPNPRC_IDX": "-",
      "HGPRC_IDX": "-",
      "LWPRC_IDX": "-"
    }
  ]
}
```
