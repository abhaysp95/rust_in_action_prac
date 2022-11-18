fn main() {
    let fruit = vec!["peach", "banana", "pineapple"];

    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, "watermelon");
}
