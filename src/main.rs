use chrono::{TimeZone, DateTime, Duration};
use chrono::offset::Utc;

struct ImportantEvent {
    what: String,
    when: DateTime<Utc>,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.when.cmp(&Utc::now()).is_lt()
    }
}

fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        when: Utc.with_ymd_and_hms(2020, 12, 25, 0, 0, 0)
            .unwrap(),
    };
    
    if missed_christmas.is_passed() {
        println!("oh well, maybe next year");
    } else {
        println!("☃︎");
    }
}

#[test]
fn in_past() {
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Utc::now() - Duration::hours(25),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Utc::now() + Duration::hours(25),
    };

    assert!(!event.is_passed())
}

