# KRX API Field Domain Specification

This document defines the valid value domains for each field type in the KRX Open API, based on analysis of official sample data files.

## Overview

The KRX API uses specific conventions for representing missing, unavailable, or zero values. **These are intentional design choices, not data errors:**

- **`"-"` (dash)**: Used as a placeholder when data is not available or not applicable
- **`""` (empty string)**: Used for optional fields when no value is present
- **`"0"`**: Represents actual zero values (valid numeric data)

## Field Type Domains

### 1. Price Fields
**Fields**: `TDD_CLSPRC`, `TDD_OPNPRC`, `TDD_HGPRC`, `TDD_LWPRC`, `CLSPRC`, `OPNPRC`, `HGPRC`, `LWPRC`

**Valid Values**:
- Numeric strings with decimals: `"10158.0"`, `"76600"`, `"3650"`
- Dash placeholder: `"-"` (indicates no trading occurred)
- Zero value: `"0"` (valid zero price)

**Examples from sample data**:
```json
"TDD_CLSPRC": "35365"     // Valid price
"TDD_CLSPRC": "-"         // No trading occurred (valid)
"TDD_CLSPRC": "0"         // Zero price (valid)
```

### 2. Change/Comparison Fields
**Fields**: `CMPPREVDD_PRC`, `CMPPREVDD_PRC_ULY`

**Valid Values**:
- Numeric strings (positive/negative): `"60"`, `"-80"`, `"-4.5"`
- Empty string: `""` (no change data available)
- Dash placeholder: `"-"` (no change data available)
- Zero: `"0"` (no change)

**Examples from sample data**:
```json
"CMPPREVDD_PRC": "60"     // Price increased by 60
"CMPPREVDD_PRC": "-80"    // Price decreased by 80
"CMPPREVDD_PRC": ""       // No change data (valid)
"CMPPREVDD_PRC": "-"      // No change data (valid)
"CMPPREVDD_PRC": "0"      // No change (valid)
```

### 3. Volume/Value Fields
**Fields**: `ACC_TRDVAL`, `ACC_TRDVOL`, `LIST_SHRS`, `MKTCAP`

**Valid Values**:
- Numeric strings: `"104622050000"`, `"410628"`, `"15800000"`
- Zero: `"0"` (no trading volume/value)

**Examples from sample data**:
```json
"ACC_TRDVAL": "2137622110"  // Valid trading value
"ACC_TRDVAL": "0"           // No trading (valid)
"ACC_TRDVOL": "410628"      // Valid trading volume
"ACC_TRDVOL": "0"           // No trading (valid)
```

### 4. Percentage/Rate Fields  
**Fields**: `FLUC_RT`, `FLUC_RT_IDX`, yield fields (`*_YD`), volatility fields

**Valid Values**:
- Decimal strings: `"1.15"`, `-2.14"`, `"3.381"`, `"0.00"`
- Zero: `"0.00"` (no change, valid)

**Examples from sample data**:
```json
"FLUC_RT": "1.15"         // 1.15% increase
"FLUC_RT": "-2.14"        // 2.14% decrease  
"FLUC_RT": "0.00"         // No change (valid)
"CLSPRC_YD": "3.381"      // 3.381% yield
```

### 5. Categorical/Text Fields
**Fields**: `SECT_TP_NM`, `MKT_NM`, `GOVBND_ISU_TP_NM`, `BND_EXP_TP_NM`, `RGHT_TP_NM`

**Valid Values**:
- Text strings: `"KOSPI"`, `"국채전문유통시장"`, `"CALL"`, `"지표"`
- Dash placeholder: `"-"` (category not applicable)

**Examples from sample data**:
```json
"SECT_TP_NM": "-"         // Sector not applicable (valid)
"MKT_NM": "KOSPI"         // Valid market name
"RGHT_TP_NM": "CALL"      // Valid option type
"GOVBND_ISU_TP_NM": "지표" // Valid bond issue type
```

### 6. Date Fields
**Fields**: `BAS_DD`

**Valid Values**:
- 8-digit date strings in YYYYMMDD format: `"20240105"`

**Invalid Values**:
- Any format other than YYYYMMDD: `"2024-01-05"`, `"05/01/2024"`
- Invalid dates: `"20240230"`, `"20241301"`

### 7. Code/ID Fields
**Fields**: `ISU_CD`, `ISU_NM`

**Valid Values**:
- Alphanumeric strings: `"095570"`, `"KR103503GD97"`, `"58J003"`
- Names with Korean/English text: `"AJ네트웍스"`, `"ACE 200"`

## Validation Strategy

### What to Validate (Errors)
1. **Date format**: Reject non-YYYYMMDD formats in date fields
2. **JSON structure**: Reject malformed JSON responses
3. **Required fields**: Reject responses missing mandatory fields
4. **Type mismatches**: Reject obviously wrong data types

### What NOT to Validate (Valid Data)
1. **Placeholder values**: `-`, `""` are valid placeholders
2. **Zero values**: `"0"`, `"0.00"` are valid numeric values  
3. **Empty trading data**: Zero volumes/values during non-trading periods
4. **Missing optional data**: Empty strings in optional fields

## Implementation Notes

### Deserializer Behavior
- `deserialize_optional_f64`: Converts `"-"` and `""` to `None`, preserves `"0"` as `Some(0.0)`
- `deserialize_optional_u64`: Converts `"-"` and `""` to `None`, preserves `"0"` as `Some(0)`  
- `deserialize_krx_date`: Strictly validates YYYYMMDD format
- String fields: Preserve `"-"` and `""` as-is (they are meaningful data)

### Error Handling
- Return `Ok(data)` for responses with placeholder values (`"-"`, `""`)
- Return `Err(ValidationError)` only for truly malformed data (invalid dates, non-numeric values in numeric fields)
- Current implementation provides the right balance between accepting valid KRX data patterns and catching genuine errors

### Current Status
The krx-rs library's deserializers correctly implement the field domain requirements:
- ✅ Accept dash (`"-"`) and empty string (`""`) as valid placeholders  
- ✅ Preserve zero values (`"0"`, `"0.00"`) as meaningful data
- ✅ Strictly validate date formats (YYYYMMDD only)
- ✅ Handle comma-separated numbers (when they occur)
- ✅ Convert placeholders to `None` for optional numeric fields
- ✅ Preserve placeholders as-is for string fields (categorical data)

### Future Enhancement Considerations
If stricter validation is needed in the future, consider adding:
- Optional `ValidationMode` enum (Lenient, Strict)
- Configurable field-specific validation rules
- Warning logging for unusual but valid patterns

## Field-by-Field Reference

### Stock Daily Trading (`stk_bydd_trd`)
- `ACC_TRDVAL`: Volume field, allows `"0"`
- `ACC_TRDVOL`: Volume field, allows `"0"`
- `CMPPREVDD_PRC`: Change field, allows `"-"`, `""`, `"0"`
- `FLUC_RT`: Rate field, allows `"0.00"`
- `SECT_TP_NM`: Category field, allows `"-"`
- `TDD_*PRC`: Price fields, allow `"-"` when no trading

### Bond Daily Trading (`bnd_bydd_trd`, `kts_bydd_trd`)
- `CMPPREVDD_PRC`: Allows empty string `""` and dash `"-"`
- `*PRC`: Price fields allow `"-"` for no trading
- `*_YD`: Yield fields are numeric decimals

### Options Daily Trading (`opt_bydd_trd`)
- All price fields (`TDD_*PRC`) allow `"-"` when no trading
- `CMPPREVDD_PRC`: Allows `"-"` for no price change data
- `RGHT_TP_NM`: Category field with values like `"CALL"`, `"PUT"`

### ETF/ETN/ELW Daily Trading
- Similar to stock trading patterns
- `ULY_PRC` in ELW may contain formatted numbers: `"76,600"`
- Zero trading volumes are common and valid

This specification ensures that the krx-rs library correctly handles the intentional design patterns of the KRX API while still catching genuine data errors.