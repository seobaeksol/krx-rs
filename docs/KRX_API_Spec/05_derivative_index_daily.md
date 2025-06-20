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
      "BAS_DD": "20240105",
      "CLSPRC_IDX": "1407.82",
      "CMPPREVDD_IDX": "9.19",
      "FLUC_RT": "0.66",
      "HGPRC_IDX": "1407.82",
      "IDX_CLSS": "선물지수",
      "IDX_NM": "미국달러선물지수",
      "LWPRC_IDX": "1400.87",
      "OPNPRC_IDX": "1403.01"
    },
    {
      "BAS_DD": "20240105",
      "CLSPRC_IDX": "501.57",
      "CMPPREVDD_IDX": "-3.10",
      "FLUC_RT": "-0.61",
      "HGPRC_IDX": "503.01",
      "IDX_CLSS": "선물지수",
      "IDX_NM": "엔선물지수",
      "LWPRC_IDX": "501.30",
      "OPNPRC_IDX": "502.13"
    },
    {
      "BAS_DD": "20240105",
      "CLSPRC_IDX": "846.32",
      "CMPPREVDD_IDX": "2.35",
      "FLUC_RT": "0.28",
      "HGPRC_IDX": "848.38",
      "IDX_CLSS": "선물지수",
      "IDX_NM": "유로선물지수",
      "LWPRC_IDX": "842.97",
      "OPNPRC_IDX": "842.97"
    }
  ]
}
```
