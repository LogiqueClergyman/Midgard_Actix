use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RunepoolHistory {
    pub id: Option<i32>,       // Auto-incrementing unique ID for each record
    pub startTime: i64,        // The start time as UNIX timestamp (seconds)
    pub endTime: i64,          // The end time as UNIX timestamp (seconds)
    pub units: i64,            // The units value from the API
    pub count: i32,            // The count value from the API
}
