# KRX API 문서 인덱스

KRX Open API의 전체 문서 목록입니다.

## 📋 전체 API 목록

[전체 API 명세 보기](./KRX_API_Spec.md)

## 1. 지수 (Index) APIs

- [01. KRX 시리즈 일별시세정보](./KRX_API_Spec/01_krx_index_daily.md)
- [02. KOSPI 시리즈 일별시세정보](./KRX_API_Spec/02_kospi_index_daily.md)
- [03. KOSDAQ 시리즈 일별시세정보](./KRX_API_Spec/03_kosdaq_index_daily.md)
- [04. 채권지수 시세정보](./KRX_API_Spec/04_bond_index_daily.md)
- [05. 파생상품지수 시세정보](./KRX_API_Spec/05_derivative_index_daily.md)

## 2. 주식 (Stock) APIs

### 일별매매정보
- [06. 유가증권 일별매매정보](./KRX_API_Spec/06_stock_kospi_daily.md)
- [07. 코스닥 일별매매정보](./KRX_API_Spec/07_stock_kosdaq_daily.md)
- [08. 코넥스 일별매매정보](./KRX_API_Spec/08_stock_konex_daily.md)
- [09. 신주인수권증권 일별매매정보](./KRX_API_Spec/09_stock_warrant_daily.md)
- [10. 신주인수권증서 일별매매정보](./KRX_API_Spec/10_stock_right_daily.md)

### 종목기본정보
- [11. 유가증권 종목기본정보](./KRX_API_Spec/11_stock_kospi_info.md)
- [12. 코스닥 종목기본정보](./KRX_API_Spec/12_stock_kosdaq_info.md)
- [13. 코넥스 종목기본정보](./KRX_API_Spec/13_stock_konex_info.md)

## 3. ETP (Exchange Traded Products) APIs

- [14. ETF 일별매매정보](./KRX_API_Spec/14_etf_daily.md)
- [15. ETN 일별매매정보](./KRX_API_Spec/15_etn_daily.md)
- [16. ELW 일별매매정보](./KRX_API_Spec/16_elw_daily.md)

## 4. 채권 (Bond) APIs

- [17. 국채전문유통시장 일별매매정보](./KRX_API_Spec/17_bond_government_daily.md)
- [18. 일반채권시장 일별매매정보](./KRX_API_Spec/18_bond_general_daily.md)
- [19. 소액채권시장 일별매매정보](./KRX_API_Spec/19_bond_small_daily.md)

## 5. 파생상품 (Derivatives) APIs

### 선물
- [20. 선물 일별매매정보 (주식선물 제외)](./KRX_API_Spec/20_futures_general_daily.md)
- [21. 주식선물(유가) 일별매매정보](./KRX_API_Spec/21_futures_kospi_stock_daily.md)
- [22. 주식선물(코스닥) 일별매매정보](./KRX_API_Spec/22_futures_kosdaq_stock_daily.md)

### 옵션
- [23. 옵션 일별매매정보 (주식옵션 제외)](./KRX_API_Spec/23_options_general_daily.md)
- [24. 주식옵션(유가) 일별매매정보](./KRX_API_Spec/24_options_kospi_stock_daily.md)
- [25. 주식옵션(코스닥) 일별매매정보](./KRX_API_Spec/25_options_kosdaq_stock_daily.md)

## 6. 일반상품 (General Commodities) APIs

- [26. 석유시장 일별매매정보](./KRX_API_Spec/26_oil_daily.md)
- [27. 금시장 일별매매정보](./KRX_API_Spec/27_gold_daily.md)
- [28. 배출권 시장 일별매매정보](./KRX_API_Spec/28_ets_daily.md)

## 7. ESG APIs

- [29. 사회책임투자채권 정보](./KRX_API_Spec/29_sri_bond_info.md)

---

## 📌 API 사용 안내

### 기본 정보
- **Base URL**: `http://data-dbg.krx.co.kr/svc/apis/`
- **Method**: GET
- **Content-Type**: application/json

### 공통 요청 파라미터
- `basDd`: 기준일자 (YYYYMMDD 형식)

### 예시
```bash
curl -X GET "http://data-dbg.krx.co.kr/svc/apis/idx/krx_dd_trd?basDd=20240105" \
     -H "Content-Type: application/json" \
     -H "AUTH_KEY: YOUR_AUTH_KEY"
```

### 참고사항
- 모든 응답 필드는 string 타입으로 반환됩니다
- 금액 단위는 원(KRW)입니다
- 거래량 단위는 주식/증권 수입니다
- 등락률은 백분율(%)로 표시됩니다 