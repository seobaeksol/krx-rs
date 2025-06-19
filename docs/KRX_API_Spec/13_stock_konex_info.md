## 13. 코넥스 종목기본정보

### 13.1 Description
코넥스 종목기본정보

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/sto/knx_isu_base_info`

### 13.2 Request

#### 13.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 13.3 Response

#### 13.3.1 OutBlock_1
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

### 13.4 Request Sample
```json
{"basDd": "20240105"}
```

### 13.5 Response Sample

```json
{
  "OutBlock_1": [
    {
      "ISU_ABBRV": "SK시그넷",
      "ISU_CD": "KR7260870001",
      "ISU_ENG_NM": "SK Signet",
      "ISU_NM": "SK시그넷",
      "ISU_SRT_CD": "260870",
      "KIND_STKCERT_TP_NM": "보통주",
      "LIST_DD": "20170830",
      "LIST_SHRS": "6112710",
      "MKT_TP_NM": "KONEX",
      "PARVAL": "500",
      "SECT_TP_NM": "일반기업부",
      "SECUGRP_NM": "주권"
    },
    {
      "ISU_ABBRV": "가이아코퍼레이션",
      "ISU_CD": "KR7296520000",
      "ISU_ENG_NM": "GAIA CORPORATION",
      "ISU_NM": "가이아코퍼레이션",
      "ISU_SRT_CD": "296520",
      "KIND_STKCERT_TP_NM": "보통주",
      "LIST_DD": "20230620",
      "LIST_SHRS": "4667008",
      "MKT_TP_NM": "KONEX",
      "PARVAL": "500",
      "SECT_TP_NM": "일반기업부",
      "SECUGRP_NM": "주권"
    },
    {
      "ISU_ABBRV": "골프존데카",
      "ISU_CD": "KR7183410000",
      "ISU_ENG_NM": "GolfzonDeca",
      "ISU_NM": "골프존데카",
      "ISU_SRT_CD": "183410",
      "KIND_STKCERT_TP_NM": "보통주",
      "LIST_DD": "20131112",
      "LIST_SHRS": "4111161",
      "MKT_TP_NM": "KONEX",
      "PARVAL": "500",
      "SECT_TP_NM": "일반기업부",
      "SECUGRP_NM": "주권"
    }
  ]
}
```

