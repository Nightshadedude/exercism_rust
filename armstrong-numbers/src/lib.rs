pub fn is_armstrong_number(num: u32) -> bool {
    let str_num = num.to_string();
    str_num.chars().into_iter().map(|c| c.to_digit(10).unwrap().pow(str_num.len() as u32)).sum::<u32>() == num
}
