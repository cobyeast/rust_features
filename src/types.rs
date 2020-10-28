pub fn strs() -> String {
    let i = "test".to_string();
    my_str.remove(1);
    i
}

pub fn types() {
    let a: u8 = 255;
    let b: i16 = -127;
    let c: isize = 123;
    println!(
        "a = {}, b = {}, takes up this many bytes {}",
        a,
        b,
        std::mem::size_of_val(&c) * 8
    );
}
