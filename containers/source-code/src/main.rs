use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Get current system time
    let now = SystemTime::now();

    // Seconds since UNIX epoch
    let timestamp = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Convert to human-readable date/time
    let (year, month, day, hour, minute, second) = unix_to_ymdhms(timestamp);

    println!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}: Hello World",
        year, month, day, hour, minute, second
    );
}

// Convert UNIX timestamp (seconds since epoch) to YYYY-MM-DD HH:MM:SS
fn unix_to_ymdhms(mut secs: u64) -> (u64, u64, u64, u64, u64, u64) {
    let sec = secs % 60;
    secs /= 60;
    let min = secs % 60;
    secs /= 60;
    let hour = secs % 24;
    let mut days = secs / 24;

    let mut year = 1970;
    while days >= days_in_year(year) {
        days -= days_in_year(year);
        year += 1;
    }

    let mut month = 1;
    while days >= days_in_month(year, month) {
        days -= days_in_month(year, month);
        month += 1;
    }

    let day = days + 1;

    (year, month, day, hour, min, sec)
}

// Days in a year (leap year aware)
fn days_in_year(year: u64) -> u64 {
    if is_leap_year(year) { 366 } else { 365 }
}

// Days in each month
fn days_in_month(year: u64, month: u64) -> u64 {
    match month {
        1 => 31,
        2 => if is_leap_year(year) { 29 } else { 28 },
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 30, // fallback, should not happen
    }
}

// Leap year calculation
fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}