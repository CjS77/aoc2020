
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1)
        .map(|s| s.as_str())
        .unwrap_or("None");
    let result = match problem {
        "day1a" => "Foo",
        _ => "We haven't solved that yet"
    };
    println!("{}", result);
}
