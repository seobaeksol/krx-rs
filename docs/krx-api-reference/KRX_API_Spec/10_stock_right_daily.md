## 10. 신주인수권증서 일별매매정보

### 10.1 Description
유가증권/코스닥시장에 상장되어 있는 신주인수권증서의 매매정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/sto/sr_bydd_trd`

### 10.2 Request

#### 10.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 10.3 Response

#### 10.3.1 OutBlock_1
| Name                  | Type   | Description            |
|-----------------------|--------|------------------------|
| BAS_DD                | string | 기준일자               |
| MKT_NM                | string | 시장구분               |
| ISU_CD                | string | 종목코드               |
| ISU_NM                | string | 종목명                 |
| TDD_CLSPRC            | string | 종가                   |
| CMPPREVDD_PRC         | string | 대비                   |
| FLUC_RT               | string | 등락률                 |
| TDD_OPNPRC            | string | 시가                   |
| TDD_HGPRC             | string | 고가                   |
| TDD_LWPRC             | string | 저가                   |
| ACC_TRDVOL            | string | 거래량                 |
| ACC_TRDVAL            | string | 거래대금               |
| MKTCAP                | string | 시가총액               |
| LIST_SHRS             | string | 상장증서수             |
| ISU_PRC               | string | 신주발행가             |
| DELIST_DD             | string | 상장폐지일             |
| TARSTK_ISU_SRT_CD     | string | 목적주권_종목코드      |
| TARSTK_ISU_NM         | string | 목적주권_종목명        |
| TARSTK_ISU_PRSNT_PRC  | string | 목적주권_종가          |

### 10.4 Request Sample
```json
{"basDd": "20240105"}
```

### 10.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "2339910682",
      "ACC_TRDVOL": "1543502",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "1",
      "DELIST_DD": "20240112",
      "FLUC_RT": "0.00",
      "ISU_CD": "1035901D",
      "ISU_NM": "일진전기 2R",
      "ISU_PRC": "9380",
      "LIST_SHRS": "8474850",
      "MKTCAP": "12246158250",
      "MKT_NM": "KOSPI",
      "TARSTK_ISU_NM": "일진전기",
      "TARSTK_ISU_PRSNT_PRC": "10900",
      "TARSTK_ISU_SRT_CD": "103590",
      "TDD_CLSPRC": "1445",
      "TDD_HGPRC": "1800",
      "TDD_LWPRC": "1201",
      "TDD_OPNPRC": "1800"
    }
  ]
}
```
