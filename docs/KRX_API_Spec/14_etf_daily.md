## 14. ETF 일별매매정보

### 14.1 Description
ETF(상장지수펀드)의 매매정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/etp/etf_bydd_trd`

### 14.2 Request

#### 14.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 14.3 Response

#### 14.3.1 OutBlock_1
| Name                      | Type   | Description            |
|---------------------------|--------|------------------------|
| BAS_DD                    | string | 기준일자               |
| ISU_CD                    | string | 종목코드               |
| ISU_NM                    | string | 종목명                 |
| TDD_CLSPRC                | string | 종가                   |
| CMPPREVDD_PRC             | string | 대비                   |
| FLUC_RT                   | string | 등락률                 |
| NAV                       | string | 순자산가치(NAV)        |
| TDD_OPNPRC                | string | 시가                   |
| TDD_HGPRC                 | string | 고가                   |
| TDD_LWPRC                 | string | 저가                   |
| ACC_TRDVOL                | string | 거래량                 |
| ACC_TRDVAL                | string | 거래대금               |
| MKTCAP                    | string | 시가총액               |
| INVSTASST_NETASST_TOTAMT  | string | 순자산총액             |
| LIST_SHRS                 | string | 상장좌수               |
| IDX_IND_NM                | string | 기초지수_지수명        |
| OBJ_STKPRC_IDX            | string | 기초지수_종가          |
| CMPPREVDD_IDX             | string | 기초지수_대비          |
| FLUC_RT_IDX               | string | 기초지수_등락률        |

### 14.4 Request Sample
```json
{"basDd": "20240105"}
```

### 14.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "286826330",
      "ACC_TRDVOL": "28458",
      "BAS_DD": "20240105",
      "CMPPREVDD_IDX": "0.05",
      "CMPPREVDD_PRC": "5",
      "FLUC_RT": "0.05",
      "FLUC_RT_IDX": "0.04",
      "IDX_IND_NM": "KIS 11월 만기자동연장회사채(AA-이상) 총수익지수",
      "INVSTASST_NETASST_TOTAMT": "131011910572",
      "ISU_CD": "473440",
      "ISU_NM": "ACE 11월만기자동연장회사채AA-이상액티브",
      "LIST_SHRS": "13000000",
      "MKTCAP": "131040000000",
      "NAV": "10077.84",
      "OBJ_STKPRC_IDX": "113.63",
      "TDD_CLSPRC": "10080",
      "TDD_HGPRC": "10080",
      "TDD_LWPRC": "10075",
      "TDD_OPNPRC": "10080"
    },
    {
      "ACC_TRDVAL": "263621860",
      "ACC_TRDVOL": "7452",
      "BAS_DD": "20240105",
      "CMPPREVDD_IDX": "-0.85",
      "CMPPREVDD_PRC": "-125",
      "FLUC_RT": "-0.35",
      "FLUC_RT_IDX": "-0.24",
      "IDX_IND_NM": "코스피 200",
      "INVSTASST_NETASST_TOTAMT": "425594163746",
      "ISU_CD": "105190",
      "ISU_NM": "ACE 200",
      "LIST_SHRS": "12050000",
      "MKTCAP": "426148250000",
      "NAV": "35466.18",
      "OBJ_STKPRC_IDX": "347.22",
      "TDD_CLSPRC": "35365",
      "TDD_HGPRC": "35560",
      "TDD_LWPRC": "35295",
      "TDD_OPNPRC": "35510"
    },
    {
      "ACC_TRDVAL": "42653585",
      "ACC_TRDVOL": "1902",
      "BAS_DD": "20240105",
      "CMPPREVDD_IDX": "-1.08",
      "CMPPREVDD_PRC": "-95",
      "FLUC_RT": "-0.42",
      "FLUC_RT_IDX": "-0.25",
      "IDX_IND_NM": "코스피 200 TR",
      "INVSTASST_NETASST_TOTAMT": "35921981619",
      "ISU_CD": "332500",
      "ISU_NM": "ACE 200TR",
      "LIST_SHRS": "1600000",
      "MKTCAP": "35816000000",
      "NAV": "22451.24",
      "OBJ_STKPRC_IDX": "439.16",
      "TDD_CLSPRC": "22385",
      "TDD_HGPRC": "22515",
      "TDD_LWPRC": "22365",
      "TDD_OPNPRC": "22480"
    }
  ]
}
```
