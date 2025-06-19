## 12. 코스닥 종목기본정보

### 12.1 Description
코스닥 종목기본정보

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/sto/ksq_isu_base_info`

### 12.2 Request

#### 12.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 12.3 Response

#### 12.3.1 OutBlock_1
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

### 12.4 Request Sample
```json
{"basDd": "20240105"}
```

### 12.5 Response Sample

```json
{
  "OutBlock_1": [
    {
      "ISU_ABBRV": "마이크로컨텍솔",
      "ISU_CD": "KR7098120009",
      "ISU_ENG_NM": "Micro Contact Solution Co.,Ltd.",
      "ISU_NM": "(주)마이크로컨텍솔루션",
      "ISU_SRT_CD": "098120",
      "KIND_STKCERT_TP_NM": "보통주",
      "LIST_DD": "20080923",
      "LIST_SHRS": "8312766",
      "MKT_TP_NM": "KOSDAQ",
      "PARVAL": "500",
      "SECT_TP_NM": "중견기업부",
      "SECUGRP_NM": "주권"
    },
    {
      "ISU_ABBRV": "포스코엠텍",
      "ISU_CD": "KR7009520008",
      "ISU_ENG_NM": "POSCO M-TECH CO.,LTD.",
      "ISU_NM": "(주)포스코엠텍",
      "ISU_SRT_CD": "009520",
      "KIND_STKCERT_TP_NM": "보통주",
      "LIST_DD": "19971110",
      "LIST_SHRS": "41642703",
      "MKT_TP_NM": "KOSDAQ",
      "PARVAL": "500",
      "SECT_TP_NM": "우량기업부",
      "SECUGRP_NM": "주권"
    },
    {
      "ISU_ABBRV": "CMG제약",
      "ISU_CD": "KR7058820002",
      "ISU_ENG_NM": "CMG Pharmaceutical Co., Ltd.",
      "ISU_NM": "CMG제약",
      "ISU_SRT_CD": "058820",
      "KIND_STKCERT_TP_NM": "보통주",
      "LIST_DD": "20010831",
      "LIST_SHRS": "138892244",
      "MKT_TP_NM": "KOSDAQ",
      "PARVAL": "500",
      "SECT_TP_NM": "중견기업부",
      "SECUGRP_NM": "주권"
    }
  ]
}
```

