// Project Euler 19: Count Sundays on first of month, 1901-2000

fn is_leap(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        2 if is_leap(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn main() {
    // Jan 1, 1901 is a Tuesday (dow=2, 0=Sunday)
    let mut dow = 2u32;
    let mut count = 0u32;

    for year in 1901..=2000 {
        for month in 1..=12 {
            if dow == 0 {
                count += 1;
            }
            dow = (dow + days_in_month(year, month)) % 7;
        }
    }

    println!("{count}");
}
