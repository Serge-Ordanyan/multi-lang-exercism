use time::{Duration, PrimitiveDateTime as DateTime};

/// Returns a DateTime one gigasecond (1_000_000_000 seconds) after `start`.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1_000_000_000)
}
