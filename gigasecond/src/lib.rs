use chrono::{DateTime, Utc};
use chrono::offset::TimeZone;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let new_dt = start.timestamp() + 1_000_000_000i64;
    Utc.timestamp(new_dt, 0)
}
