use std::ops::Add;

use serde::{Deserialize, Serialize};
use time::{UtcDateTime, ext::NumericalDuration};

#[derive(Serialize, Deserialize)]
pub struct Schedule {
    pub last: Option<String>,
    pub next: Option<String>,
    pub count: u32,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule {
            last: None,
            next: Some(UtcDateTime::now().add(1.days()).to_string()),
            count: 0,
        }
    }

    pub fn _increment(self) -> Self {
        // TODO: add max count
        let count = self.count + 1;
        let last = Some(UtcDateTime::now().to_string());
        let next = Some(
            UtcDateTime::now()
                .add(i64::pow(2, count).days())
                .to_string(),
        );

        Self { last, next, count }
    }
}
