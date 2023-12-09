pub fn run() {
    // TODO 1 - fix the sum
    let a: u32 = 32;
    let b: i32 = 32;
    println!("the sum is {}", a + b as u32);

    // TODO 2 - fix the printing
    let t = 'n';
    println!("the next character after {} is {}", t, (t as u8 + 1) as char);
}
