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
      "BAS_DD": "20240105",
      "BND_TP_NM": "회사채",
      "ISUR_NM": "신한은행",
      "ISU_AMT": "400000000000",
      "ISU_CD": "KR6000011B58",
      "ISU_DD": "20210506",
      "ISU_NM": "신한은행25-05-이10갑후(녹)",
      "ISU_RT": "2.58000",
      "LIST_AMT": "400000000000",
      "LIST_DD": "20210507",
      "REDMPT_DD": "20310506",
      "SRI_BND_TP_NM": "녹색채권"
    },
    {
      "BAS_DD": "20240105",
      "BND_TP_NM": "회사채",
      "ISUR_NM": "신한은행",
      "ISU_AMT": "260000000000",
      "ISU_CD": "KR6000012BB1",
      "ISU_DD": "20211109",
      "ISU_NM": "신한은행25-11-이10갑후(녹)",
      "ISU_RT": "2.84000",
      "LIST_AMT": "260000000000",
      "LIST_DD": "20211110",
      "REDMPT_DD": "20311109",
      "SRI_BND_TP_NM": "녹색채권"
    },
    {
      "BAS_DD": "20240105",
      "BND_TP_NM": "회사채",
      "ISUR_NM": "신한은행",
      "ISU_AMT": "100000000000",
      "ISU_CD": "KR6000017D84",
      "ISU_DD": "20230823",
      "ISU_NM": "신한은행27-08-이-3-A(녹)",
      "ISU_RT": "4.22000",
      "LIST_AMT": "100000000000",
      "LIST_DD": "20230824",
      "REDMPT_DD": "20260823",
      "SRI_BND_TP_NM": "녹색채권"
    }
  ]
}
```