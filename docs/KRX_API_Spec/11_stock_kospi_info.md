## 11. 유가증권 종목기본정보

### 11.1 Description
유가증권 종목기본정보

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/sto/stk_isu_base_info`

### 11.2 Request

#### 11.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 11.3 Response

#### 11.3.1 OutBlock_1
| Name                | Type   | Description    |
|---------------------|--------|----------------|
| ISU_CD              | string | 표준코드       |
| ISU_SRT_CD          | string | 단축코드       |
| ISU_NM              | string | 한글 종목명    |
| ISU_ABBRV           | string | 한글 종목약명  |
| ISU_ENG_NM          | string | 영문 종목명    |
| LIST_DD             | string | 상장일         |
| MKT_TP_NM           | string | 시장구분       |
| SECUGRP_NM          | string | 증권구분       |
| SECT_TP_NM          | string | 소속부         |
| KIND_STKCERT_TP_NM  | string | 주식종류       |
| PARVAL              | string | 액면가         |
| LIST_SHRS           | string | 상장주식수     |

### 11.4 Request Sample
```json
{P240105"}
```

### 11.5 
{
  "OutBlock_1": [
    {
      "ISU_ABBRV": "AJ네트웍스",
      "ISU_CD": "KR7095570008",
      "ISU_ENG_NM": "AJ Networks Co.,Ltd.",
      "ISU_NM": "AJ네트웍스보통주",
      "ISU_SRT_CD": "095570",
      "KIND_STKCERT_TP_NM": "보통주",
      "LIST_DD": "20150821",
      "LIST_SHRS": "45252759",
      "MKT_TP_NM": "KOSPI",
      "PARVAL": "1000",
      "SECT_TP_NM": "-",
      "SECUGRP_NM": "주권"
    },
    {
      "ISU_ABBRV": "AK홀딩스",
      "ISU_CD": "KR7006840003",
      "ISU_ENG_NM": "AK Holdings, Inc.",
      "ISU_NM": "AK홀딩스보통주",
      "ISU_SRT_CD": "006840",
      "KIND_STKCERT_TP_NM": "보통주",
      "LIST_DD": "19990811",
      "LIST_SHRS": "13247561",
      "MKT_TP_NM": "KOSPI",
      "PARVAL": "5000",
      "SECT_TP_NM": "-",
      "SECUGRP_NM": "주권"
    },
    {
      "ISU_ABBRV": "BGF리테일",
      "ISU_CD": "KR7282330000",
      "ISU_ENG_NM": "BGF Retail",
      "ISU_NM": "BGF리테일보통주",
      "ISU_SRT_CD": "282330",
      "KIND_STKCERT_TP_NM": "보통주",
      "LIST_DD": "20171208",
      "LIST_SHRS": "17283906",
      "MKT_TP_NM": "KOSPI",
      "PARVAL": "1000",
      "SECT_TP_NM": "-",
      "SECUGRP_NM": "주권"
    }
  ]
}

