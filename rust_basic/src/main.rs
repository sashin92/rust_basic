mod whatthe;

fn main() {
    whatthe();
}

fn whatthe() {
    whatthe::no01_loop::for_if_else();
    println!("--------------------");
    whatthe::no01_loop::for_match_pattern();
    println!("--------------------");
    whatthe::no01_loop::for_match_if();
    println!("--------------------");
    whatthe::no02_if::if_else();
    println!("--------------------");
    whatthe::no03_mutable::mut_ex();
    println!("--------------------");
    whatthe::no04_as::as_atoi();
    println!("--------------------");
    whatthe::no04_as::int_matcher("52434");
    whatthe::no04_as::int_matcher("abcde");
    println!("--------------------");
    whatthe::no05_expression::express_01(12);
    whatthe::no05_expression::express_01(30);
}
