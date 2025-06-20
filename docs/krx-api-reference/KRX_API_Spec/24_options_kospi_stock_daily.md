## 24. 주식옵션(유가) 일별매매정보

### 24.1 Description
파생상품시장의 주식옵션 중 기초자산이 유가증권시장에 속하는 주식옵션의 거래정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/drv/eqsop_bydd_trd`

### 24.2 Request

#### 24.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 24.3 Response

#### 24.3.1 OutBlock_1
| Name            | Type   | Description      |
|-----------------|--------|------------------|
| BAS_DD          | string | 기준일자         |
| PROD_NM         | string | 상품구분         |
| RGHT_TP_NM      | string | 권리유형(CALL/PUT) |
| ISU_CD          | string | 종목코드         |
| ISU_NM          | string | 종목명           |
| TDD_CLSPRC      | string | 종가             |
| CMPPREVDD_PRC   | string | 대비             |
| TDD_OPNPRC      | string | 시가             |
| TDD_HGPRC       | string | 고가             |
| TDD_LWPRC       | string | 저가             |
| IMP_VOLT        | string | 내재변동성       |
| NXTDD_BAS_PRC   | string | 익일정산가       |
| ACC_TRDVOL      | string | 거래량           |
| ACC_TRDVAL      | string | 거래대금         |
| ACC_OPNINT_QTY  | string | 미결제약정       |

### 24.4 Request Sample
```json
{"basDd": "20240105"}
```

### 24.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "BAS_DD": "__",
      "PROD_NM": "__",
      "RGHT_TP_NM": "__",
      "ISU_CD": "__",
      "ISU_NM": "__",
      "TDD_CLSPRC": "-",
      "CMPPREVDD
