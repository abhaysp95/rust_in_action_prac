fn main() {
    let a: f32 = 42.42;
    let franketype: u32 = unsafe {
        std::mem::transmute(a)
    };

    println!("{}", franketype);
    println!("{:032b}", franketype);

    let b: f32 = unsafe {
        std::mem::transmute(franketype)
    };

    println!("{}", b);
    assert_eq!(a, b);
}
