extern crate chrono;

use self::chrono::*;
//    pub fn after_date(start: DateTime<Utc>) -> DateTime<Utc> {
//        start + Duration::seconds(1_000_000_000)
//    }
// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
//    unimplemented!()
    start + Duration::seconds(1_000_000_000)

}

