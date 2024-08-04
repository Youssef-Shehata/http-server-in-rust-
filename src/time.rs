use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_time() -> String {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let secs = since_epoch.as_secs();
    let _nanos = since_epoch.subsec_nanos();

    // Calculate the current date and time in UTC
    let _leap_years = (1970..2023).filter(|&y| is_leap_year(y)).count();
    let days_since_epoch = secs / 86400;
    let current_year = 1970 + days_since_epoch / 365;
    let current_month = 1; // Placeholder, needs more calculation
    let current_day = 1; // Placeholder, needs more calculation
    let current_hour = (secs % 86400) / 3600;
    let current_minute = (secs % 3600) / 60;
    let current_second = secs % 60;

    // Format the date and time
    let formatted_date = format!(
        "{}, {:02} {} {} {:02}:{:02}:{:02} GMT",
        get_weekday(days_since_epoch % 7),
        current_day,
        get_month_name(current_month),
        current_year,
        current_hour,
        current_minute,
        current_second
    );

    return formatted_date;
}

fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

fn get_weekday(day_of_week: u64) -> &'static str {
    match day_of_week {
        0 => "Thu",
        1 => "Fri",
        2 => "Sat",
        3 => "Sun",
        4 => "Mon",
        5 => "Tue",
        6 => "Wed",
        _ => unreachable!(),
    }
}

fn get_month_name(month: u64) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => unreachable!(),
    }
}
