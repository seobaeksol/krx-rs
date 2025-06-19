## 6. 유가증권 일별매매정보

### 6.1 Description
유가증권시장에 상장되어 있는 주권의 매매정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/sto/stk_bydd_trd`

### 6.2 Request

#### 6.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 6.3 Response

#### 6.3.1 OutBlock_1
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

### 6.4 Request Sample
```json
{P240105"}
```

### 6.5 
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "2137622110",
      "ACC_TRDVOL": "410628",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "60",
      "FLUC_RT": "1.15",
      "ISU_CD": "095570",
      "ISU_NM": "AJ네트웍스",
      "LIST_SHRS": "45252759",
      "MKTCAP": "238934567520",
      "MKT_NM": "KOSPI",
      "SECT_TP_NM": "-",
      "TDD_CLSPRC": "5280",
      "TDD_HGPRC": "5290",
      "TDD_LWPRC": "5070",
      "TDD_OPNPRC": "5270"
    },
    {
      "ACC_TRDVAL": "51105470",
      "ACC_TRDVOL": "3005",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-10",
      "FLUC_RT": "-0.06",
      "ISU_CD": "006840",
      "ISU_NM": "AK홀딩스",
      "LIST_SHRS": "13247561",
      "MKTCAP": "224678634560",
      "MKT_NM": "KOSPI",
      "SECT_TP_NM": "-",
      "TDD_CLSPRC": "16960",
      "TDD_HGPRC": "17090",
      "TDD_LWPRC": "16950",
      "TDD_OPNPRC": "16950"
    },
    {
      "ACC_TRDVAL": "524078440",
      "ACC_TRDVOL": "143026",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-80",
      "FLUC_RT": "-2.14",
      "ISU_CD": "027410",
      "ISU_NM": "BGF",
      "LIST_SHRS": "95716791",
      "MKTCAP": "349366287150",
      "MKT_NM": "KOSPI",
      "SECT_TP_NM": "-",
      "TDD_CLSPRC": "3650",
      "TDD_HGPRC": "3730",
      "TDD_LWPRC": "3625",
      "TDD_OPNPRC": "3730"
    }
  ]
}

