## 9. 신주인수권증권 일별매매정보

### 9.1 Description
유가증권/코스닥시장에 상장되어 있는 신주인수권증권의 매매정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/sto/sw_bydd_trd`

### 9.2 Request

#### 9.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 9.3 Response

#### 9.3.1 OutBlock_1
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
| LIST_SHRS             | string | 상장증권수             |
| EXER_PRC              | string | 행사가격               |
| EXST_STRT_DD          | string | 존속기간_시작일        |
| EXST_END_DD           | string | 존속기간_종료일        |
| TARSTK_ISU_SRT_CD     | string | 목적주권_종목코드      |
| TARSTK_ISU_NM         | string | 목적주권_종목명        |
| TARSTK_ISU_PRSNT_PRC  | string | 목적주권_종가          |

### 9.4 Request Sample
```json
{"basDd": "20240105"}
```

### 9.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "59712974",
      "ACC_TRDVOL": "45938",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "1",
      "EXER_PRC": "8411",
      "EXST_END_DD": "20281105",
      "EXST_STRT_DD": "20240105",
      "FLUC_RT": "0.08",
      "ISU_CD": "0036221D",
      "ISU_NM": "KG모빌리티 122WR",
      "LIST_SHRS": "17893060",
      "MKTCAP": "23243084940",
      "MKT_NM": "KOSPI",
      "TARSTK_ISU_NM": "KG모빌리티",
      "TARSTK_ISU_PRSNT_PRC": "8150",
      "TARSTK_ISU_SRT_CD": "003620",
      "TDD_CLSPRC": "1299",
      "TDD_HGPRC": "1330",
      "TDD_LWPRC": "1271",
      "TDD_OPNPRC": "1330"
    },
    {
      "ACC_TRDVAL": "21819628",
      "ACC_TRDVOL": "50893",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-3",
      "EXER_PRC": "1125",
      "EXST_END_DD": "20260306",
      "EXST_STRT_DD": "20230506",
      "FLUC_RT": "-0.70",
      "ISU_CD": "2552221D",
      "ISU_NM": "SG 17WR",
      "LIST_SHRS": "11132302",
      "MKTCAP": "4708963746",
      "MKT_NM": "KOSDAQ",
      "TARSTK_ISU_NM": "SG",
      "TARSTK_ISU_PRSNT_PRC": "1683",
      "TARSTK_ISU_SRT_CD": "255220",
      "TDD_CLSPRC": "423",
      "TDD_HGPRC": "434",
      "TDD_LWPRC": "388",
      "TDD_OPNPRC": "429"
    },
    {
      "ACC_TRDVAL": "607035",
      "ACC_TRDVOL": "2447",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "-2",
      "EXER_PRC": "865",
      "EXST_END_DD": "20241122",
      "EXST_STRT_DD": "20200128",
      "FLUC_RT": "-0.79",
      "ISU_CD": "00532219",
      "ISU_NM": "국동 9WR",
      "LIST_SHRS": "4805514",
      "MKTCAP": "1201378500",
      "MKT_NM": "KOSPI",
      "TARSTK_ISU_NM": "국동",
      "TARSTK_ISU_PRSNT_PRC": "752",
      "TARSTK_ISU_SRT_CD": "005320",
      "TDD_CLSPRC": "250",
      "TDD_HGPRC": "268",
      "TDD_LWPRC": "232",
      "TDD_OPNPRC": "253"
    }
  ]
}
```
