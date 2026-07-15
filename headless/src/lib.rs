pub mod cli;

use serde_json::Value;
use zcash_eta_offline::CurrentHeightDifficultyTimestamp;

fn field_u64(value: &Value, key: &str) -> Option<u64> {
    value.get(key)?.as_u64()
}

fn field_i64(value: &Value, key: &str) -> Option<i64> {
    value.get(key)?.as_i64()
}

fn field_f64(value: &Value, key: &str) -> Option<f64> {
    value.get(key)?.as_f64()
}

pub fn parse_current_height_difficulty_timestamp(
    value: &Value,
) -> Option<CurrentHeightDifficultyTimestamp> {
    if let (Some(height), Some(difficulty), Some(timestamp)) = (
        field_u64(value, "height"),
        field_f64(value, "difficulty"),
        field_i64(value, "timestamp"),
    ) {
        return Some(CurrentHeightDifficultyTimestamp {
            height,
            difficulty,
            timestamp,
        });
    }

    let result = value.get("result")?;
    if let (Some(height), Some(difficulty), Some(timestamp)) = (
        field_u64(result, "blocks"),
        field_f64(result, "difficulty"),
        field_i64(result, "time"),
    ) {
        return Some(CurrentHeightDifficultyTimestamp {
            height,
            difficulty,
            timestamp,
        });
    }

    None
}

pub async fn fetch_current_height_difficulty_timestamp(
    endpoint: &str,
) -> color_eyre::Result<CurrentHeightDifficultyTimestamp> {
    let value: Value = reqwest::Client::new()
        .get(endpoint)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    parse_current_height_difficulty_timestamp(&value).ok_or_else(|| {
        color_eyre::eyre::eyre!(
            "endpoint response did not include supported height/difficulty/timestamp fields"
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_simple_api_shape() {
        let value = json!({"height": 200, "difficulty": 2.0, "timestamp": 1_700_000_100_i64});
        let sample = parse_current_height_difficulty_timestamp(&value).unwrap();
        assert_eq!(sample.height, 200);
        assert_eq!(sample.difficulty, 2.0);
        assert_eq!(sample.timestamp, 1_700_000_100);
    }

    #[test]
    fn parses_rpc_shape() {
        let value = json!({
            "result": {"blocks": 333, "difficulty": 1.2, "time": 1_700_000_222_i64}
        });
        let sample = parse_current_height_difficulty_timestamp(&value).unwrap();
        assert_eq!(sample.height, 333);
        assert_eq!(sample.difficulty, 1.2);
        assert_eq!(sample.timestamp, 1_700_000_222);
    }
}
