/*
match구문에서 {로 시작하고 }로 끝나는 블럭을
하나의 표현식(expression)이라고 한다. if-else도 하나의 표현식이다.
표현식은 값을 반환한다.
*/
pub fn express_01(age: i32) {
    let gen = match age {
        0..=20 => "MZ",
        21..=50 => "X",
        51..=100 => "A",
        _ => "?",
    };
    let var = match (age % 3, age % 5) {
        (0, 0) => 15,
        (0, _) => 3,
        (_, 0) => 5,
        (_, _) => 0,
    };
    println!("age: {}, fizzbuzz: {}", gen, var);
}