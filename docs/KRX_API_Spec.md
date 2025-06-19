# KRX OPEN API 명세 (전체)

## 목차

1. [지수 (Index)](#1-지수-index)
2. [주식 (Stock)](#2-주식-stock)
3. [ETP (Exchange Traded Products)](#3-etp-exchange-traded-products)
4. [채권 (Bond)](#4-채권-bond)
5. [파생상품 (Derivatives)](#5-파생상품-derivatives)
6. [일반상품 (General Commodities)](#6-일반상품-general-commodities)
7. [ESG](#7-esg)

---

## 1. 지수 (Index)

### 1.1. KRX 시리즈 일별시세정보

KRX 시리즈 지수의 시세정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/idx/krx_dd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| IDX_CLSS | string | 계열구분 |
| IDX_NM | string | 지수명 |
| CLSPRC_IDX | string | 종가 |
| CMPPREVDD_IDX | string | 대비 |
| FLUC_RT | string | 등락률 |
| OPNPRC_IDX | string | 시가 |
| HGPRC_IDX | string | 고가 |
| LWPRC_IDX | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| MKTCAP | string | 상장시가총액 |

### 1.2. KOSPI 시리즈 일별시세정보

KOSPI 시리즈 지수의 시세정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/idx/kospi_dd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| IDX_CLSS | string | 계열구분 |
| IDX_NM | string | 지수명 |
| CLSPRC_IDX | string | 종가 |
| CMPPREVDD_IDX | string | 대비 |
| FLUC_RT | string | 등락률 |
| OPNPRC_IDX | string | 시가 |
| HGPRC_IDX | string | 고가 |
| LWPRC_IDX | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| MKTCAP | string | 상장시가총액 |

### 1.3. KOSDAQ 시리즈 일별시세정보

KOSDAQ 시리즈 지수의 시세정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/idx/kosdaq_dd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| IDX_CLSS | string | 계열구분 |
| IDX_NM | string | 지수명 |
| CLSPRC_IDX | string | 종가 |
| CMPPREVDD_IDX | string | 대비 |
| FLUC_RT | string | 등락률 |
| OPNPRC_IDX | string | 시가 |
| HGPRC_IDX | string | 고가 |
| LWPRC_IDX | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| MKTCAP | string | 상장시가총액 |

### 1.4. 채권지수 시세정보

채권지수의 시세정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/idx/bon_dd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| BND_IDX_GRP_NM | string | 지수명 |
| TOT_EARNG_IDX | string | 총수익지수_종가 |
| TOT_EARNG_IDX_CMPPREVDD | string | 총수익지수_대비 |
| NETPRC_IDX | string | 순가격지수_종가 |
| NETPRC_IDX_CMPPREVDD | string | 순가격지수_대비 |
| ZERO_REINVST_IDX | string | 제로재투자지수_종가 |
| ZERO_REINVST_IDX_CMPPREVDD | string | 제로재투자지수_대비 |
| CALL_REINVST_IDX | string | 콜재투자지수_종가 |
| CALL_REINVST_IDX_CMPPREVDD | string | 콜재투자지수_대비 |
| MKT_PRC_IDX | string | 시장가격지수_종가 |
| MKT_PRC_IDX_CMPPREVDD | string | 시장가격지수_대비 |
| AVG_DURATION | string | 듀레이션 |
| AVG_CONVEXITY_PRC | string | 컨벡시티 |
| BND_IDX_AVG_YD | string | YTM |

### 1.5. 파생상품지수 시세정보

파생상품지수의 시세정보를 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/idx/drvprod_dd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| IDX_CLSS | string | 계열구분 |
| IDX_NM | string | 지수명 |
| CLSPRC_IDX | string | 종가 |
| CMPPREVDD_IDX | string | 대비 |
| FLUC_RT | string | 등락률 |
| OPNPRC_IDX | string | 시가 |
| HGPRC_IDX | string | 고가 |
| LWPRC_IDX | string | 저가 |

---

## 2. 주식 (Stock)

### 2.1. 유가증권 일별매매정보

유가증권시장에 상장되어 있는 주권의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/sto/stk_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| MKT_NM | string | 시장구분 |
| SECT_TP_NM | string | 소속부 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| FLUC_RT | string | 등락률 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| MKTCAP | string | 시가총액 |
| LIST_SHRS | string | 상장주식수 |

### 2.2. 코스닥 일별매매정보

코스닥시장에 상장되어 있는 주권의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/sto/ksq_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| MKT_NM | string | 시장구분 |
| SECT_TP_NM | string | 소속부 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| FLUC_RT | string | 등락률 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| MKTCAP | string | 시가총액 |
| LIST_SHRS | string | 상장주식수 |

### 2.3. 코넥스 일별매매정보

코넥스시장에 상장되어 있는 주권의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/sto/knx_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| MKT_NM | string | 시장구분 |
| SECT_TP_NM | string | 소속부 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| FLUC_RT | string | 등락률 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| MKTCAP | string | 시가총액 |
| LIST_SHRS | string | 상장주식수 |

### 2.4. 신주인수권증권 일별매매정보

유가증권/코스닥시장에 상장되어 있는 신주인수권증권의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/sto/sw_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| MKT_NM | string | 시장구분 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| FLUC_RT | string | 등락률 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| MKTCAP | string | 시가총액 |
| LIST_SHRS | string | 상장증권수 |
| EXER_PRC | string | 행사가격 |
| EXST_STRT_DD | string | 존속기간_시작일 |
| EXST_END_DD | string | 존속기간_종료일 |
| TARSTK_ISU_SRT_CD | string | 목적주권_종목코드 |
| TARSTK_ISU_NM | string | 목적주권_종목명 |
| TARSTK_ISU_PRSNT_PRC | string | 목적주권_종가 |

### 2.5. 신주인수권증서 일별매매정보

유가증권/코스닥시장에 상장되어 있는 신주인수권증서의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/sto/sr_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| MKT_NM | string | 시장구분 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| FLUC_RT | string | 등락률 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| MKTCAP | string | 시가총액 |
| LIST_SHRS | string | 상장증서수 |
| ISU_PRC | string | 신주발행가 |
| DELIST_DD | string | 상장폐지일 |
| TARSTK_ISU_SRT_CD | string | 목적주권_종목코드 |
| TARSTK_ISU_NM | string | 목적주권_종목명 |
| TARSTK_ISU_PRSNT_PRC | string | 목적주권_종가 |

### 2.6. 유가증권 종목기본정보

유가증권 종목기본정보

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/sto/stk_isu_base_info`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| ISU_CD | string | 표준코드 |
| ISU_SRT_CD | string | 단축코드 |
| ISU_NM | string | 한글 종목명 |
| ISU_ABBRV | string | 한글 종목약명 |
| ISU_ENG_NM | string | 영문 종목명 |
| LIST_DD | string | 상장일 |
| MKT_TP_NM | string | 시장구분 |
| SECUGRP_NM | string | 증권구분 |
| SECT_TP_NM | string | 소속부 |
| KIND_STKCERT_TP_NM | string | 주식종류 |
| PARVAL | string | 액면가 |
| LIST_SHRS | string | 상장주식수 |

### 2.7. 코스닥 종목기본정보

코스닥 종목기본정보

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/sto/ksq_isu_base_info`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| ISU_CD | string | 표준코드 |
| ISU_SRT_CD | string | 단축코드 |
| ISU_NM | string | 한글 종목명 |
| ISU_ABBRV | string | 한글 종목약명 |
| ISU_ENG_NM | string | 영문 종목명 |
| LIST_DD | string | 상장일 |
| MKT_TP_NM | string | 시장구분 |
| SECUGRP_NM | string | 증권구분 |
| SECT_TP_NM | string | 소속부 |
| KIND_STKCERT_TP_NM | string | 주식종류 |
| PARVAL | string | 액면가 |
| LIST_SHRS | string | 상장주식수 |

### 2.8. 코넥스 종목기본정보

코넥스 종목기본정보

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/sto/knx_isu_base_info`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| ISU_CD | string | 표준코드 |
| ISU_SRT_CD | string | 단축코드 |
| ISU_NM | string | 한글 종목명 |
| ISU_ABBRV | string | 한글 종목약명 |
| ISU_ENG_NM | string | 영문 종목명 |
| LIST_DD | string | 상장일 |
| MKT_TP_NM | string | 시장구분 |
| SECUGRP_NM | string | 증권구분 |
| SECT_TP_NM | string | 소속부 |
| KIND_STKCERT_TP_NM | string | 주식종류 |
| PARVAL | string | 액면가 |
| LIST_SHRS | string | 상장주식수 |

---

## 3. ETP (Exchange Traded Products)

### 3.1. ETF 일별매매정보

ETF(상장지수펀드)의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/etp/etf_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| FLUC_RT | string | 등락률 |
| NAV | string | 순자산가치(NAV) |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| MKTCAP | string | 시가총액 |
| INVSTASST_NETASST_TOTAMT | string | 순자산총액 |
| LIST_SHRS | string | 상장좌수 |
| IDX_IND_NM | string | 기초지수_지수명 |
| OBJ_STKPRC_IDX | string | 기초지수_종가 |
| CMPPREVDD_IDX | string | 기초지수_대비 |
| FLUC_RT_IDX | string | 기초지수_등락률 |

### 3.2. ETN 일별매매정보

ETN(상장지수증권)의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/etp/etn_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| FLUC_RT | string | 등락률 |
| PER1SECU_INDIC_VAL | string | 지표가치(IV) |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| MKTCAP | string | 시가총액 |
| INDIC_VAL_AMT | string | 지표가치총액 |
| LIST_SHRS | string | 상장증권수 |
| IDX_IND_NM | string | 기초지수_지수명 |
| OBJ_STKPRC_IDX | string | 기초지수_종가 |
| CMPPREVDD_IDX | string | 기초지수_대비 |
| FLUC_RT_IDX | string | 기초지수_등락률 |

### 3.3. ELW 일별매매정보

ELW(주식워런트증권)의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/etp/elw_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| MKTCAP | string | 시가총액 |
| LIST_SHRS | string | 상장증권수 |
| ULY_NM | string | 기초자산_자산명 |
| ULY_PRC | string | 기초자산_종가 |
| CMPPREVDD_PRC_ULY | string | 기초자산_대비 |
| FLUC_RT_ULY | string | 기초자산_등락률 |

---

## 4. 채권 (Bond)

### 4.1. 국채전문유통시장 일별매매정보

국채전문유통시장에 상장되어있는 채권의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/bon/kts_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| MKT_NM | string | 시장구분 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| BND_EXP_TP_NM | string | 만기년수 |
| GOVBND_ISU_TP_NM | string | 종목구분 |
| CLSPRC | string | 종가_가격 |
| CMPPREVDD_PRC | string | 종가_대비 |
| CLSPRC_YD | string | 종가_수익률 |
| OPNPRC | string | 시가_가격 |
| OPNPRC_YD | string | 시가_수익률 |
| HGPRC | string | 고가_가격 |
| HGPRC_YD | string | 고가_수익률 |
| LWPRC | string | 저가_가격 |
| LWPRC_YD | string | 저가_수익률 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |

### 4.2. 일반채권시장 일별매매정보

일반채권시장에 상장되어있는 채권의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/bon/bnd_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| MKT_NM | string | 시장구분 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| CLSPRC | string | 종가_가격 |
| CMPPREVDD_PRC | string | 종가_대비 |
| CLSPRC_YD | string | 종가_수익률 |
| OPNPRC | string | 시가_가격 |
| OPNPRC_YD | string | 시가_수익률 |
| HGPRC | string | 고가_가격 |
| HGPRC_YD | string | 고가_수익률 |
| LWPRC | string | 저가_가격 |
| LWPRC_YD | string | 저가_수익률 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |

### 4.3. 소액채권시장 일별매매정보

소액채권시장에 상장되어있는 채권의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/bon/smb_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| MKT_NM | string | 시장구분 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| CLSPRC | string | 종가_가격 |
| CMPPREVDD_PRC | string | 종가_대비 |
| CLSPRC_YD | string | 종가_수익률 |
| OPNPRC | string | 시가_가격 |
| OPNPRC_YD | string | 시가_수익률 |
| HGPRC | string | 고가_가격 |
| HGPRC_YD | string | 고가_수익률 |
| LWPRC | string | 저가_가격 |
| LWPRC_YD | string | 저가_수익률 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |

---

## 5. 파생상품 (Derivatives)

### 5.1. 선물 일별매매정보 (주식선물 제외)

파생상품시장의 선물 중 주식선물을 제외한 선물의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/drv/fut_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| PROD_NM | string | 상품구분 |
| MKT_NM | string | 시장구분(정규/야간) |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| SPOT_PRC | string | 현물가 |
| SETL_PRC | string | 정산가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| ACC_OPNINT_QTY | string | 미결제약정 |

### 5.2. 주식선물(유가) 일별매매정보

파생상품시장의 주식선물 중 기초자산이 유가증권시장에 속하는 주식선물의 거래정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/drv/eqsfu_stk_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| PROD_NM | string | 상품구분 |
| MKT_NM | string | 시장구분(정규/야간) |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| SPOT_PRC | string | 현물가 |
| SETL_PRC | string | 정산가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| ACC_OPNINT_QTY | string | 미결제약정 |

### 5.3. 주식선물(코스닥) 일별매매정보

파생상품시장의 주식선물 중 기초자산이 코스닥시장에 속하는 주식선물의 거래정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/drv/eqkfu_ksq_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| PROD_NM | string | 상품구분 |
| MKT_NM | string | 시장구분(정규/야간) |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| SPOT_PRC | string | 현물가 |
| SETL_PRC | string | 정산가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| ACC_OPNINT_QTY | string | 미결제약정 |

### 5.4. 옵션 일별매매정보 (주식옵션 제외)

파생상품시장의 옵션 중 주식옵션을 제외한 옵션의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/drv/opt_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| PROD_NM | string | 상품구분 |
| RGHT_TP_NM | string | 권리유형(CALL/PUT) |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| IMP_VOLT | string | 내재변동성 |
| NXTDD_BAS_PRC | string | 익일정산가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| ACC_OPNINT_QTY | string | 미결제약정 |

### 5.5. 주식옵션(유가) 일별매매정보

파생상품시장의 주식옵션 중 기초자산이 유가증권시장에 속하는 주식옵션의 거래정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/drv/eqsop_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| PROD_NM | string | 상품구분 |
| RGHT_TP_NM | string | 권리유형(CALL/PUT) |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| IMP_VOLT | string | 내재변동성 |
| NXTDD_BAS_PRC | string | 익일정산가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| ACC_OPNINT_QTY | string | 미결제약정 |

### 5.6. 주식옵션(코스닥) 일별매매정보

파생상품시장의 주식옵션 중 기초자산이 코스닥시장에 속하는 주식옵션의 거래정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/drv/eqkop_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| PROD_NM | string | 상품구분 |
| RGHT_TP_NM | string | 권리유형(CALL/PUT) |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| IMP_VOLT | string | 내재변동성 |
| NXTDD_BAS_PRC | string | 익일정산가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |
| ACC_OPNINT_QTY | string | 미결제약정 |

---

## 6. 일반상품 (General Commodities)

### 6.1. 석유시장 일별매매정보

KRX 석유시장의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/gen/oil_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| OIL_NM | string | 유종구분 |
| WT_AVG_PRC | string | 가중평균가격_경쟁 |
| WT_DIS_AVG_PRC | string | 가중평균가격_협의 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |

### 6.2. 금시장 일별매매정보

KRX 금시장 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/gen/gold_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| FLUC_RT | string | 등락률 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |

### 6.3. 배출권 시장 일별매매정보

KRX 탄소배출권 시장의 매매정보 제공

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/gen/ets_bydd_trd`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| ISU_CD | string | 종목코드 |
| ISU_NM | string | 종목명 |
| TDD_CLSPRC | string | 종가 |
| CMPPREVDD_PRC | string | 대비 |
| FLUC_RT | string | 등락률 |
| TDD_OPNPRC | string | 시가 |
| TDD_HGPRC | string | 고가 |
| TDD_LWPRC | string | 저가 |
| ACC_TRDVOL | string | 거래량 |
| ACC_TRDVAL | string | 거래대금 |

---

## 7. ESG

### 7.1. 사회책임투자채권 정보

사회책임투자채권 정보를 제공 ('19년 데이터부터 제공)

**Endpoint URL:** `http://data-dbg.krx.co.kr/svc/apis/esg/sri_bond_info`

**Request Parameters:**

| Name | Type | Description |
| :--- | :--- | :--- |
| basDd | string | 기준일자 (YYYYMMDD) |

**Response Fields:**

| Name | Type | Description |
| :--- | :--- | :--- |
| BAS_DD | string | 기준일자 |
| ISUR_NM | string | 발행기관 |
| ISU_CD | string | 표준코드 |
| SRI_BND_TP_NM | string | 채권종류 |
| ISU_NM | string | 종목명 |
| LIST_DD | string | 상장일 |
| ISU_DD | string | 발행일 |
| REDMPT_DD | string | 상환일 |
| ISU_RT | string | 표면이자율 |
| ISU_AMT | string | 발행금액 |
| LIST_AMT | string | 상장금액 |
| BND_TP_NM | string | 채권유형 |

---

## 공통 사항

### 요청 방법
- HTTP Method: GET
- Content-Type: application/json

### 응답 형식
- Content-Type: application/json
- 모든 응답 필드는 string 타입으로 반환됩니다

### 날짜 형식
- 기준일자(basDd): YYYYMMDD 형식 (예: 20241219)
- 기타 날짜 필드: YYYY/MM/DD 형식으로 반환

### 오류 처리
- 잘못된 요청 시 적절한 HTTP 상태 코드와 함께 오류 메시지 반환
- 데이터가 없는 경우 빈 배열 또는 null 값 반환

### 참고사항
- 모든 금액 단위는 원(KRW)입니다
- 거래량 단위는 주식/증권 수입니다
- 등락률은 백분율(%)로 표시됩니다