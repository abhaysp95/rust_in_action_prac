struct Hostname(String);  // new type

fn connect(host: Hostname) {
    println!("conntected to {}", host.0);
}

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string);

    // connect(ordinary_string);
    connect(host);
}
