#!/usr/bin/env python3
"""
Project Euler Problem 19: Counting Sundays

How many Sundays fell on the first of the month during the twentieth century
(1 Jan 1901 to 31 Dec 2000)?
"""

# Day of week constants (0 = Sunday, 1 = Monday, ..., 6 = Saturday)
SUNDAY = 0
MONDAY = 1


def is_leap_year(year: int) -> bool:
    """Determine if a year is a leap year per Gregorian calendar rules."""
    return (year % 4 == 0 and year % 100 != 0) or (year % 400 == 0)


def days_in_month(year: int, month: int) -> int:
    """Return the number of days in a given month of a given year."""
    base_days = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    days = base_days[month]
    if month == 2 and is_leap_year(year):
        days += 1
    return days


def main():
    # January 1, 1900 was a Monday
    start_dow = MONDAY
    
    # Calculate days from Jan 1, 1900 to Jan 1, 1901
    # 1900 is not a leap year, so exactly 365 days
    # 365 % 7 = 1, so Jan 1, 1901 is Tuesday (Monday + 1 day)
    days_to_1901 = 365
    dow_1901 = (start_dow + days_to_1901) % 7
    
    # Count Sundays on the first of each month from 1901-2000
    sunday_count = 0
    current_dow = dow_1901
    
    for year in range(1901, 2001):
        for month in range(1, 13):
            if current_dow == SUNDAY:
                sunday_count += 1
            current_dow = (current_dow + days_in_month(year, month)) % 7
    
    print(sunday_count)


if __name__ == "__main__":
    main()
