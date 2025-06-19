# 사회책임투자채권 정보

## 29. 사회책임투자채권 정보

### 29.1 Overview
사회책임투자채권 정보를 제공 ('19년 데이터부터 제공)

### 29.2 How to Request
**Endpoint URL:** http://data-dbg.krx.co.kr/svc/apis/esg/sri_bond_info

```bash
curl -X GET "http://data-dbg.krx.co.kr/svc/apis/esg/sri_bond_info?basDd=20240105" \
     -H "Content-Type: application/json"
```

### 29.3 Request & Response

| Name            | Type   | Description      |
|-----------------|--------|------------------|
| basDd           | string | 기준일자         |

| Name            | Type   | Description      |
|-----------------|--------|------------------|
| BAS_DD          | string | 기준일자         |
| ISUR_NM         | string | 발행기관         |
| ISU_CD          | string | 표준코드         |
| SRI_BND_TP_NM   | string | 채권종류         |
| ISU_NM          | string | 종목명           |
| LIST_DD         | string | 상장일           |
| ISU_DD          | string | 발행일           |
| REDMPT_DD       | string | 상환일           |
| ISU_RT          | string | 표면이자율       |
| ISU_AMT         | string | 발행금액         |
| LIST_AMT        | string | 상장금액         |
| BND_TP_NM       | string | 채권유형         |

### 29.4 Request Sample
```json
{"basDd": "20240105"}
```

### 29.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "BAS_DD": "2024/01/05",
      "ISUR_NM": "한국전력공사",
      "ISU_CD": "KR6015722AC7",
      "SRI_BND_TP_NM": "녹색채권",
      "ISU_NM": "한국전력1094",
      "LIST_DD": "2022/12/15",
      "ISU_DD": "2022/12/14",
      "REDMPT_DD": "2027/12/14",
      "ISU_RT": "4.25",
      "ISU_AMT": "300000000000",
      "LIST_AMT": "300000000000",
      "BND_TP_NM": "회사채"
    },
    {
      "BAS_DD": "2024/01/05",
      "ISUR_NM": "한국수자원공사",
      "ISU_CD": "KR6380702BA8",
      "SRI_BND_TP_NM": "사회적채권",
      "ISU_NM": "한국수자원52",
      "LIST_DD": "2021/11/09",
      "ISU_DD": "2021/11/08",
      "REDMPT_DD": "2026/11/08",
      "ISU_RT": "2.45",
      "ISU_AMT": "200000000000",
      "LIST_AMT": "200000000000",
      "BND_TP_NM": "특수채"
    }
  ]
}
```