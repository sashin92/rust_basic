use std::num::ParseIntError;

pub fn as_atoi() {
    let mut num = 0;
    let my_str: String = "52432".to_string();
    for c in my_str.chars() {
        num = num * 10;
        num += c as i32 - '0' as i32;
    }
    println!("{}", num);
}

fn parse_int(input: &str) -> Result<i32, ParseIntError> {
    input.parse()
}

pub fn int_matcher(input: &str) {
    let val = parse_int(input);
    match val {
        Ok(value) => {
            println!("int: {}", value);
        }
        Err(_) => {
            println!("fail. val is not integer.");
        }
    }
}