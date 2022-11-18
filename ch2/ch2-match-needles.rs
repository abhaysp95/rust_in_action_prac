fn main() {
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 123, 132, 429, 1348, 989324];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if "hit!" == result {
            println!("{}: {}", item, result);
        }
    }
}
