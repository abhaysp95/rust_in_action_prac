#[allow(arithmetic_overflow)]

fn main() {
    let (a, b) = (200, 200);
    let c: u8 = a + b;  // explicitliy mention type, to tell you are creating impossible
                        // situtation
    println!("{}", c)
}
