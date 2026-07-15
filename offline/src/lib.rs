pub mod cli;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CurrentHeightDifficultyTimestamp {
    pub height: u64,
    pub difficulty: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimestampIntervals {
    pub target_height: u64,
    pub earliest_timestamp: i64,
    pub estimate_timestamp: i64,
    pub latest_timestamp: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HeightIntervals {
    pub target_timestamp: i64,
    pub earliest_height: u64,
    pub estimate_height: u64,
    pub latest_height: u64,
}

fn expected_seconds_per_block(difficulty: f64) -> f64 {
    let normalized = if difficulty.is_finite() {
        difficulty.max(0.01)
    } else {
        1.0
    };
    (75.0 / normalized).clamp(30.0, 600.0)
}

pub fn timestamp_intervals_for_height(
    current: CurrentHeightDifficultyTimestamp,
    target_height: u64,
) -> TimestampIntervals {
    let delta_blocks = target_height as i128 - current.height as i128;
    let per_block = expected_seconds_per_block(current.difficulty);
    let estimate_delta = (delta_blocks as f64 * per_block).round() as i64;
    let uncertainty = ((delta_blocks.unsigned_abs() as f64) * per_block * 0.2).round() as i64;
    let estimate_timestamp = current.timestamp.saturating_add(estimate_delta);

    TimestampIntervals {
        target_height,
        earliest_timestamp: estimate_timestamp.saturating_sub(uncertainty),
        estimate_timestamp,
        latest_timestamp: estimate_timestamp.saturating_add(uncertainty),
    }
}

pub fn height_intervals_for_timestamp(
    current: CurrentHeightDifficultyTimestamp,
    target_timestamp: i64,
) -> HeightIntervals {
    let delta_seconds = target_timestamp - current.timestamp;
    let per_block = expected_seconds_per_block(current.difficulty);
    let estimate_delta_blocks = (delta_seconds as f64 / per_block).round() as i128;
    let uncertainty_blocks =
        ((delta_seconds.unsigned_abs() as f64 / per_block) * 0.2).round() as i128;

    let estimate_height = (current.height as i128 + estimate_delta_blocks).max(0) as u64;
    let earliest_height = (estimate_height as i128 - uncertainty_blocks).max(0) as u64;
    let latest_height = (estimate_height as i128 + uncertainty_blocks).max(0) as u64;

    HeightIntervals {
        target_timestamp,
        earliest_height,
        estimate_height,
        latest_height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_current() -> CurrentHeightDifficultyTimestamp {
        CurrentHeightDifficultyTimestamp {
            height: 1_000,
            difficulty: 1.5,
            timestamp: 1_700_000_000,
        }
    }

    #[test]
    fn forward_height_conversion_is_deterministic() {
        let out = timestamp_intervals_for_height(sample_current(), 1_100);
        assert_eq!(out.target_height, 1_100);
        assert_eq!(out.estimate_timestamp, 1_700_005_000);
        assert!(out.earliest_timestamp <= out.estimate_timestamp);
        assert!(out.estimate_timestamp <= out.latest_timestamp);
    }

    #[test]
    fn backward_timestamp_conversion_is_deterministic() {
        let out = height_intervals_for_timestamp(sample_current(), 1_699_997_000);
        assert_eq!(out.target_timestamp, 1_699_997_000);
        assert_eq!(out.estimate_height, 940);
        assert!(out.earliest_height <= out.estimate_height);
        assert!(out.estimate_height <= out.latest_height);
    }
}
