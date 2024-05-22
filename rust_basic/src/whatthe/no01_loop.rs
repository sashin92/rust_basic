// 조건을 만족시키지 못할 때의 경우도 처리해주어야 에러 없이
// 정상적으로 컴파일 해준다.
// 별로 좋지못한 코드
pub fn for_if_else() {
    for i in 1..16 {
        let rem_three = i % 3;
        let rem_five = i % 5;
        if rem_three == 0 && rem_five == 0 {
            println!("{} - fizzbuzz", i);
        } else if rem_three == 0 {
            println!("{} - fizz", i);
        } else if rem_five == 0 {
            println!("{} - buzz", i);
        } else {}
    }
}

// 패턴을 통한 매칭
pub fn for_match_pattern() {
    for i in 1..16 {
        match (i % 3, i % 5) {
            (0, 0) => println!("{} - fizzbuzz", i),
            (0, _) => println!("{} - fizz", i),
            (_, 0) => println!("{} - buzz", i),
            (_, _) => (),
        }
    }
}

// match if 조건을 통한 매칭
pub fn for_match_if() {
    for i in 1..16 {
        match i {
            n if n % 15 == 0 => println!("{} - fizzbuzz", i),
            n if n % 3 == 0 => println!("{} - fizz", i),
            n if n % 5 == 0 => println!("{} - buzz", i),
            _ => (),
        }
    }
}