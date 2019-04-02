use chrono::{Datelike, Duration, NaiveDate};

fn main() {
    let start_date = NaiveDate::from_ymd(1901, 1, 1);
    let end_date = NaiveDate::from_ymd(2000, 12, 31);

    /* Initialize with a sunday. */
    let date = start_date + Duration::days(7 - start_date.weekday().num_days_from_sunday() as i64);

    let answer = (0..).map(|weeks| date + Duration::weeks(weeks))
        .take_while(|date| date <= &end_date)
        .filter(|date| date.day() == 1)
        .count();

    println!("Answer: {}", answer);
}