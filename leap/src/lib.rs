pub fn is_leap_year(year: i64) -> bool {
    return (year % 4 == 0)
        && (!(year % 100 == 0) || (year % 400 == 0));
}
