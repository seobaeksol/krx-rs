## 7. 코스닥 일별매매정보

### 7.1 Description
코스닥시장에 상장되어 있는 주권의 매매정보 제공

### How to Request

```bash
curl --location 'http://data-dbg.krx.co.kr/svc/apis/sto/ksq_bydd_trd' \
  --header 'Content-Type: application/json; charset=utf-8' \
  --header 'AUTH_KEY: YOUR_AUTH_KEY' \
  --data '{"basDd": "20240105"}'
```

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/sto/ksq_bydd_trd`

### 7.2 Request

#### 7.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 7.3 Response

#### 7.3.1 OutBlock_1
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

### 7.4 Request Sample
```json
{"basDd": "20240105"}
```

### 7.5 Response Sample

```json
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "5774925915",
      "ACC_TRDVOL": "1768963",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "30",
      "FLUC_RT": "0.92",
      "ISU_CD": "060310",
      "ISU_NM": "3S",
      "LIST_SHRS": "48536642",
      "MKTCAP": "160170918600",
      "MKT_NM": "KOSDAQ",
      "SECT_TP_NM": "중견기업부",
      "TDD_CLSPRC": "3300",
      "TDD_HGPRC": "3325",
      "TDD_LWPRC": "3190",
      "TDD_OPNPRC": "3210"
    },
    {
      "ACC_TRDVAL": "817598270",
      "ACC_TRDVOL": "112058",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "360",
      "FLUC_RT": "5.19",
      "ISU_CD": "054620",
      "ISU_NM": "APS",
      "LIST_SHRS": "20394221",
      "MKTCAP": "148877813300",
      "MKT_NM": "KOSDAQ",
      "SECT_TP_NM": "중견기업부",
      "TDD_CLSPRC": "7300",
      "TDD_HGPRC": "7460",
      "TDD_LWPRC": "6900",
      "TDD_OPNPRC": "6900"
    },
    {
      "ACC_TRDVAL": "2386649500",
      "ACC_TRDVOL": "104119",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-250",
      "FLUC_RT": "-1.08",
      "ISU_CD": "265520",
      "ISU_NM": "AP시스템",
      "LIST_SHRS": "15281421",
      "MKTCAP": "350708611950",
      "MKT_NM": "KOSDAQ",
      "SECT_TP_NM": "우량기업부",
      "TDD_CLSPRC": "22950",
      "TDD_HGPRC": "23400",
      "TDD_LWPRC": "22700",
      "TDD_OPNPRC": "23250"
    }
  ]
}
```

