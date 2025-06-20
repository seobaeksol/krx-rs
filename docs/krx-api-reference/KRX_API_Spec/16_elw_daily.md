## 16. ELW 일별매매정보

### 16.1 Description
ELW(주식위런트증권)의 매매정보 제공

**Server endpoint URL**: `http://data-dbg.krx.co.kr/svc/apis/etp/elw_bydd_trd`

### 16.2 Request

#### 16.2.1 InBlock_1
| Name   | Type   | Description |
|--------|--------|-------------|
| basDd  | string | 기준일자    |

### 16.3 Response

#### 16.3.1 OutBlock_1
| Name                | Type   | Description         |
|---------------------|--------|---------------------|
| BAS_DD              | string | 기준일자            |
| ISU_CD              | string | 종목코드            |
| ISU_NM              | string | 종목명              |
| TDD_CLSPRC          | string | 종가                |
| CMPPREVDD_PRC       | string | 대비                |
| TDD_OPNPRC          | string | 시가                |
| TDD_HGPRC           | string | 고가                |
| TDD_LWPRC           | string | 저가                |
| ACC_TRDVOL          | string | 거래량              |
| ACC_TRDVAL          | string | 거래대금            |
| MKTCAP              | string | 시가총액            |
| LIST_SHRS           | string | 상장증권수          |
| ULY_NM              | string | 기초자산_자산명     |
| ULY_PRC             | string | 기초자산_종가       |
| CMPPREVDD_PRC_ULY   | string | 기초자산_대비       |
| FLUC_RT_ULY         | string | 기초자산_등락률     |

### 16.4 Request Sample
```json
{"basDd": "20240105"}
```

### 16.5 Response Sample
```json
{
  "OutBlock_1": [
    {
      "ACC_TRDVAL": "0",
      "ACC_TRDVOL": "0",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "0",
      "CMPPREVDD_PRC_ULY": "0",
      "FLUC_RT_ULY": "0.00",
      "ISU_CD": "58J003",
      "ISU_NM": "KBJ003삼성전자풋",
      "LIST_SHRS": "15800000",
      "MKTCAP": "158000000",
      "TDD_CLSPRC": "10",
      "TDD_HGPRC": "0",
      "TDD_LWPRC": "0",
      "TDD_OPNPRC": "0",
      "ULY_NM": "삼성전자",
      "ULY_PRC": "76,600"
    },
    {
      "ACC_TRDVAL": "0",
      "ACC_TRDVOL": "0",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "0",
      "CMPPREVDD_PRC_ULY": "0",
      "FLUC_RT_ULY": "0.00",
      "ISU_CD": "58J126",
      "ISU_NM": "KBJ126삼성전자콜",
      "LIST_SHRS": "15000000",
      "MKTCAP": "1875000000",
      "TDD_CLSPRC": "125",
      "TDD_HGPRC": "0",
      "TDD_LWPRC": "0",
      "TDD_OPNPRC": "0",
      "ULY_NM": "삼성전자",
      "ULY_PRC": "76,600"
    },
    {
      "ACC_TRDVAL": "0",
      "ACC_TRDVOL": "0",
      "BAS_DD": "20240105",
      "CMPPREVDD_PRC": "0",
      "CMPPREVDD_PRC_ULY": "0",
      "FLUC_RT_ULY": "0.00",
      "ISU_CD": "58J127",
      "ISU_NM": "KBJ127삼성전자콜",
      "LIST_SHRS": "13200000",
      "MKTCAP": "1782000000",
      "TDD_CLSPRC": "135",
      "TDD_HGPRC": "0",
      "TDD_LWPRC": "0",
      "TDD_OPNPRC": "0",
      "ULY_NM": "삼성전자",
      "ULY_PRC": "76,600"
    }
  ]
}
```
