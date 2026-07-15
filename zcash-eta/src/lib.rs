pub use zcash_eta_headless as headless;
pub use zcash_eta_offline as offline;
pub use zcash_eta_web_server as web_server;

#[cfg(test)]
mod tests {
    use crate::offline::{CurrentHeightDifficultyTimestamp, timestamp_intervals_for_height};

    #[test]
    fn re_exports_are_usable() {
        let current = CurrentHeightDifficultyTimestamp {
            height: 1,
            difficulty: 1.0,
            timestamp: 1,
        };
        let out = timestamp_intervals_for_height(current, 2);
        assert_eq!(out.target_height, 2);
    }
}
