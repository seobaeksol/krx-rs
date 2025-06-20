# KRX API Specification

## 1. KRX 시리즈 일별시세정보

### 1.1 Description
KRX 시리즈 지수의 시세정보 제공

### How to Request

```bash
curl --location 'http://data-dbg.krx.co.kr/svc/apis/idx/krx_dd_trd' \
  --header 'Content-Type: application/json; charset=utf-8' \
  --header 'AUTH_KEY: YOUR_AUTH_KEY' \
  --data '{"basDd": "20240105"}'
```

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/idx/krx_dd_trd`

### 1.2 Request

#### 1.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 1.3 Response

#### 1.3.1 OutBlock_1
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

### 1.4 Request Sample
```json
{"basDd": "20240105"}
```

### 1.5 Response Sample

```json
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "-",
      "ACC_TRDVOL": "-",
      "BAS_DD": "20240105",
      "CLSPRC_IDX": "967.89",
      "CMPPREVDD_IDX": "2.10",
      "FLUC_RT": "0.22",
      "HGPRC_IDX": "-",
      "IDX_CLSS": "KRX",
      "IDX_NM": "코리아 밸류업 지수",
      "LWPRC_IDX": "-",
      "MKTCAP": "-",
      "OPNPRC_IDX": "-"
    },
    {
      "ACC_TRDVAL": "-",
      "ACC_TRDVOL": "-",
      "BAS_DD": "20240105",
      "CLSPRC_IDX": "1610.58",
      "CMPPREVDD_IDX": "0.27",
      "FLUC_RT": "0.02",
      "HGPRC_IDX": "-",
      "IDX_CLSS": "KRX",
      "IDX_NM": "KRX TMI",
      "LWPRC_IDX": "-",
      "MKTCAP": "-",
      "OPNPRC_IDX": "-"
    },
    {
      "ACC_TRDVAL": "8252309091974",
      "ACC_TRDVOL": "139123344",
      "BAS_DD": "20240105",
      "CLSPRC_IDX": "1608.11",
      "CMPPREVDD_IDX": "-0.15",
      "FLUC_RT": "-0.01",
      "HGPRC_IDX": "1612.19",
      "IDX_CLSS": "KRX",
      "IDX_NM": "KRX 300",
      "LWPRC_IDX": "1603.75",
      "MKTCAP": "1912238454981230",
      "OPNPRC_IDX": "1608.71"
    }
  ]
}
```

