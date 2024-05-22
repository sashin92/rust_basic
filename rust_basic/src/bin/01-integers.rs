/*
integer: i8, i16, i32, i64, i128, isize(computer cascade)
unsigned: u8, u16, u32, u64, u128, usize(computer cascade)
*/
fn main() {
    let my_number: u8 = 100; // 255
    let my_other_number = 50; // i32
    // let my_other_number: u16 = 50; // error! mismatch types
    println!("{}", my_number + my_other_number); // type inference
}