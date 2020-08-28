pub fn is_leap_year(year: u64) -> bool {
    //on every year that is evenly divisible by 4
    //except every year that is evenly divisible by 100
    //unless the year is also evenly divisible by 400
    match year {
        year if year % 400 == 0 => true,
        year if year % 100 == 0 => false,
        year if year %4 == 0 => true,
        _ => false,
    }
}
