use chrono::{NaiveDate};

fn weeks_between(a: &str, b: &str) -> i32 {
    let start = NaiveDate::parse_from_str(a, "%Y-%m-%d")
        .expect("Not valid format aor date");

    let end = NaiveDate::parse_from_str(b, "%Y-%m-%d")
        .expect("Not valid format aor date");

    (end - start).num_weeks() as i32
}

fn main() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");

    println!("hello: {}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}
