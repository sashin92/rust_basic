pub fn mut_ex() {
    let mut a = 1;
    let mut b = 2;
    let mut t;
    let mut index = 5;

    loop {
        t = a + b;
        a = b;
        b = t;

        index -= 1;
        if index <= 0 {
            break;
        }
    }
    println!("{}", b);
}