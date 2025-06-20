## 8. 코넥스 일별매매정보

### 8.1 Description
코넥스시장에 상장되어 있는 주권의 매매정보 제공

### How to Request

```bash
curl --location 'http://data-dbg.krx.co.kr/svc/apis/sto/knx_bydd_trd' \
  --header 'Content-Type: application/json; charset=utf-8' \
  --header 'AUTH_KEY: YOUR_AUTH_KEY' \
  --data '{"basDd": "20240105"}'
```

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/sto/knx_bydd_trd`

### 8.2 Request

#### 8.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 8.3 Response

#### 8.3.1 OutBlock_1
| Name            | Type   | Description  |
|-----------------|--------|--------------|
| BAS_DD          | string | 기준일자     |
| ISU_CD          | string | 종목코드     |
| ISU_NM          | string | 종목명       |
| MKT_NM          | string | 시장구분     |
| SECT_TP_NM      | string | 소속부       |
| TDD_CLSPRC      | string | 종가         |
| CMPPREVDD_PRC   | string | 대비         |
| FLUC_RT         | string | 등락률       |
| TDD_OPNPRC      | string | 시가         |
| TDD_HGPRC       | string | 고가         |
| TDD_LWPRC       | string | 저가         |
| ACC_TRDVOL      | string | 거래량       |
| ACC_TRDVAL      | string | 거래대금     |
| MKTCAP          | string | 시가총액     |
| LIST_SHRS       | string | 상장주식수   |

### 8.4 Request Sample
```json
{"basDd": "20240105"}
```

### 8.5 Response Sample

```json
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "3939120",
      "ACC_TRDVOL": "338",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-200",
      "FLUC_RT": "-1.64",
      "ISU_CD": "278990",
      "ISU_NM": "EMB",
      "LIST_SHRS": "4747454",
      "MKTCAP": "56969448000",
      "MKT_NM": "KONEX",
      "SECT_TP_NM": "일반기업부",
      "TDD_CLSPRC": "12000",
      "TDD_HGPRC": "12000",
      "TDD_LWPRC": "11160",
      "TDD_OPNPRC": "11500"
    },
    {
      "ACC_TRDVAL": "14255415",
      "ACC_TRDVOL": "6447",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "100",
      "FLUC_RT": "4.59",
      "ISU_CD": "343090",
      "ISU_NM": "HLB사이언스",
      "LIST_SHRS": "16321365",
      "MKTCAP": "37212712200",
      "MKT_NM": "KONEX",
      "SECT_TP_NM": "일반기업부",
      "TDD_CLSPRC": "2280",
      "TDD_HGPRC": "2400",
      "TDD_LWPRC": "2180",
      "TDD_OPNPRC": "2395"
    },
    {
      "ACC_TRDVAL": "266948",
      "ACC_TRDVOL": "151",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-15",
      "FLUC_RT": "-0.84",
      "ISU_CD": "112190",
      "ISU_NM": "KC산업",
      "LIST_SHRS": "7125253",
      "MKTCAP": "12640198822",
      "MKT_NM": "KONEX",
      "SECT_TP_NM": "일반기업부",
      "TDD_CLSPRC": "1774",
      "TDD_HGPRC": "1779",
      "TDD_LWPRC": "1600",
      "TDD_OPNPRC": "1779"
    }
  ]
}
```

