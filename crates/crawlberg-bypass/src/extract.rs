//! Cost extraction helpers.
//!
//! Extracts the per-request cost from a vendor response using the strategy
//! declared in `ResponseShape::cost_extraction`.

use crate::config::{CostCurrency, CostExtraction};

/// Extract the per-request cost from a vendor response.
///
/// Returns:
/// - `Some(usd)` when cost was successfully extracted and converted to USD.
/// - `None` when extraction is configured as `None`, or when a dynamic
///   extraction fails (e.g. header absent, parse error) and no fallback is set.
///
/// The `fallback_cost_usd` is returned unchanged for `Static` extraction and
/// as a fallback when dynamic extraction fails.
pub fn cost(
    response_headers: &reqwest::header::HeaderMap,
    response_body: &str,
    extraction: &CostExtraction,
    fallback_cost_usd: Option<f64>,
) -> Option<f64> {
    match extraction {
        CostExtraction::None => None,
        CostExtraction::Static => fallback_cost_usd,
        CostExtraction::Header { name, currency } => {
            let value = response_headers.get(name.as_str())?.to_str().ok()?;
            let raw: f64 = value.parse().ok()?;
            Some(apply_currency(raw, currency))
        }
        CostExtraction::JsonField { field } => {
            let v: serde_json::Value = serde_json::from_str(response_body).ok()?;
            let raw = v.get(field.as_str())?.as_f64()?;
            Some(raw)
        }
    }
}

fn apply_currency(raw: f64, currency: &CostCurrency) -> f64 {
    match currency {
        CostCurrency::Usd => raw,
        CostCurrency::Credits { conversion_rate_to_usd } => raw * conversion_rate_to_usd,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_extraction_returns_none() {
        let headers = reqwest::header::HeaderMap::new();
        let result = cost(&headers, "", &CostExtraction::None, Some(0.001));
        assert!(result.is_none());
    }

    #[test]
    fn static_extraction_returns_fallback() {
        let headers = reqwest::header::HeaderMap::new();
        let result = cost(&headers, "", &CostExtraction::Static, Some(0.003));
        assert_eq!(result, Some(0.003));
    }

    #[test]
    fn static_extraction_returns_none_when_no_fallback() {
        let headers = reqwest::header::HeaderMap::new();
        let result = cost(&headers, "", &CostExtraction::Static, None);
        assert!(result.is_none());
    }

    #[test]
    fn header_extraction_parses_usd_value() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-cost", "0.005".parse().unwrap());
        let extraction = CostExtraction::Header {
            name: "x-cost".into(),
            currency: CostCurrency::Usd,
        };
        let result = cost(&headers, "", &extraction, None);
        assert_eq!(result, Some(0.005));
    }

    #[test]
    fn header_extraction_converts_credits() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-credits", "10".parse().unwrap());
        let extraction = CostExtraction::Header {
            name: "x-credits".into(),
            currency: CostCurrency::Credits {
                conversion_rate_to_usd: 0.001,
            },
        };
        let result = cost(&headers, "", &extraction, None);
        assert!((result.unwrap() - 0.01).abs() < 1e-10);
    }

    #[test]
    fn header_extraction_returns_none_when_header_absent() {
        let headers = reqwest::header::HeaderMap::new();
        let extraction = CostExtraction::Header {
            name: "x-missing".into(),
            currency: CostCurrency::Usd,
        };
        let result = cost(&headers, "", &extraction, Some(0.002));
        assert!(result.is_none());
    }

    #[test]
    fn json_field_extraction_finds_top_level_field() {
        let headers = reqwest::header::HeaderMap::new();
        let body = r#"{"cost": 0.007, "other": "value"}"#;
        let extraction = CostExtraction::JsonField { field: "cost".into() };
        let result = cost(&headers, body, &extraction, None);
        assert_eq!(result, Some(0.007));
    }

    #[test]
    fn json_field_extraction_returns_none_for_missing_field() {
        let headers = reqwest::header::HeaderMap::new();
        let body = r#"{"other": "value"}"#;
        let extraction = CostExtraction::JsonField { field: "cost".into() };
        let result = cost(&headers, body, &extraction, None);
        assert!(result.is_none());
    }
}
